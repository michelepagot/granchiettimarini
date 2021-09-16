pub fn run(){
    let ciao = String::from("Ciao");
    let mut nome = String::from("Mici");

    println!("{} mondo", ciao);
    println!("Length:{}", ciao.len());

    println!("{} {}", ciao, nome);

    nome.push('o');
    println!("{} {}", ciao, nome);

    nome.push_str("_matto");
    println!("{} {}", ciao, nome);

    println!("Capacity: {}bytes", ciao.capacity());

    println!("Is empty:{}", ciao.is_empty());

    println!("'ciao' contains iao:{}", ciao.contains("iao"));
    println!("'ciao' contains bau:{}", ciao.contains("bau"));

    println!("Replace:{}", ciao.replace("iao", "ane"));

    let proverbio = String::from("Tanto va la gatta al lardo che ci lascia lo zampino");
    for word in proverbio.split_whitespace(){
        print!("{} -- ", word);
    }
}