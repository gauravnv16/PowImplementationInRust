/*
Below funtion implements pow functionality in rust
*/
fn powg(x:i32,y:i32) -> f32{
    if y == 0 {
        return 1 as f32;
    } else if y == 1{
        return x as f32;
    }

    let temp:f32 = powg(x, y/2);

    if y>0 {
        if y%2 == 0 {
            return temp * temp;
        } else {
            return temp * temp * (x as f32);
        }
    } else {
        if y%2 == 0 {
            return temp * temp;
        } else {
            return temp * temp / (x as f32);
        }        
    }
}
fn main() {
    println!("pow(2,5) is {}",powg(2, 5));
}
