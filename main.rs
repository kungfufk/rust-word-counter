use std::io::Read;
use std::fs::File;

fn main() {
    let v: Vec<String> = std::env::args().collect();

    if v.len() < 2 {
        println!("Usage: <File>");
        return;
    }

        let file = File::open(v[1]);
 let mut hash: std::collections::HashMap<String, usize> =
    std::collections::HashMap::new();
        match file {
            Ok(mut file) => {
                let mut content = String::new();

                match file.read_to_string(&mut content)
                 {
                    Ok(_) => {

for i in content.split_whitespace() {
    let word=i.to_lowercase();
    *hash.entry(word).or_insert(0) += 1;
}
                    }

                    Err(err) => eprintln!("{}", err),
                }
                for (a,b) in hash.iter(){
                    println!("{}:{}",a,b)
            }

            Err(err) => eprintln!("{}", err),
        }

       
    }
}
