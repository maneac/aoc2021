fn main() {
    let data = read_data();

    println!("Part 1: {}", part_1(&data));
    println!("Part 2: {}", part_2(&data));
}

fn read_data() -> Vec<usize> {
    std::fs::read_to_string("../../data/day_1.txt")
        .unwrap()
        .trim()
        .lines()
        .map(|l| l.parse::<usize>().unwrap())
        .collect()
}

fn part_1(input: &Vec<usize>) -> String {
    input
        .iter()
        .fold((0usize, None), |(mut acc, last_val_opt), val| {
            let last_val = match last_val_opt {
                Some(v) => v,
                None => return (0, Some(val)),
            };

            if val > last_val {
                acc += 1;
            }

            (acc, Some(val))
        })
        .0
        .to_string()
}

fn part_2(input: &Vec<usize>) -> String {
    input
        .iter()
        .fold((0usize, [None; 3]), |(mut acc, last_vals_opts), val| {
            let lv0 = match last_vals_opts[0] {
                Some(v) => v,
                None => return (0, [Some(val), None, None]),
            };

            let lv1 = match last_vals_opts[1] {
                Some(v) => v,
                None => return (0, [Some(lv0), Some(val), None]),
            };

            let lv2 = match last_vals_opts[2] {
                Some(v) => v,
                None => return (0, [Some(lv0), Some(lv1), Some(val)]),
            };

            if [*lv1, *lv2, *val].iter().sum::<usize>() > [*lv0, *lv1, *lv2].iter().sum() {
                acc += 1;
            }

            (acc, [Some(lv1), Some(lv2), Some(val)])
        })
        .0
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_real() {
        let data = read_data();

        assert_eq!("1374", part_1(&data));
    }

    #[test]
    fn test_part_2_real() {
        let data = read_data();

        assert_eq!("1418", part_2(&data));
    }

    #[test]
    fn test_part_1_example() {
        let data = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

        assert_eq!("7", part_1(&data));
    }

    #[test]
    fn test_part_2_example() {
        let data = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

        assert_eq!("5", part_2(&data));
    }
}
