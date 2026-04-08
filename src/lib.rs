use std::collections::HashMap;
use std::io;
use wasm_bindgen::prelude::*;
// use dont_disappear; For release only
// use std::time::Instant;
// use rayon::prelude::*;
// Current winrate 97.42%
const WORDS: &str = include_str!("../wordle-words.txt");
#[wasm_bindgen]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}
#[wasm_bindgen]
pub fn suggest_guess(wordlist: Vec<String>, bannedlets: String, bannedatlets: Vec<String>, mustinclets: String, incatlets: String) -> String {
    let words = filter_words(&bannedlets.chars().collect(), &bannedatlets, &mustinclets.chars().collect(), &incatlets, &wordlist);
    if words.len() == 1 {
        return words[0].clone()
    }
    if words.len() <= 20 { for i in 0..words.len() { println!("{}: {}", i+1, words[i]); } }
    else { println!("{} words left", words.len()); }
    word_chooser(&wordlist, words)
}
#[allow(dead_code)]
pub fn solve_wordle_binary() {
    let wordlist = get_words();
    let mut no_guess: bool = false;
    let mut input: String;
    let mut bannedlets: Vec<char> = vec![];
    let mut bannedatlets: Vec<String> = vec![String::new(), String::new(), String::new(), String::new(), String::new()];
    let mut mustinclets: Vec<char> = vec![];
    let mut incatlets: String = String::from("#####");
    let mut guess= "crane".to_string();
    for _ in 0..6 {
        (input, guess, no_guess) = get_user_input(guess, &wordlist, no_guess);
        (bannedlets, bannedatlets, mustinclets, incatlets) = lists_from_input(&guess, bannedlets, bannedatlets, mustinclets, incatlets, input);
        let words = filter_words(&bannedlets, &bannedatlets, &mustinclets, &incatlets, &wordlist);
        if words.len() == 1 {
            println!("{} is the answer", words[0]);
            break;
        } else if words.len() == 0 {
            println!("Could not find word. Dataset incomplete or incorrect guess input.");
            break;
        }
        if words.len() <= 20 { for i in 0..words.len() { println!("{}: {}", i+1, words[i]); } }
        else { println!("{} words left", words.len()); }
        guess = word_chooser(&wordlist, words)
    }
    // dont_disappear::any_key_to_continue::default(); For release only
}
fn get_user_input(mut guess: String, wordlist: &Vec<String>, mut ng: bool) -> (String, String, bool) {
    let old_guess = guess.clone();
    if ng { guess = "-----".to_string(); }
    loop {
        println!("Results for {guess}:");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let input: String = input.trim().to_lowercase();
        if input.len() == 8 && input.starts_with("or") {
            let new_guess = input[3..8].to_string();
            if wordlist.contains(&new_guess) { guess = new_guess; }
        } else if input.starts_with("fw") {
            let new_input = format!("{}?", input[3..].to_string());
            guess = word_chooser(&get_words(), vec![new_input])
        } else if input == "ng" { ng = !ng; if !ng && guess == "-----".to_string() { guess = old_guess.clone() } }
        else if guess != "-----".to_string() && input.len() == 5 && input.chars().all(|x| ['g', 'n', 'y'].contains(&x)) { return (input, guess, ng) }
    }
}
// incatlets must be equal to ##### unless a leter is found, in which case it must be ##a##
// bannedatlets is ["abc", "abc", "abc", "", ""]
// bannedlets & mustinclets are ['a', 'b', 'c']
fn filter_words(bannedlets: &Vec<char>, bannedatlets: &Vec<String>, mustinclets: &Vec<char>, incatlets: &str, wordlist: &Vec<String>) -> Vec<String> {
    let mut poswords: Vec<String> = vec![];
    'word: for word in wordlist {
        for lett in bannedlets { if word.contains(*lett) { continue 'word; }}
        for lett in mustinclets { if !word.contains(*lett) { continue 'word; }}
        for i in 0..5 {
            let wordvec: Vec<char> = word.chars().collect();
            let pattern_char = incatlets.chars().nth(i);
            if pattern_char != Some('#') {
                if pattern_char.unwrap() != wordvec[i] {
                    continue 'word;
                }
            }
            let pattern_str = bannedatlets.get(i).unwrap();
            for j in 0..pattern_str.len() {
                if pattern_str.chars().nth(j).unwrap() == wordvec[i] {
                    continue 'word;
                }
            }
        }
        poswords.push(word.clone());
    }
    poswords
}
fn word_chooser(wordlist: &Vec<String>, narrowed_down_list: Vec<String>) -> String {
    let not_fw_mode = !(narrowed_down_list[0].chars().last() == Some('?'));
    if narrowed_down_list.len() <= 2 && not_fw_mode { return narrowed_down_list.get(0).unwrap().clone() }
    let mut wordcount: HashMap<char, i32> = HashMap::new();
    for word in &narrowed_down_list {
        for char in word.chars() {
            *wordcount.entry(char).or_insert(0) += 1;
        }
    }
    if not_fw_mode { for (_, v) in &mut wordcount { if *v >= narrowed_down_list.len() as i32 { *v = 0; } } }
    let mut wordscore: HashMap<String, i32> = HashMap::new();
    for word in wordlist {
        wordscore.insert(word.clone(), 0);
        let mut seen: Vec<char> = vec![];
        for char in word.chars() {
            if !seen.contains(&char) {
                *wordscore.entry(word.clone()).or_insert(0) += wordcount.get(&char).unwrap_or(&0);
            }
            seen.push(char);
        }
    }
    let mut wordscore: Vec<(&String, &i32)> = wordscore.iter().collect();
    wordscore.sort_by(|a, b| b.1.cmp(a.1).then_with(|| a.0.cmp(b.0)));
    if wordscore[0].1 > &0 { return wordscore[0].0.to_string() }
    narrowed_down_list.get(0).unwrap().clone()
}
#[wasm_bindgen]
pub fn get_words() -> Vec<String> {
    let mut wordlist = vec![];
    for word in WORDS.to_string().lines() {
        wordlist.push(word.to_string());
    }
    wordlist
}
#[wasm_bindgen]
#[allow(dead_code)]
pub struct ListJsOutput {
    // (bannedlets, bannedatlets, mustinclets, incatlets)
    bannedlets: String,
    bannedatlets: Vec<String>,
    mustinclets: String,
    incatlets: String
}
#[wasm_bindgen]
impl ListJsOutput {
    pub fn get_bannedlets(&self) -> String { self.bannedlets.clone() }
    pub fn get_bannedatlets(&self) -> Vec<String> { self.bannedatlets.clone() }
    pub fn get_mustinclets(&self) -> String { self.mustinclets.clone() }
    pub fn get_incatlets(&self) -> String { self.incatlets.clone() }
}
#[wasm_bindgen]
pub fn lists_from_input_js(guess: &str, bannedlets: String, bannedatlets: Vec<String>, mustinclets: String, incatlets: String, input: String) -> ListJsOutput {
    let (bannedlets, bannedatlets, mustinclets, incatlets) = lists_from_input(guess, bannedlets.chars().collect(), bannedatlets, mustinclets.chars().collect(), incatlets, input);
    ListJsOutput { bannedlets: bannedlets.iter().collect(), bannedatlets, mustinclets: mustinclets.iter().collect(), incatlets }
}
fn lists_from_input(guess: &str, mut bannedlets: Vec<char>, mut bannedatlets: Vec<String>, mut mustinclets: Vec<char>, mut incatlets: String, input: String) -> (Vec<char>, Vec<String>, Vec<char>, String) {
    let guess_chars: Vec<char> = guess.chars().collect();
    let input_chars: Vec<char> = input.chars().collect();
    for i in 0..5 {
        let guess_slice = guess_chars[i];
        let input_slice = input_chars[i];
        if input_slice == 'n' {
            if mustinclets.contains(&guess_slice) {
                bannedatlets.get_mut(i).unwrap().push(guess_slice);
            } else {
                bannedlets.push(guess_slice);
            }
        } else if input_slice == 'y' {
            mustinclets.push(guess_slice);
            bannedatlets.get_mut(i).unwrap().push(guess_slice);
        } else if input_slice == 'g' {
            incatlets.replace_range(i..i+1, &guess_slice.to_string());
            mustinclets.push(guess_slice);
        }
    }
    bannedlets.sort();
    bannedlets.dedup();
    let mut temp: Vec<char> = vec![];
    for char in &bannedlets {
        if incatlets.contains(*char) || mustinclets.iter().any(|c| c == char) { continue; }
        temp.push(*char);
    }
    bannedlets = temp;
    for i in 0..5 {
        let mut temp: Vec<char> = bannedatlets.get(i).unwrap().chars().collect();
        temp.sort();
        temp.dedup();
        let temp: String = temp.into_iter().collect();
        bannedatlets[i]= temp;
    }
    mustinclets.sort();
    mustinclets.dedup();
    (bannedlets, bannedatlets, mustinclets, incatlets)
}
// #[allow(dead_code)]
// pub fn get_guess_result(ans: &str, guess: &str) -> String {
//     let ans_chars: Vec<char> = ans.chars().collect();
//     let guess_chars: Vec<char> = guess.chars().collect();
//     let mut result = vec!['n'; 5];
//     let mut letter_counts = std::collections::HashMap::new();
//     for &c in &ans_chars {
//         *letter_counts.entry(c).or_insert(0) += 1;
//     }
//     for i in 0..5 {
//         if guess_chars[i] == ans_chars[i] {
//             result[i] = 'g';
//             *letter_counts.get_mut(&guess_chars[i]).unwrap() -= 1;
//         }
//     }
//     for i in 0..5 {
//         if result[i] == 'g' { continue; }
//         if let Some(count) = letter_counts.get_mut(&guess_chars[i]) {
//             if *count > 0 {
//                 result[i] = 'y';
//                 *count -= 1;
//             }
//         }
//     }
//     result.into_iter().collect()
// }
// #[allow(dead_code)]
// fn test_bot(word: &str) {
//     let wordlist = get_words();
//     let mut bannedlets: Vec<char> = vec![];
//     let mut bannedatlets: Vec<String> = vec![String::new(), String::new(), String::new(), String::new(), String::new()];
//     let mut mustinclets: Vec<char> = vec![];
//     let mut incatlets: String = String::from("#####");
//     let mut guess= "aeros".to_string();
//     for i in 0..6 {
//         let input = get_guess_result(&word, &guess);
//         println!("Guess {}     {guess}\n            {input}\n------------------", i+1);
//         if input == "ggggg" { break; }
//         if i == 5 { break; }
//         (bannedlets, bannedatlets, mustinclets, incatlets) = lists_from_input(&guess, bannedlets, bannedatlets, mustinclets, incatlets, input);
//         let words = filter_words(&bannedlets, &bannedatlets, &mustinclets, &incatlets, &wordlist);
//         if words.len() == 0 { break; }
//         if words.len() <= 25 { for word in &words { println!("{word}"); } println!("------------------"); }
//         guess = word_chooser(&wordlist, words)
//     } 
// }
// #[allow(dead_code)]
// fn test_bot_allwords() {
//     let start = Instant::now();
//     let allwords = get_words();
//     let games = allwords.len();
//     let wins: usize = allwords
//         .into_par_iter()
//         .map(|word| {
//             let wordlist = get_words();
//             let mut bannedlets: Vec<char> = vec![];
//             let mut bannedatlets: Vec<String> = vec![String::new(), String::new(), String::new(), String::new(), String::new()];
//             let mut mustinclets: Vec<char> = vec![];
//             let mut incatlets: String = String::from("#####");
//             let mut guess= "aeros".to_string();
//             for i in 0..6 {
//                 let input = get_guess_result(&word, &guess);
//                 if input == "ggggg" { return 1; }
//                 if i == 5 { println!("Failed {word}"); return 0; }
//                 (bannedlets, bannedatlets, mustinclets, incatlets) = lists_from_input(&guess, bannedlets, bannedatlets, mustinclets, incatlets, input);
//                 let words = filter_words(&bannedlets, &bannedatlets, &mustinclets, &incatlets, &wordlist);
//                 if words.len() == 0 { return 0; }
//                 guess = word_chooser(&wordlist, words);
//             }
//             return 0;
//         })
//         .sum();
//     let win_percent = (wins as f64 / games as f64) * 100.0;
//     println!("Wins {:.2}% of the time\nTook {} mins {} secs", win_percent, start.elapsed().as_secs()/60, start.elapsed().as_secs()%60);
// }