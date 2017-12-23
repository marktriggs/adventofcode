/*

///////////////// Day 1

const DAY1_INPUT: &str = "494751136895345894732582362629576539599184296195318162664695189393364372585778868512194863927652788149779748657989318645936221887731542718562643272683862627537378624843614831337441659741281289638765171452576466381314558821636595394981788588673443769343597851883955668818165723174939893841654914556681324133667446412138511724424292394454166623639872425168644336248217213826339741267546823779383343362789527461579565822966859349777937921933694912369552152772735167832762563719664315456987186713541153781499646178238762644186484381142249926194743713139262596264878458636595896487362658672224346241358667234115974528626523648311919886566497837217169673923935143386823757293148719377821517314629812886912412829924484513493885672343964151252433622341141661523814465991516961684511941471572895453711624986269342398786175846925783918686856442684489873327497698963658862856336682422797551251489126661954848572297228765445646745256499679451426358865477844467458533962981852292513358871483321161973583245698763531598395467675529181496911117769834127516441369261275244225978893617456524385518493112272169767775861256649728253754964675812534546226295535939697352141217337346738553495616832783757866928174519145357234834584788253893618549484385733283627199445369658339175644484859385884574943219267922729967571943843794565736975716174727852348441254492886794362934343868643337828637454277582276962353246357835493338372219824371517526474283541714897994127864461433627894831268659336264234436872715374727211764167739169341999573855627775114848275268739159272518673316753672995297888734844388928439859359992475637439771269232916542385876779616695129412366735112593669719335783511355773814685491876721452994714318863716542473187246351548626157775143333161422867924437526253865859969947366972895674966845993244925218766937543487875485647329995285821739359369998935331986126873726737672159265827566443794515755939813676194755474477224152139987944419463371386499841415227734673733555261543871359797796529847861748979527579985757964742667473767269248335229836818297477665453189662485548925521497365877771665365728224394427883312135322325169141784";

fn day1_pt1() {
    let digit_str = DAY1_INPUT.to_string();

    let digits: Vec<_> = digit_str.chars().map(|c| c.to_digit(10).unwrap()).collect();

    let mut sum = 0;

    for i in 0..digits.len() {
        if digits[i] == digits[(i + 1) % digits.len()] {
            sum += digits[i]
        }
    }

    println!("Sum was: {}", sum);
}

fn day1_pt1_functional() {
    let digits: Vec<_> = DAY1_INPUT.chars().map(|c| c.to_digit(10).unwrap()).collect();

    let sum = digits.iter().enumerate().fold(0, |acc, (i, n)| {
        if digits[i] == digits[(i + 1) % digits.len()] {
            acc + n
        } else {
            acc
        }
    });

    println!("Sum was: {}", sum);
}

fn day1_pt2(input: &str) -> u32 {
    let digit_str = input.to_string();

    let digits: Vec<_> = digit_str.chars().map(|c| c.to_digit(10).unwrap()).collect();
    let steps = digits.len() / 2;

    let mut sum = 0;

    for i in 0..digits.len() {
        if digits[i] == digits[(i + steps) % digits.len()] {
            sum += digits[i]
        }
    }

    sum
}

fn day1() {
    assert_eq!(day1_pt2("1212"), 6);
    assert_eq!(day1_pt2("1221"), 0);
    assert_eq!(day1_pt2("123425"), 4);
    assert_eq!(day1_pt2("123123"), 12);
    assert_eq!(day1_pt2("12131415"), 4);

    println!("day 2 pt 2: {}", day1_pt2(DAY1_INPUT));
}

///////////////// Day 2

const DAY_2_DATA: &str = "278	1689	250	1512	1792	1974	175	1639	235	1635	1690	1947	810	224	928	859
160	50	55	81	68	130	145	21	211	136	119	78	174	155	149	72
4284	185	4499	273	4750	4620	4779	4669	2333	231	416	1603	197	922	5149	2993
120	124	104	1015	1467	110	299	320	1516	137	1473	132	1229	1329	1430	392
257	234	3409	2914	2993	3291	368	284	259	3445	245	1400	3276	339	2207	233
1259	78	811	99	2295	1628	3264	2616	116	3069	2622	1696	1457	1532	268	82
868	619	139	522	168	872	176	160	1010	200	974	1008	1139	552	510	1083
1982	224	3003	234	212	1293	1453	3359	326	3627	3276	3347	1438	2910	248	2512
4964	527	5108	4742	4282	4561	4070	3540	196	228	3639	4848	152	1174	5005	202
1381	1480	116	435	980	1022	155	1452	1372	121	128	869	1043	826	1398	137
2067	2153	622	1479	2405	1134	2160	1057	819	99	106	1628	1538	108	112	1732
4535	2729	4960	241	4372	3960	248	267	230	5083	827	1843	3488	4762	2294	3932
3245	190	2249	2812	2620	2743	2209	465	139	2757	203	2832	2454	177	2799	2278
1308	797	498	791	1312	99	1402	1332	521	1354	1339	101	367	1333	111	92
149	4140	112	3748	148	815	4261	138	1422	2670	32	334	2029	4750	4472	2010
114	605	94	136	96	167	553	395	164	159	284	104	530	551	544	18";

fn day2_pt1(data: &str) -> i32 {

    let rows: Vec<Vec<i32>> = data.split("\n").map(|line| line.split("\t").map(|s| s.parse().unwrap()).collect()).collect();

    let result = rows.iter().fold(0, |sum, row| {
        let min = row.iter().fold(row[0], |acc, i| std::cmp::min(acc, *i));
        let max = row.iter().fold(row[0], |acc, i| std::cmp::max(acc, *i));

        sum + (max - min)
    });

    result
}

fn day2_pt2(data: &str) -> i32 {

    let rows: Vec<Vec<i32>> = data.split("\n").map(|line| line.split("\t").map(|s| s.parse().unwrap()).collect()).collect();

    rows.iter().fold(0, |sum, row| {
        for a in row.iter() {
            for b in row.iter() {
                if a > b && a % b == 0 {
                    return sum + (a / b)
                }
            }
        }

        sum
    })
}


fn day2() {
    println!("Day 2 pt1: {}", day2_pt1(DAY_2_DATA));

    assert_eq!(day2_pt2("5\t9\t2\t8\n9\t4\t7\t3\n3\t8\t6\t5"), 9);

    println!("Day 2 pt2: {}", day2_pt2(DAY_2_DATA));

}

///////////////// Day 3


#[derive(Debug)]
struct Pos {
    x: i32,
    y: i32
}

fn day3_move_right(pos: Pos, n: i32) -> Pos { Pos {x: pos.x + n, y: pos.y} }
fn day3_move_up(pos: Pos, n: i32) -> Pos { Pos {x: pos.x, y: pos.y + n} }
fn day3_move_left(pos: Pos, n: i32) -> Pos { Pos {x: pos.x - n, y: pos.y} }
fn day3_move_down(pos: Pos, n: i32) -> Pos { Pos {x: pos.x, y: pos.y - n} }

const DAY3_TRANSFORMS: [fn(Pos, i32) -> Pos; 4] = [
    day3_move_right,
    day3_move_up,
    day3_move_left,
    day3_move_down,
];


fn day3_pt1(n: i32) -> i32 {
    let mut pos = Pos { x: 0, y: 0 };
    let mut increment = 1;
    let mut count = 0;

    let mut upto = 1;

    loop {
        let transform = DAY3_TRANSFORMS[(count % (DAY3_TRANSFORMS.len() as i32)) as usize];

        for _ in 0..increment {
            if upto == n {
                return pos.x.abs() + pos.y.abs()
            }

            pos = transform(pos, 1);
            upto += 1;
        }

        count += 1;

        if count % 2 == 0 {
            increment += 1
        }
    }
}

fn day3_pt2(n: i32) -> i32 {
    let mut grid  = vec![vec![0; 2000]; 2000];

    grid[1000][1000] = 1;
    let mut pos = Pos { x: 1000, y: 1000 };
    let mut increment = 1;

    let mut count = 0;
    loop {
        let transform = DAY3_TRANSFORMS[(count % (DAY3_TRANSFORMS.len() as i32)) as usize];

        for _ in 0..increment {
            pos = transform(pos, 1);

            // Fill in our box
            grid[(pos.x) as usize][(pos.y) as usize] =
                grid[(pos.x - 1) as usize][(pos.y - 1) as usize] +
                grid[(pos.x - 1) as usize][(pos.y) as usize] +
                grid[(pos.x - 1) as usize][(pos.y + 1) as usize] +
                grid[(pos.x) as usize][(pos.y - 1) as usize] +
                grid[(pos.x) as usize][(pos.y + 1) as usize] +
                grid[(pos.x + 1) as usize][(pos.y - 1) as usize] +
                grid[(pos.x + 1) as usize][(pos.y) as usize] +
                grid[(pos.x + 1) as usize][(pos.y + 1) as usize];

            if grid[(pos.x) as usize][(pos.y) as usize] > n {
                return grid[(pos.x) as usize][(pos.y) as usize]
            }
        }

        count += 1;
        if count % 2 == 0 {
            increment += 1
        }
    }
}

fn day3() {
    assert_eq!(day3_pt1(1), 0);
    assert_eq!(day3_pt1(12), 3);
    assert_eq!(day3_pt1(23), 2);
    assert_eq!(day3_pt1(1024), 31);

    println!("day3 pt 1: {}", day3_pt1(368078));
    println!("day 3 pt 2: {}", day3_pt2(368078));
}


///////////////// Day 4

use std::iter::FromIterator;
use std::collections::HashSet;
use std::io::{BufRead, BufReader};
use std::fs::File;

fn day4_pt1(passphrase: String) -> bool {
    let words: Vec<&str> = passphrase.split(" ").collect();
    let set: HashSet<&str> = HashSet::from_iter(words.iter().map(|s| *s));

    words.len() == set.len()
}

fn sort_chars(s: String) -> String {
    let mut chars: Vec<char> = s.chars().collect();

    chars.sort();
    String::from_iter(chars)
}

fn day4_pt2(passphrase: String) -> bool {
    let words: Vec<&str> = passphrase.split(" ").collect();
    let set: HashSet<String> = HashSet::from_iter(words.iter().map(|s| sort_chars((*s).to_owned())));

    words.len() == set.len()
}


fn day4() {
    assert_eq!(day4_pt1("aa bb cc dd ee".to_string()), true);
    assert_eq!(day4_pt1("aa bb cc dd aa".to_string()), false);
    assert_eq!(day4_pt1("aa bb cc dd aaa".to_string()), true);

    {
        let f = File::open("advent-files/day4.txt").expect("open file");
        let br = BufReader::new(f);

        let mut valid = 0;
        for line in br.lines() {
            if day4_pt1(line.unwrap()) {
                valid += 1;
            }
        }

        println!("day4 pt 1: {}", valid);
    }

    assert_eq!(day4_pt2("abcde fghij".to_string()), true);
    assert_eq!(day4_pt2("abcde xyz ecdab".to_string()), false);
    assert_eq!(day4_pt2("a ab abc abd abf abj".to_string()), true);
    assert_eq!(day4_pt2("iiii oiii ooii oooi oooo".to_string()), true);
    assert_eq!(day4_pt2("oiii ioii iioi iiio".to_string()), false);

    {
        let f = File::open("advent-files/day4.txt").expect("open file");
        let br = BufReader::new(f);

        let mut valid = 0;
        for line in br.lines() {
            if day4_pt2(line.unwrap()) {
                valid += 1;
            }
        }

        println!("day4 pt 2: {}", valid);
    }
}

///////////////// Day 5

const DAY5_DATA: &[i32] =
    &[2, 2, 0, 0, -2, -1, -3, 0, 0, -3, -5, -5, 1, -10, -8, -1, -8,
      -5, -12, -5, 1, -6, -18, -17, -9, -12, -24, -16, -6, -12, -14,
      -15, -28, -1, -10, -2, -2, 0, -16, -4, -22, -33, -34, -28, -41,
      -11, -16, -12, -25, -13, -12, -14, -17, -24, -48, -54, -7, -10,
      -8, -49, -24, -49, -39, -8, -53, 2, -65, -55, -52, 1, -54, -3,
      -60, -28, -3, -33, -41, -66, -70, -46, -68, -26, -22, 0, -82, -72,
      -82, -61, -33, -15, -9, -19, -83, -46, -21, -92, -47, -72, -86,
      -7, -2, -65, -4, -64, -52, -30, -34, -50, -46, -107, -20, -61,
      -49, -82, -18, -108, -59, -7, -97, -66, -78, -31, -49, -89, -16,
      -27, -107, -120, -87, -74, -50, -11, -53, -14, -128, -124, -99,
      -42, -73, -129, -112, -85, -52, -23, -120, -22, -82, -65, -51,
      -118, -37, -59, -105, -59, -152, -6, -61, -96, -30, -126, -83,
      -65, -144, -106, 0, -156, -79, -22, -15, -132, 0, -144, -132,
      -119, -20, -92, -96, -21, -110, -124, -59, -23, -128, -67, -48, 1,
      -185, -175, -70, -103, -71, -40, -76, -96, -85, 1, -96, -165, -94,
      -129, -104, -165, -127, -135, -83, -103, -77, -61, -115, -33,
      -203, -174, -82, -81, -22, -86, -172, -143, -197, -70, -126, -193,
      -152, -213, -129, -176, -182, -9, -51, -108, -132, -28, -106,
      -163, -201, -128, -49, -48, -90, -163, -217, -146, -117, -122,
      -96, -40, -23, -125, -46, -121, -127, -50, -193, -40, -220, -253,
      -224, -86, -252, -129, -188, -154, -103, -110, -66, -205, -138,
      -256, -134, -39, -233, -90, -95, -76, -179, -27, -245, -242, -6,
      -124, -137, -275, -75, -99, -62, 1, -15, -175, -9, -193, -22,
      -128, -140, -290, -119, -127, -271, -137, -188, -21, -82, -143,
      -210, -246, -94, -188, -238, -2, -10, -185, -142, -73, -213, -170,
      -150, -238, -23, -69, -13, -186, -56, -22, -297, -258, 0, -302,
      -287, -209, -288, -280, -257, -164, 0, -158, -197, -313, -229,
      -249, -240, -218, -169, -126, -186, -22, -105, -176, -270, -337,
      -129, -260, -100, -43, -301, -281, -258, -82, -110, -144, -193,
      -253, -115, -117, -230, -261, -299, -63, -118, -257, -17, -364,
      -214, -223, -182, -329, -299, 1, -116, -306, -198, -34, -121,
      -132, -76, -27, -103, -118, -262, -383, -195, -323, -142, -279,
      -162, -318, -15, -362, -272, -291, -397, -49, -309, -158, -368,
      -215, -301, -168, -317, -24, -247, -33, -193, -309, -90, 0, -104,
      -335, -42, -149, -241, -35, -397, -235, -10, -206, -45, -21, -80,
      -215, -411, -16, -338, -253, -169, -278, -339, -50, -321, -189,
      -72, -91, -411, -257, -139, -270, -253, -82, -139, -168, -195,
      -76, -125, -230, -194, -386, -216, -242, -407, -238, -173, -15,
      -424, -72, -363, -66, -462, -412, -171, -349, -342, -109, -358,
      -285, -196, -178, -268, -464, -410, -344, -374, -193, -156, -170,
      -157, -362, -473, -329, -96, -30, -26, -157, -434, -406, -349,
      -463, -156, -166, -423, -61, -268, -182, -66, -155, -426, -396,
      -207, -210, -129, -454, -277, -324, 1, -76, -247, -9, -147, -155,
      -318, -494, -325, -348, -507, -391, -209, -481, -112, -236, -157,
      -515, -3, -245, -447, -521, -349, -429, -271, -93, -29, -482, -4,
      -174, -390, -278, -240, -208, -317, -331, -175, -319, -438, -337,
      -91, -26, -460, -479, -321, -464, -216, -379, -75, -215, -109,
      -465, -280, -189, -439, -345, -170, -250, -59, -257, -525, -475,
      -547, -504, -101, -238, -394, -501, -265, -426, -469, -68, -252,
      -216, -234, -395, -89, -353, -287, -559, -371, -400, -377, -385,
      -504, -159, -22, -378, -515, -133, -286, -414, -478, -205, -489,
      -37, -64, -556, -171, -366, -49, -540, -474, -501, -51, -457,
      -174, -231, -96, -365, -475, -385, -257, -271, -129, -616, -474,
      -127, -389, -407, -557, -448, -49, -324, -143, -271, -363, -82,
      -311, -593, -303, -355, -91, -181, -462, -453, -548, -171, -96,
      -110, -475, -412, -49, -379, -294, -294, -324, -382, -327, -233,
      -482, -209, -28, -375, -236, -538, -7, -427, -424, -169, -152,
      -421, -503, -17, -390, -615, -113, -45, -113, -150, -329, -111,
      -9, -649, -647, -652, 0, -610, -127, -286, -405, -38, -225, -595,
      -195, -509, -61, -651, -279, -270, -54, -110, -324, -220, -630,
      -490, -313, -672, -591, -379, -27, -599, -232, -593, -463, -243,
      -375, -414, -476, -324, -269, -103, -65, -576, -452, -591, -7,
      -402, -696, -383, -498, -622, -690, -33, -660, -83, -393, -70,
      -197, -522, -616, -716, -342, -142, -374, -412, -241, -155, -22,
      -593, -691, -28, -150, -26, -681, -290, -688, -369, -552, -601,
      -231, -120, -484, -342, -497, -412, -342, -728, -600, -275, -88,
      -341, -752, -602, -41, -519, -663, -578, -758, -658, -69, -710,
      -567, -278, -299, -658, -363, -651, -138, -394, -403, -771, -569,
      -234, -230, -268, -130, -104, -507, -148, -400, -473, -699, -506,
      -497, -110, -279, -470, -776, -21, -10, -412, -419, -6, -488, -19,
      -86, -70, -386, -263, -59, -813, -776, -494, -644, -67, -450,
      -384, -232, -552, -227, -480, -599, -412, -190, -87, -483, -446,
      -153, -309, -729, -14, -163, -698, -27, -404, -656, -571, -686,
      -333, -49, -829, -541, -812, -782, -614, -534, -399, -100, -560,
      -547, -258, -808, -754, -543, -581, -785, -581, -500, -210, -709,
      -774, -263, -124, -469, -840, -374, -695, -747, -439, -260, -119,
      -792, -554, -310, -177, -749, -292, -617, -112, -303, -207, -785,
      -457, -608, -628, -654, -107, -510, -522, -701, -171, -102, -303,
      -804, -60, -771, -51, -570, -76, -440, -746, -704, -135, -738,
      -377, -23, -452, -732, -169, -262, -689, -271, -676, -503, -543,
      -529, -158, -547, -413, -898, -448, -810, -637, -440, -251, -798,
      -161, -334, -512, -214, -912, -571, -80, -192, -777, -298, -403,
      -909, -244, 2, -377, -291, -86, -742, -71, -88, -137, -455, -671,
      -689, -243, -760, -229, -183, -516, -789, -205, -710, -607, -866,
      -634, -913, -105, -648, -895, -576, -165, -667, -89, -890, -481,
      -258, -434, -788, -417, -608, -855, -642, -152, -621, -558, -65,
      -259, -742, -195, -451, -85, -310, -402, -586, -508, -201, -775,
      -466, -80, -402, -565, -574, -351, -891, -704, -411, -266, -830,
      -1012, -712, -749, -842, -175, -927, -1003, -484, -723, -677,
      -607, -338, -367, -488, -618, -189, -109, -181, -547, -852];

fn day5_pt1(data: &Vec<i32>) -> u32 {
    let mut jumps: Vec<i32> = data.clone();

    let mut count: u32 = 0;
    let mut pc: i32 = 0;

    while pc >= 0 && pc < jumps.len() as i32 {
        let offset = jumps[pc as usize];
        jumps[pc as usize] += 1;
        pc += offset;

        count += 1
    }

    return count;
}

fn day5_pt2(data: &Vec<i32>) -> u32 {
    let mut jumps: Vec<i32> = data.clone();

    let mut count: u32 = 0;
    let mut pc: i32 = 0;

    while pc >= 0 && pc < jumps.len() as i32 {
        let offset = jumps[pc as usize];

        if offset >= 3 {
            jumps[pc as usize] -= 1;
        } else {
            jumps[pc as usize] += 1;
        }

        pc += offset;

        count += 1
    }

    return count;
}

fn day5() {
    let vec = DAY5_DATA.to_vec();
    println!("Part 1: exited in jumps: {}", day5_pt1(&vec));
    println!("Part 2: exited in jumps: {}", day5_pt2(&vec));

}

///////////////// Day 6

use std::collections::HashSet;

fn day6_run_cycle(state: Vec<u32>) -> Vec<u32> {
    let mut result = state.clone();

    let max = result.iter().fold(result[0], |acc, i| std::cmp::max(acc, *i));
    let mut idx = result.iter().position(|&elt| elt == max).unwrap();

    let mut val = result[idx];
    result[idx] = 0;

    while val > 0 {
        idx = (idx + 1) % result.len();
        result[idx] += 1;
        val -= 1;
    }

    return result;
}

fn day6_pt1(initial_state: Vec<u32>) -> u32 {
    let mut state = initial_state.clone();
    let mut seen: HashSet<Vec<u32>> = HashSet::new();

    let mut cycles = 0;
    loop {
        cycles += 1;
        seen.insert(state.clone());

        let new_state = day6_run_cycle(state);
        if seen.contains(&new_state) {
            return cycles;
        }

        state = new_state;
    }
}

fn day6_pt2_find_cycle_start(initial_state: Vec<u32>) -> Vec<u32> {
    let mut state = initial_state.clone();
    let mut seen: HashSet<Vec<u32>> = HashSet::new();

    loop {
        seen.insert(state.clone());

        let new_state = day6_run_cycle(state);
        if seen.contains(&new_state) {
            return new_state;
        }

        state = new_state;
    }
}


fn day6() {
    assert_eq!(5, day6_pt1(vec!(0, 2, 7, 0)));
    println!("Part 1: cycled in steps: {}", day6_pt1(vec!(0, 5, 10, 0, 11, 14, 13, 4, 11, 8, 8, 7, 1, 4, 12, 11)));

    let cycle_start = day6_pt2_find_cycle_start(vec!(0, 5, 10, 0, 11, 14, 13, 4, 11, 8, 8, 7, 1, 4, 12, 11));
    println!("Part 2: cycled length: {}", day6_pt1(cycle_start));
}

///////////////// Day 7

use std::io::{BufRead, BufReader};
use std::fs::File;
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;


fn day7_pt1() {
    let f = File::open("advent-files/day7_input.txt").expect("open file");
    let br = BufReader::new(f);

    let mut names = Vec::<String>::new();
    let mut non_roots = Vec::<String>::new();

    for line in br.lines() {
        let s = line.unwrap();
        let parts: Vec<&str> = s.splitn(4, " ").collect();

        names.push(parts[0].to_owned());

        if parts.len() == 4 {
            let names: Vec<String> = parts[3].split(", ").map(|n| n.to_owned()).collect();
            non_roots.extend(names);
        }
    }

    for name in names {
        if !non_roots.contains(&name) {
            println!("Root: {}", name);
        }
    }
}

// Heavily inspired by https://github.com/nrc/r4cppp/blob/master/graphs/README.md
//
// The idea: children is a vector containing reference counted
// pointers to mutable cells that point to Program instances.
//
// RefCell is needed because Rc is a reference counted pointer to an
// immutable thing, so we need an extra level of indirection.
struct Program {
    name: String,
    weight: u32,
    children: Vec<Rc<RefCell<Program>>>,
}

fn day7_weigh(programs: &HashMap<String, Rc<RefCell<Program>>>,
              root: &Rc<RefCell<Program>>,
              indent: usize) -> u32 {
    return root.borrow().weight + root.borrow().children.iter().fold(0, |acc, child| {
        let child_weight = day7_weigh(&programs, child, indent + 2);
        println!("{:indent$}Child {} weight: {}", "", child.borrow().name, child_weight, indent = indent);
        acc + child_weight
    })
}

fn day7_check_balance(programs: &HashMap<String, Rc<RefCell<Program>>>, root: String) {
    day7_weigh(&programs, programs.get(&root).unwrap(), 0);
}

fn day7_pt2() {
    // Solution pulled from pt1
    let root_node = "svugo".to_owned();

    // Read each node
    let programs = {
        let f = File::open("advent-files/day7_input.txt").expect("open file");
        let br = BufReader::new(f);

        let mut result = HashMap::new();

        for line in br.lines() {
            let s = line.unwrap();
            let parts: Vec<&str> = s.splitn(4, " ").collect();

            let name = parts[0].to_owned();
            let weight = parts[1].to_owned().replace("(", "").replace(")", "").parse().unwrap();

            result.insert(name.clone(),
                          Rc::new(RefCell::new(Program { name: name, weight: weight, children: Vec::new() })));
        }

        result
    };

    // link them up
    {
        let f = File::open("advent-files/day7_input.txt").expect("open file");
        let br = BufReader::new(f);

        for line in br.lines() {
            let s = line.unwrap();
            let parts: Vec<&str> = s.splitn(4, " ").collect();

            let name = parts[0].to_owned();
            let this_program = programs.get(&name).unwrap();

            if parts.len() == 4 {
                let child_names: Vec<String> = parts[3].split(", ").map(|n| n.to_owned()).collect();

                for child_name in child_names {
                    let child = programs.get(&child_name).unwrap();
                    this_program.borrow_mut().children.push(child.clone());
                }
            }
        }
    };

    // Find which
    day7_check_balance(&programs, root_node);
}


fn day7() {
    day7_pt1();
    day7_pt2();
}


// Just as an experiment, here's a more tidy graph that just uses
// integer indexes instead of trying to track pointers.
//
// use std::collections::HashMap;
//
// #[derive(Debug)]
// struct Graph {
//     nodes: Vec<Node>,
//     connections: HashMap<usize, Vec<usize>>,
// }
//
// #[derive(Eq, Hash, PartialEq, Debug)]
// struct Node {
//     name: String,
//     weight: u64,
// }
//
// impl Graph {
//     pub fn new() -> Graph {
//         Graph { nodes: Vec::new(), connections: HashMap::new() }
//     }
//
//     pub fn add_node(self: &mut Graph, node: Node) -> usize {
//         let result = self.nodes.len();
//
//         self.connections.insert(result, Vec::new());
//         self.nodes.push(node);
//
//         result
//     }
//
//     pub fn link(self: &mut Graph, from_idx: usize, to_idx: usize) {
//         self.nodes.get(from_idx).unwrap();
//
//         self.connections.get_mut(&from_idx).unwrap().push(to_idx);
//     }
// }
//
//
// fn main() {
//     let mut g = Graph::new();
//
//     let a = g.add_node(Node { name: "A".to_owned(), weight: 65 });
//     let b = g.add_node(Node { name: "B".to_owned(), weight: 100 });
//     let c = g.add_node(Node { name: "C".to_owned(), weight: 200 });
//
//     g.link(a, b);
//     g.link(a, c);
//
//     println!("{:?}", g);
// }


///////////////// Day 8


use std::collections::HashMap;
use std::io::{BufRead, BufReader};
use std::fs::File;

#[derive(Debug)]
struct Instruction {
    target_register: String,
    operator: String,
    operand: i32,
    condition_register: String,
    condition_operator: String,
    condition_operand: i32,
}

fn parse_instruction(instruction: String) -> Instruction {
    let bits: Vec<String> = instruction.split(" ").map(|s| s.to_owned()).collect();

    Instruction {
        target_register: bits[0].clone(),
        operator: bits[1].clone(),
        operand: bits[2].parse().unwrap(),
        condition_register: bits[4].clone(),
        condition_operator: bits[5].clone(),
        condition_operand: bits[6].parse().unwrap(),
    }
}

// const DAY_8_SAMPLE: &[&str] = &[
//     "b inc 5 if a > 1",
//     "a inc 1 if b < 5",
//     "c dec -10 if a >= 1",
//     "c inc -20 if c == 10",
// ];

fn day8_pt1() {
    let mut registers: HashMap<String, i32> = HashMap::new();

    let f = File::open("advent-files/day8_input.txt").expect("open file");
    let br = BufReader::new(f);

    for line in br.lines() {
        let instruction = parse_instruction(line.unwrap());

        // Init our condition register if needed
        registers.entry(instruction.condition_register.clone()).or_insert(0);

        let condition_register_value = registers.get(&instruction.condition_register).unwrap().clone();

        let condition_matched = match instruction.condition_operator.as_ref() {
            "==" => (condition_register_value == instruction.condition_operand),
            ">=" => (condition_register_value >= instruction.condition_operand),
            ">" => (condition_register_value > instruction.condition_operand),
            "<" => (condition_register_value < instruction.condition_operand),
            "<=" => (condition_register_value <= instruction.condition_operand),
            "!=" => (condition_register_value != instruction.condition_operand),
            _ => panic!("Invalid instruction: {:?}", instruction),
        };

        if condition_matched {
            let target_register = registers.entry(instruction.target_register).or_insert(0);

            if instruction.operator == "inc" {
                *target_register += instruction.operand;
            } else {
                *target_register -= instruction.operand;
            }
        }
    }

    println!("max: {}", registers.values().fold(std::i32::MIN, |m, v| std::cmp::max(m, *v)))
}


fn day8_pt2() {
    let mut registers: HashMap<String, i32> = HashMap::new();

    let f = File::open("advent-files/day8_input.txt").expect("open file");
    let br = BufReader::new(f);

    let mut point_max = 0;

    for line in br.lines() {
        let instruction = parse_instruction(line.unwrap());

        // Init our condition register if needed
        registers.entry(instruction.condition_register.clone()).or_insert(0);

        let condition_register_value = registers.get(&instruction.condition_register).unwrap().clone();

        let condition_matched = match instruction.condition_operator.as_ref() {
            "==" => (condition_register_value == instruction.condition_operand),
            ">=" => (condition_register_value >= instruction.condition_operand),
            ">"  => (condition_register_value > instruction.condition_operand),
            "<"  => (condition_register_value < instruction.condition_operand),
            "<=" => (condition_register_value <= instruction.condition_operand),
            "!=" => (condition_register_value != instruction.condition_operand),
            _ => panic!("Invalid instruction: {:?}", instruction),
        };

        if condition_matched {
            let target_register = registers.entry(instruction.target_register).or_insert(0);

            if instruction.operator == "inc" {
                *target_register += instruction.operand;
            } else {
                *target_register -= instruction.operand;
            }

            if *target_register > point_max {
                point_max = *target_register;
            }
        }
    }

    println!("max at any point: {}", point_max)
}

fn day8() {
    day8_pt1();
    day8_pt2();
}


///////////////// Day 9

use std::io::{BufReader, Read};
use std::fs::File;

fn score(s: &str) -> (u32, u32) {
    let mut input: Vec<char> = s.chars().collect();
    let mut total_score = 0;
    let mut this_group_score = 0;
    let mut garbage_count = 0;

    while input.len() > 0 {
        let ch = input.remove(0);

        match ch {
            '{' => {
                this_group_score += 1;
                total_score += this_group_score;
            },
            '}' => {
                this_group_score -= 1;
            },
            ',' => {},
            '<' => {
                while input.len() > 0 {
                    let garbage_ch = input.remove(0);

                    match garbage_ch {
                        '>' => { break; }
                        '!' => {
                            // Skip the next too
                            if input.len() > 0 {
                                input.remove(0);
                            }
                        },
                        _ => { garbage_count += 1 },
                    }
                }
            }
            _ => { panic!("Invalid input: {}", ch) }
        }
    }

    (total_score, garbage_count)
}

fn day9() {
    assert_eq!(score("{}").0, 1);
    assert_eq!(score("{{{}}}").0, 6);
    assert_eq!(score("{{},{}}").0, 5);
    assert_eq!(score("{{{},{},{{}}}}").0, 16);
    assert_eq!(score("{<a>,<a>,<a>,<a>}").0, 1);
    assert_eq!(score("{{<ab>},{<ab>},{<ab>},{<ab>}}").0, 9);
    assert_eq!(score("{{<!!>},{<!!>},{<!!>},{<!!>}}").0, 9);
    assert_eq!(score("{{<a!>},{<a!>},{<a!>},{<ab>}}").0, 3);

    let f = File::open("advent-files/day9_input.txt").expect("open file");
    let mut br = BufReader::new(f);

    let mut input = String::new();
    br.read_to_string(&mut input).unwrap();

    let (score, garbage_count) = score(input.trim_right());

    println!("The score for my input is: {}", score);
    println!("The garbage count for my input is: {}", garbage_count);
}

///////////////// Day 10

// use std::cell::RefCell;
// use std::collections::HashMap;
// use std::collections::HashSet;
// use std::fs::File;
// use std::io::{BufRead, BufReader};
// use std::io::{BufReader, Read};
// use std::iter::FromIterator;
// use std::rc::Rc;


fn reverse_subseq(vec: &mut Vec<u32>, idx: usize, len: usize) {
    let mut start = idx;
    let mut end = (idx + len - 1) % vec.len();

    for _ in 0 .. (len / 2) {
        let tmp = vec[end];
        vec[end] = vec[start];
        vec[start] = tmp;

        start = (start + 1) % vec.len();
        end = if end == 0 { vec.len() - 1 } else { end - 1 }
    }
}


fn day10_pt1() {
    let mut nums: Vec<u32> = (0..256).collect();
    let mut pos = 0;
    let mut skip = 0;

    let inputs = vec!(130, 126, 1, 11, 140, 2, 255, 207, 18, 254, 246, 164, 29, 104, 0, 224);

    for i in inputs {
        reverse_subseq(&mut nums, pos, i);
        pos = (pos + i + skip) % nums.len();
        skip += 1;
    }

    println!("Result: {}", nums[0] * nums[1]);
}

fn day10_pt2() {
    let mut nums: Vec<u32> = (0..256).collect();
    let mut pos = 0;
    let mut skip = 0;

    let mut inputs: Vec<usize> = "130,126,1,11,140,2,255,207,18,254,246,164,29,104,0,224"
        .chars()
        .map(|c| c as usize)
        .collect();

    inputs.extend(vec!(17, 31, 73, 47, 23));

    for _round in 0..64 {
        for i in &inputs {
            reverse_subseq(&mut nums, pos, *i);
            pos = (pos + i + skip) % nums.len();
            skip += 1;
        }
    }

    let mut result = Vec::new();

    for block in 0..16 {
        let block_numbers: Vec<&u32> = nums.iter().skip(block * 16).take(16).collect();
        result.push(format!("{:02x}", block_numbers.iter().skip(1).fold(*block_numbers[0], |acc, n| acc ^ *n)))
    }

    println!("Result part 2: {}", result.join(""));
}

fn day10() {
    day10_pt1();
    day10_pt2();
}

///////////////// Day 11

use std::fs::File;
use std::io::{BufReader, Read};
use std::collections::HashMap;


fn sum_distance(map: &HashMap<&str, i32>) -> i32 {
    let mut nw = *map.get("nw").unwrap_or(&0);
    let mut ne = *map.get("ne").unwrap_or(&0);
    let mut sw = *map.get("sw").unwrap_or(&0);
    let mut se = *map.get("se").unwrap_or(&0);
    let mut n = *map.get("n").unwrap_or(&0);
    let mut s = *map.get("s").unwrap_or(&0);

    let mut total = 0;

    loop {
        // ne cancels sw
        let mut diff = std::cmp::min(sw, ne);
        sw -= diff;
        ne -= diff;

        // se cancels nw
        diff = std::cmp::min(se, nw);
        se -= diff;
        nw -= diff;

        // ne + nw = n
        diff = std::cmp::min(ne, nw);
        ne -= diff;
        nw -= diff;
        n += diff;

        // se + sw = s
        diff = std::cmp::min(se, sw);
        se -= diff;
        sw -= diff;
        s += diff;

        // ne + s = se
        diff = std::cmp::min(ne, s);
        ne -= diff;
        s -= diff;
        se += diff;

        // se + n = ne
        diff = std::cmp::min(se, n);
        se -= diff;
        n -= diff;
        ne += diff;

        // nw + s = sw
        diff = std::cmp::min(nw, s);
        nw -= diff;
        s -= diff;
        sw += diff;

        // sw + n = nw
        diff = std::cmp::min(sw, n);
        sw -= diff;
        n -= diff;
        nw += diff;

        // n cancels s
        diff = std::cmp::min(n, s);
        n -= diff;
        s -= diff;

        let new = nw + ne + sw + se + n + s;

        if new == total {
            // Hit a fixed point
            break;
        }

        total = new;
    }

    total
}

fn day11_pt1(directions: Vec<&str>) -> i32 {
    let map = directions.iter().fold(HashMap::new(), |mut map, dir| {
        {
            let entry = map.entry(*dir).or_insert(0);
            *entry += 1
        }
        map
    });

    sum_distance(&map)
}

fn day11_pt2(directions: Vec<&str>) -> i32 {
    let mut furthest_distance = 0;

    let mut map = HashMap::new();

    for dir in directions {
        {
            let entry = map.entry(dir).or_insert(0);
            *entry += 1
        }

        let distance = sum_distance(&map);

        furthest_distance = std::cmp::max(furthest_distance, distance);
    }

    furthest_distance
}


fn day11() {
    assert_eq!(day11_pt1("ne,ne,ne".split(",").collect()), 3);
    assert_eq!(day11_pt1("ne,ne,sw,sw".split(",").collect()), 0);
    assert_eq!(day11_pt1("ne,ne,s,s".split(",").collect()), 2);
    assert_eq!(day11_pt1("se,sw,se,sw,sw".split(",").collect()), 3);

    let f = File::open("advent-files/day11_input.txt").expect("open file");
    let mut br = BufReader::new(f);

    let mut input = String::new();
    br.read_to_string(&mut input).unwrap();

    println!("{}", day11_pt1(input.trim().split(",").collect()));
    println!("{}", day11_pt2(input.trim().split(",").collect()));

}


///////////////// Day 12

use std::fs::File;
use std::io::{BufReader, BufRead};
use regex::Regex;

extern crate regex;

fn day12_pt1() -> usize {
    let mut groups = [0; 2000];

    for i in 0..groups.len() {
        groups[i] = i;
    }

    let f = File::open("advent-files/day12_input.txt").expect("open file");
    let br = BufReader::new(f);

    let delim = Regex::new("(, | <-> )").unwrap();

    for line in br.lines().map(Result::unwrap) {
        let nodes: Vec<usize> = delim.split(&line).map(|s| s.parse().unwrap()).collect();

        // Nodes in the same group have the same value in `groups`
        let val = groups[nodes[0]];

        for n in nodes.iter().skip(1) {
            let replaceme = groups[*n];

            for i in 0 .. groups.len() {
                if groups[i] == replaceme {
                    groups[i] = val;
                }
            }
        }
    }

    let zero_id = groups[0];

    groups.iter().filter(|&&v| v == zero_id).count()
}


fn day12_pt2() -> usize {
    let mut groups = [0; 2000];

    for i in 0..groups.len() {
        groups[i] = i;
    }

    let f = File::open("advent-files/day12_input.txt").expect("open file");
    let br = BufReader::new(f);

    let delim = Regex::new("(, | <-> )").unwrap();

    for line in br.lines().map(Result::unwrap) {
        let nodes: Vec<usize> = delim.split(&line).map(|s| s.parse().unwrap()).collect();

        // Nodes in the same group have the same value in `groups`
        let val = groups[nodes[0]];

        for n in nodes.iter().skip(1) {
            let replaceme = groups[*n];

            for i in 0 .. groups.len() {
                if groups[i] == replaceme {
                    groups[i] = val;
                }
            }
        }
    }

    let mut v = groups.to_vec();
    v.sort();
    v.dedup();

    v.len()
}

fn day12() {
    println!("Number in group zero: {}", day12_pt1());
    println!("Total groups: {}", day12_pt2());
}


#[derive(Debug)]
struct Firewall {
    active: bool,
    at_position: usize,
    sequence: Vec<usize>,
    range: usize,
}

impl Firewall {
    fn step(&mut self, n: usize) {
        if self.active {
            self.at_position = self.at_position + n;

            while self.at_position >= self.sequence.len() {
                self.at_position = self.at_position - self.sequence.len();
            }
        }
    }

    fn position(&self) -> i32 {
        if self.active {
            self.sequence[self.at_position] as i32
        } else {
            -1
        }
    }

    fn _reset(&mut self, n: usize) {
        self.at_position = n;
    }
}

fn day13_pt1(mut layers: Vec<Firewall>) -> usize {
    let mut packet_position = 0;
    let mut severity = 0;
    for _ in  0..layers.len() {
        if layers[packet_position].position() == 0 {
            severity += packet_position * layers[packet_position].range;
        }

        packet_position += 1;

        for layer in layers.iter_mut() {
            layer.step(1);
        }
    }

    severity
}

fn _day13_pt2(mut layers: Vec<Firewall>) -> usize {
    let mut delay = 0;

    loop {
        let start_positions: Vec<usize> = layers.iter().map(|layer| layer.at_position ).collect();

        // println!("{:?}", layers);
        // println!("{:?}", start_positions);

        let mut packet_position = 0;
        let mut hit = false;

        for _ in  0..layers.len() {
            if layers[packet_position].position() == 0 {
                hit = true;
                break;
            }

            packet_position += 1;

            for layer in layers.iter_mut() {
                layer.step(1);
            }
        }

        if !hit {
            return delay;
        }

        for (ref mut layer, &position) in layers.iter_mut().zip(&start_positions) {
            layer._reset(position);
            layer.step(1);
        }

        delay += 1;
    }
}

fn day13_pt2_faster(mut layers: Vec<Firewall>) -> usize {
    let mut delay = 0;

    for i in 0..layers.len() {
            layers[i].step(i);
    }

    loop {
        if !layers.iter().any(|layer| layer.position() == 0) {
            return delay;
        }

        for layer in layers.iter_mut() {
            layer.step(1);
        }

        delay += 1
    }
}

fn day13_input() -> Vec<Firewall> {
    // let input = "0: 3\n1: 2\n4: 4\n6: 4\n";
    let input = "0: 5\n1: 2\n2: 3\n4: 4\n6: 6\n8: 4\n10: 8\n12: 6\n14: 6\n16: 8\n18: 6\n20: 9\n22: 8\n24: 10\n26: 8\n28: 8\n30: 12\n32: 8\n34: 12\n36: 10\n38: 12\n40: 12\n42: 12\n44: 12\n46: 12\n48: 14\n50: 12\n52: 14\n54: 12\n56: 14\n58: 12\n60: 14\n62: 14\n64: 14\n66: 14\n68: 14\n70: 14\n72: 14\n76: 14\n80: 18\n84: 14\n90: 18\n92: 17\n";

    let max_depth = input.trim().split("\n").map(|line| {
        let bits: Vec<&str> = line.split(": ").collect();
        bits[0].parse().unwrap()
    }).fold(0, std::cmp::max);

    let mut layers: Vec<Firewall> = (0..max_depth + 1).map(|_| Firewall { active: false, range: 0, at_position: 0, sequence: Vec::new() }).collect();

    for descr in input.trim().split("\n") {
        let v: Vec<usize> = descr.split(": ").map(|s| { s.parse().unwrap() }).collect();
        let (depth, range) = (v[0], v[1]);

        layers[depth] = Firewall { active: true, at_position: 0, range, sequence: (0..range).chain((0..range).skip(1).rev().skip(1)).collect() }
    }

    layers
}

fn day13() {
    println!("Part1: {}", day13_pt1(day13_input()));
    println!("Optimal delay: {}", day13_pt2_faster(day13_input()));
    // println!("Optimal delay: {}", day13_pt2(&mut layers));
}


///////////////// Day 14

fn reverse_subseq(vec: &mut Vec<u32>, idx: usize, len: usize) {
    let mut start = idx;
    let mut end = (idx + len - 1) % vec.len();

    for _ in 0 .. (len / 2) {
        let tmp = vec[end];
        vec[end] = vec[start];
        vec[start] = tmp;

        start = (start + 1) % vec.len();
        end = if end == 0 { vec.len() - 1 } else { end - 1 }
    }
}

fn knot_hash(input: &str) -> Vec<u8> {
    let mut nums: Vec<u32> = (0..256).collect();
    let mut pos = 0;
    let mut skip = 0;

    let mut inputs: Vec<usize> = input.chars().map(|c| c as usize).collect();
    inputs.extend(vec!(17, 31, 73, 47, 23));


    for _round in 0..64 {
        for i in &inputs {
            reverse_subseq(&mut nums, pos, *i);
            pos = (pos + i + skip) % nums.len();
            skip += 1;
        }
    }

    let result: Vec<u8> = (0..16).map(|block| {
        let block_numbers: Vec<&u32> = nums.iter().skip(block * 16).take(16).collect();
        block_numbers.iter().skip(1).fold(*block_numbers[0], |acc, n| acc ^ *n) as u8
    }).collect();

    result
}


fn day14_pt1() {
    let key = "hxtvlmkl";

    let mut squares_used = 0;

    for round in 0..128 {
        let input = format!("{}-{}", key, round);

        for mut b in knot_hash(&input) {
            while b > 0 {
                squares_used += (b & 1) as usize;
                b = b >> 1;
            }
        }
    }

    println!("Squares used: {}", squares_used);
}

fn day14_pt2() {
    let key = "hxtvlmkl";

    // Populate our grid
    let mut grid: Vec<Vec<u32>> = Vec::new();

    for round in 0..128 {
        let mut row: Vec<u32> = Vec::new();
        let input = format!("{}-{}", key, round);

        for mut b in knot_hash(&input) {
            let mut bits: Vec<u32> = Vec::new();
            for _ in 0..8 {
                bits.insert(0, (b & 1) as u32);
                b = b >> 1;
            }

            row.extend(bits);
        }

        grid.push(row);
    }

    // Each cell in its own group to start with
    let mut groups: Vec<Vec<u32>> = Vec::new();

    for y in 0..128 {
        let mut row = Vec::new();

        for x in 0..128 {
            row.push((y * 128 + x) as u32);
        }

        groups.push(row);
    }

    // Merge groups left to right within rows
    for y in 0..128 {
        for x in 0..127 {
            if grid[y][x] == 1 && grid[y][x + 1] == 1 {
                groups[y][x + 1] = groups[y][x];
            }
        }
    }

    // Merge groups top to bottom globally
    for y in 0..127 {
        for x in 0..128 {
            if grid[y][x] == 1 && grid[y + 1][x] == 1 {
                let victor_value = groups[y][x];
                let victim_value = groups[y + 1][x];

                for i in 0..128 {
                    for j in 0..128 {
                        if groups[i][j] == victim_value {
                            groups[i][j] = victor_value;
                        }
                    }
                }
            }
        }
    }

    let mut group_numbers: Vec<u32> = Vec::new();
    for y in 0..128 {
        for x in 0..128 {
            if grid[y][x] == 1 {
                group_numbers.push(groups[y][x]);
            }
        }
    }

    group_numbers.sort();
    group_numbers.dedup();
    println!("Unique groups: {}", group_numbers.len());
}


fn day14() {
    day14_pt1();
    day14_pt2();
}


///////////////// Day 15

struct Generator {
    factor: usize,
    value: usize,
    required_multiple: usize,
}

impl Iterator for Generator {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        loop {
            self.value = (self.value * self.factor) % 2147483647;

            if (self.value % self.required_multiple) == 0 {
                return Some(self.value)
            }
        }
    }
}

fn day15_pt1() {
    let a = Generator { value: 679, factor: 16807, required_multiple: 1 };
    let b = Generator { value: 771, factor: 48271, required_multiple: 1 };

    let sample_size = 40000000;

    println!("{}", a.take(sample_size).zip(b.take(sample_size)).filter(|&(a, b)| {
        (a & 0xFFFF) == (b & 0xFFFF)
    }).count());
}

fn day15_pt2() {
    let a = Generator { value: 679, factor: 16807, required_multiple: 4 };
    let b = Generator { value: 771, factor: 48271, required_multiple: 8 };

    let sample_size = 5000000;

    println!("{}", a.take(sample_size).zip(b.take(sample_size)).filter(|&(a, b)| {
        (a & 0xFFFF) == (b & 0xFFFF)
    }).count());
}

fn day15() {
    day15_pt1();
    day15_pt2();
}


///////////////// Day 16

use std::fs::File;
use std::io::{BufReader, Read};


fn day16_input() -> String {
    let f = File::open("advent-files/day16_input.txt").expect("open file");
    let mut br = BufReader::new(f);

    let mut input = String::new();
    br.read_to_string(&mut input).unwrap();

    input
}

fn day16_apply_moves<'a>(state: &'a mut Vec<&str>, moves: &Vec<&str>) -> &'a Vec<&'a str> {
    for m in moves {
        match m.chars().nth(0).unwrap() {
            's' => {
                let spin_length: usize = m[1..].parse().unwrap();

                for _ in 0..spin_length {
                    let len = state.len();
                    let elt = state.remove(len - 1);
                    state.insert(0, elt);
                }
            },
            'x' => {
                let bits: Vec<&str> = m[1..].split("/").collect();
                let a: usize  = bits[0].parse().unwrap();
                let b: usize  = bits[1].parse().unwrap();

                let tmp = state[a];
                state[a] = state[b];
                state[b] = tmp;
            },
            'p' => {
                let bits: Vec<&str> = m[1..].split("/").collect();

                let a = state.iter().position(|&x| x == bits[0]).unwrap();
                let b = state.iter().position(|&x| x == bits[1]).unwrap();

                let tmp = state[a];
                state[a] = state[b];
                state[b] = tmp;
            },
            _ => { panic!("Unrecognised line") },
        }
    }

    state
}

fn day16_pt1(state: &mut Vec<&str>, moves: &Vec<&str>)  {
   println!("{}", day16_apply_moves(state, moves).join(""));
}

fn build_index_mapping(mut state: Vec<&str>, moves: &Vec<&str>, repeats: usize) -> Vec<usize> {
    let pre_state = state.clone();
    let index_transforms: Vec<&str> = moves.iter().filter(|m| m.chars().nth(0).unwrap() != 'p').cloned().collect();

    for _ in 0..repeats {
        day16_apply_moves(&mut state, &index_transforms);
    }

    let mut mapping = Vec::new();

    for i in 0..pre_state.len() {
        let ch = state[i];
        let target_pos = pre_state.iter().position(|&c| c == ch);

        mapping.push(target_pos.unwrap());
    }

    mapping
}

fn day16_pt2(state: &mut Vec<&str>, moves: &Vec<&str>)  {
    {
        // optimization: batch together 1000 runs so we don't have to do as many iterations below
        let batch_size = 1000;
        let mapping = build_index_mapping(state.clone(), moves, batch_size);
        let mut scratch = vec!(""; mapping.len());

        for _ in 0..(1000000000 / batch_size) {
            for i in 0..mapping.len() {
                scratch[i] = state[mapping[i]];
            }

            for i in 0..mapping.len() {
                state[i] = scratch[i];
            }
        }
    }

    // Observation: the 'swap by value' rules cycle every 8 iterations.  Since 1
    // billion mod 8 == 0, the effect of running them a billion times is the
    // same as not running them at all.  So we don't!

    println!("{}", state.join(""));
}


fn day16() {
    let input = day16_input();
    let state: Vec<&str> = "abcdefghijklmnop".split("").filter(|s| s.len() > 0).collect();
    let moves: Vec<&str> = input.trim().split(",").collect();

    day16_pt1(&mut state.clone(), &moves);
    day16_pt2(&mut state.clone(), &moves);
}

fn day17_pt1() {
    let step = 303;

    let mut buf = vec!(0);
    let mut pos = 0;

    for r in 1..2018 {
        pos = (pos + step + 1) % buf.len();
        buf.insert(pos, r);
    }


    println!("{}", buf[pos + 1]);
}

fn day17_pt2() {
    let step = 303;

    let mut bufsize = 1;
    let mut pos = 0;
    let mut answer = 0;

    // The insight here is that you don't need to store all values to know what
    // comes after zero.  You'll know what it was 'cos you put it there...
    for r in 1..50000001 {
        pos = (pos + step) % bufsize;

        if pos == 0 {
            answer = r;
        }

        bufsize += 1;
        pos += 1;
    }


    println!("{}", answer);
}

fn day17() {
    day17_pt1();
    day17_pt2();
}


///////////////// Day 18

use std::collections::HashMap;

const _SAMPLE_PROGRAM: &str = "
set a 1
add a 2
mul a a
mod a 5
snd a
set a 0
rcv a
jgz a -1
set a 1
jgz a -2
";

const _SAMPLE_PROGRAM_2: &str = "
snd 1
snd 2
snd p
rcv a
rcv b
rcv c
rcv d
";

const DAY18_INPUT: &str = "
set i 31
set a 1
mul p 17
jgz p p
mul a 2
add i -1
jgz i -2
add a -1
set i 127
set p 618
mul p 8505
mod p a
mul p 129749
add p 12345
mod p a
set b p
mod b 10000
snd b
add i -1
jgz i -9
jgz a 3
rcv b
jgz b -1
set f 0
set i 126
rcv a
rcv b
set p a
mul p -1
add p b
jgz p 4
snd a
set a b
jgz 1 3
snd b
set f 1
add i -1
jgz i -11
snd a
jgz f -16
jgz a -19
";

fn to_register(name: &str) -> char {
    name.chars().nth(0).unwrap()
}

fn deref_value(value: &str, registers: &HashMap<char, i64>) -> i64 {
    match to_register(value) {
        r @ 'a'...'z' => {
            *registers.get(&r).unwrap()
        },
        _ => { value.parse().unwrap() }
    }
}


fn day18_pt1() {
    let instructions: Vec<&str> = DAY18_INPUT.trim().split("\n").collect();

    let mut registers = "abcdefghijklmnopqrstuvwxyz".chars().fold(HashMap::new(), |mut acc, register| {
        acc.insert(register, 0);
        acc
    });

    let mut pc: i64 = 0;
    let mut last_snd: i64 = 0;
    let mut recovered: Vec<i64> = Vec::new();

    loop {
        if pc < 0 || pc >= (instructions.len() as i64) {
            break;
        }

        let instruction = instructions[pc as usize];

        let bits: Vec<&str> = instruction.split(" ").collect();

        match bits[0] {
            "snd" => {
                last_snd = deref_value(bits[1], &registers);
            },
            "set" => {
                let value = deref_value(bits[2], &registers);
                registers.insert(to_register(bits[1]), value);
            },
            "add" => {
                let new_value = deref_value(bits[1], &registers) + deref_value(bits[2], &registers);
                registers.insert(to_register(bits[1]), new_value);
            },
            "mul" => {
                let new_value = deref_value(bits[1], &registers) * deref_value(bits[2], &registers);
                registers.insert(to_register(bits[1]), new_value);
            },
            "mod" => {
                let new_value = deref_value(bits[1], &registers) % deref_value(bits[2], &registers);
                registers.insert(to_register(bits[1]), new_value);
            },
            "rcv" => {
                let x = deref_value(bits[1], &registers);

                if x != 0 {
                    recovered.push(last_snd);
                    break;
                }
            },
            "jgz" => {
                let x = deref_value(bits[1], &registers);
                let y = deref_value(bits[2], &registers);

                if x > 0 {
                    // Compensate for the increment we're going to get anyway.
                    pc -= 1;
                    pc += y;
                }
            },
            _ => { panic!("WTF?!"); },
        }

        pc += 1;
    }

    println!("{:?}", recovered);
}

#[derive(PartialEq)]
enum ProgramState {
    RUNNING,
    FINISHED,
    WAITING,
}

struct Program {
    program_id: i64,
    instructions: Vec<&'static str>,
    registers: HashMap<char, i64>,
    send_count: usize,
    pc: i64,
    state: ProgramState,
    outbox: Vec<i64>,
    inbox: Vec<i64>,
}

impl Program {
    fn new(instruction_text: &'static str, program_id: i64) -> Program {
        Program {
            program_id: program_id,
            instructions: instruction_text.trim().split("\n").collect(),
            registers: "abcdefghijklmnopqrstuvwxyz".chars().fold(HashMap::new(), |mut acc, register| {
                acc.insert(register, if register == 'p' { program_id } else { 0 });
                acc
            }),
            pc: 0,
            send_count: 0,
            state: ProgramState::RUNNING,
            outbox: Vec::new(),
            inbox: Vec::new(),
        }
    }

    fn deref_value(self: &Program, value: &str) -> i64 {
        match to_register(value) {
            r @ 'a'...'z' => {
                *self.registers.get(&r).unwrap()
            },
            _ => { value.parse().unwrap() }
        }
    }

    fn step(self: &mut Program) {
        if self.state == ProgramState::FINISHED {
            return;
        }

        let instruction = self.instructions[self.pc as usize];

        // println!("{}: [{}] {}", self.program_id, self.pc, instruction);

        let bits: Vec<&str> = instruction.split(" ").collect();

        match bits[0] {
            "snd" => {
                let value = self.deref_value(bits[1]);
                self.send_count += 1;
                self.outbox.push(value);
            },
            "set" => {
                let value = self.deref_value(bits[2]);
                self.registers.insert(to_register(bits[1]), value);
            },
            "add" => {
                let new_value = self.deref_value(bits[1]) + self.deref_value(bits[2]);
                self.registers.insert(to_register(bits[1]), new_value);
            },
            "mul" => {
                let new_value = self.deref_value(bits[1]) * self.deref_value(bits[2]);
                self.registers.insert(to_register(bits[1]), new_value);
            },
            "mod" => {
                let new_value = self.deref_value(bits[1]) % self.deref_value(bits[2]);
                self.registers.insert(to_register(bits[1]), new_value);
            },
            "rcv" => {
                if self.inbox.len() > 0 {
                    let value = self.inbox.remove(0);
                    // println!("{}: received {}", self.program_id, value);
                    self.registers.insert(to_register(bits[1]), value);
                    self.state = ProgramState::RUNNING;
                } else {
                    self.pc -= 1;
                    self.state = ProgramState::WAITING;
                }
            },
            "jgz" => {
                let x = self.deref_value(bits[1]);
                let y = self.deref_value(bits[2]);

                if x > 0 {
                    // Compensate for the increment we're going to get anyway.
                    self.pc -= 1;
                    self.pc += y;
                }
            },
            _ => { panic!("WTF?!"); },
        }

        self.pc += 1;

        if self.pc < 0 || self.pc >= (self.instructions.len() as i64) {
            self.state = ProgramState::FINISHED;
        }
    }
}

fn day18_pt2() {
    let mut p0 = Program::new(DAY18_INPUT, 0);
    let mut p1 = Program::new(DAY18_INPUT, 1);

    while p0.state == ProgramState::RUNNING || p1.state == ProgramState::RUNNING {
        p0.step();
        p1.step();

        // Deliver mail!
        while p0.outbox.len() > 0 { p1.inbox.push(p0.outbox.pop().unwrap()); }
        while p1.outbox.len() > 0 { p0.inbox.push(p1.outbox.pop().unwrap()); }
    }

    println!("Program 1 sent {} times", p1.send_count);
}


fn day18() {
    day18_pt1();
    day18_pt2();
}


///////////////// Day 19

#[derive(Debug, Copy, Clone)]
struct Position (usize, usize);


#[derive(Debug, Copy, Clone)]
struct Direction (i32, i32);

const UP: Direction = Direction(0, -1);
const RIGHT: Direction = Direction(1, 0);
const DOWN: Direction = Direction(0, 1);
const LEFT: Direction = Direction(-1, 0);

const DIRECTIONS: &[Direction] = &[UP, RIGHT, DOWN, LEFT];

const SAMPLE_INPUT: &str = "     |          
     |  +--+    
     A  |  C    
 F---|----E|--+ 
     |  |  |  D 
     +B-+  +--+ ";

const REAL_INPUT: &str =" |                                                                                                                                                                                                       
 | +-----------------------------------------+         +-----------------------------------+           +-+ +-+     +---------------------------------------------------------+ +-+           +---+       
 | |                                         |         |                                   |           | | | |     |                                                         | | |           |   |       
 | |                                         |   +-----------------------------------------------------|-|-|-------|---------------------------------------------------------+ | |           |   |       
 | |                                         |   |     |                                   |           | | | |     |                                                           | |           |   |       
 | |                                         |   |     |                                   +---+       | +-|-----+ | +-----------+             +-+             +---------------|-|-----------+   |       
 | |                                         |   |     |                                       |       |   | |   | | |           |             | |             |               | |               |       
 | |                                         |   |     |                         +-------------|-------------------|-|-----------------------------------------+       +-------|-|---------+     |       
 | |                                         |   |     |                         |             |       |   | |   | | |           |             | |                     |       | |         |     |       
 | |                                         |   |     |                         +-------------|-------|-------------------------|-----------------------------------------+   | |         |     |       
 | |                                         |   |     |                                       |       |   | |   | | |           |             | |                     |   |   | |         |     |       
 | |                 +-----------------------|---------|-----------------------------------------------|---|-----|-|-|-----------|---------------------+   +-------+   |   |   | |   +-----|-+   |       
 | |                 |                       |   |     |                                       |       |   | |   | | |           |             | |     |   |       |   |   |   | |   |     | |   |       
 | |                 |                       |   |     |                       +-----------------------|---+ |   | | |           |             | |     |   |     +-|-------|---|-----+     | |   |       
 | |                 |                       |   |     |                       |               |       |     |   | | |           |             | |     |   |     | |   |   |   | |         | |   |       
 | |       +---------|-------------------------------------------------------------------------|-------------------|-|-----------|---------------|---------------|---------------|---------------|-----+ 
 | |       |         |                       |   |     |                       |               |       |     |   | | |           |             | |     |   |     | |   |   |   | |         | |   |     | 
 | |       |         |                       |   |     |                       |               |       |     |   | | |           |             | |     |   |     | |   |   |   | |         | |   |     | 
 | |       |         |                       |   |     |                       |               |       |     |   | | |           |             | |     |   |     | |   |   |   | |         | |   |     | 
 | |       |         |                 +-----|---------------------------+     |               | +-----|---------|-|-|---------+ |             | | +---|---|---------------+   | |       +-|-----|---+ | 
 | |       |         |                 |     |   |     |                 |     |               | |     |     |   | | |         | |             | | |   |   |     | |   |       | |       | | |   |   | | 
 | |       |         +-----------------|---------|-----------------------------------------------|-----|-----------|-------------|-------------|---+   |   |     | |   |       | |       | | |   |   | | 
 | |       |                           |     |   |     |                 |     |               | |     |     |   | | |         | |             | |     |   |     | |   |       | |       | | |   |   | | 
 | |       |                           |     |   |     |                 +-----|---------------|-|-----|---------|---+         | |             +-----------|-----------|-------------+   | | |   |   | | 
 | |       |                           |     |   |     |                       |               | |     |     |   | |           | |               |     |   |     | |   |       | |   |   | | |   |   | O 
 | |       |                           |     |   |     |                       |             +-|-------|-----------|-----------|-|-------+       |     |   |     | |   |       | |   |   | | |   |   | | 
 | |       |                           |     |   |     |                       |             | | |     |     |   | |           | |       |       |     |   |     | |   |       | |   |   | | |   |   | | 
 | |       |                           |     |   |     |                       |             | | |     |     |   | |           +-+       |       |     |   |     | |   |       | |   |   | | |   |   | | 
 | |       |                           |     |   |     |                       |             | | |     |     |   | |                     |       |     |   |     | |   |       | |   |   | | |   |   | | 
 | |       |                           |     |   |     |                       |             | | |     | +-------|-|---------------------|---+ +-----+ |   |     | |   |       | |   |   | | |   |   | | 
 | |       |                           |     |   |     |                       |             | | |     | |   |   | |                     |   | | |   | |   |     | |   |       | |   |   | | |   |   | | 
 | +---------+                         |     |   +-----|-----------------------|-------------|-|-------|-|---|-----|-+   +-+ +-+ +-------|-+ | | |   | |   |     | |   |       | |   |   | | |   |   | | 
 |         | |                         |     |         |                       |             | | |     | |   |   | | |   | | | | |       | | | | |   | |   |     | |   |       | |   |   | | |   |   | | 
 |         | |                         |     |         |                       |             | | |     | |   |   | | |   | | | | |       | | | | |   | |   |     | |   |   +-----|-------|-|-|---|-+ | | 
 |         | |                         |     |         |                       |             | | |     | |   |   | | |   | | | | |       | | | | |   | |   |     | |   |   |   | |   |   | | |   | | | | 
 |         | |                         |     |         |                       |             | | |     | |   |   | | |   | | | | |       | | | | |   | |   |     | |   |   |   | |   |   | | |   | | | | 
 |         | |                         |     |         |                       |             | | |     | |   |   | | |   | | | | |       | | | | |   | |   |     | |   |   |   | |   |   | | |   | | | | 
 |         | |                         +-----|---------------------------------------------------|-----|-----|---|-|K--------|---|-------|-----|-----|-------------|-------|---|-+   |   | | |   | | | | 
 |         | |                               |         |                       |             | | |     | |   |   | | |   | | | | |       | | | | |   | |   |     | |   |   |   |     |   | | |   | | | | 
 |         | |       +-----------------------|-----------------------------------------------|---|-----|-|-------|---+   | | | | |       | +-----------|---|-----|-|-----+ |   | +---|-+ | | |   | | | | 
 |         | |       |                       |         |                       |             | | |     | |   |   | |     | | | | |       |   | | |   | |   |     | |   | | |   | |   | | | | |   | | | | 
 |         | +-------|---------------------------------|-------------------------------------|---------|-|-------|-|-----------|-|-----+ | +---+ |   | |   |     | |   | | |   | |   | | | | |   | | | | 
 |         |         |                       |         |                       |             | | |     | |   |   | |     | | | | |     | | | |   |   | |   |     | |   | | |   | |   | | | | |   | | | | 
 |         |         |                       | +---------------------------------------------------------|---|---|-------|---|-|-|---------|-|-----+ +-|-----------|---|---|---|-|-------|---------+ | | 
 |         |         |                       | |       |                       |             | | |     | |   |   | |     | | | | |     | | | |   | |   |   |     | |   | | |   | |   | | | | |   |   | | 
 |         |         |                 +-+   | |       | +---------------------------------------|-----|-|---|-----------|---|---|-----|-|-|-|---|-+   |   |     | |   | | |   | |   | | | | |   |   | | 
 |         |         |                 | |   | |       | |                     |             | | |     | |   |   | |     | | | | |     | | | |   |     |   |     | |   | | |   | |   | | | | |   |   | | 
 |         |         |         +---------|---|-|-------|---------+             |             | | |     | |   | +---|---------|-|---------|---|-------------------------|-|-----|-|---|-|-+ | |   |   | | 
 |         |         |         |       | |   | |       | |       |             |             | | |     | |   | | | |     | | | | |     | | | |   |     |   |     | |   | | |   | |   | |   | |   |   | | 
 |         |     +-----------------------------|-----------+     |             |             | | |     | |   | | | |     | | | | |     | | | |   |     |   |     | |   | | |   | |   | |   | |   |   | | 
 |         |     |   |         |       | |   | |       | | |     |             |             | | |     | |   | | | |     | | | | |     | | | |   |     |   |     | |   | | |   | |   | |   | |   |   | | 
 |     +-----------+ | +-+     |       | |   | |       | | |     |             |             | | |     | |   | | | |     | | | | |     | | | |   |     |   |     | |   | | |   | |   | |   | |   |   | | 
 |     |   |     | | | | |     |       | |   | |       | | |     |             |             | | |     | |   | | | |     | | | | |     | | | |   |     |   |     | |   | | |   | |   | |   | |   |   | | 
 |     |   |     | | | | |     |       | |   | |       | | |     |             +-------------+ +-|-----------|-|---------+ | | | |     | | | |   |     |   |     | |   | | |   | |   | |   | |   |   | | 
 |     |   |     | | | | |     |       | |   | |       | | |     |                               |     | |   | | | |       | | | |     | | | |   |     |   |     | |   | | |   | |   | |   | |   |   | | 
 |     |   |     | | | | |     |       | |   | |       | | |     |                               |     | |   | | | |       | | | |     | +-----+ |     |   |     | |   | | |   | |   | |   | |   |   | | 
 |     |   |     | | | | |     |       | |   | |       | | |     |                               |     | |   | | | |       | | | |     |   | | | |     |   |     | |   | | |   | |   | |   | |   |   | | 
 |     |   |     | +-----|-------------|-|---|---------------------------------------+           |     | |   | | | |       | | | | +---|-----|-------------|-----|-----|-|-------|---------|-+   | +-|-+ 
 |     |   |     |   | | |     |       | |   | |       | | |     |                   |           |     | |   | | | |       | | | | |   |   | | | |     |   |     | |   | | |   | |   | |   |     | | |   
 |     |   |     | +-|---|-------------|-----|-|-----------|-------------------------|-----------|-------|---------|-----------|---|---------|-|-------|-----------|-------|---|-------------+   | | |   
 |     |   |     | | | | |     |       | |   | |       | | |     |                   |           |     | |   | | | |       | | | | |   |   | | | |     |   |     | |   | | |   | |   | |   | |   | | |   
 |     |   |     | | | | |     |       | |   | |       | | |     |                   |           |     | |   | | | |   +-----|---|-|-------------|-----|---|-----|---+ | | +---|-|---|-----------|---|-+ 
 |     |   |     | | | | |     |       | |   | |       | | |     |                   |           |     | |   | | | |   |   | | | | |   |   | | | |     |   |     | | | | |     | |   | |   | |   | | | | 
 | +-----------------|-|-|-------------|-|---+ |       | | |     |           +-------|-------+   |     | |   | | | |   |   | | | | |   |   | | | |     |   |     | | | | |     | |   | |   | |   | | | | 
 | |   |   |     | | | | |     |       | |     |       | | |     |           |       |       |   |     | |   | | | |   |   | | | | |   |   | | | |     |   |     | | | | |     | |   | |   | |   | | | | 
 | |   |   |     | | | | |     |       | |     |       | | |     |           |       |       |   |     | |   | | | |   |   | | | | |   |   | | | |     |   |     | | | | |     | +-----------------|-|-+ 
 | |   |   |     | | | | W     |       | |     |       | | |     |           |       |       |   |     | |   | | | |   |   | | | | |   |   | | | |     |   |     | | | | |     |     | |   | |   | | |   
 | |   |   |     | | | | |     |       | |     |     +---|-------|-------------------|-+     |   |     | |   | | | |   |   | | | | |   | +-|-|-|-----------------|---|-+ +-----|---+ | |   | |   | | |   
 | |   |   |     | | | | |     |       | |     |     | | | |     |           |       | |     |   |     | |   | | | |   |   | | | | |   | | | | | |     |   |     | | |         |   | | |   | |   | | |   
 | |   |   |     | | | | |     |       | |     |     | | | |     |           |       | |     |   |     | |   | | +-----|-----|-----|-----|-|-|---+     |   |     | | | +-+     |   | | |   | |   | | |   
 | |   |   |     | | | | |     |       | |     |     | | | |     |           |       | |     |   |     | |   | |   |   |   | | | | |   | | | | |       |   |     | | | | |     |   | | |   | |   | | |   
 | |   |   |     | | | | |     |     +-|-|-----|-------|-|-----------------------------|---+ |   |     | |   | |   +---|---|-|-|-|-----|-|-|-|-|-----------|--R--|---+ | |     |   +-|-+   | |   | | |   
 | |   |   |     | | | | |     |     | | |     |     | | | |     |           |       | |   | |   |     | |   | |       |   | | | | |   | | | | |       |   |     | |   | |     |     |     | |   | | |   
 | |   +---------|-------------|---------------|-----|-|-|-------------------+       | |   | |   |     | |   | |       |   | | | | |   | | | | |       |   |     | |   | |     |   +-+ +---|---+ | | |   
 | |       |     | | | | |     |     | | |     |     | | | |     |                   | |   | |   |     | |   | |       |   | | | | |   | | | | |       |   |     | |   | |     |   |   |   | | | | | |   
 | |       |     | | | | |     |     | | |     |     | | | |     |                   +-|---------------|-|---|-|-------------|-|-|-----|-|-|-----------|---------+ |   | |     |   |   |   | | | | | |   
 | |       |     | | | | |     |     | | |     |     | | | |     |                     |   | |   |     | |   | |       |   | | | | |   | | | | |       |   |       |   | |     |   |   |   | | | | | |   
 | |       |     | | | | |     |     | | +-----------------------|---------------------|---|-----|-----|-------|-------|---|-----|-----|---|-|---------|-+ |       |   | |     +---+   |   | | | | | |   
 | |       |     | | | | |     |     | |       |     | | | |     |                     |   | |   |     | |   | |       |   | | | | |   | | | | |       | | |       |   | |             |   | | | | | |   
 | | +-----|---------|-----------------|-------|-----|-|---|-----|---------------------------|---|-------------|-----------------|-|-----------|-----+ | | |       |   | |             |   | | | | | |   
 | | |     |     | | | | |     |     | |       |     | | | |     |                     |   | |   |     | |   | |       |   | | | | |   | | | | |     | | | |       |   | |             |   | | | | | |   
 | | +---+ |     | | | | |     |     | +-----------+ | | | |     |                     |   | |   |     | |   | |       |   | | | | |   | | | | |     | +-|-|-----------|-|-------+ +-------|-|-----|---+ 
 | |     | |     | | | | |     |     |         |   | | | | |     |                     |   | |   |     | |   | |       |   | | | | |   | | | | |     |   | |       |   | |       | |   |   | | | | | | | 
 +-|-------------|-|-|-|-------------|-------------|-------------|-------------------------|-----------|-----------------+ | | | | |   | | | | |     |   | |       |   | |       | |   |   | | | | | | | 
   |     | |     | | | | |     |     |         |   | | | | |     |                     |   | |   |     | |   | |       | | | | | | |   | | | | |     |   | |       |   | |       | |   |   | | | | | | | 
 +---+   | |     | | | | |     |     |         |   | | | | |     |                     +---------------|-+   | |       | | | | | | |   | | | | |     |   | |       |   | |       | |   |   | | | | | | | 
 | | |   | |     | | | | |     |     |         |   | | | | |     |                         | |   |     |     | |       | | | | | | |   | | | | |     |   | |       |   | |       | |   |   | | | | | | | 
 | | |   | |   +---|-----|-----|-----|-+       |   | | | | |     |                     +-----|---|-----|-----|-------------|-----------|-|---|-------------|-------|-------------|-+   |   | | | | | | | 
 | | |   | |   | | | | | |     |     | |       |   | | | | |     |                     |   | |   |     |     | |       | | | | | | |   | | | | |     |   | |       |   | |       |     |   | | | | | | | 
 | | |   | |   | | | | | |   +-+     | |       |   | | | | |     |                   +-+   | |   |     |     | |       | | | | | | |   | | | | |     |   | |       |   | |       |     |   | | | | | | | 
 | | |   | |   | | | | | |   |       | |       |   | | | | |     |                   |     | |   |     |     | |       | | | | | | |   | | | | |     |   | |       |   | |       |     |   | | | | | | | 
 +---|-----------|-|-|-|-----|-------|---------|---|-|-----|-------------------------|-----+ |   |     |     | |       | | | | | | |   | | | | |     |   | |       |   | |       |     | +-+ | | | | | | 
   | |   | |   | | | | | |   |       | |       |   | | | | |     |                   |       |   |     |     | |       | | | | | | |   | | | | |     |   | |       |   | |       |     | |   | | | | | | 
   | |   | |   | | | | +-----+       | |       |   +-|-----|-----|-------------------|-------|---------|-----|-------------|-------|-----|-----|-----|---|-------------+ |       |     | |   | | | | | | 
   | |   | |   | | | |   |           | |       |     | | | |     |                   |       |   |     |     | |       | | | | | | |   | | | | |     |   | |       E     |       |     | |   | | | | | | 
   | |   | |   | | | |   |           +-|-------------------|-------------------------------------|-----------|-|-------|---|-|-|-------|-----|-|-----|---|---------|-----------+ |     | |   | | | | | | 
   | |   | |   | | | |   |             |       |     | | | |     |                   |       |   |     |     | |       | | | | | | |   | | | | |     |   | |       |     |     | |     | |   | | | | | | 
 +---------|---|-----------------------|-------------------|---------------------+   |       | +-|-----+     | |       | | | | | | |   | | | | |     |   | +-------|-----|-------|---+ | | +-----|-|---+ 
 | | |   | |   | | | |   |             |       |     | | | |     |               |   |       | | |           | |       | | | | | | |   | | | | |     |   |         |     |     | |   | | | | | | | | |   
 | | +---|-----+ | | |   |             |       |     | | | |     |               |   |       | | |           | |       | | | | | +-----------|-|-----|-------------+     |     | |   | | | | | | | | |   
 | |     | |     | | |   |             |       |     | | | |     |               |   |       | | |           | |       | | | | |   |   | | | | |     |   |               |     | |   | | | | | | | | |   
 +-----+ | |     | | |   |             |       |     | | | |     |               |   |       | | |   +---------|-------|---|-|-----|---|-----|-|---------|-----------------------|-----|-|-|---|-|---|-+ 
   |   | | |     | | |   |             |       |     | | | |     |               |   |       | | |   |       | |       | | | | |   |   | | | | |     |   |               |     | |   | | | | | | | | | | 
   |   +-|-----------|---|---------------+     |     | | | |     |               |   |       | | |   |       | |       | | | | |   |   | | | | |     |   |               |     | |   | | | | | | | | | | 
   |     | |     | | |   |             | |     |     | | | |     |               |   |       | | |   |       | |       | | | | |   |   | | | | |     |   |               |     | |   | | | | | | | | | | 
 +---------------|-------|---------------|-+   |     | | | |   +-----------------------------|---|-+ |       | |       | | | | |   |   | | | | |     |   |               +-----|-|-+ | | | | | | | | | | 
 | |     | |     | | |   |             | | |   |     | | | |   | |               |   |       | | | | |       | |       | | | | |   |   | | | | |     |   |                     | | | | | | | | | | | | | 
 | |     | +-----|-------|---------------------|-------|---------|---------------------------|-|-----|-------|-----------|-+ +-----|---|---|---------|---|-+                   | | | | | | | | | | | | | 
 | |     |       | | |   |             | | |   |     | | | |   | |               |   |       | | | | |       | |       | |     |   |   | | | | |     |   | |                   | | | | | | | | | | | | | 
 | |     |       | | |   |             | | |   |     | | | +---------------------------------|-|-|-|-|---------|-------|-|-------------|-------|-----|---|---------------------------|---|-+ | | | | | | 
 | |     |       | | |   |             | | |   |     | | |     | |               |   |       | | | | |       | |       | |     |   |   | | | | |     |   | |                   | | | | | |   | | | | | | 
 +---+   |       | | |   |             | | |   |     | | |     | |               |   |   +---|-|-|-----------------------------|-------|---|-|-|-----|---|---------+           | | | | | |   | | | | | | 
   | |   |       | | |   |             | | |   |     | | |     | |               |   |   |   | | | | |       | |       | |     |   |   | | | | |     |   | |       |           | | | | | |   | | | | | | 
   | |   | +-------|-------------------|-|-|---------------------|---------------------+ |   | | | +-|-------|-------+ | |     |   |   | | | | |     |   | |       |           | | | | | |   | | | | | | 
   | |   | |     | | |   |             | | |   |     | | |     | |               |   | | |   | | |   |       | |     | | |     |   |   | | | | |     |   | |       |           | | | | | |   | | | | | | 
   +-------|-------|-----|-------------|---|---|-----|-----------|---------------|-----------+ | |   |       | |     | | |     |   |   | | | | |     |   | |       |           | | | | | |   | | +-+ | | 
     |   | |     | | |   |             | | |   |     | | |     | |               |   | | |     | |   |       | |     | | |     |   |   | | | | |     |   | |       |           | | | | | |   | |     | | 
     |   | |     | | |   |             | | |   |     | | |     | |               |   | | |     | |   |       | |     | | |     |   |   | | | | |     |   | |       |           | | | | | |   | |     | | 
     |   | |     | | |   |             | | |   |     | | |     | |               |   | | |     | |   |       | |     | | |     |   |   | | | | |     |   | |       |           | | | | | |   | |     | | 
 +---|-----|-----|---------------------|-|-------------|-------|-----------------|---|-|-|-------|-----------------------|-----|---|---+ | | | |     |   | |       |           | | | | | |   | |     | | 
 |   |   | |     | | |   |             | | |   |     | | |     | |               |   | | |     | |   |       | |     | | |     |   |     | | | |     |   | |       |           | | | | | |   | |     | | 
 |   |   | |     | | |   |             | | |   |     | +-|-------|---------------|-----|---+ +---|---|-------|-------|---|-----|---|-----|-+ | |     |   | |       |           | | | | | |   | |     | | 
 |   |   | |     | | |   |             | | |   |     |   |     | |               |   | | | | | | |   |       | |     | | |     |   |     |   | |     |   | |       |           | | | | | |   | |     | | 
 |   |   | |   +-----|-------------------|-|-------+ |   |     | |   +---------------|---|-|-|-|-----|-------|-------|---|-----|---|---------|-------------|-------------+   +-----|---|-|-+ | |     | | 
 |   |   | |   | | | |   |             | | |   |   | |   |     | |   |           |   | | | | | | |   |       | |     | | |     |   |     |   | |     |   | |       |     |   | | | | | | | | | |     | | 
 |   |   | |   | | | |   +-----------+ +---|-------|-+   |     | |   |           +---|-|-----|---------------|-------|---|---------|-----------------|---+ |     +-|-+   |   | | | | | | | | | |     | | 
 |   |   | |   | | | |               |   | |   |   |     |     | |   |               | | | | | | |   |       | |     | | |     |   |     |   | |     |     |     | | |   |   | | | | | | | | | |     | | 
 |   |   | |   | | | |               |   | |   |   | +---|-----------|-----------+   | | | | | | |   |       | |     | | |     |   |     |   | |     |     |     | | |   |   +-|---|-----+ | | |     | | 
 |   |   | |   | | | |               |   | |   |   | |   |     | |   |           |   | | | | | | |   |       | |     | | |     |   |     |   | |     |     |     | | |   |     | | | | |   | | |     | | 
 |   |   | |   | | | |               +---|-|---|---------|-----|-----------------+   | | | | | | |   |       | |     | | |     |   |     |   | |     |     |     | | |   |     | | | | |   | | |     | | 
 |   |   | |   | | | |                   | |   |   | |   |     | |   |               | | | | | | |   |       | |     | | |     |   |     |   | |     |     |     | | |   |     | | | | |   | | |     | | 
 | +-|-+ | |   | | | |                   | +-------------------|-----------------------|-----|-|-|-------------|---+ | | | +---|---------------|-----------|-----|-------------|-----|-+   | | |     | | 
 | | | | | |   | | | |                   |     |   | |   |     | |   |               | | | | | | |   |       | |   | | | | |   |   |     |   | |     |     |     | | |   |     | | | |     | | |     | | 
 | | | | | |   | | | |                   +-----|---------------|-+   |               | | | | | | |   |       | |   | | | | |   |   |     |   | |     |     |     | | |   |     | | | |     | | |     | | 
 | | | | | |   | | | |                         |   | |   |     |     |               | | | | | | |   |       | |   | | | | |   |   |     |   | |     |     |     | | |   |     | | | |     | | |     | | 
 | | | | | |   | | | |                         |   | |   |     |     |               | | | | | | |   |       | |   | | | | |   |   |     |   | |     |     |     | | |   |     | | | |     | | |     | | 
 | | | | | |   | | | |                         |   | |   |     |     |               | | | | | | |   |       | |   | | | | |   |   |     |   | |     |     |     | | |   |     | | | |     | | |     | | 
 | | | | | |   | | | |                         |   | |   +-----|---------------------------|-|-|-----|-------+ |   | | | | |   |   |     |   | |     |     |     | | |   |     | | | +-+   | | |     | | 
 | | | | | |   | | | |                         |   | |         |     |               | | | | | | |   |         |   | | | | |   |   |     |   | |     |     |     | | |   |     | | |   |   | | |     | | 
 | | | | | |   | | | |                         |   | |         |     |               | | | | | | | +-|-------------|-|-----+   |   |     |   | |     |     |     | | |   |     | | |   |   | | |     | | 
 | | | | | |   | | | |                         |   | |         |     |               | | | | | | | | |         |   | | | |     |   |     |   | |     |     |     | | |   |     | | |   |   | | |     | | 
 | | | | | |   | | | |                         |   | |         |     |               | | | | | | | | |         |   | | | |     |   |     |   | |     |     |     | | |   |     | | |   |   | | |     | | 
 | | | | | |   | | | |                         |   | |         |     |               | | | | | | | | |         |   | | | |     |   |     |   | |     |     |     | | |   |     | | |   |   | | |     | | 
 | | | | | |   | | | |                         |   | +---------|-----|---------------|-|-|-------|-----------------+ | | |     |   |     |   | |     |     |     | | +---|---+ | | | B +---|-|-|-+   | | 
 | | | | | |   | | | |                         |   |           |     |               | | | | | | | | |         |     | | |     |   |     |   | |     |     |     | |     |   | | | | |     | | | |   | | 
 | | | | | |   | | | |                         |   |           |     |               | | | | | | | | |         |     | | |     |   | +---+   | |     |     |     | |     |   | | | | +-+   | | | |   | | 
 | | | | | |   | | | |                         |   |           |     |               | | | | | | | | |         |     | | |     |   | |       | |     |     |     | |     |   | | | |   |   | | | |   | | 
 | | | | | |   +-|-|-------------+             |   |           |     |               +-+ | +---|-|---|-------------+ | | |     |   | |       | |     |     |     | | +-------|-----|---|-+ | | | |   | | 
 | | | | | |     | | |           |             |   |           |     |                   |   | | | | |         |   | | | |     |   | |       | |     |     |     | | |   |   | | | |   | | | | | |   | | 
 | | | | | |     | | |           |             |   |   +-------+     |                   |   | | | | |       +-|---|---|-|-----------|---------------|---+ | +-------|-----------|-----|-|-------|-+ | | 
 | | | | | |     | | |           |             |   |   |             |                   |   | | | | |       | |   | | | |     |   | |       | |     |   | | |   | | |   |   | | | |   | | | | | | | | | 
 | | | | | |     | | |           |             |   |   |             |                   |   | | +-|-|-------|-|---|---|-|---------|-+       | |     |   +-|---+ | +-+   |   | | | +---+ +-+ | | | | | | 
 | | | | | |     | | |           |             |   |   |             |                   |   | |   | |       | |   | | | |     |   |         | |     |     | | | |       |   | | |           | | | | | | 
 | | +-|---|-----+ | |           |             |   |   |             |                   |   | |   | |       | |   | | | |     |   |         | |     |     | | | |       |   | | |           | | | | | | 
 | |   | | |       | |           |             |   |   |             |                   |   | |   | |       | |   | | | |     |   |         | |     |     | | | |       |   | | |           | | | | | | 
 | |   | | |       | |           +---------+   |   |   |             |           +---------------------------|-----|-|---|-----|---|---------|-|-------------------------|-----|---------+   | | | | | | 
 | |   | | |       | |                     |   |   |   |             |           |       |   | |   | |       | |   | | | |     |   |         | |     |     | | | |       |   | | |       |   | | | | | | 
 | |   | | |       | |                     |   |   |   +-------------+   +-------|-----------|-+   | |       | |   | | | |     |   |         | |     |     | | | |       |   | | |       |   | | | | | | 
 | |   | | |       | |                     |   |   |                     |       |       |   |     | |       | |   | | | |     |   |         | |     |     | | | |       |   | | |       |   | | | | | | 
 +-+   | +-|-+     | +---+                 |   |   |                     |       |       |   |     | |       | +---|---+ |     |   |         | |     |   +-+ | +-|-------|---|-------------------+ | +-+ 
       |   | |     |     |                 |   |   |                     |       |       |   |     | |       |     | |   |     |   |         | |     |   |   |   |       |   | | |       |   | |   |     
       |   | |     +---+ +-------------------------|---------------------|-------------------------|-|-------|-----|-----+     |   |         | |     |   |   |   |       |   | | |       |   | |   |     
       |   | |         |                   |   |   |                     |       |       |   |     | |       |     | |         |   |         | |     |   |   |   |       |   | | |       |   | |   |     
       |   +-----------------------+       | +-+   |               +-+   |       |       +---|-------|-------|-------|---------|---|---------|-|-----|-+ |   |   |       |   | | |       |   | |   |     
       |     |         |           |       | |     |               | |   |       |           |     | |       |     | |         |   |         | |     | | |   |   |       |   | | |       |   | |   |     
   +---------+         |           +-------|-----------------------+ +-----------------------|-----|-----------+   | |         |   +-----------|-----------------------+ |   | | |       |   | |   +---+ 
   |   |               |                   | |     |                     |       |           |     | |       | |   | |         |             | |     | | |   |   |     | |   | | |       |   | |       | 
   |   |               |                   | +--N------------------------|-------------------|-----+ |       | +---|-|-+       |             | |     | | |   |   |     | |   | | |       |   | |       | 
   |   |               |                   |       |                     |       |           |       |       |     | | |       |             | |     | | |   |   |     | |   | | |       |   | |       | 
   |   |               |         +-----------------|---------------+ +-----------|-----------+ +-------------|-----|---+       +-------------|-|-----|---|---+   |     | |   | | |       |   | |       | 
   |   |               |         |         |       |               | |   |       |             |     |       |     | |                       | |     | | |       |     | |   | | |       |   | |       | 
 +---------------------+         | +---------------+     +-----+   | |   |       |             |     |       |     | |                       | |     | | |       |     | |   | | |       |   | |       | 
 | |   |                         | |       |             |     |   | |   |       |             |     |       |     | |                       | |     | | |       |     | |   | | |       |   | |       | 
 | |   |                         | |       |             +-----|-----|---|-----------------------------------+     | |                       | |     | | |       |     | |   | | |       |   | |       | 
 | |   |                         | |       |                   |   | |   |       |             |     |             | |                       | |     | | |       |     | |   | | |       |   | |       | 
 +-|-----+                       +-+       |                   |   | +---|-------|-------------+     |             | |                       +-|--------L--------------+ |   | | |       |   | |       | 
   |   | |                                 |                   |   |     |       |                   |             | |                         |     | | |       |       |   | | |       |   | |       | 
   |   | |                                 |                   |   |     +---------------------------|---------------|-------------------------------+ | |       |       |   +---+       |   | |       | 
   |   | |                                 |                   |   |             |                   |             | |                         |       | |       |       |     |         |   | |       | 
   |   | |                                 |                   |   |             |                   +-----------------------------------------|-------|-----------------|---+ |         |   | | +---+ | 
   |   | |                                 |                   |   |             |                                 | |                         |       | |       |       |   | |         |   | | |   | | 
   |   | |                                 |                   |   |             |                                 | |                         |       | |       |       |   | |         |   | | |   | | 
   |   | |                                 |                   |   |             |                                 | |                         |       | |       |       |   | |         |   | | |   | | 
   |   | +---------------------------------|-----------------------|-------------------------------------------------+                         |       | |       |       |   | |         +-----|-----+ | 
   |   |                                   |                   |   |             |                                 |                           |       | |       |       |   | |             | | |     | 
   |   |             +---------------------------------------------+             |                                 |                           |       | |       |       |   +---------------+ +---+   | 
   |   |             |                     |                   |                 |                                 |                           |       | |       |       |     |                 | |   | 
 +-+   |             |                     |                   |                 |                                 |                           |       | |       |     +-------------------------+ +---+ 
 |     |             |                     |                   |                 |                                 |                           |       | |       |     | |     |                         
 |     +-------------------------------------------------------|-----------------|---------------------------------|---------------------------|-------|---------|-----+ |     |                         
 |                   |                     |                   |                 |                                 |                           |       | |       |       |     |                         
 |                   +-----------------------------------------+                 |         +-----------------------|---------------------------+       | |       |       |     |                         
 |                                         |                                     |         |                       |                                   | |       |       |     |                         
 |                                         |   +---------------------------------|-------------+                   |         +---------------------------+       |       |     |                         
 |                                         |   |                                 |         |   |                   |         |                         |         |       |     |                         
 +-----------------------------------------+   +---------------------------------|---------|-+ |                   |         +---------+               |         |       |     |                         
                                                                                 |         | | |                   |                   |               |         |       |     |                         
 +--------Y----------------------------------------------------------------------|-----------|-|-------------------------------------------------------|---------|-------+     |                         
 |                                                                               |         | | |                   |                   |               |         |             |                         
 +-------------------------------------------------------------------------------+         +-+ +-------------------+                   +---------------+         +-------------+                         
                                                                                                                                                                                                         ";


type Grid = Vec<Vec<char>>;

fn find_start(grid: &Grid) -> Position {
    Position(grid[0].iter().position(|&ch| ch == '|').unwrap(), 0)
}

fn parse_input(input: &str) -> Grid {
    input.split("\n").map (|line| line.chars().collect()).collect()
}

fn elt(grid: &Grid, pos: Position) -> Option<char> {
    if pos.1 < grid.len() {
        Some(grid[pos.1][pos.0])
    } else {
        None
    }
}

fn next_position(position: Position, direction: Direction) -> Position {
    Position((position.0 as i32 + direction.0) as usize, (position.1 as i32 + direction.1) as usize)
}

fn day19() {
    let mut current_direction = 2;  // DOWN

    let grid = parse_input(REAL_INPUT);
    let mut position = find_start(&grid);

    let mut letters: Vec<char> = Vec::new();
    let mut steps = 0;

    loop {
        let ch = elt(&grid, position).unwrap();

        match ch {
            ' ' => { break; },
            '-' | '|' => { },
            '+' => {
                let candidate_direction = DIRECTIONS[(current_direction + 1) % DIRECTIONS.len()];
                let value = elt(&grid, next_position(position, candidate_direction));

                if !value.is_some() || value.unwrap() == ' ' {
                    current_direction += 3;
                } else {
                    current_direction += 1;
                }
            },
            _ => { letters.push(ch) },
        }

        steps += 1;

        let direction = DIRECTIONS[current_direction % DIRECTIONS.len()];
        position = next_position(position, direction);
    }

    println!("{:?}", letters);
    println!("Number of steps: {:?}", steps);

}

///////////////// Day 20

use std::fs::File;
use std::io::{BufReader, Read};
use std::collections::HashMap;
use regex::Regex;

extern crate regex;

fn day20_input() -> String {
    let f = File::open("advent-files/day20.txt").expect("open file");
    let mut br = BufReader::new(f);

    let mut input = String::new();
    br.read_to_string(&mut input).unwrap();

    input
}

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
struct Vec3 {
    x: i64,
    y: i64,
    z: i64,
}

impl Vec3 {
    fn new(x: i64, y: i64, z: i64) -> Vec3 {
        Vec3 {x, y, z}
    }

    fn parse(s: &str) -> Vec3 {
        let bits: Vec<i64> = (&s[3..s.len() - 1]).split(",").map (|s| {s.parse().unwrap()}).collect();
        Vec3::new(bits[0], bits[1], bits[2])
    }
}

#[derive(Debug)]
struct Particle {
    id: usize,
    position: Vec3,
    velocity: Vec3,
    acceleration: Vec3,
}

fn manhattan(v: Vec3) -> i64 {
    v.x.abs() + v.y.abs() + v.z.abs()
}

fn distance(particle: &Particle) -> i64 {
    manhattan(particle.position)
}

fn step(particle: &mut Particle) {
    particle.velocity.x += particle.acceleration.x;
    particle.velocity.y += particle.acceleration.y;
    particle.velocity.z += particle.acceleration.z;

    particle.position.x += particle.velocity.x;
    particle.position.y += particle.velocity.y;
    particle.position.z += particle.velocity.z;
}

fn day20_parse_particles() -> Vec<Particle> {
    let mut particle = 0;
    let mut particles: Vec<Particle> = Vec::new();

    let fs = Regex::new(", ").unwrap();

    // Parse our particles
    for line in day20_input().split("\n") {
        if line == "" {
            break;
        }

        let bits: Vec<&str> = fs.split(line).collect();

        particles.push(Particle { id: particle, position: Vec3::parse(bits[0]), velocity: Vec3::parse(bits[1]), acceleration: Vec3::parse(bits[2]) });
        particle += 1;
    }

    particles
}

fn day20_pt1() {
    let mut particles = day20_parse_particles();

    for _ in 0..1000 {
        for p in particles.iter_mut() {
            step(p)
        }
    }

    let min_particle = particles.iter().fold(&particles[0], |m, p| {
        if distance(p) < distance(m) {
            p
        } else {
            m
        }
    });

    println!("The min particle is {:?}", min_particle);
}

fn minimum_particle_of<'a>(m: &'a Particle, p: &'a Particle) -> &'a Particle {
    if manhattan(p.acceleration) != manhattan(m.acceleration) {
        if manhattan(p.acceleration) < manhattan(m.acceleration) {
            return p;
        } else {
            return m;
        }
    }

    if manhattan(p.velocity) != manhattan(m.velocity) {
        if manhattan(p.velocity) < manhattan(m.velocity) {
            return p;
        } else {
            return m;
        }
    }

    if manhattan(p.position) != manhattan(m.position) {
        if manhattan(p.position) < manhattan(m.position) {
            return p;
        } else {
            return m;
        }
    }

    return m;
}


fn day20_pt1_smart() {
    let mut particles = day20_parse_particles();

    let min_particle = particles.iter().fold(&particles[0], |m, p| {
        minimum_particle_of(m, p)
    });

    println!("The min particle is {:?}", min_particle);
}

fn day20_pt2() {
    let mut particles = day20_parse_particles();

    for _ in 0..2000 {
        let mut hash = HashMap::new();

        {
            for p in particles {
                let pos = p.position;
                let e = hash.entry(pos).or_insert(Vec::new());
                e.push(p);
            }
        }

        let mut next_particles = Vec::new();

        for (_k, mut v) in hash {
            if v.len() == 1 {
                let mut p = v.remove(0);
                step(&mut p);
                next_particles.push(p);
            }
        }


        particles = next_particles;
    }

    println!("{} particles remain", particles.len());
}



fn day20() {
    day20_pt1();
    day20_pt1_smart();
    day20_pt2();
}


///////////////// Day 21

// Abandoned!
// use std::fmt;
//
// struct PixelMap {
//     pixels: Vec<Pixel>,
//     next_id: i64,
// }
//
// struct Pixel {
//     id: i64,
//     value: char,
//
//     left: i64,
//     right: i64,
//     above: i64,
//     below: i64,
// }
//
//
// impl std::fmt::
// Display for PixelMap {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         let origin = &self.pixels[0];
//
//         let mut row = origin;
//         loop {
//             let mut col = row;
//             loop {
//                 write!(f, "{}", col.value).expect("write");
//                 if col.right >= 0 {
//                     col = &self.pixels[col.right as usize]
//                 } else {
//                     break;
//                 }
//             }
//
//             if row.below >= 0 {
//                 row = &self.pixels[row.below as usize];
//             } else {
//                 break;
//             }
//         }
//
//         Ok(())
//     }
// }
//
// impl PixelMap {
//     fn new(first_pixel: char) -> PixelMap {
//         let pixels = vec!(Pixel { id: 0, value: first_pixel, left: -1, right: -1, above: -1, below: -1 });
//
//         PixelMap { pixels: pixels , next_id: 1 }
//     }
//
//     fn origin_pixel(&self) -> &Pixel {
//         &self.pixels[0]
//     }
//
//     fn add_to_right(&mut self, p: &Pixel, c: char) -> &Pixel {
//     }
// }
//
// fn day21() {
//     let mut map = PixelMap::new('.');
//
//     let origin = map.origin_pixel();
//
//     "#.".chars().fold(origin, |p, ch| {
//         map.add_to_right(p, ch)
//     });
//
//     println!("{}", map);
//
// }

use std::fs::File;
use std::io::{BufReader, Read};

fn parse_grid(s: String) -> Vec<String> {
    s.split("/").map(str::to_owned).collect()
}

fn grid_str(grid: Vec<String>) -> String {
    grid.join("/")
}

fn flip(s: String) -> String {
    grid_str(parse_grid(s).iter().map(|row| row.chars().rev().collect::<String>()).collect())
}

fn rotate(s: String) -> String {
    let rows: Vec<String> = parse_grid(s);
    let cols = rows[0].len();

    grid_str((0..cols).map (|c| {
        (0..rows.len()).rev().map(|r| rows[r].chars().nth(c).unwrap()).collect()
    }).collect())
}

fn permute(s: String) -> Vec<String> {
    let mut rotations = vec!(s.clone());

    let mut rotation = s;
    for _ in 0..3 {
        rotation = rotate(rotation);
        rotations.push(rotation.clone());
    }

    let mut permutations = Vec::new();

    for r in rotations {
        permutations.push(r.clone());
        permutations.push(flip(r));
    }

    permutations
}

struct Rule {
    lhs: Vec<String>,
    rhs: String,
}


fn parse_input() -> Vec<Rule> {
    let f = File::open("advent-files/day21_input.txt").expect("open file");
    let mut br = BufReader::new(f);

    let mut input = String::new();
    br.read_to_string(&mut input).unwrap();

    let mut rules: Vec<Rule> = Vec::new();

    for line in input.split("\n") {
        if line == "" {
            continue;
        }

        let bits: Vec<String> = line.split(" => ").map(str::to_owned).collect();
        let (lhs, rhs) = (bits[0].clone(), bits[1].clone());

        rules.push(Rule { lhs: permute(lhs), rhs: rhs });
    }

    rules
}

fn apply_rules(p: String, rules: &Vec<Rule>) -> String {
    for rule in rules {
        for l in rule.lhs.iter() {
            if *l == p {
                return rule.rhs.clone()
            }
        }
    }

    panic!("No rule found!");
}

fn join_grids(grids: Vec<String>) -> String {
    if grids.len() == 1 {
        return grids[0].clone();
    }

    let n = (grids.len() as f64).sqrt() as usize;

    let inputs: Vec<Vec<String>> = grids.iter().map(|g| {
        parse_grid(g.to_owned())
    }).collect();

    let mut result: Vec<String> = Vec::new();

    for chunk in inputs.chunks(n) {
        for i in 0..chunk[0].len() {
            result.push((0..n).map(|j| chunk[j][i].clone()).collect::<String>())
        }
    }

    grid_str(result)
}

fn extract_grids(p: String, len: usize) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();

    let rows: Vec<String> = parse_grid(p);

    for next_rows in rows.chunks(len) {
        let mut col = 0;
        let cols = next_rows[0].len();
        while col < cols {
            let grid = grid_str(next_rows.iter().map(|s| {
                (&s[col..col + len]).to_owned()
            }).collect::<Vec<String>>());

            result.push(grid);

            col += len;
        }
    }

    result
}

fn next_pattern(p: String, rules: &Vec<Rule>) -> String {
    let len = p.split("/").nth(0).unwrap().len();

    let divisor = match len % 2 {
        0 => 2,
        _ => 3,
    };

    let new_grids = extract_grids(p, divisor).iter().map(|ref g| apply_rules((*g).clone(), &rules)).collect();

    join_grids(new_grids)
}

fn day21_pt1() {
    let rules: Vec<Rule> = parse_input();

    let mut pattern = ".#./..#/###".to_owned();

    for _ in 0..5 {
        pattern = next_pattern(pattern, &rules)
    }

    println!("Number of on pixels: {}", pattern.chars().filter(|&ch| ch == '#').count());
}

fn day21_pt2() {
    let rules: Vec<Rule> = parse_input();

    let mut pattern = ".#./..#/###".to_owned();

    for _ in 0..18 {
        pattern = next_pattern(pattern, &rules)
    }

    println!("Number of on pixels: {}", pattern.chars().filter(|&ch| ch == '#').count());
}


fn day21() {
    day21_pt1();
    day21_pt2();
}


*/

