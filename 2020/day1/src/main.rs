// AOC Day 1 Soln
mod input;

use std::vec::Vec;


/// Compute n choose k
fn choose(n: i32, k: i32) -> i32 {
    let mut num = 1;
    let mut i = n;
    let i_max = n - k + 1;
    loop {
        num *= i;
        i -= 1;
        if i < i_max {
            break;
        }
    }

    let mut den = 1;
    let mut i = k;
    loop {
        den *= i;
        i -= 1;
        if i <= 1 {
            break;
        }
    }

    return num / den;
}


/// Compute indices of all non-repeating k-combinations of an s-sized set
/// Naive loops from Art of Programming by Knuth
fn combinations(s: i32, k: i32) -> Vec<Vec<i32>> {
    let total_combinations = choose(s, k);
    let mut combinations: Vec<Vec<i32>> = Vec::with_capacity(total_combinations as usize);

    // Special case - 1-combinations are just the indices
    if k == 1 {
        for i in 0..s {
            let combination: Vec<i32> = vec![i];
            combinations.push(combination);
        }
        return combinations;
    }

    // The created combination is also the state of a stack of the inner-most
    // loop of the algorithm.
    let mut combination: Vec<i32> = Vec::with_capacity(k as usize);
    combination.push(k - 1);

    let upper = s - 1;
    let lower = k - 1;
    let mut depth: usize = 0;
    loop {

        // Exit when the 0th loop reaches s-1
        if depth == 0 && combination[depth] > upper {
            break;
        }

        // Only the lowest loop, the (k-1)th, generates combinations
        if depth == lower as usize {
            for i in 0..combination[depth-1] {
                combination.push(i);
                combinations.push(combination.clone());
                combination.pop();
            }
            combination[depth-1] += 1;
            depth -= 1;
            continue;
        }

        // Handle intermediate "loops"
        if depth > 0 && depth < lower as usize {

            // Descend if the current loop's range start is missing
            if combination.len() < depth + 1 {
                combination.push(lower - depth as i32);
                depth += 1;
                continue
            }

            // Ascend if the current loop's range start is maximal
            if combination[depth] >= combination[depth-1] {
                combination.pop();
                combination[depth-1] += 1;
                depth -= 1;
                continue;
            }

        }

        depth += 1;
    }

    return combinations;
}


// Find all non-repeating subsets of an input collection of integers.
// Homework: make this generic
fn subsets(input: &[i32], k: i32) -> Vec<Vec<i32>> {
    let combinations_of_indices = combinations(input.len() as i32, k);
    let mut combinations_of_values = Vec::with_capacity(combinations_of_indices.len());
    for combination_of_indices in combinations_of_indices.iter() {
        let values_from_input = combination_of_indices.iter().map(
            |idx| { input[*idx as usize] }
        ).collect();
        combinations_of_values.push(values_from_input);
    }
    return combinations_of_values;
}


/// Find two numbers that sum to 2020 and return their product
fn part1(expense_report: &[i32]) -> i32 {
    let combinations: Vec<Vec<i32>> = subsets(expense_report, 2);
    for combination in combinations.iter() {
        let sum: i32 = combination.iter().sum();
        if sum == 2020 {
            return combination.iter().product();
        }
    }
    panic!("Part 1 didn't complete; bad inputs or bug?");
}


/// Find three numbers that sum to 2020 and return their product
fn part2(expense_report: &[i32]) -> i32 {
    let combinations: Vec<Vec<i32>> = subsets(expense_report, 3);
    for combination in combinations.iter() {
        let sum: i32 = combination.iter().sum();
        if sum == 2020 {
            return combination.iter().product();
        }
    }
    panic!("Part 2 didn't complete; bad inputs or bug?");
}


fn main() {
    println!("Part 1 soln: {}", part1(input::INPUT));
    println!("Part 2 soln: {}", part2(input::INPUT));
}


#[cfg(test)]
mod tests {
    use super::*;

    use std::collections::HashSet;

    /// The example inputs and outputs given by AOC
    #[test]
    fn test_part1_example() {
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

    /// Compare against the example from Wikipedia
    #[test]
    fn test_choose() {
        let output = choose(52, 5);
        assert_eq!(output, 2598960);
    }

    /// Confirm the subset() generator isn't producing duplicates
    #[test]
    fn test_subset() {
        let combinations = combinations(6, 3);
        let combinations_hashed = hash_subsets(&combinations);
        let combinations_hashed_len = combinations_hashed.len();
        let mut combinations_hashed_dedup = combinations_hashed.clone();
        let combinations_hashed_set: HashSet<String> = combinations_hashed.into_iter().collect();
        combinations_hashed_dedup.sort();
        combinations_hashed_dedup.dedup();
        let combinations_hashed_dedup_len = combinations_hashed_dedup.len();
        let combinations_hashed_dedup_set: HashSet<String> = combinations_hashed_dedup.into_iter().collect();
        let difference = combinations_hashed_dedup_set.difference(&combinations_hashed_set);
        println!("DIFFERENCE: {:?}", difference);
        assert_eq!(combinations_hashed_len, combinations_hashed_set.len());
        assert_eq!(combinations.len(), combinations_hashed_dedup_len);

        // Rerun with the deck-of-cards input when modifying.
        // Note it will take some time to complete.
        // let combinations = subset(52, 5);
    }

    /// Map each vector of i32 to a string such that repeated combinations map
    /// to the same "hash" (e.g. hash([1, 2]) == hash([2, 1])).
    fn hash_subsets(input: &Vec<Vec<i32>>) -> Vec<String> {
        let mut hashes: Vec<String> = Vec::with_capacity(input.len());
        for idx1 in 0..input.len() {
            let inner = &input[idx1 as usize];
            let mut sorted_inner: Vec<i32> = inner.clone();
            sorted_inner.sort();
            let stringified_sorted_inner: Vec<String> = sorted_inner.into_iter().map(
                |num| {
                    num.to_string()
                }
            ).collect::<Vec<String>>();
            hashes.push(stringified_sorted_inner.join("."));
        }
        return hashes;
    }

    /// Test against the answer confirmed for the inputs
    #[test]
    fn test_real_answers() {
        assert_eq!(712075, part1(input::INPUT));
        assert_eq!(145245270, part2(input::INPUT));
    }

}
