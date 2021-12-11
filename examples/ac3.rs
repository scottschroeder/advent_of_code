use advent_of_code::ac3::{PairwiseConstraint, AC3};

fn main() {
    pretty_env_logger::init();

    let mut ac3 = AC3::default();
    ac3.add_domain("A", vec![1, 2, 3]);
    ac3.add_domain("B", vec![1, 2, 3]);
    ac3.add_domain("C", vec![1, 2, 3]);
    ac3.add_constraint("A", "B", PairwiseConstraint::rule(|a, b| a > b));
    ac3.add_constraint("B", "A", PairwiseConstraint::rule(|b, a| b < a));
    ac3.add_constraint("B", "C", PairwiseConstraint::rule(|b, c| b == c));
    ac3.add_constraint("C", "B", PairwiseConstraint::rule(|c, b| c == b));
    let data = ac3.solve().unwrap();
    println!("{:?}", data);
}
