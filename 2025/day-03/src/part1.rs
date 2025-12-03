use itertools::Itertools;
use tracing::info;

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<String> {
    let input = input.trim();
    let result = input
        .lines()
        .map(|bank| {
            let (index, first_max) = &bank
                [..(bank.len() - 1)]
                .chars()
                .enumerate()
                .max_set_by_key(|(_index, battery)| {
                    *battery
                })
                .first()
                .cloned()
                .unwrap();

            let (_second_index, second_max) = &bank
                [(index + 1)..]
                .chars()
                .enumerate()
                .max_by_key(|(_index, battery)| *battery)
                .unwrap();

            format!("{first_max}{second_max}")
                .parse::<u64>()
                .unwrap()
        })
        .sum::<u64>();

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_process() -> miette::Result<()> {
        let input = "987654321111111
811111111111119
234234234234278
818181911112111";
        assert_eq!("357", process(input)?);
        Ok(())
    }
}
