use crate::Data::data::{PrivateKeys,PublicKeys};

pub fn encrypt(word: String, keys: &PublicKeys) -> Vec<i128> {

    let mut wordspilt: Vec<i128> = Vec::new();

    for w in word.chars() {
        let s = w as u32 as i128;
        wordspilt.push(s);
    }

    let mut cryptword: Vec<i128> = Vec::new();

    for n in wordspilt {
        let c = power(n, keys.k, keys.m);
        cryptword.push(c);
    }

    cryptword
    
}

pub fn decrypt(cryptword: Vec<i128>, prikeys: &PrivateKeys, pubkeys: &PublicKeys) -> Vec<i128> {
   
    let ss = modularinverse(pubkeys.k, prikeys.øm());

    let mut decryptlist: Vec<i128> = Vec::new();

    for i in cryptword {
        let decrypt = power(i,ss,pubkeys.m);
        decryptlist.push(decrypt);
    }

    decryptlist

}

pub fn decryptword(decryptlist: Vec<i128>) -> String{
    let mut decrypted = String::new();

    for n in decryptlist {
        let c = n as u32;
        let s = std::char::from_u32(c).unwrap();
        decrypted += &s.to_string();
    }

    decrypted
}

fn power(n: i128, k: i128, m: i128) -> i128{ // funktion der udføre opløftning
    let mut c = n;
        for _i in 1..k {
            c = ((n % m) * (c % m)) % m;
        }
    c
}


fn sfd(k: i128, øm: i128) -> (i128,i128,i128){
    if k == 0{
        (øm,0,1)
    }
    else {
        let (d, s1, t1) = sfd(øm%k , k); // rekursivt kald -- udføre euklids algoritme
        (d, t1 - (øm/k) * s1, s1)
    }
}

pub fn modularinverse(k: i128, øm: i128) -> i128 {
    let (d, s, _) = sfd(k, øm);
    if d != 1 {
        panic!("sfd({},{}) er ikke lig 1", øm, k);
    }
    (s % øm + øm) % øm // hvis s er negativ ændres til positiv 
}
