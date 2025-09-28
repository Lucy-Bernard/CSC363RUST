fn chop_1(s: String) -> Vec<char> {
    // create a vector of type char
    let mut v1 = Vec::new();
    // use char() method on s
    let s_chopped = s.chars();

    // for every letter in s, store it into the vector ?
    for letter in s_chopped {
        v1.push(letter); // instead of this could also use collect()
    }

    // println!("{:?}", v1);
    return v1;
}

fn chop_3(s: String) -> Vec<[char; 3]> {
    let mut result: Vec<[char; 3]> = Vec::new();
    let chars: Vec<char> = s.chars().collect(); // converted string letters into array of letters

    let mut i = 0;

    while i + 2 < chars.len() {
        let group = [chars[i], chars[i + 1], chars[i + 2]];
        result.push(group);
        i += 3;
    }

    return result;
}

fn reverse_string(s: String) -> String {
    // collect letters into vector
    let chars: Vec<char> = s.chars().into_iter().rev().collect();
    // reverse the vector

    //construct new string f rom vector
    let mut my_string: String = String::new();
    for letter in chars {
        my_string.push(letter);
    }
    return my_string;
}

fn main() {
    let s: String = String::from("Hello, world!");
    println!("{:?}", chop_1(s.clone()));

    println!("{:?}", chop_3(s.clone()));

    println!("{}", reverse_string(s));
}
