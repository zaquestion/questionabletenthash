use itertools::Itertools;

pub fn tenthash(bytes: &Vec<u8>) -> [u8; 20] {
    let [mut A, mut B, mut C, mut D]: [u64; 4] = [
        0x5d6daffc4411a967,
        0xe22d4dea68577f34,
        0xca50864d814cbc2e,
        0x894e29b9611eb173,
    ];

    fn mix(a: &mut u64, b: &mut u64, c: &mut u64, d: &mut u64) {
        // or put them in an array if that’s easier
        for (l, r) in [
            (16u32, 28u32),
            (14, 57),
            (11, 22),
            (35, 34),
            (57, 16),
            (59, 40),
            (44, 13),
        ] {
            *a = a.wrapping_add(*c);
            *b = b.wrapping_add(*d);
            *c = (c.rotate_left(l)) ^ *a;
            *d = (d.rotate_left(r)) ^ *b;
            std::mem::swap(a, b);
        }
    }

    for block in bytes.chunks(32).into_iter() {
        let padded = block.into_iter().pad_using(32, |_| &0u8);

        // Now turn that into 4 u64s
        let (a, b, c, d) = padded
            .chunks(8)
            .into_iter()
            .map(|ch| {
                let arr: [u8; 8] = ch.map(|b| *b).collect_vec().try_into().expect("must arr");
                u64::from_le_bytes(arr)
            })
            .collect_tuple()
            .unwrap_or_default();

        A ^= a;
        B ^= b;
        C ^= c;
        D ^= d;
        mix(&mut A, &mut B, &mut C, &mut D);
    }

    // todo something faster than count lol
    A ^= bytes.len() as u64 * 8;
    mix(&mut A, &mut B, &mut C, &mut D);
    mix(&mut A, &mut B, &mut C, &mut D);
    let truncate = || -> [u8; 20] {
        let mut buf = [0u8; 20];

        // Convert each u64 to bytes, then copy them into the buffer.
        buf[0..8].copy_from_slice(&A.to_le_bytes());
        buf[8..16].copy_from_slice(&B.to_le_bytes());
        buf[16..20].copy_from_slice(&C.to_le_bytes()[0..4]);
        buf
    };
    truncate()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    // Cases are built from the test vectors provided in the tenthash spec
    // Ref: https://github.com/cessen/tenthash/blob/main/docs/specification.md#test-vectors
    #[rstest]
    #[case(b"", "68c8213b7a76b8ed267dddb3d8717bb3b6e7cc0a")]
    #[case(b"\0", "3cf6833cca9c4d5e211318577bab74bf12a4f090")]
    #[case(b"0123456789", "a7d324bde0bf6ce3427701628f0f8fc329c2a116")]
    #[case(
        b"abcdefghijklmnopqrstuvwxyz",
        "f1be4be1a0f9eae6500fb2f6b64f3daa3990ac1a"
    )]
    #[case(
        b"This string is exactly 32 bytes.",
        "f7c5e4763d89bddce33e97712b712d869aabcfe9"
    )]
    #[case(
        b"The quick brown fox jumps over the lazy dog.",
        "de77f1c134228be1b5b25c941d5102f87f3e6d39"
    )]
    fn test_tenthash(#[case] data: &[u8], #[case] expected: String) {
        assert_eq!(
            expected,
            tenthash(&data.to_vec())
                .iter()
                .map(|b| format!("{:02x}", b))
                .collect::<String>()
        );
    }
}
