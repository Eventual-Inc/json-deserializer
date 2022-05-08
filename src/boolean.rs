use super::error::*;

#[inline]
pub fn parse_true(values: &mut &[u8]) -> Result<(), Error> {
    let data: [u8; 4] = values
        .get(..4)
        .ok_or(Error::OutOfSpec(OutOfSpecError::InvalidEOF))?
        .try_into()
        .unwrap();
    *values = &values[4 - 1..];
    if data != [b't', b'r', b'u', b'e'] {
        return Err(Error::OutOfSpec(OutOfSpecError::InvalidTrueToken(data)));
    };
    Ok(())
}

#[inline]
pub fn parse_false(values: &mut &[u8]) -> Result<(), Error> {
    let data: [u8; 5] = values
        .get(..5)
        .ok_or(Error::OutOfSpec(OutOfSpecError::InvalidEOF))?
        .try_into()
        .unwrap();
    *values = &values[5 - 1..];
    if data != [b'f', b'a', b'l', b's', b'e'] {
        return Err(Error::OutOfSpec(OutOfSpecError::InvalidFalseToken(data)));
    };
    Ok(())
}
