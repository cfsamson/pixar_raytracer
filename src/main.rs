use rand::random;
use rayon::prelude::*;
use std::io::Write;
use std::ops::{Add, Mul, Not, Rem};

#[derive(Debug, Clone, Copy)]
struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

impl Vec3 {
    fn new_abc(a: f32, b: f32, c: f32) -> Self {
        Vec3 { x: a, y: b, z: c }
    }
    fn new_ab(a: f32, b: f32) -> Self {
        Vec3 { x: a, y: b, z: 0.0 }
    }
}

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

impl Rem for Vec3 {
    type Output = f32;
    fn rem(self, other: Vec3) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

impl Not for Vec3 {
    type Output = Vec3;
    fn not(self) -> Vec3 {
        self * (1.0 / (self % self).sqrt()).into()
    }
}

impl From<f32> for Vec3 {
    fn from(n: f32) -> Vec3 {
        Vec3::new_abc(n, n, n)
    }
}

fn min(l: f32, r: f32) -> f32 {
    if l < r {
        l
    } else {
        r
    }
}

fn random_val() -> f32 {
    random()
}

fn fmodf(x: f32, y: f32) -> f32 {
    x % y
}

fn fabsf(x: f32) -> f32 {
    x.abs()
}

fn sqrtf(x: f32) -> f32 {
    x.sqrt()
}

fn powf(x: f32, y: f32) -> f32 {
    x.powf(y)
}

fn cosf(x: f32) -> f32 {
    x.cos()
}

fn sinf(x: f32) -> f32 {
    x.sin()
}

fn box_test(position: Vec3, lower_left: Vec3, upper_right: Vec3) -> f32 {
    let lower_left = position + lower_left * Vec3::from(-1.0);
    let upper_right = upper_right + position * Vec3::from(-1.0);

    -min(
        min(
            min(lower_left.x, upper_right.x),
            min(lower_left.y, upper_right.y),
        ),
        min(lower_left.z, upper_right.z),
    )
}

const HIT_NONE: u8 = 0;
const HIT_LETTER: u8 = 1;
const HIT_WALL: u8 = 2;
const HIT_SUN: u8 = 3;

fn query_database(
    letter_blocks: &[(Vec3, Vec3, f32)],
    curves: &[Vec3; 2],
    position: Vec3,
    hit_type: &mut u8,
) -> f32 {
    let mut distance = std::f32::MAX;
    let mut f = position;
    f.z = 0.0;

    for (begin, e, e_mod_e) in letter_blocks.iter() {
        let o_part1 = -min((*begin + f * Vec3::from(-1.0)) % *e / e_mod_e, 0.0);
        let o = f + (*begin + *e * min(o_part1, 1.0).into()) * Vec3::from(-1.0);
        distance = min(distance, o % o);
    }
    distance = sqrtf(distance);

    for curve in curves.iter().rev() {
        let mut o = f + *curve;
        let temp = if o.x > 0.0 {
            fabsf(sqrtf(o % o) - 2.0)
        } else {
            o.y += if o.y > 0.0 { -2.0 } else { 2.0 };
            sqrtf(o % o)
        };
        distance = min(distance, temp);
    }

    distance = powf(distance.powi(8) + position.z.powi(8), 0.125) - 0.5;
    *hit_type = HIT_LETTER;

    let room_dist = min(
        -min(
            box_test(
                position,
                Vec3::new_abc(-30.0, -0.5, -30.0),
                Vec3::new_abc(30.0, 18.0, 30.0),
            ),
            box_test(
                position,
                Vec3::new_abc(-25.0, 17.0, -25.0),
                Vec3::new_abc(25.0, 20.0, 25.0),
            ),
        ),
        box_test(
            Vec3::new_abc(fmodf(fabsf(position.x), 8.0), position.y, position.z),
            Vec3::new_abc(1.5, 18.5, -25.0),
            Vec3::new_abc(6.5, 20.0, 25.0),
        ),
    );

    if room_dist < distance {
        distance = room_dist;
        *hit_type = HIT_WALL;
    }

    let sun = 19.9 - position.y;
    if sun < distance {
        distance = sun;
        *hit_type = HIT_SUN;
    }
    distance
}

fn ray_marching(
    letter_blocks: &[(Vec3, Vec3, f32)],
    curves: &[Vec3; 2],
    origin: Vec3,
    direction: Vec3,
    hit_pos: &mut Vec3,
    hit_norm: &mut Vec3,
) -> u8 {
    let mut hit_type = HIT_NONE;
    let mut no_hit_count = 0;
    let mut total_d = 0.0;

    while total_d < 100.0 {
        *hit_pos = origin + direction * total_d.into();
        let d = query_database(letter_blocks, curves, *hit_pos, &mut hit_type);

        no_hit_count += 1;
        if d < 0.01 || no_hit_count > 99 {
            *hit_norm = !Vec3::new_abc(
                query_database(
                    letter_blocks,
                    curves,
                    *hit_pos + Vec3::new_ab(0.01, 0.0),
                    &mut no_hit_count,
                ) - d,
                query_database(
                    letter_blocks,
                    curves,
                    *hit_pos + Vec3::new_ab(0.0, 0.01),
                    &mut no_hit_count,
                ) - d,
                query_database(
                    letter_blocks,
                    curves,
                    *hit_pos + Vec3::new_abc(0.0, 0.0, 0.01),
                    &mut no_hit_count,
                ) - d,
            );

            return hit_type;
        }
        total_d += d;
    }
    0
}

fn trace(
    letter_blocks: &[(Vec3, Vec3, f32)],
    curves: &[Vec3; 2],
    mut origin: Vec3,
    mut direction: Vec3,
) -> Vec3 {
    let mut sampled_position = Vec3::from(0.0);
    let mut normal = Vec3::from(0.0);
    let mut color = Vec3::from(0.0);
    let mut attenuation = Vec3::from(1.0);
    let light_direction = !Vec3::new_abc(0.6, 0.6, 1.0);

    for _ in 0..3 {
        let hit_type = ray_marching(
            letter_blocks,
            curves,
            origin,
            direction,
            &mut sampled_position,
            &mut normal,
        );
        if hit_type == HIT_NONE {
            break;
        }
        if hit_type == HIT_LETTER {
            direction = direction + normal * ((normal % direction) * -2.0).into();
            origin = sampled_position + direction * Vec3::from(0.1);
            attenuation = attenuation * Vec3::from(0.2);
        }
        if hit_type == HIT_WALL {
            let incidence = normal % light_direction;
            let p = 6.283185 * random_val();
            let c = random_val();
            let s = sqrtf(1.0 - c);
            let g = if normal.z < 0.0 { -1.0 } else { 1.0 };
            let u = -1.0 / (g + normal.z);
            let v = normal.x * normal.y * u;

            direction = Vec3::new_abc(v, g + normal.y * normal.y * u, -normal.y)
                * cosf(p).into()
                * s.into()
                + Vec3::new_abc(1.0 + g * normal.x * normal.x * u, g * v, -g * normal.x)
                    * (Vec3::from(sinf(p)) * Vec3::from(s))
                + normal * sqrtf(c).into();
            origin = sampled_position + direction * Vec3::from(0.1);
            attenuation = attenuation * Vec3::from(0.2);

            if incidence > 0.0
                && ray_marching(
                    letter_blocks,
                    curves,
                    sampled_position + normal * Vec3::from(0.1),
                    light_direction,
                    &mut sampled_position,
                    &mut normal,
                ) == HIT_SUN
            {
                color = color + attenuation * Vec3::new_abc(500.0, 400.0, 100.0) * incidence.into();
            }
        }
        if hit_type == HIT_SUN {
            color = color + attenuation * Vec3::new_abc(50.0, 80.0, 100.0);
            break;
        }
    }
    color
}

const BYTES_PER_PIXEL: usize = 3;
fn main() {
    let w: usize = 960;
    let h: usize = 540;
    let samples_count = 2;

    let position = Vec3::new_abc(-22.0, 5.0, 25.0);
    let goal = !(Vec3::new_abc(-3.0, 4.0, 0.0) + position * Vec3::from(-1.0));
    let left = !Vec3::new_abc(goal.z, 0.0, -goal.x) * (1.0 / w as f32).into();

    let up = Vec3::new_abc(
        goal.y * left.z - goal.z * left.y,
        goal.z * left.x - goal.x * left.z,
        goal.x * left.y - goal.y * left.x,
    );

    let filename = String::from("output-rust.ppm");
    println!(
        "Width: = {}, Height: = {}, Samples = {}",
        w, h, samples_count
    );
    println!("Writing data to {}", filename);

    let mut file = std::fs::File::create(filename).unwrap();
    write!(file, "P6 {} {} 255 ", w, h).unwrap();

    let letter_blocks = {
        let x: String = [
            "5O5_", "5W9W", "5_9_", // P (without curve)
            "AOEO", "COC_", "A_E_", // I
            "IOQ_", "I_QO", // X
            "UOY_", "Y_]O", "WW[W", // A
            "aOa_", "aWeW", "a_e_", "cWiO", // R (without curve)
        ]
        .concat();
        let mut blocks: Vec<(i32, i32, i32, i32)> = vec![];
        let chs: Vec<i32> = x.chars().map(|c| c as i32).collect();
        for i in (0..x.len()).step_by(4) {
            blocks.push((chs[i], chs[i + 1], chs[i + 2], chs[i + 3]))
        }

        blocks
            .iter()
            .map(|(a, b, c, d)| {
                let begin = Vec3::new_ab((a - 79) as f32, (b - 79) as f32) * Vec3::from(0.5);
                let e = Vec3::new_ab((c - 79) as f32, (d - 79) as f32) * Vec3::from(0.5)
                    + begin * Vec3::from(-1.0);
                (begin, e, e % e)
            })
            .collect::<Vec<(Vec3, Vec3, f32)>>()
    };
    let curves = [
        Vec3::new_abc(-11.0, 6.0, 0.0) * Vec3::from(-1.0),
        Vec3::new_abc(11.0, 6.0, 0.0) * Vec3::from(-1.0),
    ];
    let mut bytes = vec![0u8; h as usize * w as usize * BYTES_PER_PIXEL];
    bytes
        // take mutable chunks of three items
        .par_chunks_mut(BYTES_PER_PIXEL)
        // Turn this into a parallel iterator using Rayon
        .into_par_iter()
        // reverse the order in which we iterate so our picture doesn't end upside down
        .rev()
        // enumerate() changes the closure argument from |item| => |(index, item)| tuple
        .enumerate()
        .for_each(|(idx, chunk)| {
            // determine the x- and y-coordinates based on the index in row-major order
            // https://en.wikipedia.org/wiki/Row-major_order
            let y = (idx / w ) as usize;
            let x = (idx % w ) as usize;
            let mut color = Vec3::from(0.0);

            for _ in 0..samples_count {
                color = color
                    + trace(
                        &letter_blocks,
                        &curves,
                        position,
                        !(goal
                            + left * ((x - w) as f32 / 2.0 + random_val()).into()
                            + up * ((y - h) as f32 / 2.0 + random_val()).into()),
                    );
            }

            color = color * (1.0 / samples_count as f32).into() + (14.0 / 241.0).into();

            let o: Vec3 = color + Vec3::from(1.0);
            color = Vec3::new_abc(color.x / o.x, color.y / o.y, color.z / o.z) * Vec3::from(255.0);
            // We write the new values to our buffer
            chunk[0] = color.x as u8;
            chunk[1] = color.y as u8;
            chunk[2] = color.z as u8;
        });

    file.write_all(&bytes).unwrap();
}
