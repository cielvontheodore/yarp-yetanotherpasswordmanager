fn main() {
    let command : Vec<String> = std::env::args().collect();
    if command[2] == "init" {
        println!("Initializing YARP!");
        // std::fs::File::create("vault.yarp");
        std::fs::OpenOptions::new();
    };

}
