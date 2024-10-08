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

fn decode(x:char) -> char {
    if x.is_lowercase() {
        let first = 'a' as u8;
        let offset = (x as u8 - first + 13) % 26;
        return (first + offset) as char;
    } else {
        let first = 'A' as u8;
        let offset = (x as u8 - first + 13) % 26;
        return (first + offset) as char;
    }
}
