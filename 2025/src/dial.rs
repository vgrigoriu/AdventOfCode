pub struct Dial {
    point_at: i32,
    times_at_zero_after_rotation: u32,
    times_at_zero_after_click: u32,
}

impl Default for Dial {
    fn default() -> Self {
        Self {
            point_at: 50,
            times_at_zero_after_rotation: 0,
            times_at_zero_after_click: 0,
        }
    }
}

impl Dial {
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub fn apply(&mut self, rotation: &str) {
        let direction = rotation.as_bytes()[0];
        let distance: i32 = rotation[1..]
            .parse()
            .unwrap_or_else(|_| panic!("Invalid rotation: {}", rotation));

        let distance = match direction {
            b'R' => distance,
            b'L' => -distance,
            _ => panic!("Invalid rotation: {}", rotation),
        };

        self.point_at += distance;

        let crossings = self.point_at.div_euclid(100).unsigned_abs();
        self.times_at_zero_after_click += crossings;

        self.point_at = self.point_at.rem_euclid(100);

        if self.point_at == 0 {
            self.times_at_zero_after_rotation += 1;
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
    assert_eq!(dial.password(), 0);
    assert_eq!(dial.point_at, 50);

    // can turn right
    dial.apply("R12");
    assert_eq!(dial.password(), 0);
    assert_eq!(dial.point_at, 62);

    // can turn left
    dial.apply("L30");
    assert_eq!(dial.password(), 0);
    assert_eq!(dial.point_at, 32);

    // counts times at 0 turning left
    dial.apply("L32");
    assert_eq!(dial.password(), 1);
    assert_eq!(dial.point_at, 0);

    // wraps around turning right
    dial.apply("R105");
    assert_eq!(dial.password(), 1);
    assert_eq!(dial.point_at, 5);

    // wraps around turning left
    dial.apply("L15");
    assert_eq!(dial.password(), 1);
    assert_eq!(dial.point_at, 90);

    // counts times at 0 turning right
    dial.apply("R10");
    assert_eq!(dial.password(), 2);
    assert_eq!(dial.point_at, 0);
}

#[test]
fn test_times_at_zero_after_click() {
    let mut dial = Dial::new();

    dial.apply("R1000");
    assert_eq!(dial.point_at, 50);
    assert_eq!(dial.times_at_zero_after_click, 10);

    dial.apply("L1000");
    assert_eq!(dial.point_at, 50);
    assert_eq!(dial.times_at_zero_after_click, 20);
}

#[test]
#[should_panic]
fn test_invalid_direction() {
    let mut dial = Dial::default();

    dial.apply("😍12");
}

#[test]
#[should_panic]
fn test_invalid_distance() {
    let mut dial = Dial::default();

    dial.apply("Lpapageno");
}
