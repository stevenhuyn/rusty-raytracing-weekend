use std::{
    error::Error,
    io::{stdout, Write},
};

mod ray;
mod vec3;

fn main() -> Result<(), Box<dyn Error>> {
    const IMAGE_WIDTH: i32 = 256;
    const IMAGE_HEIGHT: i32 = 256;

    print!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScnalines remaining: {}", j);
        stdout().flush()?;
        for i in 0..IMAGE_WIDTH {
            let r = i as f64 / (IMAGE_WIDTH - 1) as f64;
            let g = j as f64 / (IMAGE_HEIGHT - 1) as f64;
            let b = 0.25f64;

            let color = vec3::Color::new(r, g, b);

            color.write_color();
        }
    }

    Ok(())
}
