use std::str::FromStr;

pub struct Dial {
    point_at: i16,
    times_at_zero_after_rotation: u32,
    times_at_zero_after_click: u32,
}

impl Dial {
    pub fn new() -> Dial {
        Dial {
            point_at: 50,
            times_at_zero_after_rotation: 0,
            times_at_zero_after_click: 0,
        }
    }

    pub fn apply(&mut self, rotation: &str) {
        let distance = i16::from_str(&rotation[1..]).expect(&format!("Invalid rotation: {}", rotation));
        match &rotation[0..1] {
            "R" => {
                for _ in 0..distance {
                    self.click(1);
                }
            },
            "L" => {
                for _ in 0..distance {
                    self.click(-1);
                }
            },
            _ => panic!("Invalid rotation: {}", rotation),
        }
        if self.point_at == 0 {
            self.times_at_zero_after_rotation += 1;
        }
    }

    fn click(&mut self, n: i16) {
        self.point_at += n;
        self.point_at = self.point_at.rem_euclid(100);
        if self.point_at == 0 {
            self.times_at_zero_after_click += 1;
        }
    }

    pub fn password(&self) -> u32 {
        self.times_at_zero_after_rotation
    }

    pub fn real_password(&self) -> u32 {
        self.times_at_zero_after_click
    }
}

#[test]
fn test_dial() {
    let mut dial = Dial::new();
    assert_eq!(0, dial.password());
    assert_eq!(50, dial.point_at);

    // can turn right
    dial.apply("R12");
    assert_eq!(0, dial.password());
    assert_eq!(62, dial.point_at);

    // can turn left
    dial.apply("L30");
    assert_eq!(0, dial.password());
    assert_eq!(32, dial.point_at);

    // counts times at 0 turning left
    dial.apply("L32");
    assert_eq!(1, dial.password());
    assert_eq!(0, dial.point_at);

    // wraps aroung turning right
    dial.apply("R105");
    assert_eq!(1, dial.password());
    assert_eq!(5, dial.point_at);

    // wraps around turning left
    dial.apply("L15");
    assert_eq!(1, dial.password());
    assert_eq!(90, dial.point_at);

    // counts times at 0 turning right
    dial.apply("R10");
    assert_eq!(2, dial.password());
    assert_eq!(0, dial.point_at);
}

#[test]
fn test_times_at_zero_after_click() {
    let mut dial = Dial::new();

    dial.apply("R1000");
    assert_eq!(50, dial.point_at);
    assert_eq!(10, dial.times_at_zero_after_click);

    dial.apply("L1000");
    assert_eq!(50, dial.point_at);
    assert_eq!(20, dial.times_at_zero_after_click);
}