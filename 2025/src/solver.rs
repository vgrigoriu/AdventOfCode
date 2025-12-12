use good_lp::{Expression, ProblemVariables, Solution, SolverModel, constraint, solvers::highs::highs, variable};

pub fn solve_system_minimizing_sum(equations: &[Vec<u32>], rhs: &[u32]) -> Option<Vec<u32>> {
    let num_vars = equations[0].len();

    let mut vars = ProblemVariables::new();
    // Create all variables: ingegers >= 0.
    let xs: Vec<_> = (0..num_vars)
        .map(|i| vars.add(variable().integer().min(0).name(format!("b_{i}"))))
        .collect();

    // Minimize the sum of all variables.
    let objective: Expression = xs.iter().copied().sum();
    println!("{objective:?}");
    let mut problem = vars.minimise(objective).using(highs);

    for (coeffs, &rhs_val) in equations.iter().zip(rhs.iter()) {
        let lhs: Expression = coeffs.iter().zip(xs.iter()).map(|(&coef, &x)| x * coef).sum();
        problem = problem.with(constraint!(lhs == rhs_val));
    }

    println!("{problem:?}");

    let solution = problem.solve().ok()?;
    Some(xs.iter().map(|&x|solution.value(x) as u32).collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_system() {
        let solution = solve_system_minimizing_sum(&vec![vec![1, 2, 3]], &vec![10]);
        assert_eq!(solution, Some(vec![1, 0, 3]));
    }

    #[test]
    fn test_solve_first_example_in_2025_day_10() {
        let solution = solve_system_minimizing_sum(
            &vec![
                vec![0, 0, 0, 0, 1, 1],
                vec![0, 1, 0, 0, 0, 1],
                vec![0, 0, 1, 1, 1, 0],
                vec![1, 1, 0, 1, 0, 0],
            ],
            &vec![3, 5, 4, 7]);
        assert_eq!(solution.unwrap().iter().sum::<u32>(), 10);
    }
}
