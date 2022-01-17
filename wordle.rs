use chrono::{offset, Local, NaiveDate, TimeZone};
use crossterm::{
  cursor,
  event::{read, Event, KeyCode, KeyEvent},
  style::{Color, ResetColor, SetForegroundColor},
  terminal::{disable_raw_mode, enable_raw_mode},
  QueueableCommand,
};
use rand::{distributions::uniform::SampleRange, thread_rng, Rng};
use std::env;
use std::io::{stdout, Error, Stdout, Write};

mod words;

enum LETTER_COLOR {
  GRAY,
  YELLOW,
  GREEN,
}
fn init_guess(length: usize) -> Vec<String> {
  let mut guess = Vec::new();
  for _ in 0..length {
    guess.push('_'.to_string());
  }
  return guess;
}

fn read_word(stdout: &mut Stdout, length: usize) -> Result<Vec<String>, Error> {
  let mut guess = init_guess(length);
  let mut letter = 0;
  while let Event::Key(KeyEvent { code, .. }) = read()? {
    match code {
      KeyCode::Enter => {
        if letter == length {
          break;
        }
      }
      KeyCode::Backspace => {
        if letter == 0 {
          continue;
        }
        letter -= 1;
        guess[letter] = '_'.to_string();
        write_guess(stdout, &guess);
      }
      KeyCode::Char(c) => {
        if letter == length {
          continue;
        }
        guess[letter] = c.to_string();
        letter += 1;
        write_guess(stdout, &guess);
      }
      _ => {}
    }
  }
  return Ok(guess);
}

fn write_guess(stdout: &mut Stdout, guess: &Vec<String>) {
  stdout.queue(cursor::SavePosition);
  stdout.write(guess.join(" ").as_bytes());
  stdout.queue(cursor::RestorePosition);
  stdout.flush();
}

// if word does not contain char, return false
// else, get correct count for guessed_word[i] --> if correct < total --> return yellow
//    (i.e. if there is still another letter out there)
// else return false
fn should_be_yellow(guessed_word: &Vec<String>, word: &str, i: usize) -> bool {
  if !word.contains(guessed_word[i].as_bytes()[0] as char) {
    return false;
  }

  let mut correct = 0;
  for j in 0..word.len() {
    if guessed_word[i] == word[j..j + 1] && guessed_word[j] == word[j..j + 1] {
      correct += 1;
    }
  }
  correct < word.matches(guessed_word[i].get(0..1).unwrap()).count()
}

fn build_color_vec(guessed_word: &Vec<String>, word: &str) -> Vec<LETTER_COLOR> {
  let mut result = Vec::new();
  for i in 0..guessed_word.len() {
    if guessed_word[i].as_bytes()[0] == word.as_bytes()[i] {
      result.push(LETTER_COLOR::GREEN);
    } else if should_be_yellow(guessed_word, word, i) {
      result.push(LETTER_COLOR::YELLOW);
    } else {
      result.push(LETTER_COLOR::GRAY);
    }
  }

  result
}

fn write_colored_guess(
  stdout: &mut Stdout,
  guessed_word: &Vec<String>,
  color_vec: &Vec<LETTER_COLOR>,
) {
  stdout.queue(cursor::SavePosition);
  for i in 0..guessed_word.len() {
    if matches!(color_vec[i], LETTER_COLOR::GREEN) {
      stdout.queue(SetForegroundColor(Color::Green));
      stdout.write(guessed_word[i].as_bytes());
      stdout.write(" ".as_bytes());
      stdout.flush();
    } else if matches!(color_vec[i], LETTER_COLOR::YELLOW) {
      stdout.queue(SetForegroundColor(Color::Yellow));
      stdout.write(guessed_word[i].as_bytes());
      stdout.write(" ".as_bytes());
      stdout.flush();
    } else {
      stdout.queue(ResetColor);
      stdout.write(guessed_word[i].as_bytes());
      stdout.write(" ".as_bytes());
      stdout.flush();
    }
  }
  stdout.queue(cursor::RestorePosition);
  stdout.write("\n".as_bytes());
  stdout.flush();
  stdout.queue(ResetColor);
}

fn input_loop(word_index: usize, word: &'static str) -> Result<u32, Error> {
  let mut stdout = stdout();

  enable_raw_mode();

  let mut guesses = Vec::new();

  let mut did_win = false;
  for attempt in 1..7 {
    write_guess(&mut stdout, &init_guess(5));
    let guessed_word = read_word(&mut stdout, 5)?;
    let color_vec = build_color_vec(&guessed_word, &word);
    write_colored_guess(&mut stdout, &guessed_word, &color_vec);

    guesses.push(color_vec);

    if guessed_word.join("") == word {
      disable_raw_mode();
      println!("\n🎉 🥳 🎆 !!! Congratulations, you won !!! 🎆 🥳 🎉");
      println!("{} {}/6 attempts", word_index, attempt);
      did_win = true;
      break;
    }
  }
  if !did_win {
    disable_raw_mode();
    println!("\nTry not being so bad next time, ya dunce 🙃");
    println!("Wordle {} was: {}", word_index, word);
  }

  for guess in guesses {
    println!(
      "{}",
      guess
        .into_iter()
        .map(|color| match color {
          LETTER_COLOR::GREEN => "🟩",
          LETTER_COLOR::YELLOW => "🟨",
          LETTER_COLOR::GRAY => "⬛️",
        })
        .collect::<Vec<&str>>()
        .join("")
    );
  }

  Ok(0)
}

fn get_word_by_date() -> (usize, &'static str) {
  let date = Local::now()
    .offset()
    .from_local_date(&NaiveDate::from_ymd(2021, 6, 20))
    .single()
    .unwrap();
  let date_now = Local::now().date();
  let days_since_start = date_now.signed_duration_since(date).num_days() as usize;
  (days_since_start, words::word_set[days_since_start])
}

fn get_random_word() -> (usize, &'static str) {
  let word_index = words::word_set.len();
  (
    word_index,
    words::word_set[thread_rng().gen_range(0..word_index)],
  )
}

fn main() {
  let args: Vec<String> = env::args().collect();
  let (word_index, word) = if args.len() > 1 && args[1] == "-r" {
    get_random_word()
  } else {
    get_word_by_date()
  };

  input_loop(word_index, word);
}

// if  a win, print out the share emojis
