use glob::glob;
use std::{thread,time};
use std::collections::HashMap;
use serde::{Deserialize,Serialize};
use toml;

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
    match fans_cfg {
        Ok(fans) => {
            for (name,fan) in fans.Fans{
                println!("{}",name);
            }
        },
        Err(e) => println!("{}",e),
    }

    //let fans = .try_into::<HashMap<String,Fan>>();
    //loop
        //check temps
        //average temps
        //set new temps
        //sleep

    let time = time::Duration::from_secs(5);
    loop {

        thread::sleep(time);
    }

}
