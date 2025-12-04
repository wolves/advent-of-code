use std::collections::HashSet;

use glam::IVec2;
use tracing::info;

const NEIGHBORS: [IVec2; 8] = [
    IVec2::X,          // South
    IVec2::Y,          // East
    IVec2::NEG_X,      // West
    IVec2::NEG_Y,      // North
    IVec2::ONE,        // SouthEast
    IVec2::NEG_ONE,    // NorthWest
    IVec2::new(1, -1), // NorthEast
    IVec2::new(-1, 1), // SouthWest
];

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<String> {
    let input = input.trim();

    let positions = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(
                move |(x, value)| {
                    (value == '@').then_some(IVec2::new(
                        x as i32, y as i32,
                    ))
                },
            )
        })
        .collect::<HashSet<IVec2>>();

    let count = positions
        .iter()
        .filter(|&position| {
            NEIGHBORS
                .iter()
                .filter(|&offset| {
                    positions.contains(&(position + offset))
                })
                .count()
                < 4
        })
        .count();

    Ok(count.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_process() -> miette::Result<()> {
        let input = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
        assert_eq!("43", process(input)?);
        Ok(())
    }
}
