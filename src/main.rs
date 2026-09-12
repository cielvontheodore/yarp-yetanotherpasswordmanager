use dialoguer::Password; 
use std::fs::OpenOptions;
use std::fs::write;
use rand::rngs::SysRng;
use rand::TryRng;
use argon2::Argon2;

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

                // let writemp = write("vault.yarp", masterpass);
                // its to write the password to vault.yarp its js for testing
                
                let mut salt = [0u8; 16]; // salt to make cetakan password
                let mut argon = [0u8; 32]; //argon to encrypt the masterpassword
                
                let mut rng = SysRng; //rng

                rng.try_fill_bytes(&mut salt); // it just spawns em salt i think

                let aaron = Argon2::default()
                    .hash_password_into(
                        masterpass.as_bytes(), // because apparently argon wants a damn bytes not string
                        &salt, 
                        &mut argon
                    );

                // write("vault.yarp", salt);
                println!("salt: {:?}", salt); 
                println!("argon: {:?}", argon); 

            } //created is a variable and so is error. println is js a new line print

            Err(error) => {
                println!("oh oh something is wrong but im not sure why!");
            }
        };
    };

}
