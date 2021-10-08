fn main() {
    let  mut A:bool;
    let mut B:bool;
    let mut result:bool;
    
    //true = 1 e false = 0
    
    //OPERAÇÕES NOT
    A = true;
    result = !(!A);
    println!("NOT(NOT A) (quando A = 1): {}\n",result);
    
    A = false;
    println!("NOT(NOT A) (quando A = 0): {}\n",result);
    
    
    //OPERAÇÕES NAND
    A = false;
    B = false;
    result = !(!(A&B));
    println!("NOT(NOT(A AND B)) (quando A = 0 e B = 0): {}\n",result);
    
    A = false;
    B = true;
    result = !(!(A&B));
    println!("NOT(NOT(A AND B)) (quando A = 0 e B = 1): {}\n",result);
    
    A = true;
    B = false;
    result = !(!(A&B));
    println!("NOT(NOT(A AND B)) (quando A = 1 e B = 0): {}\n",result);
    
    A = true;
    B = true;
    result = !(!(A&B));
    println!("NOT(NOT(A AND B)) (quando A = 1 e B = 1): {}\n",result);
    
    A = false;
    B = false;
    result = !(!(A|B));
    println!("NOT(NOT(A OR B)) (quando A = 0 e B = 0): {}\n",result);
    
    A = false;
    B = true;
    result = !(!(A|B));
    println!("NOT(NOT(A OR B)) (quando A = 0 e B = 1): {}\n",result);
    
    A = true;
    B = false;
    result = !(!(A|B));
    println!("NOT(NOT(A OR B)) (quando A = 1 e B = 0): {}\n",result);
    
    A = true;
    B = true;
    result = !(!(A|B));
    println!("NOT(NOT(A OR B)) (quando A = 1 e B = 1): {}\n",result);
    
    A = false;
    B = false;
    result = !(A&B);
    println!("NOT(A AND B) (quando A = 0 e B = 0): {}\n",result);
    
    A = false;
    B = true;
    result = !(A&B);
    println!("NOT(A AND B) (quando A = 0 e B = 1): {}\n",result);
    
    A = true;
    B = false;
    result = !(A&B);
    println!("NOT(A AND B) (quando A = 1 e B = 0): {}\n",result);
    
    A = true;
    B = true;
    result = !(A&B);
    println!("NOT(A AND B) (quando A = 1 e B = 1): {}\n",result);
    
    A = false;
    B = false;
    result = !(A|B);
    println!("NOT(A OR B) (quando A = 0 e B = 0): {}\n",result);
    
    A = false;
    B = true;
    result = !(A|B);
    println!("NOT(A OR B) (quando A = 0 e B = 1): {}\n",result);
    
    A = true;
    B = false;
    result = !(A|B);
    println!("NOT(A OR B) (quando A = 1 e B = 0): {}\n",result);
    
    A = true;
    B = true;
    result = !(A|B);
    println!("NOT(A OR B) (quando A = 1 e B = 1): {}\n",result);
}
