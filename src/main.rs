fn main() {
    let mut numero = 1;

    loop {
      println!("El número actual es: {}", numero);
  
      if numero == 3 {
        break;
      }
  
      numero += 1;
    }
  
    println!("¡Salimos del ciclo!");
}
