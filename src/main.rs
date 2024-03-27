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
    #[structopt(name = "sort")]
    Sort(Elements),
}

fn bubble_sort<T: PartialOrd>(arr: &mut [T]) {
    let n = arr.len();
    for i in 0..n {
        for j in 0..n - i - 1 {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}

fn main() {
    println!("Hello, world!");
    let arg = Args::from_args();
    let arg2 = arg;
    match arg2.command {
        Command::Sort(e) => {
            let mut data = e.elements;
            println!("Before sorting: {:?}", data);
            bubble_sort(&mut data);
            println!("After sorting: {:?}", data);
        }
    }
}
