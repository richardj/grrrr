use structopt::StructOpt;
mod lines;

#[derive(StructOpt)]
struct Cli {
    pattern: String,
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() {
    println!("tab: {:?}", "	".bytes()); 
    println!("space: {:?}", " ".bytes()); 
    println!("A: {:?}", "A".bytes()); 

    let args = Cli::from_args();
    let result = std::fs::read_to_string(&args.path);
    let mut max_width:usize = 0;


    let content = match result {
        Ok(content) => { content },
        Err(error) => { panic!("Can't deal with {}, just exit here", error); }
    };


    // first loop through all the lines to check the longest line
    for line in content.lines() {
        if line.len() > max_width {
            max_width = line.len();
        }
    }

    println!("longest line is {}", max_width);


    let tab = "     ";
    let pasted_tab = "		";
    
    println!("TAB LENGTH {}", tab.len());
    println!("PASTEDTAB LENGTH {}", pasted_tab.len());


    lines::top_bar(max_width);


    for line in content.lines() {
        println!("{} {} {} {}", lines::vertical(), format!("{:width$}", line, width = max_width), lines::vertical(), line.len());

        if line.contains(&args.pattern) {
            //lines::top_bar(max_width);
            //println!("{}", args.pattern);
            //lines::bottom_bar(max_width);
        }
    }

    lines::bottom_bar(max_width);
}
