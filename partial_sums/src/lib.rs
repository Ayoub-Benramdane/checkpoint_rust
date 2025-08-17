pub fn parts_sums(arr: &[u64]) -> Vec<u64>{
    let mut res = vec![0];
    let mut nb: u64 = 0;
    for n in arr {
        nb += n;
        res.push(nb)
    }
    res.reverse();
    res
}