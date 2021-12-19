use itertools::Itertools;

pub mod dim_2 {
    use std::cmp::Ordering;
    use std::fmt::{Debug, Display, Formatter};
    use std::ops::{Add, Div, Mul, Range, Sub};

    use itertools::Itertools;

    #[derive(Clone, Copy, Hash, PartialEq, Eq)]
    pub struct Coordinates(i32, i32);

    impl Coordinates {
        /**
        Returns an iterator of all coordinates in a 2D area, range-bound on both axes.
         */
        pub fn in_area(
            x_range: Range<i32>,
            y_range: Range<i32>,
        ) -> impl Iterator<Item = Coordinates> {
            x_range.cartesian_product(y_range).map(Coordinates::from)
        }

        /// Returns a new instance of `Coordinates` given its `x`- and `y`-components.
        pub fn at(x: i32, y: i32) -> Self {
            Coordinates(x, y)
        }

        /// Returns the `x`-component of this `Coordinates`.
        pub fn x(&self) -> i32 {
            self.0
        }

        /// Returns the `y`-component of this `Coordinates`.
        pub fn y(&self) -> i32 {
            self.1
        }

        /// Returns a new instance of `Coordinates` offset by `dx` on the `x`-axis and `dy` on the
        /// `y`-axis.
        ///
        /// Equivalent to `self + Coordinates::at(dx, dy)`, by which it is implemented.
        pub fn translate(&self, dx: i32, dy: i32) -> Self {
            *self + Coordinates::at(dx, dy)
        }

        /// Returns a new instance of `Coordinates` multiplied by a scalar `k` component-wise.
        ///
        /// Equivalent to `self * Coordinates::at(k, k)`, by which it is implemented.
        pub fn scale(&self, k: i32) -> Self {
            *self * Coordinates::at(k, k)
        }

        /**
        Returns an iterator of all 2D coordinates that are offset by exactly `delta` units in
        either direction on only one axis.
         */
        pub fn axial_offset_by(&self, delta: i32) -> impl Iterator<Item = Coordinates> + '_ {
            match delta {
                0 => vec![Coordinates(0, 0)],
                _ => vec![
                    Coordinates(0, delta),
                    Coordinates(0, -delta),
                    Coordinates(delta, 0),
                    Coordinates(-delta, 0),
                ],
            }
            .into_iter()
            .map(|offset| *self + offset)
        }

