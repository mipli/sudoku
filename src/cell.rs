
bitflags! {
    pub struct Value: u32 {
        const ONE = 0b000000001;
        const TWO = 0b000000010;
        const THREE = 0b000000100;
        const FOUR = 0b000001000;
        const FIVE = 0b000010000;
        const SIX = 0b000100000;
        const SEVEN = 0b001000000;
        const EIGHT = 0b010000000;
        const NINE = 0b100000000;
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

#[derive(Copy, Clone, Debug)]
pub struct Cell {
    pub value: Value,
    count: u8
}

impl Cell {
    pub fn is_known(&self) -> bool {
        self.count == 1
    }

    pub fn eliminate(&mut self, value: Value) -> &Self {
        self.value = self.value ^ value;
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
