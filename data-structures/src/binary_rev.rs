fn binary_reverse(list: [i32; 5]) -> Vec<i32> {
    let mut sorted: Vec<i32> = vec![list[0]];

    for num in list.iter().skip(1) {
        let mut l = 0;
        let mut h = sorted.len() - 1;

        while l <= h {
            let m = (l + h) / 2;
            let el = sorted[m];

            if *num > el {
                l = m + 1;
            } else {
                h = m - 1;
            }
        }

        sorted.insert(h + 1, *num);
    }

    println!("{:?}", sorted);
    sorted
}
