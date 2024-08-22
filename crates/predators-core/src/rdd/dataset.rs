use std::fmt::Debug;

pub struct Dataset<T> {
    partitions: Vec<Vec<T>>,
}

impl<T: std::fmt::Debug> Debug for Dataset<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Dataset")
            .field("partitions", &self.partitions)
            .finish()
    }
}

impl<T> Dataset<T> {
    pub fn new(data: Vec<Vec<T>>) -> Self {
        Dataset { partitions: data }
    }

    pub fn map<U, F>(&self, func: F) -> Dataset<U>
    where
        F: Fn(&T) -> U + Send + Sync + 'static,
        T: Send + Sync + 'static,
        U: Send + Sync + 'static,
    {
        let new_partitions: Vec<Vec<U>> = self
            .partitions
            .iter()
            .map(|partitoin| partitoin.iter().map(&func).collect())
            .collect();
        Dataset::new(new_partitions)
    }

    pub fn reduce<F>(&self, func: F) -> T
    where
        F: Fn(T, T) -> T + Copy + Send + Sync + 'static,
        T: Clone + Send + Sync + 'static + std::fmt::Debug,
    {
        self.partitions
            .iter()
            .map(|partition| partition.iter().cloned().reduce(func).unwrap())
            .reduce(func)
            .unwrap()
    }
}
