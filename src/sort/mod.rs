mod bubble_sort;
mod selection_sort;
mod merge_sort;
mod insertion_sort;
mod quick_sort;
mod heap_sort;
mod tim_sort;

#[cfg(test)]
mod test;

pub use self::bubble_sort::bubble_sort;
pub use self::selection_sort::selection_sort;
pub use self::merge_sort::merge_sort;
pub use self::insertion_sort::insertion_sort;
pub use self::quick_sort::quick_sort;
pub use self::heap_sort::heap_sort;
pub use self::tim_sort::tim_sort;
