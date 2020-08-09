fn main() {
    println!(
        "My Income: ${:?}",
        patina::Budget::new().income_params.jobs[0].pay
    );
    println!("My age: {}", patina::Budget::new().misc_params.age);
}
