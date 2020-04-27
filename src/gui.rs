pub use iced::{
    button, executor, slider, text_input, window::Settings as WinSettings, Application, Background,
    Button, Color, Column, Command, Element, Length, Radio, Row, Settings, Slider, Space, Text,
    TextInput,
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

impl RollCalc {
    fn units(&self) -> &Units {
        &self.roll.units()
    }

    fn set_core(&mut self, size: Size) {
        if size.value() <= 0.0 {
            return;
        }
        self.roll.convert_mut(size.units());
        self.roll.set_coreod(size.value());
        self.core_input_value = format!("{:.4}", size.value());
    }

    fn try_set_core(&mut self, s: String) {
        if let Ok(size) = s.parse::<Size>() {
            self.set_core(size);
        } else if let Ok(f) = s.parse::<f64>() {
            self.set_core(self.units().size(f))
        }
        self.core_input_value = s;
    }

    fn set_roll(&mut self, size: Size) {
        if size.value() <= 0.0 {
            return;
        }
        self.roll.convert_mut(size.units());
        self.roll.set_rollod(size.value());
        self.roll_input_value = size.value().to_string();
    }

    fn try_set_roll(&mut self, s: String) {
        if let Ok(size) = s.parse::<Size>() {
            self.set_roll(size);
        } else if let Ok(f) = s.parse::<f64>() {
            self.set_roll(self.units().size(f))
        }
        self.roll_input_value = s;
    }

    fn set_thick(&mut self, size: Size) {
        if size.value() <= 0.0 {
            return;
        }
        self.roll.convert_mut(size.units());
        self.roll.set_thickness(size.value());
        self.thick_input_value = format!("{:.4}", size.value());
    }

    fn try_set_thick(&mut self, s: String) {
        if let Ok(size) = s.parse::<Size>() {
            self.set_thick(size);
        } else if let Ok(f) = s.parse::<f64>() {
            self.set_thick(self.units().size(f))
        }
        self.thick_input_value = s;
    }

    fn set_units(&mut self, units: Units) {
        self.roll.convert_mut(units);
        self.roll_input_value = format!("{:.4}", self.roll.rollod().value());
        self.core_input_value = format!("{:.4}", self.roll.coreod().value());
        self.thick_input_value = format!("{:.4}", self.roll.thickness().value());
    }
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
        let roll = Roll::default().convert(
            flags
                .value_of("units")
                .expect("Units are required")
                .parse()
                .expect("Unable to parse units"),
        );

        let roll_calc = RollCalc {
            core_input_value: roll.coreod().value().to_string(),
            roll_input_value: roll.rollod().value().to_string(),
            thick_input_value: roll.thickness().value().to_string(),
            roll,
            ..Default::default()
        };

        (roll_calc, Command::none())
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            RollMessage::SetCore(x) => self.set_core(self.units().size(x as f64)),
            RollMessage::TrySetCore(s) => self.try_set_core(s),
            RollMessage::SetRoll(x) => self.roll.set_rollod(x as f64),
            RollMessage::TrySetRoll(s) => self.try_set_roll(s),
            RollMessage::SetThick(x) => self.roll.set_thickness(x as f64),
            RollMessage::TrySetThick(s) => self.try_set_thick(s),
            RollMessage::ChangeUnits(units) => self.set_units(units),
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
                    .spacing(10)
                    .push(
                        TextInput::new(
                            &mut self.core_input,
                            "Core OD",
                            &self.core_input_value,
                            RollMessage::TrySetCore,
                        )
                        .padding(1)
                        .width(Length::Units(200)),
                    )
                    .push(Text::new("CoreOD:"))
                    .push(Text::new(format!(
                        "{:.4}{}",
                        self.roll.coreod().value(),
                        self.roll.units().unit_suffix()
                    ))),
            )
            // Roll OD
            .push(
                Row::new()
                    .spacing(10)
                    .push(
                        TextInput::new(
                            &mut self.roll_input,
                            "Roll OD",
                            &self.roll_input_value,
                            RollMessage::TrySetRoll,
                        )
                        .padding(1)
                        .width(Length::Units(200)),
                    )
                    .push(Text::new("RollOD:"))
                    .push(Text::new(format!(
                        "{:.4}{}",
                        self.roll.rollod().value(),
                        self.roll.units().unit_suffix()
                    ))),
            )
            // Thickness
            .push(
                Row::new()
                    .spacing(10)
                    .push(
                        TextInput::new(
                            &mut self.thick_input,
                            "Thickness",
                            &self.thick_input_value,
                            RollMessage::TrySetThick,
                        )
                        .padding(1)
                        .width(Length::Units(200)),
                    )
                    .push(Text::new("Thickness:"))
                    .push(Text::new(format!(
                        "{:.4}{}",
                        self.roll.thickness().value(),
                        self.roll.units().unit_suffix()
                    ))),
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
            .push(
                Column::new()
                    .push(Text::new("Imperial:"))
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
                    )),
            )
            .push(Space::new(Length::Units(0), Length::Units(30)))
            .push(
                Column::new()
                    .push(Text::new("Metric:"))
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
                    )),
            )
            .into()
    }
}
