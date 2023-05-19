fn main(){
    // target value chauffage allumé
    // value min chauffage allumé
    // current value
    // fenetre ouverte ?
    // mode actif (allumé, eteint, fenetres ouvertes)
    // si value actuelle, inférieur a value cible && chauffage allumé -> go to value cible
    // si value actuelle >= value cible && chauffage allumé -> eteindre chauffage jusque value min

    println!("Welcome to Thermostat !");

    struct Mode {
        opened : bool,
        on: bool,
        off: bool
    }

    let mut mode = Mode {
        opened: false,
        on: false,
        off: true,
    };

loop {

    if mode.on {
        println!("Radiators ON !");
        wait()
    } else if mode.off {
        println!("Radiators OFF !");
        wait()
    } else if mode.opened {
        mode.off = true;
        mode.on = false;
        mode.opened = true;
        println!("Radiators ");
        wait()
    }
} 


}

fn wait() {
    // Wait 2 seconds

    std::thread::sleep(std::time::Duration::from_secs(2));
}