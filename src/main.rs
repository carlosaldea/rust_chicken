mod maths;

fn main() {

    let add_result = maths::maths::add(5, 3);
    let sub_result = maths::maths::sub(7, 2);

    println!("The result of adding 5 + 3 is {}", add_result);
    println!("The result of adding 7 - 2 is {}", sub_result);
}
