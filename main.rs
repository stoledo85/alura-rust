fn main(){
    let variavel:i32 = 300;
    println!("variavel = {}, tamanho = {} bytes", variavel, std::mem::size_of_val(&variavel));
    
    let decimal:f32=2.5;
    println!("decimal = {}",decimal);
    
    let mut booleana:bool=true;
    println!("Booleana = {}, tamanho booleana = {}", booleana, std::mem::size_of_val(&booleana));

    let letra:char = 'C';
    println!("Tamanho do Char  = {}", std::mem::size_of_val(&letra));
}