use image::ImageReader;

fn main() {
    let word = "Kryptologi".to_lowercase();
    let alphabet: Vec<char> = "abcdefghijklmnopqrstuvwxyzæøå".chars().collect();
    let _alphabet_eng: Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();

    let word2 = encrypt(&word, 15, &alphabet);
    println!("{}",word2);
    let word3 = decrypt(&word2, 15, &alphabet);
    println!("{}",word3);
    println!();

    // let wordbrute = word2;
    // for i in 0..26 {
    //     let w = decrypt(&wordbrute, i, &alphabet);
    //     println!("{}", w);
    // }

    let mut img = ImageReader::open("Wicket.png").expect("Failed to open image").decode().expect("Failed to decode image").to_rgba8();
    // println!("Dimensions: {:?}", img.dimensions());
    // println!("Color type: {:?}", img.color());
    let x = 100;
    for pixel in img.pixels_mut(){
        pixel.0 = [
                    // pixel[0],
                    // pixel[1],
                    // pixel[2],
                    encrypt_num(pixel[0], x),
                    encrypt_num(pixel[1], x),
                    encrypt_num(pixel[2], x),
                    pixel[3]];
    }

    img.save("Ceasarwicket.png").expect("Failed to save image");

    // let mut img = ImageReader::open("testerValueChange.png").expect("Failed to open image").decode().expect("Failed to decode image").to_rgba8();
    // for pixel in img.pixels_mut(){
    //     pixel.0 = [decrypt_num(pixel[0], x),
    //                 decrypt_num(pixel[1], x),
    //                 decrypt_num(pixel[2], x),
    //                 pixel[3]];
    // }
    // img.save("testerValueChangeDECRYPTED.png").expect("Failed to save image");

}

pub fn encrypt_num(pixel: u8, num: u8) -> u8{
    let new = ((num as u16 + pixel as u16)  % 256).min(256) as u8;
    new 
}

pub fn decrypt_num(pixel: u8, num: u8) -> u8{
    let new = ((pixel as u16 + (256 - num as u16 )) % 256).min(256) as u8;
    new 
}

pub fn encrypt(word: &String, num: usize, alphabet: &Vec<char>) -> String{
    let mut word2 = String::new();

    for w in word.chars() {
        for (index, c) in alphabet.iter().enumerate(){
            if *c == w {
            let testnum = (index + num) % alphabet.len();
            word2.push(alphabet[testnum]);
        }
    } 
}
    word2 
}

pub fn decrypt(word: &String, num: usize, alphabet: &Vec<char>) -> String{
    let mut word2 = String::new();
    for w in word.chars(){
        for (index, c) in alphabet.iter().enumerate(){
            if *c == w {
                let testnum = (index + alphabet.len() - (num % alphabet.len())) % alphabet.len();
                word2.push(alphabet[testnum]);
            }  
        }
    }
    word2
} 