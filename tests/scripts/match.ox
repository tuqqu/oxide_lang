let x = match true {
    true => 1
};

println(x);

let mut y = 10;
let y = match y {
    1 => 11,
    10 => 12,
    12 => 13,
};

println(y);

const H = "hello";
let z = match H {
    "he" + "llo" => 1,
    "hi" => 2,
    456 => 4,
};

println(z);
