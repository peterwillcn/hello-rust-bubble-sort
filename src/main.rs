use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "app")]
pub struct Args {
    #[structopt(subcommand)]
    pub command: Command,
}

#[derive(StructOpt)]
pub struct Elements {
    pub elements: Vec<i32>,
}

#[derive(StructOpt)]
pub enum Command {
    #[structopt(name = "asc")]
    Asc(Elements),
    #[structopt(name = "desc")]
    Desc(Elements),
}

fn bubble_sort<T: PartialOrd>(arr: &mut [T], desc: bool) {
    let n = arr.len();
    for i in 0..n {
        for j in 0..n - i - 1 {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
    if desc {
        arr.reverse();
    }
}

fn main() {
    println!("Hello, world!");
    let mut data_str = vec!["banana", "apple", "orange", "grape"];
    let arg = Args::from_args();
    let arg2 = arg;
    match arg2.command {
        Command::Asc(e) => {
            let mut data = e.elements;
            println!("Before sorting: {:?}", data);
            bubble_sort(&mut data, false);
            println!("After sorting: {:?}", data);
            bubble_sort(&mut data_str, false);
            println!("Sorted string array: {:?}", data_str);
        }
        Command::Desc(e) => {
            let mut data = e.elements;
            println!("Before sorting: {:?}", data);
            bubble_sort(&mut data, true);
            println!("After sorting: {:?}", data);
            bubble_sort(&mut data_str, true);
            println!("Sorted string array: {:?}", data_str);
        }
    }
}
