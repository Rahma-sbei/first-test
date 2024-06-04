 use std::io;

  fn display(vec:Vec <String>){
     for i in 0..2{
         println!("the element number {} is : {}",i,vec[i]);
     } 
 }

 fn add(v:&mut Vec<String>){
    println!("Enter a name to add to the list: " );
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    v.push(input.to_string());
}

fn remove(v:&mut Vec<String>){
    println!("Enter a name to remove from the list: " );
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut index:usize = 6 ;
    for i in 0..2{
        if v[i] == input{
            println!("found it ");
            index=i;
            break;
        }
    } 
    if index != 6{
        v.remove(index);
    }else{
        println!("Name invalid or does not exist.")
    }
}

fn main() {
    
    let mut v:Vec <String> = Vec::new();
    v.push("rahma".to_string());
    v.push("mey".to_string());
    v.push("youssef".to_string());
    // display(v);
     
    println!("to perform a task click on its corresponding number:\n *to display a contact: 1.\n *to display the list: 2. \n *to add contact: 3.\n *to remove contact: 4. " );
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let choice:i32 = input.trim().parse().expect("Invalid input");

    match choice {
        
        1 => display(v),
        2 => add(&v),
        3 => remove(&v),
        _ => println!("Invalid input."),
       
    }

    
   
    
}
 