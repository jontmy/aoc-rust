# Advent of Code

Solutions for [Advent of Code](https://adventofcode.com) puzzles, written in Rust.

## Getting Started

1. Clone the repository.

```sh
git clone https://github.com/jontmy/aoc-rust
cd aoc-rust
```

2. Get your `SESSION_TOKEN` from the session cookie on [adventofcode.com](https://adventofcode.com) and save it in `.env`.

```sh
vim .env  # add SESSION_TOKEN=...
```

3. Run the solver for a specific puzzle. The solver will automatically download the puzzle input and cache it.

```sh
cargo run --release -- -d <day> -y <year>
```

>[!TIP]
> If Advent of Code is currently ongoing, the `-d` and `-y` flags can be omitted. The current system date will be used instead.

#### Other tips
- The `-i` flag lets you specify a custom file extension for the file which the input will be read from. Useful for example inputs.
- For example, `-d 1 -y 2024 -i ex1` will read from `input/2024/01.ex1` (note the leading zero).
- Setting the `-r` flag will re-download the input.

## Progress

|   Year   | 2024  | 2023  | 2022  | 2021  | 2020 | 2019 | 2018 | 2017 | 2016  | 2015  |
|:--------:|:-----:|:-----:|:-----:|:-----:|:----:|:----:|:----:|:----:|:-----:|:-----:|
| Progress | `24*` |  `×`  | `28*` | `41*` | `×`  | `×`  | `×`  | `2*` | `20*` | `38*` |

> Progress indicated only for solutions in Rust. Solutions in other languages:
> - [Haskell](https://github.com/jontmy/aoc-haskell)
> - [Java](https://github.com/jontmy/aoc-java)
> - [Python](https://github.com/jontmy/aoc-python)

### 2024

| S | M | T | W | T | F | S |
|:-:|:-:|:-:|:-:|:-:|:-:|:-:|
| [1](https://github.com/jontmy/aoc-rust/blob/master/src/aoc2024/day01.rs) | [2](https://github.com/jontmy/aoc-rust/blob/master/src/aoc2024/day02.rs) | [3](https://github.com/jontmy/aoc-rust/blob/master/src/aoc2024/day03.rs) | [4](https://github.com/jontmy/aoc-rust/blob/master/src/aoc2024/day04.rs) | [5](https://github.com/jontmy/aoc-rust/blob/master/src/aoc2024/day05.rs) | [6](https://github.com/jontmy/aoc-rust/blob/master/src/aoc2024/day06.rs) | [7](https://github.com/jontmy/aoc-rust/blob/master/src/aoc2024/day07.rs) |
| [8](https://github.com/jontmy/aoc-rust/blob/master/src/aoc2024/day08.rs) | [9](https://github.com/jontmy/aoc-rust/blob/master/src/aoc2024/day09.rs) | [10](https://github.com/jontmy/aoc-rust/blob/master/src/aoc2024/day10.rs) | [11](https://github.com/jontmy/aoc-rust/blob/master/src/aoc2024/day11.rs) | [12](https://github.com/jontmy/aoc-rust/blob/master/src/aoc2024/day12.rs) | 13 | 14 |
| 15 | 16 | 17 | 18 | 19 | 20 | 21 |
| 22 | 23 | 24 | 25 |

### 2022

|                                     S                                     |                                     M                                     |                                    T                                     |                                    W                                     |                                    T                                     |                                    F                                     |                                     S                                     |
|:-------------------------------------------------------------------------:|:-------------------------------------------------------------------------:|:------------------------------------------------------------------------:|:------------------------------------------------------------------------:|:------------------------------------------------------------------------:|:------------------------------------------------------------------------:|:-------------------------------------------------------------------------:|
|                                                                           |                                                                           |                                                                          |                                                                          | [1](https://github.com/jontmy/aoc-rust/blob/master/src/aoc2022/day01.rs) | [2](https://github.com/jontmy/aoc-rust/blob/master/src/aoc2022/day02.rs) | [3](https://github.com/jontmy/aoc-rust/blob/master/src/aoc2022/day03.rs)  |
| [4](https://github.com/jontmy/aoc-rust/blob/master/src/aoc2022/day04.rs)  | [5](https://github.com/jontmy/aoc-rust/blob/master/src/aoc2022/day05.rs)  | [6](https://github.com/jontmy/aoc-rust/blob/master/src/aoc2022/day06.rs) | [7](https://github.com/jontmy/aoc-rust/blob/master/src/aoc2022/day07.rs) | [8](https://github.com/jontmy/aoc-rust/blob/master/src/aoc2022/day08.rs) | [9](https://github.com/jontmy/aoc-rust/blob/master/src/aoc2022/day09.rs) | [10](https://github.com/jontmy/aoc-rust/blob/master/src/aoc2022/day10.rs) |
| [11](https://github.com/jontmy/aoc-rust/blob/master/src/aoc2022/day11.rs) | [12](https://github.com/jontmy/aoc-rust/blob/master/src/aoc2022/day12.rs) |                                   13                                    |                                   14                                    |                                   15                                    |                                   16                                    | [17](https://github.com/jontmy/aoc-rust/blob/master/src/aoc2022/day17.rs) |
| [18](https://github.com/jontmy/aoc-rust/blob/master/src/aoc2022/day18.rs) |                                    19                                     |                                    20                                    |                                    21                                    |                                    22                                    |                                    23                                    |                                    24                                     |
|                                    25                                     |                                                                           |                                                                          |                                                                          |                                                                          |                                                                          |                                                                           |

### 2021

|                                     S                                     |                                     M                                     |                                     T                                     |                                     W                                     |                                    T                                     |                                     F                                     |                                     S                                     |
|:-------------------------------------------------------------------------:|:-------------------------------------------------------------------------:|:-------------------------------------------------------------------------:|:-------------------------------------------------------------------------:|:------------------------------------------------------------------------:|:-------------------------------------------------------------------------:|:-------------------------------------------------------------------------:|
|                                                                           |                                                                           |                                                                           | [1](https://github.com/jontmy/aoc-rust/blob/master/src/aoc2021/day01.rs)  | [2](https://github.com/jontmy/aoc-rust/blob/master/src/aoc2021/day02.rs) | [3](https://github.com/jontmy/aoc-rust/blob/master/src/aoc2021/day03.rs)  | [4](https://github.com/jontmy/aoc-rust/blob/master/src/aoc2021/day04.rs)  |
| [5](https://github.com/jontmy/aoc-rust/blob/master/src/aoc2021/day05.rs)  | [6](https://github.com/jontmy/aoc-rust/blob/master/src/aoc2021/day06.rs)  | [7](https://github.com/jontmy/aoc-rust/blob/master/src/aoc2021/day07.rs)  | [8](https://github.com/jontmy/aoc-rust/blob/master/src/aoc2021/day08.rs)  | [9](https://github.com/jontmy/aoc-rust/blob/master/src/aoc2021/day09.rs) | [10](https://github.com/jontmy/aoc-rust/blob/master/src/aoc2021/day10.rs) | [11](https://github.com/jontmy/aoc-rust/blob/master/src/aoc2021/day11.rs) |
| [12](https://github.com/jontmy/aoc-rust/blob/master/src/aoc2021/day12.rs) | [13](https://github.com/jontmy/aoc-rust/blob/master/src/aoc2021/day13.rs) | [14](https://github.com/jontmy/aoc-rust/blob/master/src/aoc2021/day14.rs) | [15](https://github.com/jontmy/aoc-rust/blob/master/src/aoc2021/day15.rs) |                                    16                                    | [17](https://github.com/jontmy/aoc-rust/blob/master/src/aoc2021/day17.rs) | [18](https://github.com/jontmy/aoc-rust/blob/master/src/aoc2021/day18.rs) |
|                                    19                                     | [20](https://github.com/jontmy/aoc-rust/blob/master/src/aoc2021/day20.rs) |                                    21                                     |                                    22                                     |                                    23                                    |                                    24                                     | [25](https://github.com/jontmy/aoc-rust/blob/master/src/aoc2021/day25.rs) |

### 2016

|                                     S                                     |                                     M                                     |                                    T                                     |                                    W                                     |                                    T                                     |                                    F                                     |                                     S                                     |
|:-------------------------------------------------------------------------:|:-------------------------------------------------------------------------:|:------------------------------------------------------------------------:|:------------------------------------------------------------------------:|:------------------------------------------------------------------------:|:------------------------------------------------------------------------:|:-------------------------------------------------------------------------:|
|                                                                           |                                                                           |                                                                          |                                                                          |                                    1                                     |                                    2                                     | [3](https://github.com/jontmy/aoc-rust/blob/master/src/aoc2016/day03.rs)  |
| [4](https://github.com/jontmy/aoc-rust/blob/master/src/aoc2016/day04.rs)  | [5](https://github.com/jontmy/aoc-rust/blob/master/src/aoc2016/day05.rs)  | [6](https://github.com/jontmy/aoc-rust/blob/master/src/aoc2016/day06.rs) | [7](https://github.com/jontmy/aoc-rust/blob/master/src/aoc2016/day07.rs) | [8](https://github.com/jontmy/aoc-rust/blob/master/src/aoc2016/day08.rs) | [9](https://github.com/jontmy/aoc-rust/blob/master/src/aoc2016/day09.rs) | [10](https://github.com/jontmy/aoc-rust/blob/master/src/aoc2016/day10.rs) |
| [11](https://github.com/jontmy/aoc-rust/blob/master/src/aoc2016/day11.rs) | [12](https://github.com/jontmy/aoc-rust/blob/master/src/aoc2016/day12.rs) |                                    13                                    |                                    14                                    |                                    15                                    |                                    16                                    |                                    17                                     |
|                                    18                                     |                                    19                                     |                                    20                                    |                                    21                                    |                                    22                                    |                                    23                                    |                                    24                                     |
|                                    25                                     |                                                                           |                                                                          |                                                                          |                                                                          |                                                                          |                                                                           |

### 2015

|                                     S                                     |                                     M                                     |                                     T                                     |                                     W                                     |                                     T                                     |                                     F                                     |                                     S                                     |
|:-------------------------------------------------------------------------:|:-------------------------------------------------------------------------:|:-------------------------------------------------------------------------:|:-------------------------------------------------------------------------:|:-------------------------------------------------------------------------:|:-------------------------------------------------------------------------:|:-------------------------------------------------------------------------:|
|                                                                           |                                                                           | [1](https://github.com/jontmy/aoc-rust/blob/master/src/aoc2015/day01.rs)  | [2](https://github.com/jontmy/aoc-rust/blob/master/src/aoc2015/day02.rs)  | [3](https://github.com/jontmy/aoc-rust/blob/master/src/aoc2015/day03.rs)  | [4](https://github.com/jontmy/aoc-rust/blob/master/src/aoc2015/day04.rs)  | [5](https://github.com/jontmy/aoc-rust/blob/master/src/aoc2015/day05.rs)  |
| [6](https://github.com/jontmy/aoc-rust/blob/master/src/aoc2015/day06.rs)  | [7](https://github.com/jontmy/aoc-rust/blob/master/src/aoc2015/day07.rs)  | [8](https://github.com/jontmy/aoc-rust/blob/master/src/aoc2015/day08.rs)  | [9](https://github.com/jontmy/aoc-rust/blob/master/src/aoc2015/day09.rs)  | [10](https://github.com/jontmy/aoc-rust/blob/master/src/aoc2015/day10.rs) | [11](https://github.com/jontmy/aoc-rust/blob/master/src/aoc2015/day11.rs) | [12](https://github.com/jontmy/aoc-rust/blob/master/src/aoc2015/day12.rs) |
| [13](https://github.com/jontmy/aoc-rust/blob/master/src/aoc2015/day13.rs) | [14](https://github.com/jontmy/aoc-rust/blob/master/src/aoc2015/day14.rs) | [15](https://github.com/jontmy/aoc-rust/blob/master/src/aoc2015/day15.rs) | [16](https://github.com/jontmy/aoc-rust/blob/master/src/aoc2015/day16.rs) | [17](https://github.com/jontmy/aoc-rust/blob/master/src/aoc2015/day17.rs) | [18](https://github.com/jontmy/aoc-rust/blob/master/src/aoc2015/day18.rs) | [19](https://github.com/jontmy/aoc-rust/blob/master/src/aoc2015/day19.rs) |
|                                    20                                     |                                    21                                     |                                    22                                     |                                    23                                     |                                    24                                     |                                    25                                     |                                                                           |
