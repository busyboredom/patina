fn main() {
    // Create budget and job.
    let mut my_budget = patina::Budget::new();
    let mut my_job = patina::income::Job::new();

    // Set job parameters and push job into budget.
    my_job.pay = patina::income::Pay::Hourly(35.0);
    my_job.overtime_after = Some(45.0);
    my_job.weekly_hours = 50.0;
    my_budget.income_params.jobs.push(my_job);

    // Calculate gross income.
    patina::income::calc_gross_income(&mut my_budget);

    println!("My pay: {}", my_budget.income_params.jobs[0].pay);
    println!("My age: {}", my_budget.misc_params.age);
    println!(
        "My gross income: {}",
        my_budget.income_params.c_gross_income.unwrap()
    );
}
