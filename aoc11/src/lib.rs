const ITERATE_0: [u64; 25] = [
    1, 1, 2, 4, 4, 7, 14, 16, 20, 39, 62, 81, 110, 200, 328, 418, 667, 1059, 1546, 2377, 3572,
    5602, 8268, 12343, 19778,
];
const ITERATE_1: [u64; 25] = [
    1, 2, 4, 4, 7, 14, 16, 20, 39, 62, 81, 110, 200, 328, 418, 667, 1059, 1546, 2377, 3572, 5602,
    8268, 12343, 19778, 29165,
];
const ITERATE_2: [u64; 25] = [
    1, 2, 4, 4, 6, 12, 16, 19, 30, 57, 92, 111, 181, 295, 414, 661, 977, 1501, 2270, 3381, 5463,
    7921, 11819, 18712, 27842,
];
const ITERATE_3: [u64; 25] = [
    1, 2, 4, 4, 5, 10, 16, 26, 35, 52, 79, 114, 202, 294, 401, 642, 987, 1556, 2281, 3347, 5360,
    7914, 12116, 18714, 27569,
];
const ITERATE_4: [u64; 25] = [
    1, 2, 4, 4, 4, 8, 16, 27, 30, 47, 82, 115, 195, 269, 390, 637, 951, 1541, 2182, 3204, 5280,
    7721, 11820, 17957, 26669,
];
const ITERATE_5: [u64; 25] = [
    1, 1, 2, 4, 8, 8, 11, 22, 32, 45, 67, 109, 163, 223, 383, 597, 808, 1260, 1976, 3053, 4529,
    6675, 10627, 15847, 23822,
];
const ITERATE_6: [u64; 25] = [
    1, 1, 2, 4, 8, 8, 11, 22, 32, 54, 68, 103, 183, 250, 401, 600, 871, 1431, 2033, 3193, 4917,
    7052, 11371, 16815, 25469,
];
const ITERATE_7: [u64; 25] = [
    1, 1, 2, 4, 8, 8, 11, 22, 32, 52, 72, 106, 168, 242, 413, 602, 832, 1369, 2065, 3165, 4762,
    6994, 11170, 16509, 25071,
];
const ITERATE_8: [u64; 25] = [
    1, 1, 2, 4, 7, 7, 11, 22, 31, 48, 69, 103, 161, 239, 393, 578, 812, 1322, 2011, 3034, 4580,
    6798, 10738, 16018, 24212,
];
const ITERATE_9: [u64; 25] = [
    1, 1, 2, 4, 8, 8, 11, 22, 32, 54, 70, 103, 183, 262, 419, 586, 854, 1468, 2131, 3216, 4888,
    7217, 11617, 17059, 25793,
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