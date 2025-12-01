const STARTING_POINT: i32 = 50;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let mut current_pos = STARTING_POINT;
    let mut zero_count = 0;
    for line in input.lines() {
        let (dir, count_str) = line.split_at(1);
        let count = count_str.parse::<i32>().unwrap();

        let new_pos = match dir {
            "L" => current_pos - count,
            "R" => current_pos + count,
            _ => unreachable!(),
        };

        let mut cycles = match dir {
            "L" => {
                current_pos.div_euclid(100)
                    - new_pos.div_euclid(100)
            }
            "R" => {
                new_pos.div_euclid(100)
                    - current_pos.div_euclid(100)
            }
            _ => unreachable!(),
        };

        if dir == "L" {
            if new_pos.rem_euclid(100) == 0 {
                cycles += 1;
            }

            if current_pos == 0 && new_pos < 0 {
                cycles -= 1;
            }
        }

        zero_count += cycles;
        current_pos = new_pos.rem_euclid(100);
    }

    Ok(zero_count.to_string())
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
        assert_eq!("6", process(input)?);
        Ok(())
    }

    #[test]
    fn test_process_for_r_calculates_0_crosses()
    -> miette::Result<()> {
        let input = "R1000";
        assert_eq!("10", process(input)?);
        Ok(())
    }
    #[test]
    fn test_process_for_l_calculates_0_crosses()
    -> miette::Result<()> {
        let input = "L200";
        assert_eq!("2", process(input)?);
        Ok(())
    }
    #[test]
    fn test_process_for_L_rotations_that_end_on_0()
    -> miette::Result<()> {
        let input = "L50";
        assert_eq!("1", process(input)?);
        Ok(())
    }
    #[test]
    fn test_process_for_L_rotations_that_start_on_0()
    -> miette::Result<()> {
        let input = "L50
L5
R5
L5";
        assert_eq!("2", process(input)?);
        Ok(())
    }
}
