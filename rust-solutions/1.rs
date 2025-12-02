fn main() {
    problema();
    problemb();
}

fn process_moves<F>(mut on_move: F) -> i32
where
    F: FnMut(i32, i32, i32) -> i32, // (old_pos, new_pos, distance) -> hits
{
    let mut pos = 50;
    let mut password_count = 0;
    let input = include_str!("../data/1.txt");

    for line in input.lines().map(str::trim).filter(|l| !l.is_empty()) {
        let mut chars = line.chars();
        let direction = chars.next().expect("missing direction in line");
        let num: i32 = chars
            .as_str()
            .trim()
            .parse()
            .expect("failed to parse distance");

        let distance = match direction {
            'L' => -num,
            'R' => num,
            _ => panic!("invalid direction: {}", direction),
        };

        let new_pos = (pos + distance).rem_euclid(100);
        password_count += on_move(pos, new_pos, distance);
        pos = new_pos;
    }

    password_count
}

fn problema() {
    let hits = process_moves(|_, new_pos, _| if new_pos == 0 { 1 } else { 0 });
    println!("password: {}", hits);
}

fn problemb() {
    let hits = process_moves(|old_pos, new_pos, distance| {
        let mut count = 0;

        if new_pos == 0 {
            count += 1;
        } else if new_pos < old_pos && distance > 0 && old_pos != 0 {
            count += 1;
        } else if new_pos > old_pos && distance < 0 && old_pos != 0 {
            count += 1;
        }

        count += distance.abs() / 100;
        count
    });

    println!("password: {}", hits);
}
