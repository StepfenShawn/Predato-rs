use downcast_rs::{impl_downcast, DowncastSync};

pub(crate) struct SplitStruct {
    index: usize,
}

pub trait Split: DowncastSync {
    fn get_index(&self) -> usize;
}

impl_downcast!(Split);
