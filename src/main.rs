use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    url: String,
}

fn main() {
    let args = Args::parse();
    let decoded_url = urlencoding::decode(&args.url).unwrap();

    // print the url and then formatted query vars name: value
    for line in decoded_url.lines() {
        let parts: Vec<&str> = line.split('?').collect();
        if parts.len() > 1 {
            println!("URL: {}", parts[0]);
            let query_string = parts[1];
            let query_vars: Vec<&str> = query_string.split('&').collect();
            for var in query_vars {
                let kv: Vec<&str> = var.split('=').collect();
                if kv.len() == 2 {
                    println!("{}: {}", kv[0], kv[1]);
                } else {
                    println!("{}: (no value)", kv[0]);
                }
            }
        } else {
            println!("URL: {}", line);
            println!("(no query parameters)");
        }
    }
}
