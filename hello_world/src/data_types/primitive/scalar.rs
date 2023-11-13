pub mod scalar{
    

    pub fn boolean() -> (bool,bool){
        let value:bool= false;
        let value2:bool= true;
        (value,value2)
    }
    
    pub fn strings() -> String{
        let value = String::from("I love Jesus");
        value
    }  
    
    pub fn signed_integers() -> (i8, i16, i32, i64) {
        let a:i8 = -15;
        let b:i16 = 5;
        let c:i32= -500;
        let d:i64= 5;
    
        (a,b,c,d)
    } 
    
    pub fn unsigned_integers() -> (u8, u16, u32, u64) {
        let a2:u8 = 5;
        let b2:u16 = 5;
        let c2:u32= 5;
        let d2:u64= 5;
    
        (a2,b2,c2,d2)
    } 
    
    pub fn floats() -> (f32,f64) {
        let a2:f32 = -15.3;
        let b2:f64 = 15.3;
        (a2,b2)
    } 
    
    
    pub fn run(){
        println!("{},{}", boolean().0, boolean().1);
        println!("{}", strings());
        println!("{}, {},{}, {}",  signed_integers().0,  signed_integers().1, signed_integers().2,  signed_integers().3);
        println!("{}, {}, {}, {}", unsigned_integers().0, unsigned_integers().1,unsigned_integers().2, unsigned_integers().3);
        println!("{}, {}", floats().0, floats().1); 

        
            // Integer addition
            println!("1 + 2 = {}", 1u32 + 2);
        
            // Integer subtraction
            println!("1 - 2 = {}", 1i32 - 2);
            // TODO ^ Try changing `1i32` to `1u32` to see why the type is important
        
            // Scientific notation
            println!("1e4 is {}, -2.5e-3 is {}", 1e4, -2.5e-3);
        
            // Short-circuiting boolean logic
            println!("true AND false is {}", true && false);
            println!("true OR false is {}", true || false);
            println!("NOT true is {}", !true);
        
            // Bitwise operations
            println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
            println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
            println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
            println!("1 << 5 is {}", 1u32 << 5);
            println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);
        
            // Use underscores to improve readability!
            println!("One million is written as {}", 1_000_000u32);
        
    }   

}