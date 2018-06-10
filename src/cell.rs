use std::cmp::Ordering;

bitflags! {
    pub struct Value: u32 {
        const ONE = 0b0_0000_0001;
        const TWO = 0b0_0000_0010;
        const THREE = 0b0_0000_0100;
        const FOUR = 0b0_0000_1000;
        const FIVE = 0b0_0001_0000;
        const SIX = 0b0_0010_0000;
        const SEVEN = 0b0_0100_0000;
        const EIGHT = 0b0_1000_0000;
        const NINE = 0b1_0000_0000;
    }
}

impl From<u32> for Value {
    fn from(v: u32) -> Value {
        match v {
            1 => Value::ONE,
            2 => Value::TWO,
            3 => Value::THREE,
            4 => Value::FOUR,
            5 => Value::FIVE,
            6 => Value::SIX,
            7 => Value::SEVEN,
            8 => Value::EIGHT,
            9 => Value::NINE,
            _ => panic!("Trying to convert number not between 1 and 9")
        }
    }
}

impl From<Value> for String {
    fn from(v: Value) -> String {
        match v {
            Value::ONE => "1".to_string(),
            Value::TWO => "2".to_string(),
            Value::THREE => "3".to_string(),
            Value::FOUR => "4".to_string(),
            Value::FIVE => "5".to_string(),
            Value::SIX => "6".to_string(),
            Value::SEVEN => "7".to_string(),
            Value::EIGHT => "8".to_string(),
            Value::NINE => "9".to_string(),
            _ => panic!("Trying to convert invalid value")
        }
    }
}

#[derive(Eq, Copy, Clone, Debug)]
pub struct Cell {
    pub value: Value,
    count: u8
}

impl Cell {
    pub fn is_known(&self) -> bool {
        self.count == 1
    }

    pub fn eliminate(&mut self, value: Value) -> &Self {
        self.value ^= value;
        self.count -= 1;
        self
    }

    pub fn count(&self) -> u8 {
        self.count
    }

    pub fn nums(&self) -> Vec<u32> {
        let mut nums = vec![];
        if self.value.intersects(Value::ONE) {
            nums.push(1);
        }
        if self.value.intersects(Value::TWO) {
            nums.push(2);
        }
        if self.value.intersects(Value::THREE) {
            nums.push(3);
        }
        if self.value.intersects(Value::FOUR) {
            nums.push(4);
        }
        if self.value.intersects(Value::FIVE) {
            nums.push(5);
        }
        if self.value.intersects(Value::SIX) {
            nums.push(6);
        }
        if self.value.intersects(Value::SEVEN) {
            nums.push(7);
        }
        if self.value.intersects(Value::EIGHT) {
            nums.push(8);
        }
        if self.value.intersects(Value::NINE) {
            nums.push(9);
        }
        nums
    }
}

impl Default for Cell {
    fn default() -> Cell {
        Cell {
            value: Value::ONE |
                Value::TWO |
                Value::THREE |
                Value::FOUR |
                Value::FIVE |
                Value::SIX |
                Value::SEVEN |
                Value::EIGHT |
                Value::NINE,
            count: 9
        }
    }
}

impl PartialOrd for Cell {
    fn partial_cmp(&self, other: &Cell) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Cell {
    fn cmp(&self, other: &Cell) -> Ordering {
        let s = match self.count() {
            1 => 10,
            v => v
        };
        let o = match other.count() {
            1 => 10,
            v => v
        };
        s.cmp(&o)
    }
}

impl PartialEq for Cell {
    fn eq(&self, other: &Cell) -> bool {
        self.value == other.value
    }
}
