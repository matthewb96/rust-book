use std::io;

mod temperature_data {
    use std::fmt;

    #[derive(Debug, Clone, Copy)]
    pub enum Units {
        Celsius,
        Farenheit,
        Kelvin,
    }

    impl Units {
        pub const VALUES: [Self; 3] = [Self::Celsius, Self::Farenheit, Self::Kelvin];

        pub fn parse(text: &str) -> Result<Units, String> {
            let text = text.trim();
            match &text.to_uppercase()[..] {
                "C" | "°C" | "CELSIUS" => Ok(Units::Celsius),
                "F" | "°F" | "FARENHEIT" => Ok(Units::Farenheit),
                "K" | "KELVIN" => Ok(Units::Kelvin),
                _ => Err(format!(
                    "invalid temperature units: \'{}\', should be one of {}",
                    text,
                    Self::units().join(", ")
                )),
            }
        }

        fn units() -> Vec<String> {
            Self::VALUES.iter().map(|x| format!("'{}'", x)).collect()
        }
    }

    impl fmt::Display for Units {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let unit_str = match &self {
                Units::Celsius => "°C",
                Units::Farenheit => "°F",
                Units::Kelvin => "K",
            };
            write!(f, "{}", unit_str)
        }
    }

    #[derive(Debug)]
    pub struct Temperature {
        value: f64,
        units: Units,
    }

    impl fmt::Display for Temperature {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let precision = f.precision().unwrap_or(5);
            write!(f, "{:.*?}{}", precision, self.value, self.units)
        }
    }

    impl Temperature {
        pub fn new(value: f64, units: Units) -> Result<Temperature, String> {
            if value < Temperature::min_value(&units) {
                return Err(format!(
                    "invalid temperature: {}{}, must be >= {:.2}",
                    value,
                    units,
                    Temperature::min(units)
                ));
            }

            Ok(Temperature { value, units })
        }

        fn min_value(units: &Units) -> f64 {
            match units {
                Units::Kelvin => 0.0,
                Units::Celsius => -273.15,
                Units::Farenheit => -459.67,
            }
        }

        pub fn min(units: Units) -> Temperature {
            Temperature {
                value: Temperature::min_value(&units),
                units,
            }
        }

        pub fn value(&self) -> f64 {
            self.value
        }

        pub fn units(&self) -> &Units {
            &self.units
        }

        pub fn to_kelvin(&self) -> Temperature {
            let value = match self.units {
                Units::Kelvin => self.value,
                Units::Celsius => celsius_to_kelvin(self.value),
                Units::Farenheit => {
                    let value = farenheit_to_celsius(self.value);
                    celsius_to_kelvin(value)
                }
            };

            Temperature {
                value,
                units: Units::Kelvin,
            }
        }

        pub fn to_celsius(&self) -> Temperature {
            let value = match self.units {
                Units::Celsius => self.value,
                Units::Kelvin => kelvin_to_celsius(self.value),
                Units::Farenheit => farenheit_to_celsius(self.value),
            };

            Temperature {
                value,
                units: Units::Celsius,
            }
        }

        pub fn to_farenheit(&self) -> Temperature {
            let value = match self.units {
                Units::Farenheit => self.value,
                Units::Celsius => celsius_to_farenheit(self.value),
                Units::Kelvin => {
                    let value = kelvin_to_celsius(self.value);
                    celsius_to_farenheit(value)
                }
            };

            Temperature {
                value,
                units: Units::Farenheit,
            }
        }
    }

    fn celsius_to_farenheit(temperature: f64) -> f64 {
        (temperature * (9.0 / 5.0)) + 32.0
    }

    fn farenheit_to_celsius(temperature: f64) -> f64 {
        (temperature - 32.0) * (5.0 / 9.0)
    }

    fn celsius_to_kelvin(temperature: f64) -> f64 {
        temperature + 273.15
    }

    fn kelvin_to_celsius(temperature: f64) -> f64 {
        temperature - 273.15
    }
}

pub use temperature_data::{Temperature, Units};

fn main() -> Result<(), String> {
    let temperature = temperature_input()?;

    println!("Input temperature: {:.1}", temperature);
    println!("Temperature in kelvin: {:.1}", temperature.to_kelvin());
    println!("Temperature in celsius: {:.1}", temperature.to_celsius());
    println!(
        "Temperature in farenheit: {:.1}",
        temperature.to_farenheit()
    );

    println!("Minimum temperatures:");
    for units in Units::VALUES.iter() {
        println!("\t{:.2}", Temperature::min(*units));
    }

    Ok(())
}

fn temperature_input() -> Result<Temperature, String> {
    let mut temperature = String::new();
    println!("Please input temperature");
    io::stdin()
        .read_line(&mut temperature)
        .expect("Failed to read temperature");

    let temperature: f64 = temperature
        .trim()
        .parse()
        .expect("Temperature is not a number!");

    let mut units = String::new();
    println!("Please input units");
    io::stdin()
        .read_line(&mut units)
        .expect("Failed to read units");

    let units = Units::parse(&units)?;

    Temperature::new(temperature, units)
}
