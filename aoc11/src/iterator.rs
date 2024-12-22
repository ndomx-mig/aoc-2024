const ITERATE_0: [u64; 25] = [
    19778, 12343, 8268, 5602, 3572, 2377, 1546, 1059, 667, 418, 328, 200, 110, 81, 62, 39, 20, 16,
    14, 7, 4, 4, 2, 1, 1,
];
const ITERATE_1: [u64; 25] = [
    29165, 19778, 12343, 8268, 5602, 3572, 2377, 1546, 1059, 667, 418, 328, 200, 110, 81, 62, 39,
    20, 16, 14, 7, 4, 4, 2, 1,
];
const ITERATE_2: [u64; 25] = [
    27842, 18712, 11819, 7921, 5463, 3381, 2270, 1501, 977, 661, 414, 295, 181, 111, 92, 57, 30,
    19, 16, 12, 6, 4, 4, 2, 1,
];
const ITERATE_3: [u64; 25] = [
    27569, 18714, 12116, 7914, 5360, 3347, 2281, 1556, 987, 642, 401, 294, 202, 114, 79, 52, 35,
    26, 16, 10, 5, 4, 4, 2, 1,
];
const ITERATE_4: [u64; 25] = [
    26669, 17957, 11820, 7721, 5280, 3204, 2182, 1541, 951, 637, 390, 269, 195, 115, 82, 47, 30,
    27, 16, 8, 4, 4, 4, 2, 1,
];
const ITERATE_5: [u64; 25] = [
    23822, 15847, 10627, 6675, 4529, 3053, 1976, 1260, 808, 597, 383, 223, 163, 109, 67, 45, 32,
    22, 11, 8, 8, 4, 2, 1, 1,
];
const ITERATE_6: [u64; 25] = [
    25469, 16815, 11371, 7052, 4917, 3193, 2033, 1431, 871, 600, 401, 250, 183, 103, 68, 54, 32,
    22, 11, 8, 8, 4, 2, 1, 1,
];
const ITERATE_7: [u64; 25] = [
    25071, 16509, 11170, 6994, 4762, 3165, 2065, 1369, 832, 602, 413, 242, 168, 106, 72, 52, 32,
    22, 11, 8, 8, 4, 2, 1, 1,
];
const ITERATE_8: [u64; 25] = [
    24212, 16018, 10738, 6798, 4580, 3034, 2011, 1322, 812, 578, 393, 239, 161, 103, 69, 48, 31,
    22, 11, 7, 7, 4, 2, 1, 1,
];
const ITERATE_9: [u64; 25] = [
    25793, 17059, 11617, 7217, 4888, 3216, 2131, 1468, 854, 586, 419, 262, 183, 103, 70, 54, 32,
    22, 11, 8, 8, 4, 2, 1, 1,
];

pub fn apply_rules_once(n: u64) -> Vec<u64> {
    if n == 0 {
        return vec![1];
    }

    let as_str = n.to_string();
    let size = as_str.len();
    if size % 2 == 0 {
        let (head, tail) = as_str.split_at(size >> 1);

        let head_u64 = head.parse().unwrap();
        let tail_u64 = tail.parse().unwrap();
        return vec![head_u64, tail_u64];
    }

    return vec![n * 2024];
}

pub fn iterate(n: u64, start: usize) -> u64 {
    if start >= 24 {
        return n;
    }

    match n {
        0 => return ITERATE_0[start],
        1 => return ITERATE_1[start],
        2 => return ITERATE_2[start],
        3 => return ITERATE_3[start],
        4 => return ITERATE_4[start],
        5 => return ITERATE_5[start],
        6 => return ITERATE_6[start],
        7 => return ITERATE_7[start],
        8 => return ITERATE_8[start],
        9 => return ITERATE_9[start],
        _ => {}
    }

    let curr = apply_rules_once(n);
    return curr.into_iter().map(|x| iterate(x, start + 1)).sum();
}
