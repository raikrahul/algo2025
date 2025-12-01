use std::collections::HashSet;

fn check_wrong_spelling(input_file: &String, dictionary: &String) -> Result<Vec<String>, std::io::Error> {
  let file_result_input_file = std::fs::read_to_string(input_file);

  let input_file_contents = match file_result_input_file {

    Ok(content) => {
            content
    }
    Err(e) => {
        return Err(e); 
    }
  };

  let ip_words: Vec<String> = input_file_contents.split_whitespace().map(str::to_string).collect();
  let dict_file_result = std::fs::read_to_string(dictionary);

  let dict_file_contents = match dict_file_result{

        Ok(content) =>{
          content
        }
        Err(e) => {
          return Err(e);
        }
  };

  let dict_hash_words:HashSet<String>  = dict_file_contents.split_whitespace().map(str::to_string).collect();
  let mut wrong_spelling_words:Vec<String> = Vec::new();

  for word in ip_words.iter(){
   
    if ! dict_hash_words.contains(word){
     
      wrong_spelling_words.push(word.clone());
    }
    
  }

  





  Ok(wrong_spelling_words)
}

fn main() {
    let input = "input.txt".to_string();
    let dict = "dict.txt".to_string();
    match check_wrong_spelling(&input, &dict) {
        Ok(misspelled) => {
            println!("Misspelled words: {:?}", misspelled);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}
