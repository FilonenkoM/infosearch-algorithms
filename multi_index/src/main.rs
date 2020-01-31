use multi_index;
fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        println!("--- CREATE A VOCABULARY ---
The first argument is a folder in which are the source files
The second argument is the path to the destination file
The third file is the palgorithm that you would like to choose (--greedy, --greedy_mltrd or --quick). The default is quick
Only UTF-8 TXT files accepte
For more information use documentation");
    }  else {
         let vocabulary = multi_index::build(&args[1]);
         multi_index::save(&args[2], &vocabulary);
    }
}
