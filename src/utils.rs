#![allow(dead_code, unused_variables, unused_imports)]

use std::str::FromStr;

use map_macro::map;
use num::Integer;
use once_cell_regex::regex;

use self::grid::Grid;

pub mod advent;
pub mod coordinates;
pub mod coords;
pub mod directions;
pub mod grid;
pub mod io;
pub mod misc;

pub fn get_all_nums<T>(s: &str) -> Vec<T>
where
    T: Integer + FromStr,
{
    let re = regex!(r"[-\d]+");
    re.captures_iter(s)
        .filter_map(|c| c.get(0).and_then(|m| m.as_str().parse::<T>().ok()))
        .collect()
}

// source: https://gist.github.com/codyphobe/67a68b0a85c8653e2c0e8e5eaf73b2d6
pub fn ocr(grid: &Grid<char>) -> String {
    let map = map! {
        " ##  #  # #  # #### #  # #  # " => 'A',
        "###  #  # ###  #  # #  # ###  " => 'B',
        " ##  #  # #    #    #  #  ##  " => 'C',
        "#### #    ###  #    #    #### " => 'E',
        "#### #    ###  #    #    #    " => 'F',
        " ##  #  # #    # ## #  #  ### " => 'G',
        "#  # #  # #### #  # #  # #  # " => 'H',
        " ###   #    #    #    #   ### " => 'I',
        "  ##    #    #    # #  #  ##  " => 'J',
        "#  # # #  ##   # #  # #  #  # " => 'K',
        "#    #    #    #    #    #### " => 'L',
        " ##  #  # #  # #  # #  #  ##  " => 'O',
        "###  #  # #  # ###  #    #    " => 'P',
        "###  #  # #  # ###  # #  #  # " => 'R',
        " ### #    #     ##     # ###  " => 'S',
        "#  # #  # #  # #  # #  #  ##  " => 'U',
        "#    #     # #   #    #    #  " => 'Y',
        "####    #   #   #   #    #### " => 'Z',
    };
    let mut s = String::new();
    let (mut start, mut end) = (0, 5);
    while end < grid.width() {
        let c = grid
            .rows()
            .flat_map(|row| row.skip(start).take(5))
            .collect::<String>();
        let c = map.get(c.as_str()).unwrap_or(&'?');
        s.push(*c);
        start += 5;
        end += 5;
    }
    s
}
