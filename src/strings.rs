pub fn run(){
    let ciao = "Ciao";
    let mut nome = String::from("Mici");

    println!("{} mondo", ciao);
    println!("Length:{}", ciao.len());

    println!("{} {}", ciao, nome);

    nome.push('o');
    println!("{} {}", ciao, nome);

    nome.push_str("_matto");
    println!("{} {}", ciao, nome);

    println!("Capacity of nome: {}bytes", nome.capacity());
    nome.push_str("_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx");
    println!("Capacity of nome: {}bytes", nome.capacity());

    println!("Is empty of nome:{}", nome.is_empty());

    println!("'ciao' contains iao:{}", ciao.contains("iao"));
    println!("'ciao' contains bau:{}", ciao.contains("bau"));

    println!("Replace:{}", ciao.replace("iao", "ane"));

    let proverbio = String::from("Tanto va la gatta al lardo che ci lascia lo zampino");
    for word in proverbio.split_whitespace(){
        print!("{} -- ", word);
    }

    //creates string with capacity
    println!("Capacity");
    let mut sc = String::with_capacity(10);
    sc.push('a');
    println!("sc Capacity: {}bytes", sc.capacity());
    sc.push('b');
    sc.push('c'); 
    sc.push('d');
    println!("sc Capacity: {}bytes", sc.capacity());
    
}
