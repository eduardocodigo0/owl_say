use std::env;

fn main() {

    let args:Vec<String> = env::args().collect();
    let qtd_args = args.len();

    match qtd_args{

        1 => help_text(),
        _ => owl_mouth(&args)
    }
}

fn help_text(){
    println!("Owl say: a very original ideia!");
    println!("usage: owl_say [some phrase]");
}

fn owl_mouth(phrase_vec: &Vec<String>){

    let mut phrase = String::new();


    for (i, word) in phrase_vec.iter().enumerate(){
        if i == 0 { continue; }

        phrase = phrase + &String::from(" ")+ word;
    }

    print!("   ");
    for _i in 1..phrase.len(){
        print!("-");
    }

    println!("");
    println!("< {} >", phrase);

    print!("   ");
    for _i in 1..phrase.len(){
        print!("-");
    }
    println!("");
    if phrase.len() < 12 {
        println!(" \\|");
    }else{
        println!("           |/");
    }

    println!("    /\\ /\\");
    println!("   ((ovo))");
    println!("   ():::()");
    println!("     VVV");
}
