fn main() {
    println!("Hello, world!");
}

use std::ops::{Add, AddAssign, Mul, MulAssign};
use rand::random;
use lazy_static::lazy_static;
//#[derive(Clone, Copy)]
struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 {
    fn new(v: f64) -> Self {
        Vec3 { x: v, y: v, z: v }
    }

    fn new_abc(a: impl Into<f64>, b: impl Into<f64>, c: impl Into<f64>) -> Self {
        Vec3 {
            x: a.into(),
            y: b.into(),
            z: c.into(),
        }
    }
    fn new_ab(a: impl Into<f64>, b: impl Into<f64>) -> Self {
        Vec3 {
            x: a.into(),
            y: b.into(),
            z: 0.0,
        }
    }

    fn modu(&self, other: &Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    fn add_b(&self, other: &Vec3) -> Vec3 {
        Vec3::new_abc(self.x  + other.x, self.y + other.y, self.z + other.z)
    }

    fn mul_b(&self, other: &Vec3) -> Vec3 {
        Vec3::new_abc(self.x * other.x, self.y*other.y, self.z * other.z)
    }

    fn i_sqrt(&self) -> Vec3 {
        self.mul_b(&Vec3::from(1.0/self.modu(&self)))
    }
}

//public static float operator %(Vec q, Vec r) { return q.x * r.x + q.y * r.y + q.z * r.z; }

impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, other: Vec3) -> Self {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Vec3) {
        self.x += other.x;
        self.y += other.y;
        self.x += other.z;
    }
}

