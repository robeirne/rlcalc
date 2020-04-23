pub use iced::{
    Application,
    Background,
    Button,
    Color,
    Column,
    Command,
    Element,
    Length,
    Radio,
    Row,
    Settings,
    Slider,
    Space,
    Text,
    TextInput,
    button,
    executor,
    slider,
    text_input,
};

use crate::*;

#[derive(Default, Debug, Clone)]
pub struct RollCalc {
    roll: Roll,
    core_input: text_input::State,
    core_input_value: String,
    roll_input: text_input::State,
    roll_input_value: String,
    thick_input: text_input::State,
    thick_input_value: String,
}

#[derive(Debug, Clone)]
pub enum RollMessage {
    SetCore(f32),
    TrySetCore(String),
    SetRoll(f32),
    TrySetRoll(String),
    SetThick(f32),
    TrySetThick(String),
    ChangeUnits(Units),
}

impl Application for RollCalc {
    type Executor = executor::Default;
    type Flags = clap::ArgMatches<'static>;
    type Message = RollMessage;

    fn title(&self) -> String {
        String::from("Roll Length Calculator")
    }

    fn new(flags: Self::Flags) -> (Self, Command<Self::Message>) {
        let roll_calc = RollCalc {
            roll: Roll::default().convert(
                flags.value_of("units")
                    .expect("Units are required")
                    .parse()
                    .expect("Unable to parse units")
            ),
            ..Default::default()
        };

        (roll_calc, Command::none())
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            RollMessage::SetCore(x) => self.roll.set_coreod(x as f64),
            RollMessage::TrySetCore(s) => {
                if let Ok(size) = s.parse::<Size>() {
                    self.roll.convert_mut(size.units());
                    self.roll.set_coreod(size.value());
                } else if let Ok(f) = s.parse::<f64>() {
                    if f > 0.0 {
                        self.roll.set_coreod(f);
                    }
                }
                self.core_input_value = s;
            }
            RollMessage::SetRoll(x) => self.roll.set_rollod(x as f64),
            RollMessage::TrySetRoll(s) => {
                if let Ok(size) = s.parse::<Size>() {
                    self.roll.convert_mut(size.units());
                    self.roll.set_rollod(size.value());
                } else if let Ok(f) = s.parse::<f64>() {
                    if f > 0.0 {
                        self.roll.set_rollod(f);
                    }
                }
                self.roll_input_value = s;
            }
            RollMessage::SetThick(x) => self.roll.set_thickness(x as f64),
            RollMessage::TrySetThick(s) => {
                if let Ok(size) = s.parse::<Size>() {
                    self.roll.convert_mut(size.units());
                    self.roll.set_thickness(size.value());
                } else if let Ok(f) = s.parse::<f64>() {
                    if f > 0.0 {
                        self.roll.set_thickness(f);
                    }
                }
                self.thick_input_value = s;
            }
            RollMessage::ChangeUnits(units) => self.roll.convert_mut(units),
        }
        Command::none()
    }

    fn view(&mut self) -> Element<Self::Message> {
        Column::new()
            .push(Text::new(self.title()))
            .push(Space::new(Length::Units(0), Length::Units(30)))
            // Core OD
            .push(
                Row::new()
                    .push(
                        TextInput::new(
                            &mut self.core_input,
                            "Core OD",
                            &self.core_input_value,
                            RollMessage::TrySetCore,
                        )
                        .padding(1)
                        .width(Length::Units(100)),
                    )
                    .push(Text::new(format!(" Core OD: {}", self.roll.coreod()))),
            )
            // Roll OD
            .push(
                Row::new()
                    .push(
                        TextInput::new(
                            &mut self.roll_input,
                            "Roll OD",
                            &self.roll_input_value,
                            RollMessage::TrySetRoll,
                        )
                        .padding(1)
                        .width(Length::Units(100)),
                    )
                    .push(Text::new(format!(" Roll OD: {}", self.roll.rollod()))),
            )
            // Thickness
            .push(
                Row::new()
                    .push(
                        TextInput::new(
                            &mut self.thick_input,
                            "Thickness",
                            &self.thick_input_value,
                            RollMessage::TrySetThick,
                        )
                        .padding(1)
                        .width(Length::Units(100)),
                    )
                    .push(Text::new(format!(" Thickness: {}", self.roll.thickness()))),
            )
            .push(Space::new(Length::Units(0), Length::Units(30)))
            // Roll Length Calculation
            .push({
                let len = self.roll.length();
                Text::new(format!(
                    "Roll Length: {:.2}{}",
                    len.value(),
                    len.units().unit_suffix()
                ))
            })
            .push(Space::new(Length::Units(0), Length::Units(30)))
            // Units
            .push(Text::new("Units:"))
            .push(Radio::new(
                Units::Mils,
                "Mils",
                Some(*self.roll.units()),
                RollMessage::ChangeUnits,
            ))
            .push(Radio::new(
                Units::Inches,
                "Inches",
                Some(*self.roll.units()),
                RollMessage::ChangeUnits,
            ))
            .push(Radio::new(
                Units::Feet,
                "Feet",
                Some(*self.roll.units()),
                RollMessage::ChangeUnits,
            ))
            .push(Radio::new(
                Units::Yards,
                "Yards",
                Some(*self.roll.units()),
                RollMessage::ChangeUnits,
            ))
            .push(Radio::new(
                Units::Milimeters,
                "Milimeters",
                Some(*self.roll.units()),
                RollMessage::ChangeUnits,
            ))
            .push(Radio::new(
                Units::Centimeters,
                "Centimeters",
                Some(*self.roll.units()),
                RollMessage::ChangeUnits,
            ))
            .push(Radio::new(
                Units::Meters,
                "Meters",
                Some(*self.roll.units()),
                RollMessage::ChangeUnits,
            ))
            .into()
    }
}
