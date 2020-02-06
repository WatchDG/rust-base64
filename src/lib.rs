use std::io::Error;

const B64_URL_ENCODE: [u8; 64] = [
    0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50,
    0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66,
    0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76,
    0x77, 0x78, 0x79, 0x7a, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x2d, 0x5f,
];

const B64_URL_DECODE: [u8; 255] = [
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x3e, 0xff, 0xff,
    0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e,
    0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0xff, 0xff, 0xff, 0xff, 0x3f,
    0xff, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28,
    0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
];

const B64_URL_PAD: u8 = 0x3d;

#[allow(dead_code)]
struct B64URL<T>(T);

trait B64URLEncode<I, O, E> {
    fn encode(input: I) -> Result<O, E>;
}

trait B64URLDecode<I, O, E> {
    fn decode(input: I) -> Result<O, E>;
}

impl B64URLEncode<String, String, Error> for B64URL<String> {
    fn encode(input: String) -> Result<String, Error> {
        let bytes = input.into_bytes();
        let length = bytes.len();
        let mut vec = Vec::<u8>::with_capacity(length * 4 / 3);
        let mut index = 0;
        if length >= 3 {
            while index <= (length - 3) {
                let value = (bytes[index] as u32) << 16
                    | (bytes[index + 1] as u32) << 8
                    | (bytes[index + 2] as u32);
                vec.push(B64_URL_ENCODE[((value >> 18) & 0b11_1111) as usize]);
                vec.push(B64_URL_ENCODE[((value >> 12) & 0b11_1111) as usize]);
                vec.push(B64_URL_ENCODE[((value >> 6) & 0b11_1111) as usize]);
                vec.push(B64_URL_ENCODE[(value & 0b11_1111) as usize]);
                index += 3;
            }
        }
        match length - index {
            2 => {
                let value = (bytes[index] as u32) << 16 | (bytes[index + 1] as u32) << 8;
                vec.push(B64_URL_ENCODE[((value >> 18) & 0b11_1111) as usize]);
                vec.push(B64_URL_ENCODE[((value >> 12) & 0b11_1111) as usize]);
                vec.push(B64_URL_ENCODE[((value >> 6) & 0b11_1111) as usize]);
                vec.push(B64_URL_PAD);
            }
            1 => {
                let value = (bytes[index] as u32) << 16;
                vec.push(B64_URL_ENCODE[((value >> 18) & 0b11_1111) as usize]);
                vec.push(B64_URL_ENCODE[((value >> 12) & 0b11_1111) as usize]);
                vec.push(B64_URL_PAD);
                vec.push(B64_URL_PAD);
            }
            _ => {}
        };
        Ok(String::from_utf8(vec).unwrap())
    }
}

impl B64URLEncode<&str, String, Error> for B64URL<String> {
    fn encode(input: &str) -> Result<String, Error> {
        let bytes = input.as_bytes();
        let length = bytes.len();
        let mut vec = Vec::<u8>::with_capacity(length * 4 / 3);
        let mut index = 0;
        if length >= 3 {
            while index <= (length - 3) {
                let value = (bytes[index] as u32) << 16
                    | (bytes[index + 1] as u32) << 8
                    | (bytes[index + 2] as u32);
                vec.push(B64_URL_ENCODE[((value >> 18) & 0b11_1111) as usize]);
                vec.push(B64_URL_ENCODE[((value >> 12) & 0b11_1111) as usize]);
                vec.push(B64_URL_ENCODE[((value >> 6) & 0b11_1111) as usize]);
                vec.push(B64_URL_ENCODE[(value & 0b11_1111) as usize]);
                index += 3;
            }
        }
        match length - index {
            2 => {
                let value = (bytes[index] as u32) << 16 | (bytes[index + 1] as u32) << 8;
                vec.push(B64_URL_ENCODE[((value >> 18) & 0b11_1111) as usize]);
                vec.push(B64_URL_ENCODE[((value >> 12) & 0b11_1111) as usize]);
                vec.push(B64_URL_ENCODE[((value >> 6) & 0b11_1111) as usize]);
                vec.push(B64_URL_PAD);
            }
            1 => {
                let value = (bytes[index] as u32) << 16;
                vec.push(B64_URL_ENCODE[((value >> 18) & 0b11_1111) as usize]);
                vec.push(B64_URL_ENCODE[((value >> 12) & 0b11_1111) as usize]);
                vec.push(B64_URL_PAD);
                vec.push(B64_URL_PAD);
            }
            _ => {}
        };
        Ok(String::from_utf8(vec).unwrap())
    }
}

