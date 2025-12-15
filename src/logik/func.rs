
pub fn mm(words: Vec<i128>)  {

    let p = 331;
    let q = 149;
    let m: i128 = p * q;
    let øm: i128 = (p -1) * (q - 1);
    let k: i128 = 17;

    println!("sfd = {:?}",sfd(k, øm));
    println!("modinv = {:?}",modularinverse(k, øm));
    let ss = modularinverse(k, øm);

    let mut cryptword: Vec<i128> = Vec::new();

    for n in words {
        let c = power(n, k, m);
        cryptword.push(c);
    }

    let mut decryptlist: Vec<i128> = Vec::new();

    println!("cryptwords {:?}", cryptword);

    for i in cryptword {
        let decrypt = power(i,ss,m);
        decryptlist.push(decrypt);
    }

    println!("this is decrypted {:?}", decryptlist);

} 

fn power(n: i128, k: i128, m: i128) -> i128{
    
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
        let (d, s1, t1) = sfd(øm%k , k);
        (d, t1 - (øm/k) * s1, s1)
    }
}

fn modularinverse(k: i128, øm: i128) -> i128 {
    let (g, x, _) = sfd(k, øm);
    if g != 1 {
        panic!("Ingen modular inverse");
    }
    (x % øm + øm) % øm
}
