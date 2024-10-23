

enum Renk {
    Kırmızı,
    Yeşil,
    Mavi,
}

fn main() {
    let benim_renk = Renk::Kırmızı;

    register_book("kitap".to_string());
}

struct Book{
    name:String
}
fn register_book(name:String) {

    let book1 = Book{
        name:name
    };

    println!("Kitabınız kaydedildia")
    
}


