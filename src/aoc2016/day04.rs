use std::{str::FromStr, collections::HashMap};

use itertools::Itertools;
use once_cell_regex::regex;

struct Room {
    name: HashMap<char, usize>,
    id: i32,
    checksum: String,
}

impl FromStr for Room {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let regex = regex!(r"(?P<name>.*?)-(?P<id>\d+)\[(?P<checksum>\w+)\]");
        let captures = regex.captures(s).unwrap();
        let (name, id, checksum) = (&captures["name"], &captures["id"], &captures["checksum"]);

        // Count the letters in the encrypted name.
        let name = name.chars()
            .filter(|c| *c != '-')
            .counts();
        let id = id.parse().unwrap();
        let checksum = checksum.to_owned();

        assert_eq!(checksum.len(), 5);
        Ok(Room{ name, id, checksum })
    }
}

impl Room {
    fn is_real(&self) -> bool {
        let letters = self.name.iter().collect_vec();

        // Sort the letters by frequency then by alphabetical order to compute the checksum.
        let checksum = letters.into_iter()
            .sorted_by(|(a, b), (c, d)| d.cmp(b).then(a.cmp(c)))
            .map(|(letter, frequency)| letter)
            .take(5)
            .collect::<String>();

        println!("{} {}", checksum, self.checksum);
        checksum == self.checksum
    }
}

pub fn solve_part_one(input: String) -> i32 {
    input.lines()
        .map(|line| line.parse::<Room>().unwrap())
        .filter(|room| room.is_real())
        .map(|room| room.id)
        .sum()
}

pub fn solve_part_two(input: String) -> String {
    "".to_string()
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::{Room, solve_part_one, solve_part_two};

    #[rstest]
    #[case("aaaaa-bbb-z-y-x-123[abxyz]".to_string(), true)]
    #[case("a-b-c-d-e-f-g-h-987[abcde]".to_string(), true)]
    #[case("not-a-real-room-404[oarel]".to_string(), true)]
    #[case("totally-real-room-200[decoy]".to_string(), false)]
    fn test_room_is_real(#[case] input: String, #[case] expected: bool) {
        assert_eq!(input.parse::<Room>().unwrap().is_real(), expected)
    }
}