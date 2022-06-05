fn main() {
    let root: [&str; 5] = ["1", "null", "2", "3", "hdsj"];
    let mut vec = Vec::new();

    for i in 0..root.len() {
        for n in root[i].chars() {
            if n.is_numeric() == true {
                vec.push(1);
            } else if n.is_numeric() == false {
                vec.push(0);
                break;
            }
        }
    }
    for n in 0..vec.len() {
        if vec[root.len() - n - 1] == 1 {
            println!("{}", root[vec.len() - n - 1]);
        }
    }
}
