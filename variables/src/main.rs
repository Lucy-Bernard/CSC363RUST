fn main() {

    let mut counter = 1;
    let mut total = 0;
    let _sum = loop {
        total += counter;
        println!("{total}");
        counter +=1;

        if counter > 10 {
            break total;
        }

    };

    
}