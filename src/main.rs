enum Autos{
    Pkw,
    Lkw,
}

struct PKW{
    ps: u16,
}

impl PKW{
    fn new(_ps: u16) -> PKW{
        PKW{ ps: _ps}
    }
}

struct LKW{
    ps: u16,
}

impl LKW{
    fn new(_ps: u16) -> LKW{
        LKW{ ps: _ps}
    }
}

trait Auto {
    fn say_who_am_i(&self);

    fn get_ps(&self) -> u16;
}

impl Auto for PKW{
    fn say_who_am_i(&self){
        println!("{}", "Ich bin ein PKW!")
    }

    fn get_ps(&self) -> u16 {
        self.ps
    }
}

impl Auto for LKW{
    fn say_who_am_i(&self){
        println!("{}", "Ich bin ein LKW!")
    }

    fn get_ps(&self) -> u16 {
        self.ps
    }
}

struct Autofabrik{
}

impl Autofabrik{
    fn new(autos: Autos, _ps: u16) -> Box<dyn Auto>{
        match autos{
            Autos::Lkw => Box::new(LKW{ps: _ps}),
            Autos::Pkw => Box::new(PKW{ps: _ps}),
        }
    }
}



fn main() {
    let _auto = Autofabrik::new(Autos::Lkw, 300);

    _auto.say_who_am_i();

    println!("Ich habe {0} PS!", _auto.get_ps())
}