///////////////// Day 22

use std::fs::File;
use std::io::{BufReader, Read};
use std::collections::HashSet;


#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
struct Pos (i64, i64);

impl Pos {
    fn add(&self, other: Pos) -> Pos {
        Pos(self.0 + other.0, self.1 + other.1)
    }
}

#[derive(Debug)]
struct World {
    infected_positions: HashSet<Pos>,
    weakened_positions: HashSet<Pos>,
    flagged_positions: HashSet<Pos>,
}

impl World {
    fn from_input(path: &str) -> World {
        let mut result = World {
            infected_positions: HashSet::new(),
            weakened_positions: HashSet::new(),
            flagged_positions: HashSet::new(),
        };

        let f = File::open(path).expect("open file");
        let mut br = BufReader::new(f);

        let mut input = String::new();
        br.read_to_string(&mut input).unwrap();

        let rows: Vec<String> = input.split("\n").map(str::to_owned).collect();
        let height = rows.len() as i64;
        let width = rows[0].len() as i64;

        let mut y = (height - 1) / 2;
        for row in rows {
            let mut x = -((width - 1) / 2);
            for ch in row.chars() {
                if ch == '#' {
                    result.infected_positions.insert(Pos(x, y));
                }
                x += 1;
            }

            y -= 1;
        }

        result
    }

