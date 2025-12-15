use std::io;


mod logik;

fn main() {

    let mut word = String::new();

    println!("Skriv noget der skal krypteres: ");

    io::stdin().read_line(&mut word).expect("Input forkert");

    let word = word.trim();
    // let word = "hemmelighed".to_lowercase();
    // let word = "hej".to_lowercase();

    let mut finallist: Vec<i128> = Vec::new();

    println!("The word: {}", word);

    for w in word.chars() {
        let s = w as u32 as i128;
        finallist.push(s);
    }

    println!("{:?}",finallist);

    logik::func::mm(finallist);

}



