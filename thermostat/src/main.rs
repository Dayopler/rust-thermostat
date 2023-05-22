

fn main(){

    println!("Welcome to Thermostat !");

    
    struct Mode<'a> {
        opened :  &'a str,
        on: &'a str,
        off: &'a  str
    };
    
    let mode:Mode = Mode {
        opened: "opened",
        on: "on",
        off: "off",
    };
    
    // Target value desired by the owner
    const TARGET_VALUE: f64 = 20.0;

    // Mode of radiators
    let mut radiatior_mode: &'static str = mode.off;

    // The current value returned by the radiator
    let mut current_value:f64 = 20.1;

    // State of window
    let mut opened_window: bool = true;
    
    
    
    loop {

    if radiatior_mode == mode.on {
        println!("Radiators ON ! \nValue : {}", current_value);

        if  current_value >= TARGET_VALUE + 0.4 {
            radiatior_mode = mode.off
        }
        current_value += 0.1;
        wait()

    } else if radiatior_mode == mode.off {
        println!("Radiators OFF !\nValue : {}", current_value);

        if current_value <= TARGET_VALUE - 0.4 {
            radiatior_mode = mode.on;
        } ;
        current_value -= 0.1;

        wait()

    } else if radiatior_mode == mode.opened {
        println!("Windows Opened \nValue : {}", current_value);

        radiatior_mode = mode.off;


        wait()
    }


    // condition if there are window sensors   
    // don't forget to swith opened_window to false
    if opened_window {
        radiatior_mode = mode.opened;
    }
} 


}

fn wait() {
    // Wait 2 seconds
    std::thread::sleep(std::time::Duration::from_secs(2));
}
