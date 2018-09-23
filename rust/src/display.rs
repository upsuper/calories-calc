use crate::utils::UnwrapAbort;
use std::i64;
use std::fmt;
use std::process;
use std::str;

pub trait Display {
    fn display(&self, output: &mut impl fmt::Write) -> fmt::Result;

    #[inline]
    fn display_string(&self) -> String {
        let mut output = String::new();
        self.display(&mut output).unwrap_abort();
        output
    }
}

pub struct Rounded<T>(pub T);

impl Display for Rounded<f32> {
    fn display(&self, output: &mut impl fmt::Write) -> fmt::Result {
        let num = self.0.round();
        if num > i64::MAX as f32 || num <= i64::MIN as f32 {
            return Err(fmt::Error);
        }
        let mut num = num as i64;
        if num == 0 {
            output.write_char('0')?;
            return Ok(());
        }
        if num < 0 {
            output.write_char('-')?;
            num = -num;
        }
        const BUF_SIZE: usize = 20;
        let mut buf = [0u8; BUF_SIZE];
        for (i, slot) in buf.iter_mut().enumerate().rev() {
            if num == 0 {
                return output.write_str(str::from_utf8(&buf[i + 1..]).unwrap_abort());
            }
            *slot = b'0' + (num % 10) as u8;
            num /= 10;
        }
        // The buffer should be enough to fit all number.
        process::abort();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rounded_float_display() {
        assert_eq!(Rounded(15.5).display_string(), Ok("16".to_string()));
        assert_eq!(Rounded(-21.7).display_string(), Ok("-22".to_string()));
        assert_eq!(Rounded(1000000000.).display_string(), Ok("1000000000".to_string()));
    }
}
