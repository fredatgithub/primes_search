mod prime_library;

fn main() {
    println!("Recherche des nombres premiers en RUST");
    for nombre in 1..=100 {
        if prime_library::is_prime_i32(nombre) {
            println!("{} est un nombre premier.", nombre);
        }
        // else {
        //     println!("{} n'est pas un nombre premier.", nombre);
        // }
        
    }
}