    fn is_infected(&self, pos: &Pos) -> bool { self.infected_positions.contains(&pos) }
    fn is_flagged(&self, pos: &Pos) -> bool { self.flagged_positions.contains(&pos) }
    fn is_weakened(&self, pos: &Pos) -> bool { self.weakened_positions.contains(&pos) }

    fn clean_node(&mut self, pos: &Pos) {
        self.infected_positions.remove(pos);
        self.weakened_positions.remove(pos);
        self.flagged_positions.remove(pos);
    }

    fn infect_node(&mut self, pos: &Pos) {
        self.clean_node(pos);
        self.infected_positions.insert((*pos).clone());
    }

    fn flag_node(&mut self, pos: &Pos) {
        self.clean_node(pos);
        self.flagged_positions.insert((*pos).clone());
    }

    fn weaken_node(&mut self, pos: &Pos) {
        self.clean_node(pos);
        self.weakened_positions.insert((*pos).clone());
    }
}


fn add_clamped(n: usize, offset: i64, max: usize) -> usize {
    let r = n as i64 + offset;

    if r >= 0 {
        r as usize % max
    } else {
        (max as i64 + (r % max as i64)) as usize
    }
}

fn day22_pt1() {
    let mut world = World::from_input("advent-files/day22_input.txt");

    // up, right, down, left
    let orientations = vec!(Pos(0, 1), Pos(1, 0), Pos(0, -1), Pos(-1, 0));

    let mut current_orientation = 0;
    let mut position = Pos(0, 0);
    let mut infected_count = 0;

    for _ in 0..10000 {
        if world.is_infected(&position) {
            current_orientation = add_clamped(current_orientation, 1, orientations.len());
            world.clean_node(&position);
        } else {
            current_orientation = add_clamped(current_orientation, -1, orientations.len());
            world.infect_node(&position);
            infected_count += 1;
        }

        position = position.add(orientations[current_orientation])
    }

    println!("Nodes infected: {}", infected_count);
}

