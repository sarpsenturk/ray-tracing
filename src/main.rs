pub mod vec3;

const IMAGE_WIDTH: i32 = 256;
const IMAGE_HEIGHT: i32 = 256;

fn main() {
    print!("P3\n");
    print!("{IMAGE_WIDTH} {IMAGE_HEIGHT}\n255\n");
    for j in 0..IMAGE_HEIGHT {
        let percentage = j as f32 / IMAGE_HEIGHT as f32;
        eprintln!("Progress {:.2}", percentage);
        for i in 0..IMAGE_WIDTH {
            let r = i as f64 / (IMAGE_WIDTH - 1) as f64;
            let g = j as f64 / (IMAGE_WIDTH - 1) as f64;
            let b = 0.0;
            let r = (255.999 * r) as i32;
            let g = (255.999 * g) as i32;
            let b = (255.999 * b) as i32;
            print!("{r} {g} {b}\n");
        }
    }
    eprintln!("Done");
}
