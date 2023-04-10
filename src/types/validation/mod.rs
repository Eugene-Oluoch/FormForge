pub mod checkbox;
pub mod color;
pub mod date;
pub mod datetimelocal;
pub mod email;
pub mod file;
pub mod month;
pub mod number;
pub mod password;
pub mod radio;
pub mod range;
pub mod tel;
pub mod text;
pub mod time;
pub mod url;
pub mod week;
use serde::{Serialize, Deserialize};
use checkbox::CheckBox;
use color::Color;
use date::Date;
use datetimelocal::DateTimeLocal;
use email::Email;
use file::File;
use month::Month;
use number::Number;
use password::Password;
use radio::Radio;
use range::Range;
use tel::Tel;
use text::Text;
use url::Url;
use week::Week;
use time::Time;

#[derive(Serialize, Deserialize, Debug)]
pub enum Validation{
  Checkbox(CheckBox),
  Color(Color),
  Date(Date),
  Datetimelocal(DateTimeLocal),
  Email(Email),
  File(File),
  Month(Month),
  Number(Number),
  Password(Password),
  Radio(Radio),
  Range(Range),
  Tel(Tel),
  Text(Text),
  Url(Url),
  Week(Week),
  Time(Time)
}


impl Validation{
  pub fn map(name:&str)->Option<Validation>{
    match name {
      "color" => Some(Validation::Color(Color::new())),
      "checkbox" => Some(Validation::Checkbox(CheckBox::new())),
      "date" => Some(Validation::Date(Date::new())),
      "email" => Some(Validation::Email(Email::new())),
      "number" => Some(Validation::Number(Number::new())),
      "password" => Some(Validation::Password(Password::new())),
      "range" => Some(Validation::Range(Range::new())),
      "tel" => Some(Validation::Tel(Tel::new())),
      "text" => Some(Validation::Text(Text::new())),
      "time" => Some(Validation::Time(Time::new())),
      "url" => Some(Validation::Url(Url::new())),
      "week" => Some(Validation::Week(Week::new())),
      "file" => Some(Validation::File(File::new())),
      "month" => Some(Validation::Month(Month::new())),
      "datetime-local" => Some(Validation::Datetimelocal(DateTimeLocal::new())),
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

    pub fn into_range(self) -> Range {
      if let Validation::Range(value) = self {
          value
      }else{
        panic!("Text");
      }
    }

    pub fn into_tel(self) -> Tel {
      if let Validation::Tel(value) = self {
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

    pub fn into_time(self) -> Time {
      if let Validation::Time(value) = self {
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

    pub fn into_week(self) -> Week {
      if let Validation::Week(value) = self {
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
