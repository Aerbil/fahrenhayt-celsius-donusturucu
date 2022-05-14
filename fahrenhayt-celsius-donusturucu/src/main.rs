use std::io;
fn main() {
    println!(
        "\n\n\nFahrenhayt-Celsius Dönüştürücü programına hoş geldiniz!
Çıkmak için \"çık\" yazabilirsiniz."
    );
    loop {
        let sonuc: f64;
        println!("\nDönüştürmek istediğiniz dereceyi girin:");
        let mut giriş = String::new();
        io::stdin().read_line(&mut giriş).expect("Giriş okunamadı.");
        if giriş.trim() == "çık" {
            break;
        }
        let derece: f64 = giriş.trim().parse().expect("Giriş sayı veya \"çık\" değil.");
        println!("Bu derecenin birimini Fahrenhayt için f, Celsius için c olacak şekilde girin:");
        giriş = String::new();
        io::stdin().read_line(&mut giriş).expect("Giriş okunamadı.");
        loop {
            if giriş.trim() == "f" {
                sonuc = (derece - 32.0) * 5.0 / 9.0;
                println!("{}°F = {:.1}°C", derece, sonuc);
                break;
            } else if giriş.trim() == "c" {
                sonuc = 9.0 / 5.0 * derece + 32.0;
                println!("{}°C = {:.1}°F", derece, sonuc);
                break;
            } else {
                println!("Sadece \"f\" veya \"c\" harfi girin.");
            }
        }
    }
}
