
fn greet_world(){
    let germany ="Welt,Hallo";
    let chinese ="世界，你好";
    let english ="world,hello";
    let regions =[germany,chinese,english];
    for region in regions.iter(){
        println!("{}",&region);
    }
}
fn main() {
    greet_world()
}
