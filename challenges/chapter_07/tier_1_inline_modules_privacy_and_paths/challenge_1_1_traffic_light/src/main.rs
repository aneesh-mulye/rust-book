fn main() {
    let l = signal::Light::Green;
    controller::run_cycle(&l);
}

pub mod signal {
    #[derive(Debug, Clone)]
    pub enum Light {
        Red,
        Yellow,
        Green,
    }

    pub fn duration(light: &Light) -> u32 {
        match light {
            Light::Red => 30,
            Light::Yellow => 5,
            Light::Green => 25,
        }
    }

    fn is_stop(light: &Light) -> bool {
        !matches!(light, Light::Green)
    }

    pub fn can_proceed(light: &Light) -> bool {
        !is_stop(light)
    }
}

pub mod controller {
    use crate::signal;

    pub fn next(light: &signal::Light) -> signal::Light {
        match light {
            signal::Light::Red => signal::Light::Green,
            signal::Light::Green => signal::Light::Yellow,
            signal::Light::Yellow => signal::Light::Red,
        }
    }

    pub fn run_cycle(start: &signal::Light) {
        let mut current = start.clone();
        for _ in 0..3 {
            println!(
                "{:?} ({}s) -> {:?}",
                current,
                signal::duration(&current),
                next(&current)
            );
            current = next(&current);
        }
    }
}
