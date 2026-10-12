mod prime_library;
mod fichiers;

fn main() {
    println!("Recherche des nombres premiers en RUST");
    let mut primes: Vec<u128> = Vec::new();
    for nombre in 1..=10 {
        if prime_library::is_prime_i32(nombre) {
            println!("{} est un nombre premier.", nombre);
        }
    }

    let end_number = 1_000_000_000;

    for nombre in 1..=end_number {
        if prime_library::is_prime_u128(nombre) {
            println!("{} est un nombre premier.", nombre);
            primes.push(nombre as u128);
        }
    }

    // Write the primes to a file
        if let Err(e) = fichiers::write_primes_to_file(&primes, &format!("primes_{}.txt", end_number)) {
            eprintln!("Erreur lors de l'écriture des nombres premiers dans le fichier: {}", e);
        } else {
            println!("Les nombres premiers ont été écrits dans le fichier primes_{}.txt", end_number);
        }
}
