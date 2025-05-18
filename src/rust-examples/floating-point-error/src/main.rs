use rand::Rng;

fn main() {
    let pi_bits: u32 = std::f32::consts::PI.to_bits();
    println!("PI: {:032b}", pi_bits);
    println!("PI: {:?}", std::f32::consts::PI);
    // PI: 01000000010010010000111111011011
    let val: f32 = 2.0 * (1.0 + 4788187.0 / (2.0 as f32).powi(23));
    println!("val: {:?}", val);

    let mut rng = rand::rng();
    let x_u32: u32 = rng.random();
    let y_u32: u32 = rng.random();
    let x: f32 = f32::from_be_bytes(x_u32.to_be_bytes());
    let y: f32 = f32::from_be_bytes(y_u32.to_be_bytes());
    let x_add_y = x + y;
    let x_sub_y = x - y;
    let x_mul_y = x * y;
    let x_div_y = x / y;

    let z: f64 = x as f64;
    let w: f64 = y as f64;
    let z_add_w = z + w;
    let z_sub_w = z - w;
    let z_mul_w = z * w;
    let z_div_w = z / w;

    let delta_add = ((x_add_y as f64 - z_add_w) / z_add_w).abs();
    let delta_sub = ((x_sub_y as f64 - z_sub_w) / z_sub_w).abs();
    let delta_mul = ((x_mul_y as f64 - z_mul_w) / z_mul_w).abs();
    let delta_div = ((x_div_y as f64 - z_div_w) / z_div_w).abs();

    println!("add: {:?}, {:?}, {:?}", x_add_y, z_add_w, delta_add);
    println!("sub: {:?}, {:?}, {:?}", x_sub_y, z_sub_w, delta_sub);
    println!("mul: {:?}, {:?}, {:?}", x_mul_y, z_mul_w, delta_mul);
    println!("div: {:?}, {:?}, {:?}", x_div_y, z_div_w, delta_div);
}
