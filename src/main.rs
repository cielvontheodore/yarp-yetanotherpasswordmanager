use dialoguer::Password; 
use std::fs::OpenOptions;

fn main() {
    let command : Vec<String> = std::env::args().collect();
    // apparently vec itu buat sesuatu yang nambah terus, std itu ngambil library rust abis itu
    // ambil library env yang di bawaan rust lalu jalanin args tapi gua lupa args itu apa
    if command[2] == "init" {
        println!("Initializing YARP!");
        // std::fs::File::create("vault.yarp"); this worked but yeah we're going harder than that,
        // angh harder daddy
        let result = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open("vault.yarp");
        match result {
            Ok(created) => {
                println!("yarp created!");
       
                let masterpass: String = Password::new() //syg jgn lupa taro use dialoguer
                    .with_prompt("Enter master password ")
                    .with_confirmation("Confirm password ", "Password do not match! ")
                    .interact()
                    .unwrap();

            } //created is a variable and so is error. println is js a new line print

            Err(error) => {
                println!("oh oh something is wrong but im not sure why!");
            }
        };
    };

}
