fn main() {
    let measurements = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

    let result_part_1 = measurements.windows(2).fold(0, |mut counter, pair| {
        if pair[1] > pair[0] {
            counter += 1;
        }

        counter
    });

    assert_eq!(result_part_1, 7);

    let result_part_2 = measurements.windows(4).fold(0, |mut counter, pair| {
        let p1_sum : usize = pair[.. pair.len() - 1].iter().sum();
        let p2_sum : usize = pair[1..].iter().sum();

        if p2_sum > p1_sum {
            counter += 1;
        }

        counter
    });

    assert_eq!(result_part_2, 5);
}
