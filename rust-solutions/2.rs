fn main() {
    problema();
    problemb();
}

const TEST_SET: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
const PROBLEM_SET: &str = "16100064-16192119,2117697596-2117933551,1-21,9999936269-10000072423,1770-2452,389429-427594,46633-66991,877764826-877930156,880869-991984,18943-26512,7216-9427,825-1162,581490-647864,2736-3909,39327886-39455605,430759-454012,1178-1741,219779-244138,77641-97923,1975994465-1976192503,3486612-3602532,277-378,418-690,74704280-74781349,3915-5717,665312-740273,69386294-69487574,2176846-2268755,26-45,372340114-372408052,7996502103-7996658803,7762107-7787125,48-64,4432420-4462711,130854-178173,87-115,244511-360206,69-86";

fn iterate_ranges(input: &str, mut count_invalids: impl FnMut(i64, i64)) {
    for line in input.split(',') {
        let mut parts = line.splitn(2, '-');
        let start: i64 = parts
            .next()
            .and_then(|s| s.trim().parse().ok())
            .expect("failed to parse range start");
        let end: i64 = parts
            .next()
            .and_then(|s| s.trim().parse().ok())
            .expect("failed to parse range end");
        count_invalids(start, end);
    }
}

fn problema() {
    let mut total: i64 = 0;
    iterate_ranges(PROBLEM_SET, |start, end| {
        for num in start..=end {
            let s = num.to_string();
            if (s.len() % 2) == 1 {
                continue;
            } else {
                let half = s.len() / 2;
                let (a, b) = s.split_at(half);
                if a == b {
                    total += num;
                }
            }
        }
    });
    println!("Problem A: {}", total);
}

fn problemb() {
    let mut total: i64 = 0;
    iterate_ranges(PROBLEM_SET, |start, end| {
        for num in start..=end {
            let s = num.to_string();
            for segsize in 1..=s.len() / 2 {
                if (s.len() % segsize) == 0 {
                    let seg = &s[..segsize];
                    let cand = seg.repeat(s.len() / segsize);
                    if cand == s {
                        total += num;
                        break;
                    }
                }
            }
        }
    });
    println!("Problem B: {}", total);
}
