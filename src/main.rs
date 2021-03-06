mod patterns;

use patterns::*;
use std::{
    fs::{self, File},
    io::Write,
    ops::{Index, IndexMut}, time::Instant,
};

const WIDTH:        usize = 1000;
const HEIGHT:       usize = 100;
const LENGTH:       usize = WIDTH * HEIGHT;
const PPM_SCALER:   usize = 1;
const VIDEO_LENGTH: usize = 10000;

#[derive(Debug, Clone, Copy)]
pub struct Field {
    data: [u8; LENGTH],
}

impl Field {
    pub fn new() -> Self {
        Field { data: [0; LENGTH] }
    }
}

impl Default for Field {
    fn default() -> Self {
        Field::new()
    }
}

impl Index<isize> for Field {
    type Output = u8;

    fn index(&self, mut i: isize) -> &u8 {
        while i < 0 {
            i += LENGTH as isize;
        }
        while i >= LENGTH as isize {
            i -= LENGTH as isize;
        }
        &self.data[i as usize]
    }
}

impl IndexMut<isize> for Field {
    fn index_mut(&mut self, mut i: isize) -> &mut u8 {
        while i < 0 {
            i += LENGTH as isize;
        }
        while i >= LENGTH as isize {
            i -= LENGTH as isize;
        }
        &mut self.data[i as usize]
    }
}

fn main() {
    let now = Instant::now();

    let folder = "video";
    if fs::read_dir(folder).is_ok() {
        fs::remove_dir(folder).unwrap()
    }
    fs::create_dir(folder).unwrap();

    let mut field: Field = Field::new();

    // field = glider(field);
    field = glider_generator(field);

    for i in 0..VIDEO_LENGTH {
        let path: &str = &format!("video/cgol-{}.ppm", i);
        if File::open(path).is_ok() {
            fs::remove_file(path).unwrap();
        }
        let mut file = File::create(path).unwrap();
        if i == 0 {
            write_field_to_ppm(&mut file, field);
            continue;
        };

        field = calculate_changes(field);
        write_field_to_ppm(&mut file, field);
        println!("rendered file {}", i);
    }

    println!("{:?}", now.elapsed());
}

pub fn calculate_changes(field: Field) -> Field {
    let mut new_field: Field = Field::new();
    for i in 0..LENGTH as isize {
        let w = WIDTH as isize;
        let neighbours =
        field[i-w-1] + field[i-w] + field[i-w+1] +
        field[i-1]   +              field[i+1]   +
        field[i+w-1] + field[i+w] + field[i+w+1];

        if (field[i] == 0 && neighbours == 3) || (field[i] == 1 && (neighbours == 2 || neighbours == 3)) {
            new_field[i] = 1;
        }
    }

    new_field
}

pub fn write_field_to_ppm(file: &mut File, field: Field) {
    let header = format!("P6 {} {} 255\n", WIDTH*PPM_SCALER, HEIGHT*PPM_SCALER);
    file.write_all(header.as_bytes()).unwrap();
    for h in 0..HEIGHT*PPM_SCALER {
        for w in 0..WIDTH*PPM_SCALER {
            let i = w/PPM_SCALER + h/PPM_SCALER*WIDTH;
            let p = field[i as isize];
            let pixel = &[
                255-p*255,
                255-p*255,
                255-p*255
                ];
            file.write_all(pixel).unwrap();
        }
    }
}
