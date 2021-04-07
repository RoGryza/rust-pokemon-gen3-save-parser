#[derive(Copy, Clone, Debug)]
pub enum Encoding {
    Char(char),
    Invalid,
    End,
}

pub const CHARMAP: [Encoding; 256] = [
    Encoding::Char(' '),  //00=
    Encoding::Char('À'), //01=À
    Encoding::Char('Á'), //02=Á
    Encoding::Char('Â'), //03=Â
    Encoding::Char('Ç'), //04=Ç
    Encoding::Char('È'), //05=È
    Encoding::Char('É'), //06=É
    Encoding::Char('Ê'), //07=Ê
    Encoding::Char('Ë'), //08=Ë
    Encoding::Char('Ì'), //09=Ì
    Encoding::Invalid,
    Encoding::Char('Î'), //0B=Î
    Encoding::Char('Ï'), //0C=Ï
    Encoding::Char('Ò'), //0D=Ò
    Encoding::Char('Ó'), //0E=Ó
    Encoding::Char('Ô'), //0F=Ô
    Encoding::Char('Œ'), //10=Œ
    Encoding::Char('Ù'), //11=Ù
    Encoding::Char('Ú'), //12=Ú
    Encoding::Char('Û'), //13=Û
    Encoding::Char('Ñ'), //14=Ñ
    Encoding::Char('ß'), //15=ß
    Encoding::Char('à'), //16=à
    Encoding::Char('á'), //17=á
    Encoding::Invalid,
    Encoding::Char('ç'), //19=ç
    Encoding::Char('è'), //1A=è
    Encoding::Char('e'),  //1B=\e
    Encoding::Char('ê'), //1C=ê
    Encoding::Char('ë'), //1D=ë
    Encoding::Char('ì'), //1E=ì
    Encoding::Invalid,
    Encoding::Char('î'), //20=î
    Encoding::Char('ï'), //21=ï
    Encoding::Char('ò'), //22=ò
    Encoding::Char('ó'), //23=ó
    Encoding::Char('ô'), //24=ô
    Encoding::Char('œ'), //25=œ
    Encoding::Char('ù'), //26=ù
    Encoding::Char('ú'), //27=ú
    Encoding::Char('û'), //28=û
    Encoding::Char('ñ'), //29=ñ
    Encoding::Char('º'), //2A=º
    Encoding::Char('ª'), //2B=ª
    Encoding::Invalid,
    Encoding::Char('&'), //2D=&
    Encoding::Char('+'), //2E=+
    Encoding::Invalid,
    Encoding::Invalid,
    Encoding::Invalid,
    Encoding::Invalid,
    Encoding::Invalid,
    Encoding::Invalid,
    Encoding::Char('='), //35==
    Encoding::Char(';'), //36=;
    Encoding::Invalid,
    Encoding::Invalid,
    Encoding::Invalid,
    Encoding::Invalid,
    Encoding::Invalid,
    Encoding::Invalid,
    Encoding::Invalid,
    Encoding::Invalid,
    Encoding::Invalid,
    Encoding::Invalid,
    Encoding::Invalid,
    Encoding::Invalid,
    Encoding::Invalid,
    Encoding::Invalid,
    Encoding::Invalid,
    Encoding::Invalid,
    Encoding::Invalid,
    Encoding::Invalid,
    Encoding::Invalid,
    Encoding::Invalid,
    Encoding::Invalid,
    Encoding::Invalid,
    Encoding::Invalid,
    Encoding::Invalid,
    Encoding::Invalid,
    Encoding::Invalid,
    Encoding::Char('¿'), //51=¿
    Encoding::Char('¡'), //52=¡
    Encoding::Invalid,
    Encoding::Invalid,
    Encoding::Invalid,
    Encoding::Invalid,
    Encoding::Invalid,
    Encoding::Invalid,
    Encoding::Invalid,
    Encoding::Char('Í'), //5A=Í
    Encoding::Char('%'),  //5B=%
    Encoding::Char('('),  //5C=(
    Encoding::Char(')'),  //5D=)
    Encoding::Invalid,
    Encoding::Invalid,
    Encoding::Invalid,
    Encoding::Invalid,
    Encoding::Invalid,
    Encoding::Invalid,
    Encoding::Invalid,
    Encoding::Invalid,
    Encoding::Invalid,
    Encoding::Invalid,
    Encoding::Char('â'), //68=â
    Encoding::Invalid,
    Encoding::Invalid,
    Encoding::Invalid,
    Encoding::Invalid,
    Encoding::Invalid,
    Encoding::Invalid,
    Encoding::Char('í'), //6F=í
    Encoding::Invalid,
    Encoding::Invalid,
    Encoding::Invalid,
    Encoding::Invalid,
    Encoding::Invalid,
    Encoding::Invalid,
    Encoding::Invalid,
    Encoding::Invalid,
    Encoding::Invalid,
    Encoding::Invalid,
    Encoding::Invalid,
    Encoding::Invalid,
    Encoding::Invalid,
    Encoding::Invalid,
    Encoding::Invalid,
    Encoding::Invalid,
    Encoding::Invalid,
    Encoding::Invalid,
    Encoding::Invalid,
    Encoding::Invalid,
    Encoding::Invalid,
    Encoding::Char('<'), //85=<
    Encoding::Char('>'), //86=>
    Encoding::Invalid,
    Encoding::Invalid,
    Encoding::Invalid,
    Encoding::Invalid,
    Encoding::Invalid,
    Encoding::Invalid,
    Encoding::Invalid,
    Encoding::Invalid,
    Encoding::Invalid,
    Encoding::Invalid,
    Encoding::Invalid,
    Encoding::Invalid,
    Encoding::Invalid,
    Encoding::Invalid,
    Encoding::Invalid,
    Encoding::Invalid,
    Encoding::Invalid,
    Encoding::Invalid,
    Encoding::Invalid,
    Encoding::Invalid,
    Encoding::Invalid,
    Encoding::Invalid,
    Encoding::Invalid,
    Encoding::Invalid,
    Encoding::Invalid,
    Encoding::Invalid,
    Encoding::Char('0'),   //A1=0
    Encoding::Char('1'),   //A2=1
    Encoding::Char('2'),   //A3=2
    Encoding::Char('3'),   //A4=3
    Encoding::Char('4'),   //A5=4
    Encoding::Char('5'),   //A6=5
    Encoding::Char('6'),   //A7=6
    Encoding::Char('7'),   //A8=7
    Encoding::Char('8'),   //A9=8
    Encoding::Char('9'),   //AA=9
    Encoding::Char('!'),   //AB=!
    Encoding::Char('?'),   //AC=?
    Encoding::Char('.'),   //AD=.
    Encoding::Char('-'),   //AE=-
    Encoding::Char('·'),  //AF=·
    Encoding::Char('…'), //B0=…
    Encoding::Char('“'), //B1=“
    Encoding::Char('”'), //B2=”
    Encoding::Char('‘'), //B3=‘
    Encoding::Char('\''),  //B4='
    Encoding::Char('♂'), //B5=♂
    Encoding::Char('♀'), //B6=♀
    Encoding::Char('$'),   //B7=\$
    Encoding::Char(','),   //B8=,
    Encoding::Char('*'),   //B9=*
    Encoding::Char('/'),   //BA=/
    Encoding::Char('A'),   //BB=A
    Encoding::Char('B'),   //BC=B
    Encoding::Char('C'),   //BD=C
    Encoding::Char('D'),   //BE=D
    Encoding::Char('E'),   //BF=E
    Encoding::Char('F'),   //C0=F
    Encoding::Char('G'),   //C1=G
    Encoding::Char('H'),   //C2=H
    Encoding::Char('I'),   //C3=I
    Encoding::Char('J'),   //C4=J
    Encoding::Char('K'),   //C5=K
    Encoding::Char('L'),   //C6=L
    Encoding::Char('M'),   //C7=M
    Encoding::Char('N'),   //C8=N
    Encoding::Char('O'),   //C9=O
    Encoding::Char('P'),   //CA=P
    Encoding::Char('Q'),   //CB=Q
    Encoding::Char('R'),   //CC=R
    Encoding::Char('S'),   //CD=S
    Encoding::Char('T'),   //CE=T
    Encoding::Char('U'),   //CF=U
    Encoding::Char('V'),   //D0=V
    Encoding::Char('W'),   //D1=W
    Encoding::Char('X'),   //D2=X
    Encoding::Char('Y'),   //D3=Y
    Encoding::Char('Z'),   //D4=Z
    Encoding::Char('a'),   //D5=a
    Encoding::Char('b'),   //D6=b
    Encoding::Char('c'),   //D7=c
    Encoding::Char('d'),   //D8=d
    Encoding::Char('e'),   //D9=e
    Encoding::Char('f'),   //DA=f
    Encoding::Char('g'),   //DB=g
    Encoding::Char('h'),   //DC=h
    Encoding::Char('i'),   //DD=i
    Encoding::Char('j'),   //DE=j
    Encoding::Char('k'),   //DF=k
    Encoding::Char('l'),   //E0=l
    Encoding::Char('m'),   //E1=m
    Encoding::Char('n'),   //E2=n
    Encoding::Char('o'),   //E3=o
    Encoding::Char('p'),   //E4=p
    Encoding::Char('q'),   //E5=q
    Encoding::Char('r'),   //E6=r
    Encoding::Char('s'),   //E7=s
    Encoding::Char('t'),   //E8=t
    Encoding::Char('u'),   //E9=u
    Encoding::Char('v'),   //EA=v
    Encoding::Char('w'),   //EB=w
    Encoding::Char('x'),   //EC=x
    Encoding::Char('y'),   //ED=y
    Encoding::Char('z'),   //EE=z
    Encoding::Char('▶'), //EF=▶
    Encoding::Char(':'),   //F0=:
    Encoding::Char('Ä'),  //F1=Ä
    Encoding::Char('Ö'),  //F2=Ö
    Encoding::Char('Ü'),  //F3=Ü
    Encoding::Char('ä'),  //F4=ä
    Encoding::Char('ö'),  //F5=ö
    Encoding::Char('ü'),  //F6=ü
    Encoding::Invalid,
    Encoding::Invalid,
    Encoding::Invalid,
    Encoding::Invalid, //FA=\l
    Encoding::Invalid, //FB=\p
    Encoding::Invalid,
    Encoding::Invalid,
    Encoding::Char('\n'), //FE=\n
    Encoding::End,        //FF=$
];
