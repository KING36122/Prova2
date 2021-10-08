fn main() {
    let mut A:u8;
    let mut B:u8;
    let mut X:u8;

    //UTILIZEI DE DOIS EXEMPLOS COM AS PORTAS LÓGICAS NOT E NAND
    
    //OPERAÇÕES NOT
    A = 0;
    X = !(!A);
    println!("NOT(NOT A): {}",X);
    
    A = 1;
    X = !(!A);
    println!("NOT(NOT A): {}",X);
    
    //OPERAÇÕES NAND
    A = 0;
    B = 0;
    X = !(!(A&B));
    println!("\nNOT(NOT(A AND B)): {}",X);
    
    A = 0;
    B = 1;
    X = !(!(A&B));
    println!("NOT(NOT(A AND B)): {}",X);
    
    A = 1;
    B = 0;
    X = !(!(A&B));
    println!("NOT(NOT(A AND B)): {}",X);
    
    A = 1;
    B = 1;
    X = !(!(A&B));
    println!("NOT(NOT(A AND B)): {}",X);
}
