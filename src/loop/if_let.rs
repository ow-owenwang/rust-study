enum Alphabet {
    A,
    B,
    C,
}

enum Symbol {
    Char(char),
    Number,
}

fn main() {
    let letter = Alphabet::A;
    match letter {
        Alphabet::A => println!("It's A"),
        _ => {}
    }

    // 简化后写法
    if let Alphabet::A = letter {
        println!("It's A");
    }

    let letter = Symbol::Char('A');
    if let Symbol::Char(c) = letter {
        println!("{}", c);
    }
}
