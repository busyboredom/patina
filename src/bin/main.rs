fn main() {
    println!("My Income: ${}", patina::Budget::new().income_params.salary);
    println!("My age: {}", patina::Budget::new().misc_params.age);
}
