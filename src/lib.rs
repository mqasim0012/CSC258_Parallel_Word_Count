use std::collections::HashMap;
use std::thread;
use std::sync::{Mutex, Arc};
use std::io::prelude::*;
use std::fs::File;

pub fn word_count(article: &str) -> HashMap<String, i64> {
    let mut word_map: HashMap<String, i64> = HashMap::new();
    let mut current_word_index = 0;

    for (index, current_character) in article.chars().enumerate() {

        if current_character.is_whitespace() || current_character == '\n' {
            if current_word_index == index {
                current_word_index = index+1;
                continue;
            } else {
                let current_word = &article[current_word_index..(index)];
                let prev_count = word_map.get(current_word);
                if prev_count == None {
                    word_map.insert((*current_word).to_string(), 1);
                } else {
                    word_map.insert((*current_word).to_string(), prev_count.unwrap()+1);
                }
                current_word_index = index+1;
            }
        }
    }

    // last word (after last whitespace)
    if current_word_index < article.len() - 1 {
        let current_word = &article[current_word_index..];
        let prev_count = word_map.get(current_word);
        if prev_count == None {
            word_map.insert((*current_word).to_string(), 1);
        } else {
            word_map.insert((*current_word).to_string(), prev_count.unwrap()+1);
        }
    }
    return word_map;
}

