use std::io;

fn main()
{
    println!("Welcome to the password checker");
    
    println!("Please input your password");
    let mut password = String::new();
    io::stdin().read_line(&mut password).expect("Please enter the password");

    let pass = password.trim();

    let length = pass.len();
    if length < 8
    {
        println!("Your password is weak and can be hacked easily");
    }

    let mut small_chars = 0;
    let mut big_chars = 0;

    let mut special_chars = -1;
    let mut numeros = 0;

    let values = password.chars();

    for c in values {
          if c.is_lowercase()
          {
            small_chars+=1;
          }
          else if c.is_uppercase()
          {
            big_chars+=1;
          }
          else if !c.is_alphanumeric()
          {
            special_chars+=1;
          }
          else if c.is_numeric()
          {
            numeros+=1;
          }
        }
        println!("{}" , special_chars);

        if special_chars >= 1 && numeros >= 2 && small_chars >= 4 && big_chars >=1
        {
             println!("Your password is Strong!");
        }
        else
        {
            println!("Your password is weak and can be hacked easily");
        }
}