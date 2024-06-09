use crate::filereading;

fn a() {
    let lines = filereading::get_lines("src/inputs/day4.txt");

    let mut sum = 0;
    for line in lines.map_while(Result::ok) {
        let card_value = line.split(':').collect::<Vec<&str>>()[1]
            .split('|')
            .map(|l| l.trim())
            .collect::<Vec<&str>>();

        let user_numbers = card_value[0]
            .split(' ')
            .filter_map(|c| c.parse().ok())
            .collect::<Vec<i32>>();

        let winning_numbers = card_value[1]
            .split(' ')
            .filter_map(|c| c.parse().ok())
            .collect::<Vec<i32>>();

        let mut winning_count = 0;
        for number in winning_numbers {
            for &user_number in &user_numbers {
                if number == user_number {
                    winning_count += 1;
                }
            }
        }

        if winning_count > 0 {
            sum += 2_i32.pow(winning_count - 1);
        }
        println!("{winning_count} {sum}")
    }
    println!("{sum}")
}
pub fn run() {
    a();
}
