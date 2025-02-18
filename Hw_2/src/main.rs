fn concat_strings(s1: &String, s2: &String) -> String{
    let mut result = s1.clone();
    result.push_str(s2);
    result
}
fn main(){
    let s1 = String::from("Hello, ");
    let s2 = String::from("World!");
    let result = concat_strings(&s1, &s2);
    println!("{}", result);
}
    ///////////////////////////////////////
    ///////////////////////////////////////
fn clone_and_modify(s: &String) -> String{
    let mut cloned = s.clone();
    cloned.push_str("World!");
    cloned
}
fn main(){
    let s = String::from("Hello, ");
    let modified = clone_and_modify(&s);
    println!("Original: {}", s);
    println!("Modified: {}", modified);
}
    ///////////////////////////////////////
    ///////////////////////////////////////

#[allow(unused_variables, unused_mut)]
fn sum(total: &mut i32, low: i32, high: i32){
    for i in low..=high{
        *total += i;
    }
}
fn main(){
    let mut total = 0;
    sum(&mut total, 0, 100);
    println!("The sum from 0 to 100 is: {}", total);
}