impl B64URLEncode<&[u8], String, Error> for B64URL<String> {
    fn encode(input: &[u8]) -> Result<String, Error> {
        let bytes = input;
        let length = bytes.len();
        let mut vec = Vec::<u8>::with_capacity(length * 4 / 3);
        let mut index = 0;
        if length >= 3 {
            while index <= (length - 3) {
                let value = (bytes[index] as u32) << 16
                    | (bytes[index + 1] as u32) << 8
                    | (bytes[index + 2] as u32);
                vec.push(B64_URL_ENCODE[((value >> 18) & 0b11_1111) as usize]);
                vec.push(B64_URL_ENCODE[((value >> 12) & 0b11_1111) as usize]);
                vec.push(B64_URL_ENCODE[((value >> 6) & 0b11_1111) as usize]);
                vec.push(B64_URL_ENCODE[(value & 0b11_1111) as usize]);
                index += 3;
            }
        }
        match length - index {
            2 => {
                let value = (bytes[index] as u32) << 16 | (bytes[index + 1] as u32) << 8;
                vec.push(B64_URL_ENCODE[((value >> 18) & 0b11_1111) as usize]);
                vec.push(B64_URL_ENCODE[((value >> 12) & 0b11_1111) as usize]);
                vec.push(B64_URL_ENCODE[((value >> 6) & 0b11_1111) as usize]);
                vec.push(B64_URL_PAD);
            }
            1 => {
                let value = (bytes[index] as u32) << 16;
                vec.push(B64_URL_ENCODE[((value >> 18) & 0b11_1111) as usize]);
                vec.push(B64_URL_ENCODE[((value >> 12) & 0b11_1111) as usize]);
                vec.push(B64_URL_PAD);
                vec.push(B64_URL_PAD);
            }
            _ => {}
        };
        Ok(String::from_utf8(vec).unwrap())
    }
}

#[cfg(test)]
mod b64_url_encode {
    use super::*;

    mod g1 {
        use super::*;

        #[test]
        fn t1() {
            let result = B64URL::<String>::encode(String::from("")).unwrap();
            assert_eq!(result, "");
        }

        #[test]
        fn t2() {
            let result = B64URL::<String>::encode(String::from("f")).unwrap();
            assert_eq!(result, "Zg==");
        }

        #[test]
        fn t3() {
            let result = B64URL::<String>::encode(String::from("fo")).unwrap();
            assert_eq!(result, "Zm8=");
        }

        #[test]
        fn t4() {
            let result = B64URL::<String>::encode(String::from("foo")).unwrap();
            assert_eq!(result, "Zm9v");
        }

        #[test]
        fn t5() {
            let result = B64URL::<String>::encode(String::from("foob")).unwrap();
            assert_eq!(result, "Zm9vYg==");
        }

        #[test]
        fn t6() {
            let result = B64URL::<String>::encode(String::from("fooba")).unwrap();
            assert_eq!(result, "Zm9vYmE=");
        }

        #[test]
        fn t7() {
            let result = B64URL::<String>::encode(String::from("foobar")).unwrap();
            assert_eq!(result, "Zm9vYmFy");
        }
    }

    mod g2 {
        use super::*;

        #[test]
        fn t1() {
            let result = B64URL::<String>::encode("").unwrap();
            assert_eq!(result, "");
        }

        #[test]
        fn t2() {
            let result = B64URL::<String>::encode("f").unwrap();
            assert_eq!(result, "Zg==");
        }

        #[test]
        fn t3() {
            let result = B64URL::<String>::encode("fo").unwrap();
            assert_eq!(result, "Zm8=");
        }

        #[test]
        fn t4() {
            let result = B64URL::<String>::encode("foo").unwrap();
            assert_eq!(result, "Zm9v");
        }

        #[test]
        fn t5() {
            let result = B64URL::<String>::encode("foob").unwrap();
            assert_eq!(result, "Zm9vYg==");
        }

        #[test]
        fn t6() {
            let result = B64URL::<String>::encode("fooba").unwrap();
            assert_eq!(result, "Zm9vYmE=");
        }

        #[test]
        fn t7() {
            let result = B64URL::<String>::encode("foobar").unwrap();
            assert_eq!(result, "Zm9vYmFy");
        }
    }

    mod g3 {
        use super::*;

