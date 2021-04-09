struct Math {};

impl Math {
    pub const PI = 3.14159265359;
    pub const E = 2.71828182846;
    pub const TWO = 2;
}

println(Math::PI);
println(Math::E);

const RANDOM_NUMBER = 23;

struct Point {
    x: int,
    y: int,
}

impl Point {
    pub const DEFAULT_X = 0;
    pub const DEFAULT_Y = 0;

    const PRIVATE_X = 55;

    pub fn equals(self, x: int, y: int) -> bool {
        return self.x == x && self.y == y;
    }

    pub fn is_default(self) -> bool {
        return self.x == Self::DEFAULT_X && self.y == Point::DEFAULT_Y;
    }

    // checks if can access outside consts
    pub fn compute(self) -> int {
        return self.x * RANDOM_NUMBER * Math::TWO;
    }

    pub fn get_private_const_x2() -> int {
        return Self::PRIVATE_X * 2;
    }
}

const DEFAULT_X = 100;

let x = Point { x: 5, y: 5 };
let default = Point { x: 0, y: 0 };

let default_point_x = Point::DEFAULT_X;

println(RANDOM_NUMBER);
println(Point::DEFAULT_X == default_point_x);
println(default_point_x);

println(x.equals(5, 5));
println(x.is_default());
println(default.is_default());
println(x.compute());

println(Point::get_private_const_x2());

