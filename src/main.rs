use clap::{Arg, App};

fn main() {
    let matches = App::new("Mortgage Calculator")
        .version("0.1.0")
        .author("Joris de Vries")
        .about("Calculates total mortgage cost based")
        .arg(Arg::with_name("dur")
                 .short("d")
                 .long("dur")
                 .takes_value(true)
                 .help("Duration (years)"))
        .arg(Arg::with_name("am")
                 .short("a")
                 .long("am")
                 .takes_value(true)
                 .help("Amount borrowed (pounds)"))
        .arg(Arg::with_name("sf")
                 .short("s")
                 .long("sf")
                 .takes_value(true)
                 .help("Setup cost of mortgage (pounds)"))
        .arg(Arg::with_name("im")
                 .short("i")
                 .long("im")
                 .takes_value(true)
                 .help("Initial interest rate (%)"))
        .arg(Arg::with_name("id")
                 .short("r")
                 .long("id")
                 .takes_value(true)
                 .help("Initial interest rate dur (months)"))
        .arg(Arg::with_name("mp")
                 .short("z")
                 .long("mp")
                 .takes_value(true)
                 .help("Monthly interest (%)"))
        .arg(Arg::with_name("over")
                 .short("o")
                 .long("over")
                 .takes_value(true)
                 .help("Overpayment made (pounds)"))
        .arg(Arg::with_name("of")
                 .short("x")
                 .long("of")
                 .takes_value(true)
                 .help("Flat fee for over"))
        .arg(Arg::with_name("oa")
                 .short("c")
                 .long("oa")
                 .takes_value(true)
                 .help("Overpayment free allowance (%)"))
        .arg(Arg::with_name("or")
                 .short("f")
                 .long("or")
                 .takes_value(true)
                 .help("Overpayment interest rate (%)"))
        .get_matches();

    let duration = str::parse::<i32>(matches.value_of("dur").unwrap()).unwrap();
    let amount = str::parse::<f32>(matches.value_of("am").unwrap()).unwrap();
    let setup_fee = str::parse::<f32>(matches.value_of("sf").unwrap()).unwrap();
    let initial_monthly = str::parse::<f32>(matches.value_of("im").unwrap()).unwrap();
    let initial_duration = str::parse::<i32>(matches.value_of("id").unwrap()).unwrap();
    let monthly_interest = str::parse::<f32>(matches.value_of("mp").unwrap()).unwrap();
    let overpayment = str::parse::<f32>(matches.value_of("over").unwrap()).unwrap();
    let overpayment_fee = str::parse::<f32>(matches.value_of("of").unwrap()).unwrap();
    let overpayment_allowance = str::parse::<f32>(matches.value_of("oa").unwrap()).unwrap();
    let overpayment_rate = str::parse::<f32>(matches.value_of("or").unwrap()).unwrap();

    println!("--------------------------------------------------");
    println!("Mortgage duration:            {} years", duration);
    println!("Amount borrowed:              £{}", amount);
    println!("Setup fee:                    £{}", setup_fee);
    println!("Initial interest rate:        {}%", initial_monthly);
    println!("Duration of initial payment:  {} months", initial_duration);
    println!("Monthly interest afterwards:  {}%", monthly_interest);
    println!("Yearly overpayment amount:    £{}", overpayment);
    println!("Overpayment fee:              £{}", overpayment_fee);
    println!("Overpayment allowance:        £{}", overpayment_allowance);
    println!("Overpayment rate:             {}%", overpayment_rate);
    println!("--------------------------------------------------");

    // cargo run -- --dur 15 --am 350000 --sf 1525 --im 1.16 --id 27 --mp 4.09 --over 50000 --of 50 --oa 0 --or 1 

    let monthly_payment = amount/(duration as f32 * 12.0);
    let initial_monthly_interest = (1.0 + (initial_monthly/100.0)).powf(1.0/12.0) - 1.0; 
    let monthly_interest = (1.0 + (monthly_interest/100.0)).powf(1.0/12.0) - 1.0; 

    let mut outstanding_amount = amount;
    let mut initial_months_remaining = initial_duration;

    let mut interest_and_fees = setup_fee;
    let mut year = 0;

    while outstanding_amount > 0.0 {
        for i in 0..12 {
            if initial_months_remaining > 0 {
                interest_and_fees += initial_monthly_interest*outstanding_amount; 
                outstanding_amount -= monthly_payment;
                println!("  > Monthly payment {}:     £{}", i+1, monthly_payment + (initial_monthly_interest*outstanding_amount));
                initial_months_remaining -= 1; 
            } else {
                interest_and_fees += monthly_interest*outstanding_amount; 
                outstanding_amount -= monthly_payment;
                println!("  > Monthly payment {}:     £{}", i+1, monthly_payment + (initial_monthly_interest*outstanding_amount));
            } 
        }

        let real_overpayment = overpayment.min(outstanding_amount);
        let free_overpayment = overpayment_allowance * outstanding_amount; 

        let chargeable_overpayment = (real_overpayment - free_overpayment).max(0.0); 
        let mut overpayment_total_fee = overpayment_fee + ((overpayment_rate/100.0) * chargeable_overpayment); 

        if initial_months_remaining == 0 {
            overpayment_total_fee = 0.0;
        }

        outstanding_amount -= real_overpayment;
        interest_and_fees += overpayment_total_fee; 
        year += 1;

        println!("YEAR:                     {}", year);
        println!("Amount remaining:         £{}", outstanding_amount);
        println!("Interest and fees paid:   £{}", interest_and_fees);
        println!("--------------------------------------------------");
    } 

    let total_cost = amount + interest_and_fees;
    let functional_yearly_interest = (total_cost/amount).powf(1.0/(year as f32));
    println!("Total cost:               £{:.2}", total_cost);
    println!("Functional interest:      {:.2}% (yearly)", 100.0 * (functional_yearly_interest - 1.0));
    println!("--------------------------------------------------");
}
