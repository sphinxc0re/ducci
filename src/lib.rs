/*
    Ducci - A library for creating Ducci sequences from n-tuples
    Copyright (C) 2017  Julian Laubstein

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <http://www.gnu.org/licenses/>.
*/

//! A library for creating Ducci sequences from n-tuples

pub type Tuple = Vec<i64>;

pub fn create_ducci_sequence(entry_sequence: Tuple) -> Vec<Tuple> {
    if !entry_sequence.len().is_power_of_two() {
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

pub fn create_following_ducci(tuple: Tuple) -> Tuple {
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
}
