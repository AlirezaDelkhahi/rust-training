pub fn variables_practice(){
    let a: u8;
    let b: u8;
    a = 250;
    b = 10;
    println!("a => {a}");
    println!("b => {b}");
    println!("a.wrapping_add(b) ====> {}", a.wrapping_add(b));
    match a.checked_add(b){
        Some(v) => println!("a.checked_add(b) ==> {v}"),
        None => println!("a.checked_add(b) got None"),
    }
    println!("a.saturating_add(b) => {}", a.saturating_add(b));
    match a.checked_add(b){
        Some(v) => println!("a.checked_add(b) ==> {v}"),
        None => println!("a.checked_add(b) got None"),
    }
    // println!("a + b => {:?}", c.0);
    // println!("a + b => {:#?}", c.1);

}