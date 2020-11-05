use glob::{glob};
use std::{thread, time, fs};
use std::collections::HashMap;
use serde::{Deserialize,Serialize};
use std::fs::File;
use std::io::Read;

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
    table: HashMap<String,Sensor>,
    fans: Vec<String>,
}

#[derive(Deserialize,Debug,Serialize)]
struct Sensors {
    sensors: HashMap<String,SensorGroup>,
}

#[derive(Deserialize,Debug,Serialize)]
struct FanOption {
    min: u16,
    max: Option<u16>,
    path: String,
}

#[derive(Deserialize,Debug,Serialize)]
struct Fans {
    fans: HashMap<String,FanOption>
}

fn main() -> Result<(),std::io::Error>{

    //read config file
    let mut fan_cfg = config::Config::default();
    let mut cpu_cfg = config::Config::default();
    build_config(&mut fan_cfg, "/etc/emmyfanctld/*fans*")?;
    build_config(&mut cpu_cfg, "/etc/emmyfanctld/*cpus*")?;
    //load settings

    let fans_cfg= fan_cfg.try_into::<Fans>();
    let cpus_cfg = cpu_cfg.try_into::<Sensors>();
    let my_fans = get_fans_from_fanconfig(fans_cfg);
    let my_sensors = get_sensors_from_cpuconfig(cpus_cfg);
    if my_fans.fans.is_empty()
    {
        println!("Could not locate any fans in config files!");
        return Err(std::io::Error::last_os_error());
    }

    if my_sensors.sensors.is_empty()
    {
        println!("No Sensors are defined :(");
    }

    let time = time::Duration::from_secs(5);
    loop {
        for (key,value) in &my_sensors.sensors
        {
            println!("Key {}",key);
            for (name,sensor) in &value.table
            {
                println!("reading {} at {}.",name,sensor.path);
                let file_list = glob(sensor.path.as_str()).unwrap();
                let mut count = 0;
                /* min, max, sum for the group */
                let mut min: f64 = 999.0;
                let mut max: f64 = -100.0;
                let mut sum: f64 = 0.0;
                for file in file_list
                {
                    match file {
                        Ok(file_path) => calculator_sensor_sum(sensor, &mut count, &mut sum, file_path, &mut min, &mut max),
                        Err(e) => {
                            println!("Couldn't unwrap! {}",e);
                        }
                    }
                    println!("LOOP Min: {}, Max: {}",min, max);
                } //end loop
                println!("Sum is {}, avg is {}",sum, sum / count as f64);
                println!("Min: {}, Max: {}",min, max);
            }

        }
        thread::sleep(time);
    }
    Ok(())
}

fn calculator_sensor_sum(sensor: &Sensor, mut count: &mut i32, sum: &mut f64, file_path: std::path::PathBuf, min: &mut f64, max: &mut f64) {
    println!("Reading {}", file_path.to_str().unwrap());
    let mut value_read = fs::read_to_string(file_path.to_str().unwrap()).unwrap();
    let value_read_float = get_float_from_string(&mut count, &mut value_read);
    let divisor = match sensor.div {
        Some(div) => div,
        None => 1.0f64,
    };
    let val = (value_read_float / divisor) as f64;
    *sum = *sum + val;

    if val > *max {
        *max = val;
    }
    if val < *min {
        *min = val;
    }

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
                sensors: Default::default()
            }
        },
    };
    my_sensors
}



fn get_fans_from_fanconfig(fans_cfg: Result<Fans, config::ConfigError>) -> Fans {
    let my_fans = match fans_cfg {
        Ok(fans) => {
            let mut fans_table = Fans {
                fans: Default::default()
            };
            for (name, mut fan) in fans.fans {
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
                fans_table.fans.insert(name, fan);
            }
            //return fans
            fans_table
        },
        Err(e) => {
            println!("{}", e);
            Fans {
                fans: Default::default()
            }
        },
    };
    my_fans
}

fn build_config(cnf: &mut config::Config, path: &str) -> Result<u8,std::io::Error>{
    for path in glob(path).expect("Failed to load fans config paths")
    {
        //load settings
        match path {
            Ok(t) =>
                {
                    let mut f = File::open(t.to_str().unwrap())?;
                    let mut buffer = String::new();
                    f.read_to_string(&mut buffer)?;
                    match cnf.merge(config::File::from_str(buffer.to_lowercase().as_str(),config::FileFormat::Toml))
                    {
                        Ok(_t) => println!("Loaded config file!"),
                        Err(_e) => println!("Failed to load {}",_e),
                    }
                },
            Err(_e) => println!(""),
        }
    }
    Ok(0xff)
}
