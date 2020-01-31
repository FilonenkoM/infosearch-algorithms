use dict;
use dict::Algorithm;

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
        let algorithm = match args.get(3) {
            None => Algorithm::Quick,
            Some(value) => match value.as_str() {
                    "--greedy" => Algorithm::Greedy,
                    "--greedy_mlrtd" => Algorithm::GreedyMultithread,
                    _ => Algorithm::Quick,
                }
            };
         let stats = dict::vocabulary(&args[1], &args[2], algorithm);
         println!("Statistics:
files: {},
entries: {},
vocabulary size: {} bit", stats.files(), stats.entries(), stats.vocabulary_size());
    }
}
