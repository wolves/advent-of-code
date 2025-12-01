const STARTING_POINT: i32 = 50;

pub fn process(input: &str) -> miette::Result<String> {
    for line in input.lines() {
        let (dir, count) = line.split_at(1);
    }

    let result = 0;

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
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
