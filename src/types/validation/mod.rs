pub mod checkbox;
pub mod color;
pub mod date;
pub mod email;
pub mod file;
pub mod month;
pub mod password;
pub mod radio;
pub mod text;
pub mod url;
use serde::{Serialize, Deserialize};
use checkbox::CheckBox;
use color::Color;
use date::Date;
use email::Email;
use file::File;
use month::Month;
use password::Password;
use radio::Radio;
use text::Text;
use url::Url;


/*
Validation that share similar fields
* Date , Datetimelocal
* Month, Number, Range, Time, Week
* Tel, Url


*/

#[derive(Serialize, Deserialize, Debug)]
pub enum Validation{
  Checkbox(CheckBox),
  Color(Color),
  Date(Date),
  Datetimelocal(Date),
  Email(Email),
  File(File),
  Month(Month),
  Number(Month),
  Password(Password),
  Radio(Radio),
  Range(Month),
  Tel(Url),
  Text(Text),
  Url(Url),
  Week(Month),
  Time(Month)
}


impl Validation{
  pub fn map(name:&str)->Option<Validation>{
    match name {
      "color" => Some(Validation::Color(Color::new())),
      "checkbox" => Some(Validation::Checkbox(CheckBox::new())),
      "date" => Some(Validation::Date(Date::new())),
      "email" => Some(Validation::Email(Email::new())),
      "number" => Some(Validation::Number(Month::new())),
      "password" => Some(Validation::Password(Password::new())),
      "range" => Some(Validation::Range(Month::new())),
      "tel" => Some(Validation::Tel(Url::new())),
      "text" => Some(Validation::Text(Text::new())),
      "time" => Some(Validation::Time(Month::new())),
      "url" => Some(Validation::Url(Url::new())),
      "week" => Some(Validation::Week(Month::new())),
      "file" => Some(Validation::File(File::new())),
      "month" => Some(Validation::Month(Month::new())),
      "datetime-local" => Some(Validation::Datetimelocal(Date::new())),
      _=>None
    }
    }


    pub fn into_color(self) -> Color {
      if let Validation::Color(value) = self {
          value
      }else{
        panic!("Text");
      }
    }

    pub fn into_checkbox(self) -> CheckBox {
      if let Validation::Checkbox(value) = self {
          value
      }else{
        panic!("Text");
      }
    }

    pub fn into_date(self) -> Date {
      if let Validation::Date(value) = self {
          value
      }else{
        panic!("Text");
      }
    }

    pub fn into_email(self) -> Email {
      if let Validation::Email(value) = self {
          value
      }else{
        panic!("Text");
      }
    }

    pub fn into_number(self) -> Password {
      if let Validation::Password(value) = self {
          value
      }else{
        panic!("Text");
      }
    }


    pub fn into_passowrd(self) -> Password {
      if let Validation::Password(value) = self {
          value
      }else{
        panic!("Text");
      }
    }

    pub fn into_range(self) -> Month {
      if let Validation::Month(value) = self {
          value
      }else{
        panic!("Text");
      }
    }

    pub fn into_tel(self) -> Url {
      if let Validation::Url(value) = self {
          value
      }else{
        panic!("Text");
      }
    }

    pub fn into_text(self) -> Text {
      if let Validation::Text(value) = self {
          value
      }else{
        panic!("Text");
      }
    }

    pub fn into_time(self) -> Month {
      if let Validation::Month(value) = self {
          value
      }else{
        panic!("Text");
      }
    }

    pub fn into_url(self) -> Url {
      if let Validation::Url(value) = self {
          value
      }else{
        panic!("Text");
      }
    }

    pub fn into_week(self) -> Month {
      if let Validation::Month(value) = self {
          value
      }else{
        panic!("Text");
      }
    }

    pub fn into_file(self) -> File {
      if let Validation::File(value) = self {
          value
      }else{
        panic!("Text");
      }
    }


    pub fn into_month(self) -> Month {
      if let Validation::Month(value) = self {
          value
      }else{
        panic!("Text");
      }
    }


  }
