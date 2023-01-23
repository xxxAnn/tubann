use tubann::tubes::prelude::*;

#[test]
fn basic_test() {
    let my_bowl = BowlBuilder::base(Box::new(|x: &u32| println!("{}", x*2)));
    let mut my_tube = TubeBuilder::base::<u32>();

    my_tube.connect().base(my_bowl);

    my_tube.roll(5);
}