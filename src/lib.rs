//! A library for creating Ducci sequences from n-tuples

type Tuple = Vec<i64>;

pub fn create_ducci_sequence(entry_sequence: Tuple) -> Vec<Tuple> {
    if !is_power_of_two(entry_sequence.len()) {
        return Vec::new();
    }

    let mut res_vector = Vec::new();
    let mut current = entry_sequence.clone();

    loop {
        res_vector.push(current.clone());

        if current == vec![0; current.len()] {
            break;
        }

        current = create_following_ducci(current);
    }

    res_vector
}

fn create_following_ducci(tuple: Tuple) -> Tuple {
    let mut result = Vec::new();

    let mut index = 0;

    while index < tuple.len() {
        let (a, b) = if index == (tuple.len() - 1) {
            (tuple[index], tuple[0])
        } else {
            (tuple[index], tuple[index + 1])
        };

        result.push(sub_abs(a, b));

        index += 1;
    }

    result
}

fn sub_abs(a: i64, b: i64) -> i64 {
    (a - b).abs()
}

fn is_power_of_two(number: usize) -> bool {
    (number != 0) && ((number & (number - 1)) == 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_ducci_sequence() {
        let res = create_ducci_sequence(vec![1, 2, 3, 4]);

        assert_eq!(res.len(), 6);


        let res = create_ducci_sequence(vec![1, 2, 3, 4, 5]);

        // We are currently unable to calculate infinite sequences to the end ;)
        assert_eq!(res.len(), 0);
    }

    #[test]
    fn test_create_following_ducci() {
        let start = vec![1, 2, 3, 4];

        let mut next = create_following_ducci(start);

        assert_eq!(next, vec![1, 1, 1, 3]);

        next = create_following_ducci(next);

        assert_eq!(next, vec![0, 0, 2, 2]);

        next = create_following_ducci(next);

        assert_eq!(next, vec![0, 2, 0, 2]);

        next = create_following_ducci(next);

        assert_eq!(next, vec![2, 2, 2, 2]);

        next = create_following_ducci(next);

        assert_eq!(next, vec![0, 0, 0, 0]);
    }

    #[test]
    fn test_sub_abs() {
        let (a, b) = (23, 67);

        let res = sub_abs(a, b);

        assert_eq!(res, 44);
    }

    #[test]
    fn test_is_power_of_two() {
        let powers = vec![1, 2, 4, 8, 16, 32];
        let non_powers = vec![0, 3, 5, 7, 12, 34, 53];

        for power in powers {
            assert!(is_power_of_two(power));
        }

        for non_power in non_powers {
            assert!(!is_power_of_two(non_power));
        }
    }
}
