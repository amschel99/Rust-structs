


fn main() {
let target="Amschel";
let optional_target:Option<&str>=Some(target);
match optional_target{
    Some(word)=>{
   
   println!("Now lets see using match ,{}",word) ;
        
    },
    
    _=>println!("How match handles None")
}


if let Some(word)=optional_target{
    println!("Now let's see using let if, {}", word);
}
else{
    println!("if let handles the None case perfectly fine too");
}

}
