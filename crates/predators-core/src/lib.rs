mod rdd;
mod datatypes;
mod split;

use rdd::dataset::Dataset;
use datatypes::*;

#[cfg(test)]
mod tests {
    use std::vec;

    use crate::rdd::dataset::{self, Dataset};

    #[test]
    fn test_rdd1() {
        let data = vec![vec![1, 2, 3], vec![4, 5, 6]];
        let dataset = Dataset::new(data);

        let mapped = dataset.map(|x| x * 2);
        // assert_eq!(mapped, Dataset::<i32>{partitions: vec![vec![2, 4, 6], vec![8, 10, 12]]});
        println!("{:?}", mapped);

        // let sum = dataset.reduce(|a, b| a + b);
        // println!("{}", sum);
        // assert_eq!(32, sum);
    }
}
