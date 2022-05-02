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

#[no_mangle]
pub async extern "C" fn run_simon() {
  let window = web_sys::window().expect("global window does not exists");
  let document = window.document().expect("expecting a document on window");

  let game_elements = GameElements::new(&document);

  run_game_intro(&game_elements).await;
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
