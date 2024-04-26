use std::cell::RefCell;

fn main() {
    /*
    Rules of references:
    At any given time, you can have either one mutable reference or any number of immutable references.
    References must always be valid. 
    
a trait is like an interface, where for a data structure to be of that trait, it must implement
the defined functions following the signatures, using 
impl [trait] for [struct] { 
    fn function_definition_that_comply_with_signatures_in_the_trait(){
        ...
    }
}

    */
    let mut a = 5;
     println!("value of a before: {:?}",&a);
    {let  b = &mut a;
    
    *b = 4;
   println!("address of b: {:p}", &b);
     println!("address which b references: {:p}", *&b);}
     println!("address of a: {:p}",&a);
          println!("value of a after: {:?}",&a);
   
}
