use dialoguer::Select;

fn main() {
    println!("====================================================");
    println!("   Welcome to YARP (Yet AnotheR Password manager)   ");
    println!("====================================================");

    let menu_options = vec![
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


}
