use image::ImageReader;
use crate::Data::data::MyKeys;

use crate::logik::func::modularinverse;

pub fn imageencrypt(image: &str, keys: &MyKeys) {

    let mut img = ImageReader::open(image).expect("Failed to open image").decode().expect("Failed to decode image").to_rgba16();

    for pixel in img.pixels_mut(){
        pixel.0 = [
                    encrypt_pix(pixel[0] as u8, keys.m(), keys.k),
                    encrypt_pix(pixel[1] as u8, keys.m(), keys.k),
                    encrypt_pix(pixel[2] as u8, keys.m(), keys.k),
                    pixel[3]
                    ];
    }

    img.save("RSAwicket2.png").expect("Failed to save image");

    let ss = modularinverse(keys.k, keys.øm()) as u128;

    let encrypted_img = ImageReader::open("RSAwicket2.png").expect("Failed to open image").decode().expect("Failed to decode image").to_rgba16();
    
    let (width, height) = encrypted_img.dimensions();
    let mut decrypted_img = image::RgbaImage::new(width, height);

    for (x, y, pixel) in encrypted_img.enumerate_pixels() {
        let r = decrypt_pix(pixel[0], keys.m() as u128, ss);
        let g = decrypt_pix(pixel[1], keys.m() as u128, ss);
        let b = decrypt_pix(pixel[2], keys.m() as u128, ss);
        let a = (pixel[3] % 256) as u8;
        decrypted_img.put_pixel(x, y, image::Rgba([r, g, b, a]));
    }
    decrypted_img.save("DECRYPTEDwicket.png").expect("Failed to save image");

}

fn decrypt_pix(pixel: u16, m: u128, ss: u128) -> u8 {
    let c = mod_pow(pixel as u128, ss, m);
    // let c = encrypt(pixel as u128, m , ss);
    (c % 256) as u8
    // let new = (c % 256).min(256)as u8;
    // new 
}

fn encrypt(pixel: u128, m: u128, k: u128) -> u128{
    let mut c = pixel;
    for _i in 1..k {
        c = (pixel * c) % m;
    }
    c 
}

fn encrypt_pix(pixel: u8, m: i128, k: i128) -> u16{
    let c = mod_pow(pixel as u128, k as u128, m as u128);
    c as u16
    // let c = encrypt(pixel as u128, m as u128, k as u128);
    // let new =(c % 256).min(256)as u8;
    // new 
}

fn mod_pow(pixel: u128, exp: u128, mod_: u128) -> u128 {
    let mut res = 1;
    let mut b = pixel % mod_;
    let mut e = exp;
    while e > 0 {
        if e % 2 == 1 {
            res = (res * b) % mod_;
        }
        b = (b * b) % mod_;
        e /= 2;
    }
    res
}