fn day22_pt2() {
    let mut world = World::from_input("advent-files/day22_input.txt");

    // up, right, down, left
    let orientations = vec!(Pos(0, 1), Pos(1, 0), Pos(0, -1), Pos(-1, 0));

    let mut current_orientation = 0;
    let mut position = Pos(0, 0);
    let mut infected_count = 0;

    for _ in 0..10000000 {
        if world.is_weakened(&position) {
            world.infect_node(&position);
            infected_count += 1;
        } else if world.is_infected(&position) {
            current_orientation = add_clamped(current_orientation, 1, orientations.len());
            world.flag_node(&position);
        } else if world.is_flagged(&position) {
            current_orientation = add_clamped(current_orientation, 2, orientations.len());
            world.clean_node(&position);
        } else {
            current_orientation = add_clamped(current_orientation, -1, orientations.len());
            world.weaken_node(&position);
        }

        position = position.add(orientations[current_orientation])
    }

    println!("Nodes infected: {}", infected_count);
}


fn day22() {
    day22_pt1();
    day22_pt2();
}



fn main() {
    // day1();
    // day2();
    // day3();
    // day4();
    // day5();
    // day6();
    // day7();
    // day8();
    // day9();
    // day10();
    // day11();
    // day12();
    // day13();
    // day14();
    // day15();
    // day16();
    // day17();
    // day18();
    // day19();
    // day20();
    // day21();
    day22();
}
