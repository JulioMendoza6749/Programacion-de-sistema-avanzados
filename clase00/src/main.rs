fn main() {
    //let x=5;
    //x=10; esto no es valido

    //let mut y=10;
    //y=20

    let x=5;
    {//ambiente la varibable x se maneja de manera local
        let x=x+1;//nueva variable (shadowing)
        println!("el valor de x es:{}", x);
    }
    println!("el valor de x es:{}", x);

    //variables /// datos escalares
    let entero: i32 = 42;
    let flotante: f64 = 3.1416;
    let booleano: bool=true;
    let caracter: char = 'a';

    //tupla -> structs ///datos compuestos
    let firulais: (i32,f64,char)=(10, 22.22, 'b');//tipo de dato tupla
    let arreglo: [i32; 3] = [1,2,3];// tipo de dato arreglo

    println!("Tupla(firulais) forma1: {:?}", firulais);//imprimir toda la tupla

    println!("Tupla(firulais) forma2: ({},{},{})", firulais.0,firulais.1,firulais.2);//imprimir ciertos valores de la tupla
}
