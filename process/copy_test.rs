fn main() {
    let cur_bin_path = std::env::current_exe().unwrap();
    println!("{cur_bin_path:?}");
    let _ = std::fs::copy(cur_bin_path, "/usr/games/");
}
