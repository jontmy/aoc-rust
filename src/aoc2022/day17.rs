use itertools::Itertools;

use crate::aoc2022::day17::Shape::{Cross, Horizontal, Perpendicular, Square, Vertical};
use crate::utils::directions::Direction;
use crate::utils::{advent, coords::Coordinates, grid::Grid};

pub struct Solver;

#[derive(Debug)]
enum Shape {
    // ####
    Horizontal,
    //
    // .#.
    // ###
    // .#.
    Cross,
    //
    // ..#
    // ..#
    // ###
    Perpendicular,
    //
    // #
    // #
    // #
    // #
    Vertical,
    //
    // ##
    // ##
    Square,
}

impl Shape {
    fn new(i: usize) -> Self {
        match i % 5 {
            0 => Horizontal,
            1 => Cross,
            2 => Perpendicular,
            3 => Vertical,
            4 => Square,
            _ => panic!(),
        }
    }

    fn width(&self) -> i32 {
        match self {
            Horizontal => 4,
            Cross => 3,
            Perpendicular => 3,
            Vertical => 1,
            Square => 2,
        }
    }

    fn height(&self) -> i32 {
        match self {
            Horizontal => 1,
            Cross => 3,
            Perpendicular => 3,
            Vertical => 4,
            Square => 2,
        }
    }

    fn fill_coords(&self, c: Coordinates<i32>) -> Vec<Coordinates<i32>> {
        match self {
            Horizontal => vec![c, c.right(), c.right_by(2), c.right_by(3)],
            Cross => vec![
                c.up(),
                c.up().right(),
                c.up().right_by(2),
                c.right(),
                c.up_by(2).right(),
            ],
            Perpendicular => vec![
                c,
                c.right(),
                c.right_by(2),
                c.right_by(2).up(),
                c.right_by(2).up_by(2),
            ],
            Vertical => vec![c, c.up(), c.up_by(2), c.up_by(3)],
            Square => vec![c, c.up(), c.up().right(), c.right()],
        }
    }
}

#[derive(Debug)]
struct Rock {
    shape: Shape,
    position: Coordinates<i32>, // bottom-left edge
}

impl Rock {
    fn new(i: usize, x: i32, y: i32) -> Self {
        Self {
            shape: Shape::new(i),
            position: Coordinates::new(x, y),
        }
    }

    fn push(&mut self, d: Direction, grid: &Grid<bool>) -> bool {
        if let Direction::Left = d {
            if self.position.x() == 0 {
                return false;
            }
            for c in self.shape.fill_coords(self.position.left()) {
                let (x, y) = c.into();
                if *grid.get(x as usize, y as usize) {
                    return false;
                }
            }
        }

        if let Direction::Right = d {
            if self.position.x() + self.shape.width() == Solver::WIDTH as i32 {
                return false;
            }
            for c in self.shape.fill_coords(self.position.right()) {
                let (x, y) = c.into();
                if *grid.get(x as usize, y as usize) {
                    return false;
                }
            }
        }

        if let Direction::Down = d {
            if self.position.y() == 0 {
                return false;
            }
            for c in self.shape.fill_coords(self.position.down()) {
                let (x, y) = c.into();
                if *grid.get(x as usize, y as usize) {
                    return false;
                }
            }
        }

        self.position = self.position.step(d);
        true
    }

    fn settle(self, grid: &mut Grid<bool>) {
        for c in self.shape.fill_coords(self.position) {
            let (x, y) = c.into();
            let (x, y) = (x as usize, y as usize);
            assert!(!grid.get(x, y));
            grid.set(x, y, true);
        }
    }
}

impl Solver {
    const WIDTH: usize = 7;

    fn grid(rocks: usize) -> Grid<bool> {
        let height = rocks * 4 + 8;
        Grid::from_value(height, Solver::WIDTH, false)
    }
}

impl advent::Solver<2022, 17> for Solver {
    type Part1 = i32;
    type Part2 = String;

    fn solve_part_one(&self, input: &str) -> Self::Part1 {
        let jets = input
            .trim()
            .chars()
            .map(|c| match c {
                '<' => Direction::Left,
                '>' => Direction::Right,
                _ => panic!(),
            })
            .collect_vec();

        const ROCKS: usize = 2022;
        let mut grid = Solver::grid(ROCKS);
        let mut height = 0;
        let mut jet_ptr = 0;

        // let the rocks fall one by one
        for i in 0..ROCKS {
            let mut rock = Rock::new(i, 2, height + 3);

            // let the rock fall until it is stable
            loop {
                // jet movement
                let jet = jets[jet_ptr % jets.len()];
                rock.push(jet, &grid);
                jet_ptr += 1;

                // gravity movement
                if !rock.push(Direction::Down, &grid) {
                    break;
                }
            }

            // update the total height of the rock formation
            height = height.max(rock.position.y() + rock.shape.height());

            // add the rock's final resting position to the grid
            rock.settle(&mut grid);
        }

        height
    }

    fn solve_part_two(&self, _input: &str) -> Self::Part2 {
        "[not done programmatically - find the cycle delta manually]".to_owned()
    }
}
