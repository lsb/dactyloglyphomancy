use slurp;
use sha2;
use sha2::Digest;
use rayon::prelude::*;
use std::env;
use itertools::Itertools;
use rand::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    assert!(args.len() == 3, format!("{}{}{}","\n\n🙀 ｏｈ 🙀 ｎｏ 🙀\nargv2 needs to be a ~_~-separated list of strings to intersperse with emoji, and argv1 needs to be a csv of [0-9a-f] strings to look for in the sha256, and /dev/stdin needs to be a newline-separated list of (emoji) to intersperse\n\n😻 ｔｒｙ 😻 ｉｎｓｔｅａｄ 😻\n", args[0], " 88888 '~_~ protect me from what I want ~_~'\n\n"));
    let numbers_to_match: Vec<String> = args[1].split(',').map(|s| s.to_owned()).collect();
    let strings_to_intercalate: Vec<String> = args[2].split("~_~").map(|s| s.to_owned()).collect();
    assert!(strings_to_intercalate.len() > 1, format!("{}{}{}","\n\n🙀 ｏｈ 🙀 ｎｏ 🙀\nargv2 needs to be a ~_~-separated list of strings to intersperse with emoji, and argv1 needs to be a csv of [0-9a-f] strings to look for in the sha256, and /dev/stdin needs to be a newline-separated list of (emoji) to intersperse\n\n😻 ｔｒｙ 😻 ｉｎｓｔｅａｄ 😻\n", args[0], " 88888 '~_~ protect me from what I want ~_~'\n\n"));
    let emoji = slurp::read_all_lines("/dev/stdin").unwrap();
    
    &emoji.par_iter().for_each(|a| {
        let mut rng = thread_rng();
        let first_string = format!("{}{}{}", strings_to_intercalate[0], a, strings_to_intercalate[1]);
        if strings_to_intercalate.len() == 2 {
            let h = format!("{:x}", sha2::Sha256::digest( first_string.as_bytes()));
            if numbers_to_match.iter().all(|s| h.contains(s)) {
                println!("{}  @  {}", first_string, h);
            }
        } else {
            let remaining_string_parts = &strings_to_intercalate[2..];
            let remaining_multi_cartesian_product = remaining_string_parts.iter().map(|_i| {
                let mut same_emoji = emoji.clone();
                same_emoji.shuffle(&mut rng);
                same_emoji
            }).multi_cartesian_product();
            for v in remaining_multi_cartesian_product {
               let final_string = format!("{}{}", first_string, v.iter().zip(remaining_string_parts).map(|(e,s)| format!("{}{}", e, s)).collect::<Vec<String>>().concat());
               let h = format!("{:x}", sha2::Sha256::digest( final_string.as_bytes() ));
               if numbers_to_match.iter().all(|s| h.contains(s)) {
                   println!("{}  @  {}", final_string, h);
               }
           } 
        }
    });
}
