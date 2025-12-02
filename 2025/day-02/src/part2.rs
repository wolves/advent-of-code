use std::ops::RangeInclusive;

use nom::{
    IResult, Parser, bytes::complete::tag,
    character::complete, combinator::all_consuming,
    multi::separated_list1, sequence::separated_pair,
};

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<String> {
    let input = input.trim();
    let (_, id_ranges) =
        all_consuming(ranges).parse(input).map_err(
            |e| miette::miette!("parsing failed {e}"),
        )?;

    let mut total = 0;
    for ids in id_ranges.into_iter() {
        for id in ids.into_iter() {
            let id_str = id.to_string();
            let half = id_str.len() / 2;
            for limit in 0..half {
                if id_str.len().rem_euclid(limit + 1) == 0 {
                    let all_match = id_str[0..=limit]
                        .chars()
                        .cycle()
                        .zip(id_str.chars())
                        .all(|(a, b)| a == b);
                    if all_match {
                        total += id;
                        break;
                    }
                }
            }
        }
    }

    Ok(total.to_string())
}

fn ranges(
    input: &str,
) -> IResult<&str, Vec<RangeInclusive<u64>>> {
    separated_list1(tag(","), range).parse(input)
}

fn range(
    input: &str,
) -> IResult<&str, RangeInclusive<u64>> {
    separated_pair(complete::u64, tag("-"), complete::u64)
        .map(|(start, end)| start..=end)
        .parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_process() -> miette::Result<()> {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        assert_eq!("4174379265", process(input)?);
        Ok(())
    }
}
