use glob::glob;
use std::{thread, time, fs};
use std::collections::HashMap;
use serde::{Deserialize,Serialize};
use toml;
use std::fs::read;

struct Sensor {
    path: String,
    name: String,
    min: f64,
    max: f64,
    avg: f64,
}

struct Sensors {
    sensor_span: Option<f64>,
    sensor_avg: Option<f64>,
    sensor_max: Option<f64>,
    CoreTable: HashMap<String,Sensor>,
}

#[derive(Deserialize,Debug,Serialize)]
struct FanOption {
    min: u16,
    max: Option<u16>,
    path: String,
}

struct Fan {
    min: u16,
    max: u16,
    path: String,
}

#[derive(Deserialize,Debug,Serialize)]
struct Fans {
    Fans: HashMap<String,FanOption>
}

fn main() {

    //read config file
    let mut cnf = config::Config::default();
    for path in glob("/etc/macfanctld/*fans*").expect("Failed to load fans config paths")
    {
        //load settings
        match path {
            Ok(t) =>
                {
                    match cnf.merge(config::File::with_name(t.to_str().unwrap()))
                    {
                        Ok(_t) => println!("Loaded config file!"),
                        Err(_e) => println!("Failed to load"),
                    }
                },
            Err(_e) => println!(""),
        }
    }
    //load settings
    //let l = toml::to_string(cnf.get("Fans").unwrap()).unwrap();

    let fans_cfg= cnf.try_into::<Fans>();
    let my_fans = match fans_cfg {
        Ok(fans) => {
            let mut fans_table = Fans{
                Fans: Default::default()
            };
            for (name,mut fan) in fans.Fans{

                fan.max = match fan.max {
                    Some(t) => Some(t),
                    None => {
                        //open file
                        let fan_max = format!("{}max",fan.path);
                        //read file
                        let read_val = fs::read_to_string(fan_max).expect("Problem reading for max speed");
                        //set max
                        Some(read_val.trim().parse::<u16>().unwrap_or(500))

                    },
                };
                fans_table.Fans.insert(name,fan);
            }
             //return fans
            fans_table
        },
        Err(e) => {
            println!("{}",e);
            Fans {
                Fans: Default::default()
            }
        },
    };

    if my_fans.Fans.is_empty()
    {
        println!("Could not locate any fans in config files!");
        return;
    }

    let time = time::Duration::from_secs(5);
    loop {

        thread::sleep(time);
    }

}
