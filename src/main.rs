use clap::Parser;


#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short = 's', long = "start")]
    start_date: String,

    #[clap(short = 'e', long = "end")]
    end_date: String,

    #[clap(short = 'w', long = "workdays")]
    work_days_only: bool,

    #[clap(short = 'c', long = "commits", value_parser = parse_tuple)]
    commits_per_day: (i32, i32)
}

fn parse_tuple(commits_per_day: &str) -> Result<(i32, i32), String> {
    let parts: Vec<&str> = commits_per_day.split(':').collect();
    if parts.len() != 2 {
        return Err(String::from("Input must contain exactly one : character, for example 1:3"));
    }

    let left = parts[0].parse::<i32>().map_err(|_| "Left value must be an integer")?;
    let right = parts[1].parse::<i32>().map_err(|_| "Right value must be an integer")?;

    Ok((left, right))
}

impl Args {
    pub fn open_repo() {
        todo!()
    }
}



fn main() {
    let args = Args::parse();

    println!("Start date: {:?}", args.start_date);
    println!("End date: {:?}", args.end_date);
    println!("Work days only: {:?}", args.work_days_only);
    println!("Commits per day: {:?}", args.commits_per_day);

}