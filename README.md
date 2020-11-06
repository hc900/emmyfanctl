# emmyfanctl

## A rust daemon for controlling outputs based on inputs.

### emmyfanctl is a daemon designed to replace macfanctld. 

macfanctld seems to have it's development stopped, and out of the box does not work with my iMac 2010 / Macbook Air 2013. 

I went through the code and found it didn't behave the way I expected, nor was it as configurable as I needed to keep my macs cool, and thus the project was born.

Originally written in python, it worked great when spawned in a tmux session as root. Getting it to be a daemon for startup was going to be a hassle (in my eyes). I decided to give Rust a go, and I'm learning as I code emmyfanctl.

### Use

So far, emmyfanctl requires a directory `/etc/emmyfanctl` where you place config files. The files are overrideable by having them appear later in order.

Fan files need to be named in the following format `*fans*.ext` (where ext is toml, yaml, etc).

Sensor files need to be named in the following format `*cpu*.ext` (where ext is toml, yaml, etc).

The program accepts __toml__ format currently. yaml support will be coming shortly.

### Examples

Example "fans" file:

```

[Fans] # required
[Fans.ODD] #at least one fan must be defined
min = 1100 #required
max = 2000 #optional
path = "/sys/devices/platform/applesmc.768/fan1_"
[Fans.HDD]
min = 1100
path = "/sys/devices/platform/applesmc.768/fan2_"
[Fans.CPU]
min = 950
path = "/sys/devices/platform/applesmc.768/fan3_"

```

This example defines three fans, named ODD, HDD, CPU, specifies the hard minimum speed for which you want to run the fans. You can force the minimum to be higher than the actual minimum defined by the system.
The max field is the cap for which you want to run the fan. It is optional if the max fan speed is defined by the system.
__The path does not allow for wild cards.__

__Currently, the program assumes that fans follow the apple format of fan[0-9]*_max for detecting max, fan[0-9]*_min for writing to change the speed, fan[0-9]*_manual for writing a 0 to enable manual speed control.__

Every attempt to keep this as a default behavior will be made in future versions, with it being configurable via the config file.

Example "sensor" file:

```

[Sensors] #required
[Sensors.GroupA] #required
        Fans = ["HDD","CPU","ODD"] #required
        [Sensors.GroupA.Table.CPU1] #at least one table entry is required
                path = "/sys/devices/platform/coretemp.*/hwmon/hwmon*/temp*_input" #glob grouping
                name = "ALLCORES" #name you want to call it
                min = 39.0   # required
                max = 59.0   # required
                avg = 46.0   # required
                div = 1000.0 #optional divisor
                
[Sensors.GroupB]
        Fans = ["HDDB","CPUB","ODDB"]
        [Sensors.GroupB.Table.CPU1]
                path = "/some/path/cpu1/cpu1_input"
                name = "CPU1"
                min = 39.0
                max = 59.0
                avg = 46.0
        [Sensors.GroupB.CoreTable.CPU2]
                path = "/some/path/cpu2/cpu2_core*_input"
                name = "CPU2"
                min = 39.0
                max = 59.0
                avg = 46.0

```




In this example, I define two groups of sensors.

Sensor "GroupA" controls fans HDD, CPU, ODD based on all files located in the wildcard directories that match temp*_input.
GroupA defines the minimum temp to be 39.0 and the max to be 59.0. If all cores are at or below 39.0, it will set the fan speeds for HDD, CPU, ODD to  the minimum speed defined for them.

If the temperature of any core is greater than or equal to the max, then the fans will be driven to max rpms defined.
The average is for future use.
The div is an optional field, that is the divisor for making the read temperatures fit the format desired, eg 49000 read from a temperature will be divided by 1000, and the result will be 49.00.

Sensor "GroupB" controls HDDB, CPUB, ODDB fans.
It defines two distinct cpus, one with a explicit path, the other with a wildcard glob. They do not require a divisor.


