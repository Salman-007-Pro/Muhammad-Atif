use atif_assignment::s_input;
#[derive(Debug)]
#[allow(dead_code)]
enum Items{
    Fruits,
    Vegetable,
    Household,
}
#[derive(Debug)]
#[allow(dead_code)]
struct CustomerDetails{
    name:String,
    phone_no:String,
    address:String,
    selection_of_item:Items,
    price:usize,
}
#[allow(dead_code)]
impl CustomerDetails{
    fn new(name:String,phone_no:String,address:String,selection_of_item:Items,price:usize)->CustomerDetails{
        CustomerDetails{
            name,phone_no,address,selection_of_item,price
        }
    }
    fn print_details(&self){
        println!("{:#?}",self);
    }
}
#[allow(dead_code)]
impl Items{
    fn new(item:Items)->Items{
        match item{
            Items::Fruits=>Items::Fruits,
            Items::Vegetable=>Items::Vegetable,
            Items::Household=>Items::Household,
        }
    }
    fn price_check(&self)->usize{
        match self{
            Items::Fruits=>233,
            Items::Vegetable=>236,
            Items::Household=>195,
        }
    }
}
#[allow(dead_code)]
fn main(){
    
    loop{
        
        println!("Place your Order online",);
        println!("1: Fruits", );
        println!("2: Vegetable", );
        println!("3: Household", );
        let name=s_input::string::as_string("Enter the name ");
        let ph=s_input::string::as_string("Enter the Phone_no ");
        let addr=s_input::string::as_string("Enter the Address ");
        let item_name=s_input::string::as_string("Enter the select item name(fruits) ");
        let item_name=item_name.to_lowercase();
        println!("{}",item_name );
        let items1=String::from("fruits");
        let items2=String::from("vegetable");
        let items3=String::from("household");
        let select_item= match item_name{
            items1=>Items::Fruits,
            items2=>Items::Vegetable,
            items3=>Items::Household,
            _=>panic!("Wrong value enter"),
        };
        let price=select_item.price_check();
        let customer1=CustomerDetails::new(name.trim().to_owned(), ph.trim().to_owned(), addr.trim().to_owned(),select_item, price);
        customer1.print_details();
        break;
    }
}