fn main() {
    let measurements = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

    let result = measurements.windows(2)
        .fold(0, |mut counter, pair| {
            if pair[1] > pair[0] {
                counter += 1;
            }
            
            counter
        });

    assert_eq!(result, 7);
}