        /**
        Returns an iterator of all 2D coordinates that are offset by exactly `delta` units in
        either direction on both axes.
         */
        pub fn diagonal_offset_by(&self, delta: i32) -> impl Iterator<Item = Coordinates> + '_ {
            match delta {
                0 => vec![Coordinates(0, 0)],
                _ => vec![
                    Coordinates(delta, delta),
                    Coordinates(delta, -delta),
                    Coordinates(-delta, delta),
                    Coordinates(-delta, -delta),
                ],
            }
            .into_iter()
            .map(|offset| *self + offset)
        }

        /**
        Returns an iterator of all 2D coordinates that are offset by exactly `delta` units in
        either direction on any axis.
         */
        pub fn all_offset_by(&self, delta: i32) -> impl Iterator<Item = Coordinates> + '_ {
            match delta {
                0 => vec![Coordinates(0, 0)],
                _ => vec![
                    Coordinates(0, delta),
                    Coordinates(0, -delta),
                    Coordinates(delta, 0),
                    Coordinates(-delta, 0),
                    Coordinates(delta, delta),
                    Coordinates(delta, -delta),
                    Coordinates(-delta, delta),
                    Coordinates(-delta, -delta),
                ],
            }
            .into_iter()
            .map(|offset| *self + offset)
        }

        /**
        Returns the Manhattan distance between two 2D coordinates.
         */
        pub fn manhattan(&self, other: Coordinates) -> i32 {
            (self.0 - other.0).abs() + (self.1 - other.1).abs()
        }

        /**
        Returns the Euclidean distance between two 2D coordinates.
         */
        pub fn euclidean(&self, other: Coordinates) -> f64 {
            f64::sqrt(((self.0 - other.0).pow(2) + (self.1 - other.1).pow(2)) as f64)
        }
    }

    impl From<(i32, i32)> for Coordinates {
        fn from(c: (i32, i32)) -> Self {
            Coordinates(c.0, c.1)
        }
    }

    impl Add for Coordinates {
        type Output = Self;

        fn add(self, rhs: Self) -> Self::Output {
            Coordinates(self.0 + rhs.0, self.1 + rhs.1)
        }
    }

    impl Sub for Coordinates {
        type Output = Self;

        fn sub(self, rhs: Self) -> Self::Output {
            Coordinates(self.0 - rhs.0, self.1 - rhs.1)
        }
    }

    impl Mul for Coordinates {
        type Output = Self;

        fn mul(self, rhs: Self) -> Self::Output {
            Coordinates(self.0 * rhs.0, self.1 * rhs.1)
        }
    }

    impl Div for Coordinates {
        type Output = Coordinates;

        fn div(self, rhs: Self) -> Self::Output {
            Coordinates(self.0 / rhs.0, self.1 / rhs.1)
        }
    }

    impl Debug for Coordinates {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "({}, {})", self.0, self.1)
        }
    }

    impl Display for Coordinates {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "({}, {})", self.0, self.1)
        }
    }

    impl PartialOrd for Coordinates {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Option::from(self.0.cmp(&other.0).then(self.1.cmp(&other.1)))
        }
    }

    impl Ord for Coordinates {
        fn cmp(&self, other: &Self) -> Ordering {
            self.partial_cmp(other).unwrap()
        }
    }

    #[cfg(test)]
    mod tests {
        use std::collections::HashSet;
        use std::ops::Range;

        use rstest::rstest;

        use super::Coordinates;

        #[rstest]
        #[case(0..0, 0..0, vec ! [])]
        #[case(0..0, 0..1, vec ! [])]
        #[case(0..1, 0..0, vec ! [])]
        #[case(0..1, 0..1, vec ! [(0, 0)])]
        #[case(0..2, 0..2, vec ! [(0, 0), (0, 1), (1, 0), (1, 1)])]
        #[case(- 1..2, - 1..2, vec ! [(- 1, 1), (- 1, 0), (- 1, - 1), (0, 1), (0, 0), (0, - 1), (1, 1), (1, 0), (1, - 1)])]
        fn test_in_area(
            #[case] x_range: Range<i32>,
            #[case] y_range: Range<i32>,
            #[case] expected: Vec<(i32, i32)>,
        ) {
            assert_eq!(
                Coordinates::in_area(x_range.clone(), y_range.clone()).count(),
                expected.len()
            );
            assert_eq!(
                Coordinates::in_area(x_range, y_range).collect::<HashSet<Coordinates>>(),
                expected
                    .into_iter()
                    .map(Coordinates::from)
                    .collect::<HashSet<Coordinates>>()
            );
        }

        #[rstest]
        #[case((0, 0), 0, vec ! [(0, 0)])]
        #[case((0, 0), 1, vec ! [(0, 1), (0, - 1), (1, 0), (- 1, 0)])]
        #[case((10, 10), 0, vec ! [(10, 10)])]
        #[case((10, 10), 1, vec ! [(10, 11), (10, 9), (11, 10), (9, 10)])]
        #[case((0, 0), 10, vec ! [(0, 10), (0, - 10), (10, 0), (- 10, 0)])]
        #[case((10, 10), 10, vec ! [(10, 20), (10, 0), (20, 10), (0, 10)])]
        fn test_axial_offset(
            #[case] origin: (i32, i32),
            #[case] delta: i32,
            #[case] expected: Vec<(i32, i32)>,
        ) {
            assert_eq!(
                Coordinates::from(origin).axial_offset_by(delta).count(),
                expected.len()
            );
            assert_eq!(
                Coordinates::from(origin)
                    .axial_offset_by(delta)
                    .collect::<HashSet<Coordinates>>(),
                expected
                    .into_iter()
                    .map(Coordinates::from)
                    .collect::<HashSet<Coordinates>>()
            );
        }

        #[rstest]
        #[case((0, 0), 0, vec ! [(0, 0)])]
        #[case((0, 0), 1, vec ! [(1, 1), (1, - 1), (- 1, - 1), (- 1, 1)])]
        #[case((10, 10), 0, vec ! [(10, 10)])]
        #[case((10, 10), 1, vec ! [(11, 11), (11, 9), (9, 11), (9, 9)])]
        #[case((0, 0), 10, vec ! [(10, 10), (- 10, - 10), (10, - 10), (- 10, 10)])]
        #[case((10, 10), 10, vec ! [(0, 0), (20, 20), (20, 0), (0, 20)])]
        fn test_diagonal_offset(
            #[case] origin: (i32, i32),
            #[case] delta: i32,
            #[case] expected: Vec<(i32, i32)>,
        ) {
            assert_eq!(
                Coordinates::from(origin).diagonal_offset_by(delta).count(),
                expected.len()
            );
            assert_eq!(
                Coordinates::from(origin)
                    .diagonal_offset_by(delta)
                    .collect::<HashSet<Coordinates>>(),
                expected
                    .into_iter()
                    .map(Coordinates::from)
                    .collect::<HashSet<Coordinates>>()
            );
        }

        #[rstest]
        #[case((0, 0), 0)]
        #[case((0, 0), 1)]
        #[case((10, 10), 0)]
        #[case((10, 10), 1)]
        #[case((0, 0), 10)]
        #[case((10, 10), 10)]
        fn test_all_offset(#[case] origin: (i32, i32), #[case] delta: i32) {
            assert_eq!(
                Coordinates::from(origin)
                    .all_offset_by(delta)
                    .collect::<HashSet<Coordinates>>(),
                Coordinates::from(origin)
                    .axial_offset_by(delta)
                    .into_iter()
                    .chain(Coordinates::from(origin).diagonal_offset_by(delta))
                    .map(Coordinates::from)
                    .collect::<HashSet<Coordinates>>()
            );
        }
    }
}

