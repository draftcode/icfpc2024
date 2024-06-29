use anyhow::{anyhow, bail, ensure, Result};

pub fn decode_base94(c: char) -> Result<i64> {
    if ('!'..='~').contains(&c) {
        let n = c as i64 - '!' as i64;
        Ok(n)
    } else {
        bail!("invalid base94 char: {c}")
    }
}

pub fn encode_base94(n: i64) -> Result<char> {
    if n < 0 || n >= 94 {
        bail!("invalid base94 number: {n}")
    }
    Ok((n + '!' as i64) as u8 as char)
}

const TBL: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!\"#$%&'()*+,-./:;<=>?@[\\]^_`|~ \n";

pub fn decode_char(c: char) -> Result<char> {
    Ok(TBL[decode_base94(c)? as usize] as char)
}

pub fn encode_char(c: char) -> Result<char> {
    let ix = TBL
        .iter()
        .position(|&x| x == c as u8)
        .ok_or_else(|| anyhow!("invalid char"))?;
    encode_base94(ix as i64)
}

pub fn decode_str(s: &str) -> Result<String> {
    s.chars().map(decode_char).collect()
}

pub fn encode_str(s: &str) -> Result<String> {
    s.chars().map(encode_char).collect()
}

pub fn decode_base94_int(s: &str) -> Result<i64> {
    let mut ret = 0;
    for c in s.chars() {
        ret = ret * 94 + decode_base94(c)?;
    }
    Ok(ret)
}

pub fn encode_base94_int(n: i64) -> Result<String> {
    ensure!(n >= 0);
    if n == 0 {
        return Ok("!".to_string());
    }
    let mut s = String::new();
    let mut n = n;
    while n > 0 {
        s.push(encode_base94(n % 94)?);
        n /= 94;
    }
    Ok(s.chars().rev().collect::<String>())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_decode_int() {
        assert_eq!(decode_base94_int("/6").unwrap(), 1337);
        assert_eq!(encode_base94_int(1337).unwrap(), "/6");
    }
}
