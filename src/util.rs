// use std::cmp::Ordering;

// #[inline(always)]
// pub fn swap<T>(x: &mut [T], i: usize, j: usize) {
//     let (lo, hi) = match i.cmp(&j) {
//         // no swapping necessary
//         Ordering::Equal => return,

//         // get the smallest and largest of the two indices
//         Ordering::Less => (i, j),
//         Ordering::Greater => (j, i),
//     };
//     if x.len() <= hi {
//         return;
//     }
//     swap_unchecked(x, lo, hi)
// }

#[inline(always)]
pub fn swap_unchecked<T>(x: &mut [T], lo: usize, hi: usize) {
    let (init, tail) = x.split_at_mut(hi);
    std::mem::swap(&mut init[lo], &mut tail[0]);
}
