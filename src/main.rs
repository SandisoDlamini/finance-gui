slint::include_modules!();

use std::collections::HashMap;
use std::io;
use std::fs::File;
use std::io::prelude::*;
use std::fs;
use serde_json;

const SAVINGS_PERCENTAGE: f64 = 0.5;
const SPENDING_PERCENTAGE: f64 = 0.3;
const SURPLUS_PERCENTAGE: f64 = 0.2;


fn open_file() {
    
    let mut file_name = String::new();

    println!("Please input name of month to open.");

    io::stdin()
        .read_line(&mut file_name)
        .expect("Failed to read input");

    let file_name = file_name.trim().to_string();

    let file_path_buff = fs::canonicalize("./src/month_files/").expect("Cannot get path");
    let inter_file_path = file_path_buff.into_os_string()
        .into_string()
        .expect("cannot convert file path");

    let file_path = inter_file_path + "/" + file_name.as_str() + ".json";

    let mut file = File::open(file_path).unwrap();

    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    println!("\n{}\n", contents);


}

fn create_file() -> String {
    loop {
        
        let mut file_name = String::new();

        println!("Please input name of month to save to:");

        io::stdin()
            .read_line(&mut file_name)
            .expect("Failed to read input");

        let file_name = file_name.trim().to_string();

        println!("Saving to: {}", &file_name);


        let mut choice_confirmation = String::new();

        println!("Are you sure you want to save to {}? [y/N]", file_name);

        io::stdin()
            .read_line(&mut choice_confirmation)
            .expect("Cannot save to file.");
        
        let choice_confirmation = choice_confirmation.trim();

        let choices_for = ["Y", "y", "YES", "yes"];
        let choices_against = ["N", "n", "NO", "no"];

        if choices_for.iter().any(|&i| i == choice_confirmation) {
            break file_name;
        }

        else if choices_against.iter().any(|&i| i == choice_confirmation) {
            continue;
        }

    }

}

fn save_file(filename: String) {

    let file_name = &filename;

    let file_path_buff = fs::canonicalize("./src/month_files/").expect("Cannot get path");
    let inter_file_path = file_path_buff.into_os_string()
        .into_string()
        .expect("cannot convert file path");
    let file_path = inter_file_path + "/" + file_name + ".json";
    
    let mut file = File::create(file_path).expect("Failed to create file");
    
    let amount = money_divider(user_input());

    let amount_dict = amount.1;

    let j = serde_json::to_string(&amount_dict).unwrap();
    file.write_all(j.as_bytes()).expect("could not write to json file");

    println!("\nSaving to {:?} as\n {}\n",file, j)

    

}

fn user_input() -> f64 {
    
    loop {
        
        let mut income = String::new();

        println!("Please input amount of money earned:");

        io::stdin()
            .read_line(&mut income)
            .expect("Failed to read input");

        let income: f64 = match income.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You earned: {}", income);

        return income;

    }
}

fn money_divider(a: f64) -> (Vec<f64>, HashMap<String, String>) {

    let gross_income = &a;
    let savings = gross_income * SAVINGS_PERCENTAGE;
    let spending_budget = gross_income * SPENDING_PERCENTAGE;
    let surplus_budget = gross_income * SURPLUS_PERCENTAGE;

    println!("{}",format!("Your necessary savings are: E {:.2}", &savings));
    println!("{}",format!("Your spending budget is:    E {:.2}", &spending_budget));
    println!("{}",format!("Your surplus/fun budget is: E {:.2}", &surplus_budget));

    let expenditure = vec![savings, spending_budget, surplus_budget];

    let mut dict_expenses = HashMap::new();

    dict_expenses.insert(String::from("savings"), format!("{:.2}", savings));
    dict_expenses.insert(String::from("spending_budget"), format!("{:.2}",spending_budget));
    dict_expenses.insert(String::from("surplus_budget"), format!("{:.2}", surplus_budget));

    (expenditure, dict_expenses)

}

fn main_menu() {
 
    loop {

        println!("Hello, User! Welcome to your personal finances manager.");
        println!("Here are some options to get you started.");
        println!("\t\t1. Open File");
        println!("\t\t2. Save File");
        println!("\t\t3. Quit");
        println!("\n");

        let mut option = String::new();

        println!("Please select menu option.");
    
        io::stdin()
            .read_line(&mut option)
            .expect("Failed to read input");
    
        let option = option.trim();

        if option == "1" {
            open_file();            
        }
       
        if option == "2" {
            save_file(create_file());
        }

        if option == "3" {
            panic!("You quit the program.")
        }
        
    }
 
}

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    ui.on_request_increase_value({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            ui.set_counter(ui.get_counter() + 1);
        }
    });

    ui.run()
}
