use tracing::info;

const STARTING_POINT: i32 = 50;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let mut current_pos = STARTING_POINT;
    let mut zero_count = 0;
    for line in input.lines() {
        let (dir, count_str) = line.split_at(1);
        let count = count_str.parse::<i32>().unwrap();

        info!(?dir, ?count);

        current_pos = (match dir {
            "L" => current_pos - count,
            "R" => current_pos + count,
            _ => unreachable!(),
        })
        .rem_euclid(100);

        if current_pos == 0 {
            zero_count += 1;
        };
    }

    Ok(zero_count.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_process() -> miette::Result<()> {
        let input = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";
        assert_eq!("3", process(input)?);
        Ok(())
    }
}