impl Mul for Vec3 {
    type Output = Vec3;
    fn mul(self, other: Vec3) -> Self {
        Vec3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl MulAssign for Vec3 {
    fn mul_assign(&mut self, other: Vec3) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl From<f64> for Vec3 {
    fn from(n: f64) -> Vec3 {
        Vec3::new_abc(n, n, n)
    }
}
fn sqrt(q: Vec3) -> Vec3 {
    q * (1.0 / q.modu(&q).sqrt()).into()
}
//     public override string ToString() {
//       var format = ",10:N5"; // 5 decimal spaces, padded to 10 chars in total
//       return string.Format("{{ x:{0" + format + "}, y:{1" + format + "}, z:{2" + format + "} }}", x, y, z);
//     }
//   }

fn min(l: f64, r: f64) -> f64 {
    if l < r {
        l
    } else {
        r
    }
}

fn random_val() -> f64 {
    random()
}

fn fmodf(x: f64, y: f64) -> f64 {
    x % y
}

fn fabsf(x: f64) -> f64 {
    x.abs()
}

fn sqrtf(x: f64) -> f64 {
    x.sqrt()
}

fn powf(x: f64, y: f64) -> f64 {
    x.powf(y)
}

fn cosf(x: f64) -> f64 {
    x.cos()
}

fn sinf(x: f64) -> f64 {
    x.sin()
}

fn box_test(position: &Vec3, lower_left: &Vec3, upper_right: &Vec3) -> f64 {
    let lower_left = position.add_b(&lower_left.mul_b(&Vec3::from(-1.0)));
    let upper_right = upper_right.add_b(&position.mul_b(&Vec3::from(-1.0)));

    -min(
        min(
            min(lower_left.x, upper_right.x),
            min(lower_left.y, upper_right.y)
        ),
        min(lower_left.z, upper_right.z)
    )
}

const HIT_NONE: u8 = 0;
const HIT_LETTER: u8 = 1;
const HIT_WALL: u8 = 2;
const HIT_SUN: u8 = 3;

lazy_static! {
    static ref LETTERS: Vec<i32> = {
        let x: String =  [
            "5O5_", "5W9W", "5_9_",        // P (without curve)
                "AOEO", "COC_", "A_E_",        // I
                "IOQ_", "I_QO",               // X
                "UOY_", "Y_]O", "WW[W",        // A
                "aOa_", "aWeW", "a_e_", "cWiO"
            ].concat();
        x.chars().map(|c| c as i32).collect::<Vec<i32>>()
    };

    static ref CURVES: [Vec3; 2] = {
        [Vec3::new_abc(-11, 6, 0), Vec3::new_abc(11, 6, 0)]
    };
}

fn query_database(position: &Vec3, hit_type: u8) -> f64 {
    let mut distance = std::f64::MAX;
    let mut f = position;
    f.z = 0.0;

    let letter_count = LETTERS.len();
    for i in (0..letter_count).step_by(4) {
        let begin = Vec3::new_ab(LETTERS[i] - 79, LETTERS[i + 1] - 79) * Vec3::from(0.5);
        let e = Vec3::new_ab(LETTERS[i + 2] - 79, LETTERS[i+3] - 79).mul(Vec3::from(0.5)).add_b(&begin).mul(Vec3::from(-1.0));
        let o_part1 =-min((begin.add_b(&f).mul(-1).modu(&e)/&e.modu(&e)),0.0);
        let o = f.add_b(
            &begin.add_b(&e)
            .mul(min(-o_part1 ,1.0).into())
            .mul(Vec3::from(-1.0)));
            distance = min(distance, o.modu(&o));
    }
    distance = sqrtf(distance);

    for curve in CURVES.iter().rev() {
        let mut o = f.add_b(curve).mul_b(&Vec3::from(-1.0));
        let mut temp = 0.0;

        if o.x > 0.0 {
            temp = fabsf(sqrtf(o.modu(&o)) - 2.0);
        } else {
            o.y += if o.y > 0.0 { -2.0 }else {2.0};
            temp = sqrtf(o.modu(&o));
        }
        distance = min(distance, temp);
    }

    distance = powf(powf(distance, 8.0) + powf(position.z, 8.0), 0.125) - 0.5;
    hit_type = HIT_LETTER;

    let mut room_dist = 0.0;
    room_dist = min(
        -min(
            box_test(&position, &Vec3::new_abc(-30, -0.5, -30), &Vec3::new_abc(30, 18, 30)),
            box_test(&position, &Vec3::new_abc(-25, 17, -25), &Vec3::new_abc(25, 20, 25))
        ),
        box_test(
            &Vec3::new_abc(fmodf(fabsf(position.x), 8.0), position.y, position.z), 
            &Vec3::new_abc(1.5, 18.5, -25), 
            &Vec3::new_abc(6.5, 20, 25))
    );

    if room_dist < distance {
        distance = room_dist;
        hit_type = HIT_WALL;
    }

    let sun = 19.9 - position.y;
    if sun < distance {
        distance = sun;
        hit_type = HIT_SUN;
    }

    distance
}


fn ray_marching(origin: &Vec3, direction: &Vec3, hit_pos: &Vec3, hit_norm: &Vec3) -> u8 {
    let mut hit_type = HIT_NONE;
    let mut no_hit_count = 0;
    let mut total_d = 0.0; // Option<f64> ??

    while total_d < 100.0 {
        hit_pos = &origin.add_b(&direction).mul_b(&total_d.into());
        let d = query_database(hit_pos, hit_type);
        no_hit_count += 1;
        if d < 0.01 || no_hit_count > 99 {
            hit_norm = &Vec3::new_abc(
                query_database(&hit_pos.add_b(&Vec3::new_ab(0.1, 0)), no_hit_count) - d, 
                query_database(&hit_pos.add_b(&Vec3::new_ab(0, 0.1)), no_hit_count) - d, 
                query_database(&hit_pos.add_b(&Vec3::new_abc(0, 0, 0.01)), no_hit_count) - d
                ).i_sqrt();
                return hit_type;
        } // !
    }

    0
}


fn trace(mut origin: &Vec3, mut direction: &Vec3) -> Vec3 {
    let sampled_position = Vec3::from(0.0);
    let normal = Vec3::from(0.0);
    let color = Vec3::from(0.0);
    let attenuation = Vec3::from(1.0);
    let ligt_direction = Vec3::new_abc(0.6, 0.6, 1).i_sqrt();

    for i in (0..3).rev() {
        let hit_type = ray_marching(&origin, &direction, &sampled_position, &normal);
        if hit_type == HIT_NONE { break; }
        if hit_type == HIT_LETTER {
            direction = &direction.add_b(&normal).mul_b(&(normal.modu(&direction) * -2.0).into());
            origin = &sampled_position.add_b(&direction).mul_b(&Vec3::from(0.1));
            attenuation = attenuation.mul_b(&Vec3::from(0.2));
        }
        if hit_type == HIT_WALL {
            let incidence = normal.modu(&ligt_direction);
            let p = 6.283185 * random_val();
            let c = random_val();
            let s = sqrtf(1.0 - c);
            let g = if normal.z < 0 {-1.0} else {1.0};
            let u = -1.0/(g+normal.z);
            let v = normal.x * normal.y * u;
            direction = Vec3::
        }
    }

    unimplemented!()
}

//                     direction = Vec(v,
//                                     g + normal.y * normal.y * u,
//                                     -normal.y) * (cosf(p) * s)
//                                 +
//                                 Vec(1 + g * normal.x * normal.x * u,
//                                     g * v,
//                                     -g * normal.x) * (sinf(p) * s) + normal * sqrtf(c);
//                     origin = sampledPosition + direction * .1f;
//                     attenuation = attenuation * 0.2f;
//                     if (incidence > 0 &&
//                         RayMarching(sampledPosition + normal * .1f,
//                                     lightDirection,
//                                     ref sampledPosition,
//                                     ref normal) == HIT_SUN)
//                         color = color + attenuation * Vec(500, 400, 100) * incidence;
//                 }
//                 if (hitType == HIT_SUN)
//                 { //
//                     color = color + attenuation * Vec(50, 80, 100); break; // Sun Color
//                 }
//             }
//             return color;
//         }



// ???
// macro_rules! letters {
//     () => { {
//         let x: String =  [
//             "5O5_", "5W9W", "5_9_",        // P (without curve)
//                 "AOEO", "COC_", "A_E_",        // I
//                 "IOQ_", "I_QO",               // X
//                 "UOY_", "Y_]O", "WW[W",        // A
//                 "aOa_", "aWeW", "a_e_", "cWiO"
//             ].concat();
//         x.chars().map(|c| c).collect::<Vec<char>>()
//         }
//     };
// }



