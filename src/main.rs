fn main() {
    let t2 = [
        206847684, 1056521, 7, 17, 1901, 21104421, 7, 1, 35521, 1, 7781,
    ];
    let t1 = [2, 6, 8, -10, 3];
    println!("{:?}", find_outlier(&t2));
    println!("{:?}", find_outlier(&t1));
}
fn find_outlier(values: &[i32]) -> i32 {
    let mut counter = 0;
    let mut chosen = 0;

    for i in values[0..3].iter() {
        if i % 2 == 0 {
            counter += 0;
        }
        if i % 2 != 0 {
            counter += 1;
        }
    }
    println!("{:?}", &counter);
    if counter <= 1 {
        for i in values.iter() {
            if i % 2 != 0 {
                chosen = *i;
            }
        }
    } else {
        for i in values.iter() {
            if i % 2 == 0 {
                chosen = *i;
            }
        }
    }
    chosen
}
