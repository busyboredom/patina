fn main() {
    let mut my_budget = patina::Budget::new();
    let mut my_job = patina::income::Job::new();
    
    my_job.pay = patina::income::Pay::Hourly(35.0);

    my_budget.income_params.jobs.push(my_job);

    println!(
        "My Income: {}",
        my_budget.income_params.jobs[0].pay
    );
    println!("My age: {}", patina::Budget::new().misc_params.age);
}
