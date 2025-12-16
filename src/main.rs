use std::io;
use crate::Data::data::MyKeys;

mod logik;
mod Data;

fn main() {

    let keys = MyKeys { p: 331, q: 149, k: 17 };

    // let mut word = String::new();

    // println!("Skriv noget der skal krypteres: ");

    // io::stdin().read_line(&mut word).expect("Input forkert");

    // let word = word.trim();
    // // let word = "hemmelighed".to_lowercase();

    // let list = logik::func::encrypt(word.to_string(), &keys);

    // println!("encrypted: {:?}",list);

    // let list2 = logik::func::decrypt(list, &keys);

    // println!("decrypted: {:?}",list2);

    // let decrypt = logik::func::decryptword(list2);

    // println!("Dekrypteret ord: {}", decrypt);

    logik::image_func::imageencrypt("Wicket.png", &keys);

}



