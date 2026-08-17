const ESTOESUNACONSTANTE :i8 = 1;

fn main() {
    println!("Hello, world!");
    println!("-------------------");
    prueba_inmutabilidad();
    println!("-------------------");
    scalars();
}

fn prueba_inmutabilidad(){
    // let x: i32 = 1; Esta variable es inmutable ya que carece 'mut'.
    let mut x = 1; // Aqui nos tira un warning, pero no es de importancia.
    x = 2;
    let z = 3;
    println!("El valor de x es: {}, y el valor de z es: {}", x, z);
}

fn shadowing(){
    let x = 1;
    let x = x + 23;
    let x = x - 23 / 23;
    println!("El resultado es x = {}", x);
}

fn scalars(){
    // Enteros
    let ocho_bits: i8 = 1;
    let diezseis_bits: i16 = 1;
    let tresdos_bits: u32 = 1; // Esto solamente admite enteros positivos
    let seiscuatro_bits: i64 = 1;
    let unodosocho_bits: i128 = 1;

    // Igual con los floats y tenemos operaciones matematicas
    let float_tresdos: f32 = 2.65;
    let float_dato = float_tresdos + 23.54;

    // Bool
    let variable: bool = true;

    // Character
    let my_char: char = 'a';

    println!("El valor de una de las variables int es: {} \nEl valor de un float es: {} \nEl valor de un booleano es: {} \nEl valor de un char es: {}", ocho_bits, float_tresdos, variable, my_char);
}

fn compound(){

}