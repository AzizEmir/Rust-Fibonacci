use num_bigint::BigUint;
use num_traits::One;

fn main() {
    let mut sayi_n_eksi_bir: BigUint = One::one();
    let mut sayi_n: BigUint = One::one();
    let mut sonuc: BigUint;

    for _ in 1..=5000 {
        sonuc = &sayi_n_eksi_bir + &sayi_n;

        sayi_n_eksi_bir = sayi_n;

        sayi_n = sonuc.clone();

        println!("{sayi_n}");
    }
}

/*
Start
    sayi_n_eksi_1 = 1
    sayi_n = 1
    sonuc ;

    LOOP 1
    sonuc = 2;
    sayi_n_eksi_bir = 1
    sayi_n = 2;

UPDATE
    sayi_n_eksi_1 = 1
    sayi_n = 2
    sonuc 2;

    LOOP 2
    sonuc = 3;
    sayi_n_eksi_bir = 2;
    sayi_n = 3;

    ...
*/
