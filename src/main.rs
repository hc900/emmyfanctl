use glob::glob;
use std::{thread, time, fs};
use std::collections::HashMap;
use serde::{Deserialize,Serialize};
use toml;
use std::fs::read;

#[derive(Deserialize,Debug,Serialize)]
struct Sensor {
    path: String,
    name: String,
    min: f64,
    max: f64,
    avg: f64,
}

#[derive(Deserialize,Debug,Serialize)]
struct SensorGroup {
    sensor_span: Option<f64>,
    sensor_avg: Option<f64>,
    sensor_max: Option<f64>,
    CoreTable: HashMap<String,Sensor>,
}

#[derive(Deserialize,Debug,Serialize)]
struct Sensors {
    Sensors: HashMap<String,SensorGroup>,
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
    let mut fan_cfg = config::Config::default();
    let mut cpu_cfg = config::Config::default();
    build_config(&mut fan_cfg, "/etc/macfanctld/*fans*");
    build_config(&mut cpu_cfg, "/etc/macfanctld/*cpus*");
    //load settings

    let fans_cfg= fan_cfg.try_into::<Fans>();
    let cpus_cfg = cpu_cfg.try_into::<SensorGroup>();
    let my_fans = get_fans_from_fanconfig(fans_cfg);
    //let my_sensors = get_sensors_from_cpuconfig(cpus_cfg);
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


fn get_sensors_from_cpuconfig(sensors_cfg: Result<Sensors, config::ConfigError>) -> SensorGroup {
    let my_sensors =
    match sensors_cfg {
        Ok(sensors) => {
            let mut sensors_table = SensorGroup {
                sensor_span: None,
                sensor_avg: None,
                sensor_max: None,
                CoreTable: Default::default()
            };
            for (name, mut sensor) in sensors.Sensors {
                //sensors_table.CoreTable.insert(name, sensor);
            }
            //return fans
            sensors_table
        }
        Err(e) => {
            println!("{}", e);
            SensorGroup {
                sensor_span: None,
                sensor_avg: None,
                sensor_max: None,
                CoreTable: Default::default()
            }
        },
    };
    my_sensors
}



fn get_fans_from_fanconfig(fans_cfg: Result<Fans, config::ConfigError>) -> Fans {
    let my_fans = match fans_cfg {
        Ok(fans) => {
            let mut fans_table = Fans {
                Fans: Default::default()
            };
            for (name, mut fan) in fans.Fans {
                fan.max = match fan.max {
                    Some(t) => Some(t),
                    None => {
                        //open file
                        let fan_max = format!("{}max", fan.path);
                        //read file
                        let read_val = fs::read_to_string(fan_max).expect("Problem reading for max speed");
                        //set max
                        Some(read_val.trim().parse::<u16>().unwrap_or(500))
                    },
                };
                fans_table.Fans.insert(name, fan);
            }
            //return fans
            fans_table
        },
        Err(e) => {
            println!("{}", e);
            Fans {
                Fans: Default::default()
            }
        },
    };
    my_fans
}

fn build_config(cnf: &mut config::Config, path: &str) {
    for path in glob(path).expect("Failed to load fans config paths")
    {
        //load settings
        match path {
            Ok(t) =>
                {
                    match cnf.merge(config::File::with_name(t.to_str().unwrap()))
                    {
                        Ok(_t) => println!("Loaded config file!"),
                        Err(_e) => println!("Failed to load {}",_e),
                    }
                },
            Err(_e) => println!(""),
        }
    }
}
