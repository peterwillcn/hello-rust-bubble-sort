use std::fmt::Debug;
use std::str::FromStr;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(subcommand)]
    cmd: Command,
}

#[derive(Debug, StructOpt)]
enum Command {
    Asc {
        #[structopt(short, long)]
        data: Vec<String>,
    },
    Desc {
        #[structopt(short, long)]
        data: Vec<String>,
    },
}

fn bubble_sort<T: PartialOrd>(arr: &mut [T], asc: bool) {
    let n = arr.len();
    for i in 0..n {
        for j in 0..n - i - 1 {
            if asc {
                if arr[j + 1].partial_cmp(&arr[j]).unwrap().is_lt() {
                    arr.swap(j, j + 1);
                }
            } else if arr[j + 1].partial_cmp(&arr[j]).unwrap().is_gt() {
                arr.swap(j, j + 1);
            }
        }
    }
}

fn parse_and_sort<T>(data: Vec<String>, asc: bool)
where
    T: FromStr + PartialOrd + Debug,
    <T as FromStr>::Err: Debug,
{
    let mut parsed_data: Vec<_> = data
        .iter()
        .map(|s| s.parse::<T>().unwrap_or_else(|_| panic!("Failed to parse")))
        .collect();
    println!("Before sorting: {:?}", parsed_data);
    bubble_sort(&mut parsed_data, asc);
    if asc {
        println!("After sorted array in ascending order: {:?}", parsed_data);
    } else {
        println!("After sorted array in descending order: {:?}", parsed_data);
    }
}

fn main() {
    let opt = Opt::from_args();

    match opt.cmd {
        Command::Asc { data } => {
            parse_and_sort::<String>(data, true);
        }
        Command::Desc { data } => {
            parse_and_sort::<String>(data, false);
        }
    }
}
