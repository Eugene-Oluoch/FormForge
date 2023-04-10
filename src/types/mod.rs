pub mod validation;
use serde::{Serialize, Deserialize};

// NOTE THIS IMPLEMENTATION ISN'T THE FINAL ONE -> TESTING PURPOSES




#[derive(Serialize, Deserialize, Debug,PartialEq)]
pub enum Types{
  Color,
  Date,
  Email,
  Tel,
  Text,
  Time,
  Url,
  Week,
  Range,
  Password,
  Number,
  Checkbox,
  DatetimeLocal,
  File,
  Month
}

impl Types {
  pub fn map(name:&str) -> Option<Types>{
    match name{
      "color" => Some(Types::Color),
      "checkbox" => Some(Types::Checkbox),
      "date" => Some(Types::Date),
      "email" => Some(Types::Email),
      "number" => Some(Types::Number),
      "password" => Some(Types::Password),
      "range" => Some(Types::Range),
      "tel" => Some(Types::Tel),
      "text" => Some(Types::Text),
      "time" => Some(Types::Time),
      "url" => Some(Types::Url),
      "week" => Some(Types::Week),
      "file" => Some(Types::File),
      "month" => Some(Types::Month),
      "datetime-local" => Some(Types::DatetimeLocal),
      _=>None
    }
  }
}