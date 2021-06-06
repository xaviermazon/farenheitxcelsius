use std::io;

fn main() {
    
    let mut faren = String::new();
    println!("Introduce una temperatura en Farenheit para convertirlo en Celcius");
    io::stdin()
	.read_line(&mut faren)
        .expect("Failed to read line");
    
    let farenheit: i32 = match faren.trim().parse() {
        Ok(num) => num,
        Err(_) => {
	    println!("Valor introducido es erroneo, el valor por defecto es 0");
	    0
	}
    };

    let celcius: i32 = (farenheit - 32) * 5 / 9;
    println!("Temp. Farenhait: {} | Temp. Celcius: {}",farenheit, celcius);
}
