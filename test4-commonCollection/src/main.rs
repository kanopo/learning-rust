fn main() {
    let v: Vec<char> = vec!['a', 'b', 'c', 'd'];

    //println!("{:#?}", v);


    // ! per accedere ai vector in maniera sicura
    // (non hanno una lunghezza fissa coem gli array)
    // quindi gli errori vengono visti a runtime
    // ! devo usare match e get

    match v.get(2) {
        Some(element) => println!("{}", element),
        None => println!("Niente")
    }



    // ciclare per tutti gli elementi

    for i in &v {
        println!("{:?}", i)
    }

    // TODO: devo aggiungere le stringhe e le hashmap

}
