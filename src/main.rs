// use std::string;

mod logik;

fn main() {

    // let word = "hemmelighed".to_lowercase();
    let word = "hej".to_lowercase();
    let alphabet: Vec<char> = "abcdefghijklmnopqrstuvwxyzæøå ".chars().collect();

    let mut wordsplit: Vec<String> = Vec::new();
    let mut wordsplit2: Vec<String> = Vec::new();
    let mut finallist: Vec<i128> = Vec::new();

    let mut oneword: String = "".to_string();

    println!("The word: {}", word);

    for w in word.chars() {
        for (index, c) in alphabet.iter().enumerate(){
            if *c == w {
                let s = index.to_string();
                oneword += &s;
            wordsplit.push(s);
            }
        } 
    }

    let mut onewordnumber: i128 = oneword.clone().parse().unwrap();


    println!("One word: {} \nLenght of the number: {}", oneword, oneword.len());

    let mut num2: String = " ".to_string();

    for i in (0..wordsplit.len()-1).step_by(2) {
        let nextnum: i32 = wordsplit[i+1].clone().parse().unwrap();
        if nextnum < 10{
            num2 = "0".to_owned() + &wordsplit[i + 1];
        }
        else { 
            num2 = wordsplit[i+1].clone();
        }

        let testthis = wordsplit[i].clone() + &num2;
        wordsplit2.push(testthis);

    }

    if wordsplit.len() % 2 != 0{
    let testthis = wordsplit[wordsplit.len()-1].clone();
    wordsplit2.push(testthis);
    }

    println!("{:?}",wordsplit2);

    for i in 0..wordsplit2.len(){
        let number: i128 = wordsplit2[i].clone().parse().unwrap();
        finallist.push(number);

    }

    println!("{:?}",finallist);

    logik::func::mm(finallist);
    // let p = 331;
    // let q = 149;
    // let m: i128 = p * q;
    // let øm: i128 = (p -1) * (q - 1);
    // let k: i128 = 17;
    // let ss = modularinverse(k, øm);

    // onewordnumber = power(onewordnumber, k, m);
    // println!("crypted: {} ", onewordnumber);

    // onewordnumber = power(onewordnumber, ss, m);
    // println!("decrypted: {} ", onewordnumber);


}



