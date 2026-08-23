use dialoguer::Select;
use dialoguer::Input;
use dialoguer::Password;
use serde::Deserialize;
use serde::Serialize;


fn main() {
    println!("====================================================");
    println!("   Welcome to YARP (Yet AnotheR Password manager)   ");
    println!("====================================================");

    loop {let menu_options = vec![
        "Add Password",
        "Review Password",
        "All Services",
        "Exit",
    ];

    let option: usize = Select::new() 
        .with_prompt("Choose action...")
        .items(&menu_options)
        .default(0)
        .interact()
        .unwrap();
    

    println!("\nYou choose : {}\n", menu_options[option]);

    match option {
        0 => {
            let service: String = Input::new()
            .with_prompt("Service name ")
            .interact()
            .unwrap();

            let username: String = Input::new()
            .with_prompt("Profile name ")
            .interact()
            .unwrap();

            let password: String = Password::new()
            .with_prompt("Password ")
            .with_confirmation("Confirm password ", "Passwords do not match! ")
            .interact()
            .unwrap();

            println!("Account for {service} under the username {username} is ready!\n");
        }

        1 => {
            println!("You choose this option");
        }
   
        2 => {
            println!("You choose this option");
        }
            
        3 => {
            println!("You choose this option");
            break;
        } 
        
        _ => unreachable!(),
    } }


}
