#[derive(Debug)]
struct Student{
    name:String,
    grade:char,
    age:u8,
    percentage:f32,
}
impl Student{
    fn new(name:String,grade:char,age:u8,percentage:f32)->Self{
        Self{
            name,
            grade,
            age,
            percentage,
        }
    }
    fn printPercent(&self){
        println!("Percentage: {}",self.percentage);
    }
}
fn main() {
    println!("<---Student no 1--->");
    let Salman=crate::Student::new(String::from("Muhammad Salman Asif"),'A',22,70.25);
    println!("{}",Salman.name );
    Salman.printPercent();
    println!("<----end---->");
    println!();
    println!("<---Student no 2--->");
    let Atif=crate::Student::new(String::from("Muhammad Atif"),'A',21,75.25);
    println!("{}",Atif.name );
    Atif.printPercent();
    println!("<----end---->");
}
