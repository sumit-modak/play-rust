fn my_chunks() {
    let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0];
    for i in v.chunks(3) {
        println!("{i:?}");
    }
}

fn my_chain() {
    let a = [1, 2, 3];
    let b = [4, 5, 6];
    let c = [7, 8, 9];
    let v: Vec<_> = a.iter().chain(b.iter().chain(c.iter())).collect();
    println!("{v:?}");
}

fn my_all() {
    let v = [2, 4, 6, 8, 10];
    if v.iter().all(|k| *k%2==0) {
        println!("all even :)");
    } else {
        println!("not all even");
    }
}

fn my_any() {
    let v = [12, 53, 327, 23, 298, 28];
    if v.iter().any(|k| *k>100) {
        println!("Some are above 100");
    } else {
        println!("All are below 100");
    }
}

fn my_windows() {
    let v = [1, 3, 5, 7, 17, 32];
    if v.windows(2).all(|k| k[0] < k[1]) {
        println!("Numbers are ascending");
    } else {
        println!("Numbers are not ascending");
    }
}

fn my_cycle() {
    let v = [5, 10, 15];
    let new_v = v.iter().cycle().take(100).collect::<Vec<_>>();
    println!("{new_v:?}");
}

fn main() {
    my_chunks();
    my_chain();
    my_all();
    my_any();
    my_windows();
    my_cycle();
}
