pub struct Master{
    objects: vec<Object>,
    player: Player,
    enemies: vec<Enemy>,
}
//TODO: implement structs Player and Enemy

impl Master{
    fn new() -> Self{
        //TODO: initialize Master with all Objects
    }

    fn update(){
        //TODO: update all Players, Enemies and moving objects
        //TODO: remove objects that are out of screen and spawn new ones
    }
}