extern crate num;
use num::{ToPrimitive, Unsigned};
fn int_to_2bpp<T: Unsigned + ToPrimitive>(num: T) -> Result<(u8, u8), &'static str> {
    let match_num = match num.to_u8() {
        Some(n) => n,
        None => return Err("Couldn't cast integer."),
    };
    match match_num {
        0 => Ok((0, 0)),
        1 => Ok((128, 0)),
        2 => Ok((0, 128)),
        3 => Ok((128, 128)),
        _ => Err("Values must be between 0 and 3"),
    }
}

/// Converts an array of unsigned integers and converts them to 2bpp format.
/// Expects a 64 element 1D array packed as [x*8+y]
/// 2bpp pixels only support the values 0 through 3.
pub fn twobpp<T: Unsigned + ToPrimitive + Copy + std::fmt::Debug>(pixels: Vec<T>) -> Result<Vec<u8>, &'static str> {
    if pixels.len() != 64 {
        return Err("Pixel array must be 64 pixels long.");
    }
    let mut output_bytes: Vec<u8> = Vec::new();
    for x in 0..8 {
        let mut big: u8 = 0;
        let mut small: u8 = 0;
        for y in (0..8).rev() {
            let ints = match int_to_2bpp(pixels[x * 8 + y]) {
                Ok(nums) => nums,
                Err(e) => return Err(e),
            };
            small = small >> 1;
            small = small | ints.0;
            big = big >> 1;
            big = big | ints.1;
        }
        output_bytes.push(small);
        output_bytes.push(big);
    }
    Ok(output_bytes)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_converts_arrays() {
        let test_nums: Vec<u8> = vec![
            0,3,3,3,3,3,0,0,
            2,2,0,0,0,2,2,0,
            1,1,0,0,0,1,1,0,
            2,2,2,2,2,2,2,0,
            3,3,0,0,0,3,3,0,
            2,2,0,0,0,2,2,0,
            1,1,0,0,0,1,1,0,
            0,0,0,0,0,0,0,0,
        ]; // Straight outta the manual.
        let expected: Vec<u8> = vec![0x7c, 0x7c, 0x00, 0xc6,0xc6, 0x00, 0x00, 0xfe, 0xc6, 0xc6, 0x00, 0xc6, 0xc6, 0x00, 0x00, 0x00];
        assert_eq!(expected, twobpp(test_nums).unwrap());
    }
    #[test]
    fn it_checks_length() {
        let too_many_nums: Vec<u8> = vec![1; 65];
        let just_enough_nums: Vec<u8> = vec![1; 64];
        match twobpp(too_many_nums) {
            Err(_) => (),
            Ok(_) => panic!(),
        };
        match twobpp(just_enough_nums) {
            Err(err) => panic!(err),
            Ok(_) => (),
        };
    }
    #[test]
    fn it_converts_ints() {
        let expected: Vec<(u8, u8)> = vec![(0, 0), (128, 0), (0, 128), (128, 128)];
        for n in 0..=3 as usize {
            let res = match int_to_2bpp(n) {
                Ok((a, b)) => (a, b),
                Err(err) => panic!(err),
            };
            assert_eq!(expected[n], res);
        }
    }
}
