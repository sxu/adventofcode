pub fn for_each_permutation<T, F>(xs: &mut [T], cb: &mut F)
where
    F: FnMut(&[T]),
{
    permute(xs.len(), xs, cb);
}

fn permute<T, F>(k: usize, xs: &mut [T], cb: &mut F)
where
    F: FnMut(&[T]),
{
    if k == 1 {
        cb(xs);
    } else {
        permute(k - 1, xs, cb);
        for i in 0..k - 1 {
            if k % 2 == 0 {
                xs.swap(i, k - 1);
            } else {
                xs.swap(0, k - 1);
            }
            permute(k - 1, xs, cb);
        }
    }
}
