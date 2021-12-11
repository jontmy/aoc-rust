use itertools::Itertools;

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
        .flat_map(|offset| [
            offset.clone(),
            offset.into_iter().map(|i| -i.clone()).collect_vec(),
        ])
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
    use std::collections::HashSet;

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
        format!("{}[{}]", name, xss.iter().map(|xs| format_1d_vec("", xs)).join("\n"))
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

        xss.len() == yss.len() && xss.into_iter().all(|xs| {
            yss.iter().any(|ys| {
                xs.iter().all(|x| ys.contains(&x))
            })
        })
    }
}
