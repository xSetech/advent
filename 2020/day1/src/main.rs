// AOC Day 1 Soln
mod input;


/// Add each expense item together until a combination adds to 2020.
/// Then, multiply the items and return the result.
fn part1(expense_report: &[i32]) -> i32 {

    for (idx1, expense_item1) in expense_report.iter().enumerate() {
        for (idx2, expense_item2) in expense_report.iter().enumerate() {

            // Don't add an item to itself :)
            if idx1 == idx2 {
                continue;
            }

            let expense_sum = expense_item1 + expense_item2;
            if expense_sum == 2020 {
                let expense_mul = expense_item1 * expense_item2;
                return expense_mul;
            }

        }
    }

    panic!("Part 1 didn't complete; bad inputs or bug?")
}


fn main() {
    let part1_soln = part1(input::PART_1_INPUT);
    println!("Part 1 soln: {}", part1_soln);
}


#[cfg(test)]
mod tests {
    use super::*;

    /// The example inputs and outputs given by AOC
    #[test]
    fn example() {
        let example_inputs = &[
            1721,
            979,
            366,
            299,
            675,
            1456,
        ];
        let output = part1(example_inputs);
        assert_eq!(514579, output);
    }

}
