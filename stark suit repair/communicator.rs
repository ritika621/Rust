#[derive(Debug)]
#[derive(PartialEq)]
pub enum Command
{
    Power(bool,i32),    // [Increase/Decrease] power by [number].
    Missiles(bool,i32), // [Increase/Decrease] missiles by [number].
    Shield(bool),       // Turn [On/Off] the shield.
    Try,                // Try calling pepper.
    Invalid             // [anything else]
}


/**
    Adds functionality to Command enums
    Commands can be converted to strings with the as_str method
    
    Command     |     String format
    ---------------------------------------------------------
    Power       |  /Power (increased|decreased) by [0-9]+%/
    Missiles    |  /Missiles (increased|decreased) by [0-9]+/
    Shield      |  /Shield turned (on|off)/
    Try         |  /Call attempt failed/
    Invalid     |  /Not a command/
**/
impl Command {
    pub fn as_str (&self) -> String {
match self{
    &Command::Power(bool_val, num) => {if bool_val {format!("Power increased by {}%", num)}
        else {format!("Power decreased by {}%", num)}},
        &Command::Missiles(bool_val, num) => {if bool_val {format!("Missiles increased by {}", num)}
             else {format!("Missiles decreased by {}", num)}},
             &Command::Shield(bool_val) => {if bool_val {format!("Shield turned on")} else {format!("Shield turned off")}},
             &Command::Try => format!("Call attempt failed"),
             &Command::Invalid => format!("Not a command"),
}
}
}
/**
    Complete this method that converts a string to a command 
    We list the format of the input strings below

    Command     |     String format
    ---------------------------------------------
    Power       |  /power (inc|dec) [0-9]+/
    Missiles    |  /(fire|add) [0-9]+ missiles/
    Shield      |  /shield (on|off)/
    Try         |  /try calling Miss Potts/
    Invalid     |  Anything else
**/
pub fn to_command(s: &str) -> Command {
    if s.starts_with("power ")
    {
    let str1 = s.strip_prefix("power inc ");
    match str1{
    	  Some(num) =>{
    		  	if num.starts_with("-")
 			   {Command::Invalid}
 			      else
   			      {
   			     // let n:i32 = num.parse().unwrap();
   			      //Command::Power(true, n)
			      match num.parse::<i32>(){
			      Ok(num) => Command::Power(true, num),
			      Err(e) => Command::Invalid,
			      }

}
		},

	None => { let str2 = s.strip_prefix("power dec ");
	     	  match str2{
		  	Some(num) => {
					if num.starts_with("-")
					   {Command::Invalid}
					   else
					   {
					  // let n:i32 = num.parse().unwrap();
					   //Command::Power(false, n)
					   match num.parse::<i32>(){
					   Ok(num) => Command::Power(false, num),
					   Err(e) => Command::Invalid,}
					   }
			}
			None => Command::Invalid
			}
			},
			}
			}
  
    else if s.ends_with(" missiles")
    {
        let str1 = s.strip_suffix(" missiles");
	match str1{
	Some (x) => {let str2 = x.strip_prefix("add ");
                       match str2{
		       Some(num) => { if num.starts_with("-")
		                     {Command::Invalid}
				     else
				     { match num.parse::<i32>(){
				     Ok(num) => Command::Missiles(true, num),
				     Err(e) => Command::Invalid,
				     }
				     }
				     },
		      None => {let str3 = x.strip_prefix("fire ");
		      	      match str3{
			      Some(num) => { if num.starts_with("-")
			                    {Command::Invalid}
					    else
					    {match num.parse::<i32>(){
					    Ok(num) => Command::Missiles(false, num),
					    Err(e) => Command::Invalid,
					    }
					    }
					    },
		             None => Command::Invalid,
			     }
			     },
			     }
			     },
			     None => Command::Invalid,
			     }
			}	     
    


    else if s.starts_with("shield ")
    {
if s.strip_prefix("shield ") ==  Some("on")
{
    Command::Shield(true)
}
else if s.strip_prefix("shield ") == Some("off")
{Command::Shield(false)}
else {Command::Invalid}

    }

else if s == "try calling Miss Potts"
{Command::Try}

else{
Command::Invalid
 } 
}
