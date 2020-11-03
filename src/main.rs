use glob::{glob, Paths};
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
    div: Option<f64>,
}

#[derive(Deserialize,Debug,Serialize)]
struct SensorGroup {
    sensor_span: Option<f64>,
    sensor_avg: Option<f64>,
    sensor_max: Option<f64>,
    CoreTable: HashMap<String,Sensor>,
    FanNames: Vec<String>,
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
    build_config(&mut fan_cfg, "/etc/emmyfanctld/*fans*");
    build_config(&mut cpu_cfg, "/etc/emmyfanctld/*cpus*");
    //load settings

    let fans_cfg= fan_cfg.try_into::<Fans>();
    let cpus_cfg = cpu_cfg.try_into::<Sensors>();
    let my_fans = get_fans_from_fanconfig(fans_cfg);
    let my_sensors = get_sensors_from_cpuconfig(cpus_cfg);
    if my_fans.Fans.is_empty()
    {
        println!("Could not locate any fans in config files!");
        return;
    }

    if my_sensors.Sensors.is_empty()
    {
        println!("No Sensors are defined :(");
    }

    let time = time::Duration::from_secs(5);
    loop {
        for (key,value) in &my_sensors.Sensors
        {
            println!("Key {}",key);
            for (name,sensor) in &value.CoreTable
            {
                println!("reading {} at {}.",name,sensor.path);
                let mut file_list = glob(sensor.path.as_str()).unwrap();
                let mut count = 0;
                let mut min: f64 = 9999.0;
                let mut max: f64 = -100.0;
                let mut sum: f64 = 0.0;
                for file in file_list
                {
                    match file {
                        Ok(file_path) => sum = calculator_sensor_sum(sensor, &mut count, &mut sum, file_path),
                        Err(e) => {
                            println!("Couldn't unwrap! {}",e);
                        }
                    }
                } //end loop
                println!("Sum is {}, avg is {}",sum, sum / count as f64);
            }

        }
        thread::sleep(time);
    }

}

fn calculator_sensor_sum(sensor: &Sensor, mut count: &mut i32, sum: &f64, file_path: std::path::PathBuf) -> f64{
    println!("Reading {}", file_path.to_str().unwrap());
    let mut value_read = fs::read_to_string(file_path.to_str().unwrap()).unwrap();
    let value_read_float = get_float_from_string(&mut count, &mut value_read);
    let divisor = match sensor.div {
        Some(div) => div,
        None => 1.0f64,
    };
    let val = sum + (value_read_float / divisor);
    val
}

fn get_float_from_string(count: &mut i32, value_read: &mut String) -> f64 {
    let value_read_float: f64 = match value_read.trim().parse::<f64>() {
        Ok(tt) => {
            *count = *count + 1;
            tt
        },
        Err(ee) => {
            println!("Failed to parse: {}", ee);
            0.0f64
        },
    };
    value_read_float
}


fn get_sensors_from_cpuconfig(sensors_cfg: Result<Sensors, config::ConfigError>) -> Sensors {
    let my_sensors =
    match sensors_cfg {
        Ok(sensors) => {
            sensors
        }
        Err(e) => {
            println!("{}", e);
            Sensors {
                Sensors: Default::default()
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
