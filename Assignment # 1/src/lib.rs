pub use crate::s_input::{
    boolean,
    string,
    integer_i,
    integer_u,
    character,
    float
};
pub mod s_input{
    use std::io::Write;
    //float data type
    pub mod float{
        pub fn as_f64(msg:&str)->f64{
            super::msg_print(msg);
            let input = super::taking_input();
            let input:f64=match input.trim().parse(){
                Ok(a)=>a,
                Err(_)=>panic!("Enter the wrong type"),                
            };
            input
        }
        pub fn as_f32(msg:&str)->f32{
            super::msg_print(msg);
            let input = super::taking_input();
            let input:f32=match input.trim().parse(){
                Ok(a)=>a,
                Err(_)=>panic!("Enter the wrong type"),                
            };
            input
        }           
    }
    
    //integer_u data type
    pub mod integer_u{
        pub fn as_u8(msg:&str)->u8{
            super::msg_print(msg);
            let input = super::taking_input();
            let input:u8=match input.trim().parse(){
                Ok(a)=>a,
                Err(_)=>panic!("Enter the wrong type"),                
            };
            input
        }
        pub fn as_u16(msg:&str)->u16{
            super::msg_print(msg);
            let input = super::taking_input();
            let input:u16=match input.trim().parse(){
                Ok(a)=>a,
                Err(_)=>panic!("Enter the wrong type"),                
            };
            input
        }
        pub fn as_u32(msg:&str)->u32{
            super::msg_print(msg);
            let input = super::taking_input();
            let input:u32=match input.trim().parse(){
                Ok(a)=>a,
                Err(_)=>panic!("Enter the wrong type"),                
            };
            input
        }
        pub fn as_u64(msg:&str)->u64{
            super::msg_print(msg);
            let input = super::taking_input();
            let input:u64=match input.trim().parse(){
                Ok(a)=>a,
                Err(_)=>panic!("Enter the wrong type"),                
            };
            input
        }
        pub fn as_u128(msg:&str)->u128{
            super::msg_print(msg);
            let input = super::taking_input();
            let input:u128=match input.trim().parse(){
                Ok(a)=>a,
                Err(_)=>panic!("Enter the wrong type"),                
            };
            input
        }
        pub fn as_usize(msg:&str)->usize{
            super::msg_print(msg);
            let input = super::taking_input();
            let input:usize=match input.trim().parse(){
                Ok(a)=>a,
                Err(_)=>panic!("Enter the wrong type"),                
            };
            input
        }                   
    }
    //integer_i data type
    pub mod integer_i{
        pub fn as_i8(msg:&str)->i8{
            super::msg_print(msg);
            let input = super::taking_input();
            let input:i8=match input.trim().parse(){
                Ok(a)=>a,
                Err(_)=>panic!("Enter the wrong type"),                
            };
            input
        }
        pub fn as_i16(msg:&str)->i16{
            super::msg_print(msg);
            let input = super::taking_input();
            let input:i16=match input.trim().parse(){
                Ok(a)=>a,
                Err(_)=>panic!("Enter the wrong type"),                
            };
            input
        }
        pub fn as_i32(msg:&str)->i32{
            super::msg_print(msg);
            let input = super::taking_input();
            let input:i32=match input.trim().parse(){
                Ok(a)=>a,
                Err(_)=>panic!("Enter the wrong type"),                
            };
            input
        }
        pub fn as_i64(msg:&str)->i64{
            super::msg_print(msg);
            let input = super::taking_input();
            let input:i64=match input.trim().parse(){
                Ok(a)=>a,
                Err(_)=>panic!("Enter the wrong type"),                
            };
            input
        }
        pub fn as_i128(msg:&str)->i128{
            super::msg_print(msg);
            let input = super::taking_input();
            let input:i128=match input.trim().parse(){
                Ok(a)=>a,
                Err(_)=>panic!("Enter the wrong type"),                
            };
            input
        }
        pub fn as_isize(msg:&str)->isize{
            super::msg_print(msg);
            let input = super::taking_input();
            let input:isize=match input.trim().parse(){
                Ok(a)=>a,
                Err(_)=>panic!("Enter the wrong type"),                
            };
            input
        }
    }
    pub mod character{
        pub fn as_char(msg:&str)->char{
            super::msg_print(msg);
            let input = super::taking_input();
            let input:char=match input.trim().parse(){
                Ok(a)=>a,
                Err(_)=>panic!("Enter the wrong type"),                
            };
            input
        }
    }
    pub mod boolean{
        pub fn as_bool(msg:&str)->bool{
            super::msg_print(msg);
            let input = super::taking_input();
            let input:bool=match input.trim().parse(){
                Ok(a)=>a,
                Err(_)=>panic!("Enter the wrong type"),                
            };
            input
        }
    }
    pub mod string{
        pub fn as_string(msg:&str)->String{
            super::msg_print(msg);
            let input = super::taking_input();
            input
        }
    }
    
    //private function
    fn taking_input()->String{
        let mut input =String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read line");
        input
    }
    fn msg_print(msg:&str){
        print!("{}: ",msg);
        std::io::stdout().flush().ok().expect("Could not flush stdout");
    }
}
