mod utils;

use rand::{
  distributions::{Distribution, Standard},
  Rng,
};
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
  fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
  alert("Hello, {{project-name}}!");
}
use std::time::Duration;

use async_std::task::sleep;
use web_sys::{Document, Element, Window};

pub struct GameElements {
  pub green_button: Element,
  pub red_button: Element,
  pub yellow_button: Element,
  pub blue_button: Element,
  pub score: Element,
}

impl GameElements {
  pub fn new(document: &Document) -> Self {
    Self {
      green_button: document.get_element_by_id("green_button").unwrap(),
      red_button: document.get_element_by_id("red_button").unwrap(),
      yellow_button: document.get_element_by_id("yellow_button").unwrap(),
      blue_button: document.get_element_by_id("blue_button").unwrap(),
      score: document.get_element_by_id("user_score").unwrap(),
    }
  }
}

trait Active {
  fn set_active(&self);
  fn set_inactive(&self);
}

impl Active for Element {
  fn set_active(&self) {
    self.set_class_name(&format!("{} {}", self.class_name(), "active"));
  }
  fn set_inactive(&self) {
    self.set_class_name(&self.class_name().replace(" active", ""));
  }
}

enum ChosenButton {
  GreenButton,
  RedButton,
  YellowButton,
  BlueButton,
}

impl Distribution<ChosenButton> for Standard {
  fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> ChosenButton {
    // match rng.gen_range(0, 3) { // rand 0.5, 0.6, 0.7
    match rng.gen_range(0..=3) {
      // rand 0.8
      0 => ChosenButton::GreenButton,
      1 => ChosenButton::RedButton,
      2 => ChosenButton::YellowButton,
      _ => ChosenButton::BlueButton,
    }
  }
}

#[wasm_bindgen]
pub async fn run_simon() {
  let window = web_sys::window().expect("global window does not exists");
  let document = window.document().expect("expecting a document on window");
  let mut running: bool = true;
  let mut color_sequence: Vec<ChosenButton> = Vec::new();

  let game_elements = GameElements::new(&document);
  game_elements
    .score
    .set_text_content(Some(&format!("High Score: {}", &color_sequence.len())));

  run_game_intro(&game_elements).await;

  while running {
    color_sequence.push(rand::random());
    for curr_color in &color_sequence {
      match curr_color {
        ChosenButton::GreenButton => {
          game_elements.green_button.set_active();
          sleep(Duration::from_millis(500)).await;
          game_elements.green_button.set_inactive();
        }
        ChosenButton::RedButton => {
          game_elements.red_button.set_active();
          sleep(Duration::from_millis(500)).await;
          game_elements.red_button.set_inactive();
        }
        ChosenButton::YellowButton => {
          game_elements.yellow_button.set_active();
          sleep(Duration::from_millis(500)).await;
          game_elements.yellow_button.set_inactive();
        }
        ChosenButton::BlueButton => {
          game_elements.blue_button.set_active();
          sleep(Duration::from_millis(500)).await;
          game_elements.blue_button.set_inactive();

          running = false;
        }
      }
    }
  }
}

pub async fn run_game_intro(game_elements: &GameElements) {
  game_elements.green_button.set_active();
  sleep(Duration::from_millis(500)).await;

  game_elements.green_button.set_inactive();
  game_elements.red_button.set_active();
  sleep(Duration::from_millis(500)).await;

  game_elements.red_button.set_inactive();
  game_elements.yellow_button.set_active();
  sleep(Duration::from_millis(500)).await;

  game_elements.yellow_button.set_inactive();
  game_elements.blue_button.set_active();
  sleep(Duration::from_millis(500)).await;

  game_elements.blue_button.set_inactive();
  sleep(Duration::from_millis(500)).await;

  set_all_active_then_reset(game_elements).await;
  set_all_active_then_reset(game_elements).await;
  set_all_active_then_reset(game_elements).await;
}

pub async fn set_all_active_then_reset(game_elements: &GameElements) {
  game_elements.green_button.set_active();
  game_elements.red_button.set_active();
  game_elements.yellow_button.set_active();
  game_elements.blue_button.set_active();
  sleep(Duration::from_millis(500)).await;

  game_elements.green_button.set_inactive();
  game_elements.red_button.set_inactive();
  game_elements.yellow_button.set_inactive();
  game_elements.blue_button.set_inactive();
  sleep(Duration::from_millis(500)).await;
}
