use std::io::BufRead;

fn main() {
	
    let arg = |name| std::env::args_os().skip(1).collect::<Vec<std::ffi::OsString>>().windows(2).find(|a| a[0] == name).and_then(|a| a[1].to_str()).unwrap().to_owned();
    let parse = |name| std::io::BufReader::new(std::fs::File::open(name).unwrap()).lines().map(|line| line.map(|line| line.split(',').map(|entry| entry.trim().to_string()).collect::<Vec<String>>()).unwrap()).collect::<Vec<Vec<String>>>();
    
    println!("{:#?}", parse(arg("--file")));
}