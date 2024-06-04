 use std::io;

  fn display(vec:Vec <String>){
     for i in 0..vec.len(){
         println!("the element number {} is : {}",i,vec[i]);
     } 
 }

 fn add(v:&mut Vec<String>){
    println!("Enter a name to add to the list: " );
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    v.push(input.to_string());
    println!("Item added successfully: " );

}

fn remove_contact(v:&mut Vec<String>){
    println!("Enter a name to remove from the list: " );
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut index:Option<usize> = None ;
    for i in 0..v.len(){
        if v[i].trim() == input.trim(){
            println!("found it ");
            index=Some(i);
            break;
        }
    } 
    match index {
        Some(i) => {
            v.remove(i);
            println!("Element removed. Vector is now: {:?}", v);
        }
        None => {
            println!("Name invalid or does not exist.");
        }

    }
}

fn main() {
    
    let mut v:Vec <String> = Vec::new();
    v.push("rahma".to_string());
    v.push("mey".to_string());
    v.push("youssef".to_string());
    
    
        println!("to perform a task click on its corresponding number:\n *to display list : 1.\n *to add contact: 2.\n *to remove contact: 3. " );
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let choice:i32 = input.trim().parse().expect("Invalid input");

        match choice {
            
            1 => display(v),
            2 => add(&mut v),
            3 => remove_contact(&mut v),
            _ => println!("Invalid input."),
        
        }
    
    
   
    
}
 