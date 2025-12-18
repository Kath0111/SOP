use std::io;
use crate::Data::data::{PrivateKeys,PublicKeys};

mod logik;
mod Data;

fn main() {

    let prikeys = PrivateKeys::new(331, 149);

    let pukeys = PublicKeys {k: 17, m: prikeys.m()};

    // kryptering af ord / sætninger / bare tegn der er unicode format

    let mut word = String::new();

    println!("Skriv noget der skal krypteres: ");

    io::stdin().read_line(&mut word).expect("Input forkert");

    let word = word.trim();
    // let word = "hemmelighed".to_lowercase();

    let list = logik::func::encrypt(word.to_string(), &pukeys);

    println!("encrypted: {:?}",list);

    let list2 = logik::func::decrypt(list, &prikeys, &pukeys);

    println!("decrypted: {:?}",list2);

    let decrypt = logik::func::decryptword(list2);

    println!("Dekrypteret ord: {}", decrypt);

    // kryptering over billede pixels 

    logik::image_func::imageencrypt("wicket.png", &pukeys);

    logik::image_func::imagedecrypt("Test.png", &prikeys, &pukeys);

}