fn add(i: i32, u: usize) -> Option<usize> {
    if i.is_negative() {
        u.checked_sub(i.wrapping_abs() as u32 as usize)
    } else {
        u.checked_add(i as usize)
    }
}

pub fn offset_by(origin: &[usize], offset: i32) -> impl Iterator<Item = Vec<usize>> + '_ {
    let dimensions = origin.len();
    let delta = offset;
    [0, delta, -delta]
        .into_iter()
        .combinations_with_replacement(dimensions)
        .flat_map(move |comb| comb.into_iter().permutations(dimensions))
        .unique()
        .filter(|offset| offset.into_iter().any(|i| *i != 0))
        .map(|offset| {
            offset
                .into_iter()
                .zip(origin.into_iter())
                .filter_map(|(i, &u)| add(i, u))
                .collect_vec()
        })
        .filter(move |offset| offset.len() == dimensions)
}

pub fn axial_offset_by(origin: &[usize], offset: i32) -> impl Iterator<Item = Vec<usize>> + '_ {
    let dimensions = origin.len();
    let delta = offset;

    vec![0; dimensions.saturating_sub(2)]
        .into_iter()
        .chain([delta].into_iter())
        .permutations(dimensions)
        .filter(|offset| !offset.into_iter().all(|i| *i == 0))
        .flat_map(|offset| {
            [
                offset.clone(),
                offset.into_iter().map(|i| -i.clone()).collect_vec(),
            ]
        })
        .map(|offset| {
            offset
                .into_iter()
                .filter_map(|i| usize::try_from(i).ok())
                .zip(origin.into_iter())
                .map(|(a, b)| a + *b)
                .collect_vec()
        })
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;
    use rstest::rstest;

    use super::offset_by;

    #[rstest]
    #[case(
    vec ! [], 0,
    vec ! []
    )]
    #[case(
    vec ! [], 99999,
    vec ! []
    )]
    #[case(
    vec ! [], 99999,
    vec ! []
    )]
    fn test_all_offsets_zero_dimensions(
        #[case] origin: Vec<usize>,
        #[case] offset: i32,
        #[case] expected: Vec<Vec<usize>>,
    ) {
        assert_eq!(offset_by(&origin, offset).collect_vec(), expected)
    }

    #[rstest]
    #[case(
    vec ! [0], 0,
    vec ! []
    )]
    #[case(
    vec ! [99999], 0,
    vec ! []
    )]
    #[case(
    vec ! [0], 1,
    vec ! [vec ! [1]]
    )]
    #[case(
    vec ! [0], - 1,
    vec ! [vec ! [1]]
    )]
    #[case(
    vec ! [0], 99999,
    vec ! [vec ! [99999]]
    )]
    #[case(
    vec ! [0], - 99999,
    vec ! [vec ! [99999]]
    )]
    #[case(
    vec ! [4], 2,
    vec ! [vec ! [2], vec ! [6]]
    )]
    fn test_all_offsets_one_dimension(
        #[case] origin: Vec<usize>,
        #[case] offset: i32,
        #[case] expected: Vec<Vec<usize>>,
    ) {
        assert!(iter_vec_eq(offset_by(&origin, offset), expected))
    }

    #[rstest]
    #[case(
    vec ! [0, 0], 0,
    vec ! []
    )]
    #[case(
    vec ! [99999, 99999], 0,
    vec ! []
    )]
    #[case(
    vec ! [0, 0], 1,
    vec ! [vec ! [1, 1], vec ! [0, 1], vec ! [1, 0]]
    )]
    #[case(
    vec ! [0, 0], - 1,
    vec ! [vec ! [1, 1], vec ! [0, 1], vec ! [1, 0]]
    )]
    #[case(
    vec ! [0, 0], 99999,
    vec ! [vec ! [99999, 99999], vec ! [0, 99999], vec ! [99999, 0]]
    )]
    #[case(
    vec ! [0, 0], - 99999,
    vec ! [vec ! [99999, 99999], vec ! [0, 99999], vec ! [99999, 0]]
    )]
    #[case(
    vec ! [4, 4], 2,
    vec ! [vec ! [4, 6], vec ! [6, 4], vec ! [4, 2], vec ! [2, 4], vec ! [6, 6], vec ! [6, 2], vec ! [2, 6], vec ! [2, 2]]
    )]
    fn test_all_offsets_two_dimensions(
        #[case] origin: Vec<usize>,
        #[case] offset: i32,
        #[case] expected: Vec<Vec<usize>>,
    ) {
        print_1d_vec("origin: ", &origin);
        println!("offset: {}", offset);
        assert!(iter_vec_eq(offset_by(&origin, offset), expected))
    }

    #[rstest]
    #[case(
    vec ! [1, 1, 1], 1,
    vec ! []
    )]
    fn test_all_offsets_three_dimensions(
        #[case] origin: Vec<usize>,
        #[case] offset: i32,
        #[case] expected: Vec<Vec<usize>>,
    ) {
        print_1d_vec("origin: ", &origin);
        println!("offset: {}", offset);
        let actual = offset_by(&origin, offset).collect_vec();
        /*
        actual
            .iter()
            .map(|a| i32::from_str_radix(&*format!("{}{}{}", a[0], a[1], a[2]), 3).unwrap())
            .sorted()
            .for_each(|x| {
                println!("{}", x);
            });


        print_2d_vec("actual: ", &actual);

         */
        assert_eq!(actual.len(), 3_usize.pow(3) - 1)
    }

    #[rstest]
    #[case(
    vec ! [4, 4, 4, 4], 2,
    vec ! []
    )]
    fn test_all_offsets_four_dimensions(
        #[case] origin: Vec<usize>,
        #[case] offset: i32,
        #[case] expected: Vec<Vec<usize>>,
    ) {
        print_1d_vec("origin: ", &origin);
        println!("offset: {}", offset);
        assert_eq!(offset_by(&origin, offset).count(), 3_usize.pow(4) - 1)
    }

    fn format_1d_vec(name: &str, xs: &Vec<usize>) -> String {
        format!("{}[{}]", name, xs.iter().join(", "))
    }

    fn format_2d_vec(name: &str, xss: &Vec<Vec<usize>>) -> String {
        format!(
            "{}[{}]",
            name,
            xss.iter().map(|xs| format_1d_vec("", xs)).join("\n")
        )
    }

    fn print_1d_vec(name: &str, xs: &Vec<usize>) {
        println!("{}", format_1d_vec(name, xs))
    }

    fn print_2d_vec(name: &str, xss: &Vec<Vec<usize>>) {
        println!("{}", format_2d_vec(name, xss))
    }

    fn iter_vec_eq<'a>(xss: impl Iterator<Item = Vec<usize>> + 'a, yss: Vec<Vec<usize>>) -> bool {
        let xss = xss.collect_vec();
        print_2d_vec("actual:   ", &xss);
        print_2d_vec("expected: ", &yss);

        xss.len() == yss.len()
            && xss
                .into_iter()
                .all(|xs| yss.iter().any(|ys| xs.iter().all(|x| ys.contains(&x))))
    }
}
