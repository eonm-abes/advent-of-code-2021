fn main() {
    let commands = vec![
        "forward 5",
        "down 5",
        "forward 8",
        "up 3",
        "down 8",
        "forward 2",
    ];

    let parsed_commands = commands
        .iter()
        .map(|command| {
            let splited_command = command.split_whitespace().collect::<Vec<&str>>();
            return (
                splited_command[0],
                splited_command[1].parse::<usize>().unwrap(),
            );
        })
        .collect::<Vec<(&str, usize)>>();

    let final_position_part_1 =
        parsed_commands
            .iter()
            .fold((0, 0), |(mut horizontal, mut vertical), command| {
                match command {
                    ("forward", amount) => horizontal += amount,
                    ("up", amount) => vertical -= amount,
                    ("down", amount) => vertical += amount,
                    _ => (),
                }

                (horizontal, vertical)
            });

    let result = final_position_part_1.0 * final_position_part_1.1;

    assert_eq!(result, 150);

    let final_position_part_2 = parsed_commands.iter().fold(
        (0, 0, 0),
        |(mut horizontal, mut vertical, mut aim), command| {
            match command {
                ("forward", amount) => {
                    horizontal += amount;
                    vertical += amount * aim;
                }
                ("up", amount) => {
                    aim -= amount;
                }
                ("down", amount) => {
                    aim += amount;
                }
                _ => (),
            }

            (horizontal, vertical, aim)
        },
    );

    let result = final_position_part_2.0 * final_position_part_2.1;

    assert_eq!(result, 900);
}
