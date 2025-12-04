use std::fs;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let test_set = load_set("3.test")?;
    let problem_set = load_set("3.txt")?;

    println!(
        "{}",
        problem_set.iter().map(|v| max_jolt(v, 2)).sum::<u64>()
    );
    println!(
        "{}",
        problem_set.iter().map(|v| max_jolt(v, 12)).sum::<u64>()
    );

    Ok(())
}

fn max_jolt(v: &[u32], n: u32) -> u64 {
    let mut idx = 0;
    let mut total: u64 = 0;
    let mut numleft = n;

    while numleft > 0 {
        // search bounds
        let upper = v.len() - (numleft as usize) + 1;

        // find the max digit and its position in v[idx..upper]
        let (off, &digit) = v[idx..upper]
            .iter()
            .enumerate()
            .max_by(|(i1, d1), (i2, d2)| d1.cmp(d2).then(i2.cmp(i1)))
            .unwrap();
        let pos = idx + off;

        // update state
        total = total * 10 + (digit as u64);
        idx = pos + 1;
        numleft -= 1;
    }

    total
}

fn load_set(filename: &str) -> std::io::Result<Vec<Vec<u32>>> {
    let path = Path::new("../data").join(filename);
    let contents = fs::read_to_string(path)?;

    let lines = contents
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).expect("non-digit in input"))
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();

    Ok(lines)
}
