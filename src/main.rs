use std::io;
fn main() {
    println!("truc pour faire le rot13");

    loop {
        println!{"écris ce que tu veux que ça devienne une grosse dingz en rot13"};

    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("met un enculé enculé");

    let mut result: String = String::new();
    let changed: &str = guess.trim();
    let it = changed.chars();
    
    for slice in it{
        //if isNumber => do nothing
        // isAlphabet => decode(slice);
       if slice.is_alphabetic(){
            // If slice is an alphabet, decode it
            result.push(decode(slice));
        } else {
            // If slice is a number, do nothing
            result.push(slice);
            continue;
        }
        
    }
    println!("Le résultat de la transformation par rot13 est:");
    println!("{}", result);


    }
}

fn decode(x: char) -> char{
    //mon but ici est de trouver l'index de x dans alphabet ensuite d'ajouter a cet index 13 et faire modulo 25
    let alphabet = ['a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z'];
    let index = (position(&x) + 13) % 26;
    return alphabet[index];
}

fn position (x: &char) -> usize {
    //mon but avec cette 
    let alphabet = ['a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z'];
    let mut incr = 0;
    loop{
        if x == &alphabet[incr] {
            return incr;
        } else {
            incr += 1;
        }
    }
}
