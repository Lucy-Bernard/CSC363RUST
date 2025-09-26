fn chop_1 (s : String) -> Vec<char> {

    // create a vector of type char
    let mut v1 = Vec::new ();
    // use char() method on s 
    let s_chopped = s.chars();

    // for every letter in s, store it into the vector ?
    for letter in s_chopped {
        v1.push(letter);
    }

    // println!("{:?}", v1);
    return v1;
    
  }
  
  fn main () {
    let s : String = String::from ("Hello, world!");
    println! ("{:?}", chop_1 (s.clone ()));
    
  }