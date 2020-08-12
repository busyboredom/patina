# WORK IN PROGRESS

# Patina
A swiss army knife for personal finance.

Google Sheets and Excel can help you figure out where your finances are at, and they can even tell you a bit about where your finances are going, but these common speadsheets have a weakness:

**They suck at making intelligent recommendations.**

Patina aims to be a versatile budget optimization tool for answering tough questions like

* How much of my 401k should be Roth next year?

* Is a rental property a good investment for me? 

* How much cash should I be keeping in an emergency fund?

## How it works

By default, Patina makes guesses about your financial situation and generates recommendations based on them. To get better recommendations, just give patina better information. For example, Patina assumes you are unemployed. If you make $65,000/year, you should tell it that.

```
let mut my_budget = patina::Budget::new();
let mut my_job = patina::income::Job::new();

my_job.pay = patina::income::Pay::Salary(65_000.0);

my_budget.income_params.jobs.push(my_job);
```
