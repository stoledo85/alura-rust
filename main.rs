static variavel_global: u8 = 1;

fn main() {
    const PI: f32 = 3.14;
    println!("PI = {}", PI);

    println!("variavel_global = {}", variavel_global);

    let variavel: i32 = 300;
    println!(
        "variavel = {}, tamanho = {} bytes",
        variavel,
        std::mem::size_of_val(&variavel)
    );

    let decimal: f32 = 2.5;
    println!("decimal = {}", decimal);

    let mut booleana: bool = true;
    println!(
        "Booleana = {}, tamanho booleana = {}",
        booleana,
        std::mem::size_of_val(&booleana)
    );

    let letra: char = 'C';
    println!("Tamanho do Char  = {}", std::mem::size_of_val(&letra));
}
