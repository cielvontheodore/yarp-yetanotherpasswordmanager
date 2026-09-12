fn main() {
    let command : Vec<String> = std::env::args().collect();
    // apparently vec itu buat sesuatu yang nambah terus, std itu ngambil library rust abis itu
    // ambil library env yang di bawaan rust lalu jalanin args tapi gua lupa args itu apa
    if command[2] == "init" {
        println!("Initializing YARP!");
        // std::fs::File::create("vault.yarp"); this worked but yeah we're going harder than that,
        // angh harder daddy
        use std::fs::OpenOptions;
        let result = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open("vault.yarp");
        match result {
            Ok(created) => {
                println!("yarp created!");
            }

            Err(error) => {
                println!("oh oh something is wrong but im not sure why!");
            }

        };
    };

}
