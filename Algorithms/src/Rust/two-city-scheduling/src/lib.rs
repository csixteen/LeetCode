// https://leetcode.com/problems/two-city-scheduling/

// The description of the problem is quite confusing, so here goes my take:
// you want to find the cheapest way to fly N people to two cities, such that
// half the people go to city A and half the people go to city B. You are given
// an array of costs, where costs[i] = [cost_to_cityA, cost_to_cityB].

struct Solution {}

impl Solution {
    pub fn two_city_sched_cost(mut costs: Vec<Vec<i32>>) -> i32 {
        // Since we want to fly half the people to city A and the other half
        // to city B (in the cheapest way possible), we can sort the costs
        // array by the gains of flying people to city A over flying them
        // to city B. This can be achieved with cost[i][0] - cost[i][1]. If the
        // value is negative, it means that it's cheaper to fly the person `i`
        // to city A. Since we just want HALF the people in city A and HALF
        // the people in city B, we only need to iterate over HALF the costs
        // array (we have the guarantee that this array will always have even
        // length) in order to get the total cost. Just to understand that little
        // iteration "trick", it could have been achieved this way:
        // 
        // let n = costs.len() / 2;
        // let mut total = 0;
        // for i in 0..n {
        //   total += costs[i][0];
        // }
        // for i in n..costs.len() {
        //   total += costs[i][1];
        // }
        //
        // Instead, we just iterate over half the array and use `n` as an offset
        // to get to the other half of the array.
        //
        let n = costs.len() / 2;

        &costs.sort_by_key(|k| k[0] - k[1]);

        (0..n).fold(0, |acc, i| {
            acc + costs[i][0] + costs[i+n][1]
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        assert_eq!(
            110,
            Solution::two_city_sched_cost(
                vec![vec![10, 20], vec![30, 200], vec![400, 50], vec![30, 20]]
            ),
        );
    }
}
