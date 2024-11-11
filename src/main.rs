use leptos::*;

fn greet(){
    println!("hello, world! from stdin")
}


fn main() {
    mount_to_body(|| view! {
         <p>"Hello, world! from rust"</p> 
        
        })
}