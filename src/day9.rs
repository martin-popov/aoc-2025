// I tried thinking for a while of a smart solution but can't figure out anything other than checking all possible rectangles, due to edge cases
pub fn solve(input: Vec<(i64, i64)>, part2: bool) -> u64 {
    let mut result: u64 = 0;
    for (i, start) in input.iter().enumerate() {
        for finish in input.iter().skip(i) {
            let length = start.0 - finish.0;
            let width = start.1 - finish.1;
            let area = ((length.abs() + 1) * (width.abs() + 1)) as u64;
            // println!(
            //     "{:?} * {:?} = {} * {} = {}",
            //     start, finish, length, width, area
            // );
            if area > result {
                result = area;
            }
        }
    }

    result
}
