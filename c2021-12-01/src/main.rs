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
        let p1_sum = pair[0] + pair[1] + pair[2];
        let p2_sum = pair[1] + pair[2] + pair[3];

        if p2_sum > p1_sum {
            counter += 1;
        }

        counter
    });

    assert_eq!(result_part_2, 5);
}
