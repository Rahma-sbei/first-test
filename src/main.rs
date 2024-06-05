 use std::io;
 use serde::{Deserialize,Serialize};
 use serde_json::{self, to_string, from_str};
 use std::fs::{File};
 use std::io::{Write, Read};
 
#[derive(Serialize, Deserialize)]
 struct Person{
    name: String,
    phone: i64,
    email:String
 }
 fn load_from_file() -> Vec<Person> {
    let mut file = File::open("data.json").expect("Unable to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read data");
    from_str(&contents).expect("Unable to parse JSON")
}

  fn display(v:&mut Vec <Person>){
      for i in 0..v.len(){
          println!("the element number {} is : {}",i,v[i].name);
     } 
    
 }

 fn add(v:&mut Vec <Person>){
    println!("Enter a name to add to the list: " );
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Failed to read line");
    
    println!("Enter the phone number of this person: " );
    let mut ph = String::new();
    io::stdin().read_line(&mut ph).expect("Failed to read line");
    let pho:i64 = ph.trim().parse().expect("Invalid input");
    
    println!("Enter the email number of this person: " );
    let mut mail = String::new();
    io::stdin().read_line(&mut mail).expect("Failed to read line");

    let new_person = Person {
        name: n,
        phone:pho,
        email: mail,
    };

    
    v.push(new_person);
    ser_and_write(v);
    println!("person added successfully! " );

 }

fn remove_contact(v:&mut Vec <Person>){
    println!("Enter a name to remove from the list: " );
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut index:Option<usize> = None ;
    for i in 0..v.len(){
        if v[i].name.trim() == input.trim(){
            println!("found it ");
            index=Some(i);
            break;
        }
    }  
    match index {
        Some(i) => {
            v.remove(i);
            ser_and_write(v);
            println!("Element removed. Vector is now:");
        }
        None => {
            println!("Name invalid or does not exist.");
        }

    }
}

fn display_details(v:&mut Vec <Person>){
    println!("Enter a name to display their details: " );
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut index:Option<usize> = None ;
    for i in 0..v.len(){
        if v[i].name.trim() == input.trim(){
            println!("found it ");
            index=Some(i);
            break;
        }
    } 

    match index {
        Some(i) => {
            println!("This contact is:\n *name:{}.\n *phone:{}.\n *email:{}.\n",v[i].name,v[i].phone,v[i].email);
        }
        None => {
            println!("Name invalid or does not exist.");
        }
    }
}
    
    fn ser_and_write(v: &Vec<Person>){
        let serialized = to_string(v).unwrap();
        let mut file = File::create("data.json").expect("Unable to create file");
        file.write_all(serialized.as_bytes()).expect("Unable to write data");
}







fn main() {
    
    let mut v:Vec <Person> = load_from_file();
    //  v.push(Person{
    //     name: String::from("rahma"),
    //     phone: 4569832,
    //     email:String::from("rahma@medtech.tn"),
    //  });
    //  v.push(Person{
    //     name: String::from("mey"),
    //     phone: 7524131,
    //     email:String::from("mey@medtech.tn"),
    //  });
    //  v.push(Person{
    //     name: String::from("youssef"),
    //     phone: 9632584,
    //     email:String::from("youssef@medtech.tn"),
    //  });
    
     

   
    
    loop{
        println!("\nto perform a task click on its corresponding number:\n *to display list : 1.\n *to add contact: 2.\n *to remove contact: 3.\n *to display details of a contact: 4.\n click anywhere else to exit the program. " );
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        
        let choice: i32 = match input.trim().parse() {
            Ok(numbr) => numbr,
            Err(_) => {
                println!("Exiting program..");
                break;
            }
        };
        match choice {
            
            1 => display(&mut v),
            2 => add(&mut v),
            3 => remove_contact(&mut v),
            4 => display_details(&mut v),
            _ => {
                println!("Exiting the program..");
                break; 
            },
        
        }
        
    
    }
} 
    

 