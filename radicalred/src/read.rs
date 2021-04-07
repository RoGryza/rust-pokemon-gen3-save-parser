use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::io::{self, Read, Seek, SeekFrom};
use std::marker::PhantomData;
use std::mem;
use std::slice;

use byteorder::{LittleEndian, ReadBytesExt};

#[derive(Debug)]
pub struct ReadTableError {
    pub table: &'static str,
    pub index: usize,
    pub err: io::Error,
}

pub type ReadTableResult<T> = Result<T, ReadTableError>;

impl ReadTableError {
    pub fn new<T: FromTable>(index: usize, err: io::Error) -> Self {
        ReadTableError {
            table: T::NAME,
            index,
            err,
        }
    }
}

impl Display for ReadTableError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "Error reading table {} at index {}: {}",
            self.table, self.index, self.err
        )
    }
}

impl Error for ReadTableError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.err)
    }
}

pub struct TableReader<'a, T, R> {
    remaining: usize,
    reader: &'a mut R,
    item_type: PhantomData<*const T>,
}

pub trait FromTable: Sized {
    const NAME: &'static str;
    const COUNT: usize;
    const OFFSET: u64;

    fn read<R: Read>(mut reader: R) -> io::Result<Self> {
        unsafe {
            let mut result = mem::zeroed();
            let slice =
                slice::from_raw_parts_mut(&mut result as *mut _ as *mut u8, mem::size_of::<Self>());
            reader.read_exact(slice)?;
            Ok(result)
        }
    }
}

pub struct Named<T, U> {
    pub name: T,
    pub value: U,
}

pub trait RomReadExt: Sized {
    fn read_pointer(&mut self) -> io::Result<u64>;
    fn seek_pointer(&mut self) -> io::Result<()>;
    fn seek_pointer_at(&mut self, address: u64) -> io::Result<()>;
    fn read_table<T: FromTable>(&mut self) -> ReadTableResult<TableReader<T, Self>>;
}

impl<R: Read + Seek> RomReadExt for R {
    fn read_pointer(&mut self) -> io::Result<u64> {
        self.read_u32::<LittleEndian>()
            // 0x08000000 is the offset ROM data is mapped to in the GBA, since we're operating on
            // the ROM memory addresses directly we need to subtract it from the original pointer.
            .map(|raw| (raw as u64) - 0x08000000)
    }

    fn seek_pointer(&mut self) -> io::Result<()> {
        let ptr = self.read_pointer()?;
        self.seek(SeekFrom::Start(ptr))?;
        Ok(())
    }

    fn seek_pointer_at(&mut self, address: u64) -> io::Result<()> {
        self.seek(SeekFrom::Start(address))?;
        self.seek_pointer()
    }

    fn read_table<T: FromTable>(&mut self) -> ReadTableResult<TableReader<T, Self>> {
        self.seek_pointer_at(T::OFFSET)
            .map_err(|e| ReadTableError::new::<T>(0, e))?;
        // Skip first element, as it's always a placeholder for NONE
        self.seek(SeekFrom::Current(mem::size_of::<T>() as i64))
            .map_err(|e| ReadTableError::new::<T>(0, e))?;
        Ok(TableReader::new(self))
    }
}

impl<'a, T: FromTable, R> TableReader<'a, T, R> {
    pub fn new(reader: &'a mut R) -> Self {
        TableReader {
            remaining: T::COUNT - 1, // skip the first NONE
            reader,
            item_type: PhantomData,
        }
    }
}

impl<'a, T, R> Iterator for TableReader<'a, T, R>
where
    R: Read,
    T: FromTable,
{
    type Item = ReadTableResult<T>;

    fn next(&mut self) -> Option<ReadTableResult<T>> {
        if self.remaining == 0 {
            None
        } else {
            self.remaining -= 1;
            Some(
                T::read(&mut self.reader)
                    .map_err(|e| ReadTableError::new::<T>(T::COUNT - self.remaining, e)),
            )
        }
    }
}

impl<T, U> Named<T, U>
where
    T: 'static + FromTable,
    U: 'static + FromTable,
{
    pub fn read_all<'a, R: Read + Seek>(
        reader: &'a mut R,
    ) -> ReadTableResult<impl 'a + Iterator<Item = ReadTableResult<Self>>> {
        let raw_names = reader.read_table()?.collect::<ReadTableResult<Vec<T>>>()?;
        let it = reader
            .read_table::<U>()?
            .zip(raw_names)
            .map(|(value_res, name)| value_res.map(|value| Named { name, value }));
        Ok(it)
    }
}
