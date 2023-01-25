use tubann::tubes::prelude::*;

#[test]
fn basic_test() {
    let my_bowl = LoggingBowl::new(Box::new(|x: &u32| println!("{}", x*2)));
    let mut my_tube = BaseTube::<u32>::new();

    my_tube.connect().send_to(my_bowl).expect("Bowl was not base type");

    my_tube.roll(5).expect("Failed rolling the ball.");
}