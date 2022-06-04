fn main() {
    let root: [&str; 5] = ["1", "null", "2", "3", "hdsj"];
    let mut vec = Vec::new();
    //let len: usize = root.len();

    for i in 0..root.len() {
        for n in root[i].chars() {
            //let mut key: [i8; root.len()];

            if n.is_numeric() == true {
                vec.push(1);
                //println!("n is numeric");
            } else if n.is_numeric() == false {
                vec.push(0);
                break;
            }
        }
    }
    for n in 0..vec.len() {
        if vec[root.len() - n - 1] == 1 {
            println!("{}", root[vec.len() - n -1]);
        }

    }
}

/*
fn str_to_vec(s: &str) -> Vec<char> {
    let char_vec: Vec<char> = s.chars().collect();
    for c in char_vec {
        println!("{}", c);
    }
    return char_vec;
}
 */
