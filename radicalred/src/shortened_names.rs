pub fn expand_name(rom_name: &[u8]) -> Option<&str> {
    let hash = name_hash(rom_name);
    SHORTENED_NAMES
        .iter()
        .find_map(|(h, name)| if *h == hash { Some(*name) } else { None })
}

// From https://stackoverflow.com/a/2351171
const fn name_hash(s: &[u8]) -> u32 {
    let mut h = 0u32;
    const MULT: u32 = 37;
    h = MULT.wrapping_sub(h).wrapping_add(s[0] as u32);
    h = MULT.wrapping_sub(h).wrapping_add(s[1] as u32);
    h = MULT.wrapping_sub(h).wrapping_add(s[2] as u32);
    h = MULT.wrapping_sub(h).wrapping_add(s[3] as u32);
    h = MULT.wrapping_sub(h).wrapping_add(s[4] as u32);
    h = MULT.wrapping_sub(h).wrapping_add(s[5] as u32);
    h = MULT.wrapping_sub(h).wrapping_add(s[6] as u32);
    h = MULT.wrapping_sub(h).wrapping_add(s[7] as u32);
    h = MULT.wrapping_sub(h).wrapping_add(s[8] as u32);
    h = MULT.wrapping_sub(h).wrapping_add(s[9] as u32);
    h
}

static SHORTENED_NAMES: &[(u32, &str)] = &[
    (name_hash(b"Fletchindr"), "Fletchinder"),
    (name_hash(b"Crabminble"), "Crabominable"),
    (name_hash(b"Baraskewda"), "Barraskewda"),
    (name_hash(b"Centskorch"), "Centiskorch"),
    (name_hash(b"Corvsquire"), "Corvisquire"),
    (name_hash(b"Corvknight"), "Corviknight"),
    (name_hash(b"Stonjorner"), "Stonjourner"),
    (name_hash(b"Poltegeist"), "Polteageist"),
    (name_hash(b"Blacphalon"), "Blacephalon"),
];
