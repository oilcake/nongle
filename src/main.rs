mod convert;
mod que;
mod performer;

fn main() {
    let amplitude: f64 = 0.07593;
    let bounds = convert::VelocityBounds{
        top: 1.0,
        bottom: 0.0
    };
    let velocity = convert::amplitude_to_velocity(amplitude, bounds);
    println!("velocity is {}", velocity);

    let mut que: que::Que = que::Que::new(7);
    let mut count: u8 = 0;
    while count < 10 {
        count += 1;
        que.next_in_que();
        println!("and now id in que is {}", que.q_id)
    }
    // define a dummy note
    let sample = performer::mello::Mello::new('e'.to_string());
    sample.play();
}
