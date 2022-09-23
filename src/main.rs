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
            Autos::Lkw => Box::new(LKW::new(_ps)),
            Autos::Pkw => Box::new(PKW::new(_ps)),
        }
    }
}



fn main() {
    let auto1 = Autofabrik::new(Autos::Lkw, 300);

    print!("Ich habe {0} PS: ", auto1.get_ps());

    auto1.say_who_am_i();

    let auto2 = Autofabrik::new(Autos::Pkw, 80);

    print!("Ich habe {0} PS: ", auto2.get_ps());

    auto2.say_who_am_i();
}
