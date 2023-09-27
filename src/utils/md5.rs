const VALUE_TABLE: [u32; 64] = [
    0xd76aa478, 0xe8c7b756, 0x242070db, 0xc1bdceee, 0xf57c0faf, 0x4787c62a, 0xa8304613,
    0xfd469501, 0x698098d8, 0x8b44f7af, 0xffff5bb1, 0x895cd7be, 0x6b901122, 0xfd987193,
    0xa679438e, 0x49b40821, 0xf61e2562, 0xc040b340, 0x265e5a51, 0xe9b6c7aa, 0xd62f105d,
    0x02441453, 0xd8a1e681, 0xe7d3fbc8, 0x21e1cde6, 0xc33707d6, 0xf4d50d87, 0x455a14ed,
    0xa9e3e905, 0xfcefa3f8, 0x676f02d9, 0x8d2a4c8a, 0xfffa3942, 0x8771f681, 0x6d9d6122,
    0xfde5380c, 0xa4beea44, 0x4bdecfa9, 0xf6bb4b60, 0xbebfbc70, 0x289b7ec6, 0xeaa127fa,
    0xd4ef3085, 0x04881d05, 0xd9d4d039, 0xe6db99e5, 0x1fa27cf8, 0xc4ac5665, 0xf4292244,
    0x432aff97, 0xab9423a7, 0xfc93a039, 0x655b59c3, 0x8f0ccc92, 0xffeff47d, 0x85845dd1,
    0x6fa87e4f, 0xfe2ce6e0, 0xa3014314, 0x4e0811a1, 0xf7537e82, 0xbd3af235, 0x2ad7d2bb,
    0xeb86d391,
];

pub fn calculate_hash(s: &str) -> String {
    let mut b = s.bytes().collect::<Vec<u8>>();

    {
        // Bit padding
        let input_length_bits = (b.len() as u64) * 8;
        b.push(0x80_u8);
        while b.len() % 64_usize != 56_usize {
            b.push(0_u8);
        }

        let length_bits = vec![
            input_length_bits as u8,
            (input_length_bits >> 8) as u8,
            (input_length_bits >> 16) as u8,
            (input_length_bits >> 24) as u8,
            (input_length_bits >> 32) as u8,
            (input_length_bits >> 40) as u8,
            (input_length_bits >> 48) as u8,
            (input_length_bits >> 56) as u8,
        ];
        // let length_bits = vec![
        //     (input_length_bits >> 56) as u8,
        //     (input_length_bits >> 48) as u8,
        //     (input_length_bits >> 40) as u8,
        //     (input_length_bits >> 32) as u8,
        //     (input_length_bits >> 24) as u8,
        //     (input_length_bits >> 16) as u8,
        //     (input_length_bits >> 8) as u8,
        //     input_length_bits as u8,
        // ];
        b.extend(length_bits);
    }

    assert_eq!(b.len() % 64, 0);

    let b_u32 = {
        let mut t = Vec::new();

        for chunk in b.chunks(4) {
            t.push(u32::from_ne_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]));
        }
        t
    };

    let mut a = 0x67452301u32;
    let mut b = 0xefcdab89u32;
    let mut c = 0x98badcfeu32;
    let mut d = 0x10325476u32;
    let rotate_amounts = vec![
        7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 5, 9, 14, 20, 5, 9, 14, 20, 5,
        9, 14, 20, 5, 9, 14, 20, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 6, 10,
        15, 21, 6, 10, 15, 21, 6, 10, 15, 21, 6, 10, 15, 21,
    ];

    for chunk in b_u32.chunks_exact(16) {
        let mut aa = a;
        let mut bb = b;
        let mut cc = c;
        let mut dd = d;

        for i in 0..=63 {
            let mut f = 0_u32;
            let mut g = 0_usize;
            if i <= 15 {
                f = (bb & cc) | ((!bb) & dd);
                g = i;
            } else if 16 <= i && i <= 31 {
                f = (dd & bb) | ((!dd) & cc);
                g = (5 * i + 1) % 16;
            } else if 32 <= i && i <= 47 {
                f = bb ^ cc ^ dd;
                g = (3 * i + 5) % 16;
            } else if 48 <= i && i <= 63 {
                f = cc ^ (bb | (!dd));
                g = (7 * i) % 16;
            }

            f = f.wrapping_add(aa).wrapping_add(VALUE_TABLE[i]).wrapping_add(chunk[g]);
            aa = dd;
            dd = cc;
            cc = bb;
            bb = bb.wrapping_add(f.rotate_left(rotate_amounts[i]));
        }

        a = a.wrapping_add(aa);
        b = b.wrapping_add(bb);
        c = c.wrapping_add(cc);
        d = d.wrapping_add(dd);
    }

    format!(
        "{:08x}{:08x}{:08x}{:08x}",
        a.swap_bytes(),
        b.swap_bytes(),
        c.swap_bytes(),
        d.swap_bytes()
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn md5_works() {
        assert_eq!(calculate_hash(""), "d41d8cd98f00b204e9800998ecf8427e");
        assert_eq!(
            calculate_hash("The quick brown fox jumps over the lazy dog"),
            "9e107d9d372bb6826bd81d3542a419d6"
        );
        assert_eq!(
            calculate_hash("The quick brown fox jumps over the lazy dog."),
            "e4d909c290d0fb1ca068ffaddf22cbd0"
        );
    }
}