pub fn mt_word_count(article: &str) -> HashMap<String, i64> {
    let length = article.len();

    let mut first_quarter_index = length / 4;
    let mut half_index = length / 2;
    let mut third_quarter_index = first_quarter_index * 3;

    // assuming a word is small enough (relative to string length--as the length of string grows) so as not to affect runtime in a long program, we can move our indexes a little while remaining in the main thread
    while &article[first_quarter_index..(first_quarter_index+1)] != " " && first_quarter_index < length {
        first_quarter_index += 1;
    }

    while &article[half_index..(half_index+1)] != " " && half_index < length {
        half_index += 1;
    }

    while &article[third_quarter_index..(third_quarter_index+1)] != " " && third_quarter_index < length {
        third_quarter_index += 1;
    }

    let map_locker = Arc::new(Mutex::new(HashMap::<String, i64>::new()));

    // copies for first thread
    let map_locker_ref_1 = map_locker.clone();
    let article_first_quarter = article[..first_quarter_index].to_string();

    // first thread
    let first = thread::spawn( move || {
        if 0 < first_quarter_index {
            let mut current_word_index = 0;
            for (index, current_character) in article_first_quarter.chars().enumerate() {
                if current_character.is_whitespace() || current_character == '\n' || index == article_first_quarter.len()-1 {
                    if current_word_index == index {
                        current_word_index = index+1;
                        continue;
                    } else {
                        let mut word_map = map_locker_ref_1.lock().unwrap();
                        let current_word;
                        if index == article_first_quarter.len()-1 {
                            current_word = &article_first_quarter[current_word_index..];
                        } else {
                            current_word = &article_first_quarter[current_word_index..(index)];
                        }
                        let prev_count = (*word_map).get(current_word);
                        if prev_count == None {
                            word_map.insert((*current_word).to_string(), 1);
                        } else {
                            let prev_count_val = prev_count.cloned().unwrap();
                            word_map.insert((*current_word).to_string(), prev_count_val+1);
                        }
                        current_word_index = index+1;
                    }
                }

            }
        }
    });

    // copies for second thread
    let map_locker_ref_2 = map_locker.clone();
    let article_second_quarter = article[first_quarter_index..half_index].to_string().clone();

    // second thread
    let second = thread::spawn( move || {
        if first_quarter_index < half_index {
            let mut current_word_index = 0;
            for (index, current_character) in article_second_quarter.chars().enumerate() {
                if current_character.is_whitespace() || current_character == '\n' || index == article_second_quarter.len()-1 {
                    if current_word_index == index {
                        current_word_index = index+1;
                        continue;
                    } else {
                        let mut word_map = map_locker_ref_2.lock().unwrap();
                        let current_word;
                        if index == article_second_quarter.len()-1 {
                            current_word = &article_second_quarter[current_word_index..];
                        } else {
                            current_word = &article_second_quarter[current_word_index..(index)];
                        }
                        let prev_count = (*word_map).get(current_word);
                        if prev_count == None {
                            word_map.insert((*current_word).to_string(), 1);
                        } else {
                            let prev_count_val = prev_count.cloned().unwrap();
                            word_map.insert((*current_word).to_string(), prev_count_val+1);
                        }
                        current_word_index = index+1;
                    }
                }

            }
        }
    });
    
    // copies for third thread
    let map_locker_ref_3 = map_locker.clone();
    let article_third_quarter = article[half_index..third_quarter_index].to_string().clone();

    // third thread
    let third = thread::spawn( move || {
        if half_index < third_quarter_index {
            let mut current_word_index = 0;
            for (index, current_character) in article_third_quarter.chars().enumerate() {
                if current_character.is_whitespace() || current_character == '\n' || index == article_third_quarter.len()-1 {
                    if current_word_index == index {
                        current_word_index = index+1;
                        continue;
                    } else {
                        let mut word_map = map_locker_ref_3.lock().unwrap();
                        let current_word;
                        if index == article_third_quarter.len()-1 {
                            current_word = &article_third_quarter[current_word_index..];
                        } else {
                            current_word = &article_third_quarter[current_word_index..(index)];
                        }
                        let prev_count = (*word_map).get(current_word);
                        if prev_count == None {
                            word_map.insert((*current_word).to_string(), 1);
                        } else {
                            let prev_count_val = prev_count.cloned().unwrap();
                            word_map.insert((*current_word).to_string(), prev_count_val+1);
                        }
                        current_word_index = index+1;
                    }
                }

            }
        }
    });
    
    // copies for fourth thread
    let map_locker_ref_4 = map_locker.clone();
    let article_fourth_quarter = article[third_quarter_index..].to_string().clone();

    // fourth thread
    let fourth = thread::spawn( move || {
        if third_quarter_index < length {
            let mut current_word_index = 0;
            for (index, current_character) in article_fourth_quarter.chars().enumerate() {
                if current_character.is_whitespace() || current_character == '\n' || index == article_fourth_quarter.len()-1 {
                    if current_word_index == index {
                        current_word_index = index+1;
                        continue;
                    } else {
                        let mut word_map = map_locker_ref_4.lock().unwrap();
                        let current_word;
                        if index == article_fourth_quarter.len()-1 {
                            current_word = &article_fourth_quarter[current_word_index..];
                        } else {
                            current_word = &article_fourth_quarter[current_word_index..(index)];
                        }
                        let prev_count = (*word_map).get(current_word);
                        if prev_count == None {
                            word_map.insert((*current_word).to_string(), 1);
                        } else {
                            let prev_count_val = prev_count.cloned().unwrap();
                            word_map.insert((*current_word).to_string(), prev_count_val+1);
                        }
                        current_word_index = index+1;
                    }
                }

            }
        }
    });

    first.join().unwrap();
    second.join().unwrap();
    third.join().unwrap();
    fourth.join().unwrap();
    let map = map_locker.lock().unwrap();

    return map.clone();
}

pub fn print_map(map: &HashMap<String, i64>) {
    println!("---------------------------------------------------");
    for (word, count) in (*map).iter() {
        println!("{word}: {count}");
    }
    println!("---------------------------------------------------");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn import_string_from_file(filepath: &str) -> String {
        let mut file = File::open(filepath).expect("Unable to open file");
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("Unable to read the file");
        return contents;
    }

    #[test]
    fn test_word_count() {
        let article: String = import_string_from_file("million_words.txt");
        let map = word_count(&article[..]);
        let ligula_count = map.get(&"ligula".to_string()).unwrap();
        assert_eq!(*ligula_count, 7275);
    }
    
    #[test]
    fn test_mt_word_count() {
        let article: String = import_string_from_file("million_words.txt");
        let map = mt_word_count(&article[..]);
        let ante_count = map.get(&"ante".to_string()).unwrap();
        assert_eq!(*ante_count, 6551);
    }
}
