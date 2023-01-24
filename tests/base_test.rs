use tubann::tubes::prelude::*;

#[test]
fn basic_test() {
    let my_bowl = BowlBuilder::logging(Box::new(|x: &u32| println!("{}", x*2)));
    let mut my_tube = TubeBuilder::base::<u32>();

    my_tube.connect().logging(my_bowl).expect("Bowl was not base type");

    my_tube.roll(5);
}