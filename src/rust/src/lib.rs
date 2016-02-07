//#[test]
fn sort_vec() {
    let mut sevens = vec![0; 100];
    sevens.sort();
    let mut counts = Vec::with_capacity(sevens.len());
    let mut seven_iter = sevens.iter_mut();
    let mut prev = 0u32;
    for &mut seven in seven_iter.by_ref() {
        if prev != seven {
            prev = seven;
            if let Some(snxt) = seven_iter.next() {
                *snxt = seven;
            }
            counts.push(3);
        } else {
            if let Some(last) = counts.last_mut() {
                *last += 3;
            }
        }
    }
}