        #[test]
        fn t1() {
            let result = B64URL::<String>::encode("".as_bytes()).unwrap();
            assert_eq!(result, "");
        }

        #[test]
        fn t2() {
            let result = B64URL::<String>::encode("f".as_bytes()).unwrap();
            assert_eq!(result, "Zg==");
        }

        #[test]
        fn t3() {
            let result = B64URL::<String>::encode("fo".as_bytes()).unwrap();
            assert_eq!(result, "Zm8=");
        }

        #[test]
        fn t4() {
            let result = B64URL::<String>::encode("foo".as_bytes()).unwrap();
            assert_eq!(result, "Zm9v");
        }

        #[test]
        fn t5() {
            let result = B64URL::<String>::encode("foob".as_bytes()).unwrap();
            assert_eq!(result, "Zm9vYg==");
        }

        #[test]
        fn t6() {
            let result = B64URL::<String>::encode("fooba".as_bytes()).unwrap();
            assert_eq!(result, "Zm9vYmE=");
        }

        #[test]
        fn t7() {
            let result = B64URL::<String>::encode("foobar".as_bytes()).unwrap();
            assert_eq!(result, "Zm9vYmFy");
        }
    }
}

impl B64URLDecode<String, String, Error> for B64URL<String> {
    fn decode(input: String) -> Result<String, Error> {
        let bytes = input.into_bytes();
        let size = bytes.len();
        let mut vec = Vec::<u8>::new();
        let mut index = 0;
        if size > 4 {
            while index < size - 4 {
                let value = ((B64_URL_DECODE[(bytes[index] as usize)] as u32) << 18)
                    | ((B64_URL_DECODE[(bytes[index + 1] as usize)] as u32) << 12)
                    | ((B64_URL_DECODE[(bytes[index + 2] as usize)] as u32) << 6)
                    | (B64_URL_DECODE[(bytes[index + 3] as usize)] as u32);
                vec.push(((value >> 16) & 0b1111_1111) as u8);
                vec.push(((value >> 8) & 0b1111_1111) as u8);
                vec.push((value & 0b1111_1111) as u8);
                index += 4;
            }
        }
        if index + 4 == size {
            let mut value = ((B64_URL_DECODE[(bytes[index] as usize)] as u32) << 18)
                | ((B64_URL_DECODE[(bytes[index + 1] as usize)] as u32) << 12);
            if bytes[index + 2] != B64_URL_PAD {
                value |= (B64_URL_DECODE[(bytes[index + 2] as usize)] as u32) << 6;
                if bytes[index + 3] != B64_URL_PAD {
                    value |= B64_URL_DECODE[(bytes[index + 3] as usize)] as u32;
                    vec.push(((value >> 16) & 0b1111_1111) as u8);
                    vec.push(((value >> 8) & 0b1111_1111) as u8);
                    vec.push((value & 0b1111_1111) as u8);
                } else {
                    vec.push(((value >> 16) & 0b1111_1111) as u8);
                    vec.push(((value >> 8) & 0b1111_1111) as u8);
                }
            } else {
                vec.push(((value >> 16) & 0b1111_1111) as u8);
            }
        }
        Ok(String::from_utf8(vec).unwrap())
    }
}

#[cfg(test)]
mod b64_url_decode {
    use super::*;

    mod g1 {
        use super::*;

        #[test]
        fn t1() {
            let result = B64URL::<String>::decode(String::from("")).unwrap();
            assert_eq!(result, "");
        }

        #[test]
        fn t2() {
            let result = B64URL::<String>::decode(String::from("Zg==")).unwrap();
            assert_eq!(result, "f");
        }

        #[test]
        fn t3() {
            let result = B64URL::<String>::decode(String::from("Zm8=")).unwrap();
            assert_eq!(result, "fo");
        }

        #[test]
        fn t4() {
            let result = B64URL::<String>::decode(String::from("Zm9v")).unwrap();
            assert_eq!(result, "foo");
        }

        #[test]
        fn t5() {
            let result = B64URL::<String>::decode(String::from("Zm9vYg==")).unwrap();
            assert_eq!(result, "foob");
        }

        #[test]
        fn t6() {
            let result = B64URL::<String>::decode(String::from("Zm9vYmE=")).unwrap();
            assert_eq!(result, "fooba");
        }

        #[test]
        fn t7() {
            let result = B64URL::<String>::decode(String::from("Zm9vYmFy")).unwrap();
            assert_eq!(result, "foobar");
        }
    }
}
