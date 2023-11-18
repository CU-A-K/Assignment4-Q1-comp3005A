use std::io::{Read,stdin, Write};
use postgres::{*};
use chrono::NaiveDate;
struct Credentials{
    user:String,
    password:String
}
fn main() {
    let mut buf:String=String::new();
    let mut cred=Credentials{user:String::new(),password:String::new()};
    if !get_user_input("Username:", &mut buf){return;}
    cred.user.push_str(buf.trim_end());
    buf.clear();
    if !get_user_input("password:", &mut buf){return;}
    cred.password.push_str(buf.trim_end());
    buf.clear();
    let mut selection;
    loop {
        println!("select a function,\n 1:get_all_students()\n {}\n {}\n {}\n {}",
                                     "2:add_student(first name,last name,email, enrollment date)",
                                     "3:update_student_email(student id, new_email)",
                                     "4:delete_student(student id)",
                                     "5:exit");
        if !get_user_input("selection:", &mut buf){return;}
        selection=buf.trim_end().parse().unwrap_or(0);
        match selection {
            0=>println!("invalid selection!"),
            1=> get_all_students(&cred),
            2=> add_student(&cred),
            5=>{
                println!("exiting program.");
                break;
            }
            _=>println!("invalid selection!"),
        }
        buf.clear();
    }
        
    
}
fn get_all_students(cli:&Credentials){
    let  result = Client::configure()
                                         .user(cli.user.as_str())
                                         .host("localhost")
                                         .password(cli.password.as_str())
                                         .dbname("A4").connect(NoTls);
    let mut client;
    let mut buf:Vec<u8>=Vec::new();
    if let Err(e) = result{//check if we connected, otherwise print error and return
        eprintln!("{e}");
        return;
    }
    client = result.unwrap();
    match client.copy_out("COPY students TO stdout"){//check if table was copied correctly, otherwise print error and return
        Ok(mut r)=>{
            if let Err(e)=r.read_to_end(&mut buf){
                eprintln!("{e}");
                return;
            }
        }
        Err(e)=>eprintln!("{e}"),
    }
    println!("ID      fname   lname   email                   enrollDate");
    for c in buf{ 
        print!("{}",c as char);
    }
}
fn add_student(cli:&Credentials){
    println!("New student setup");
    let mut buf:String=String::new();
    let mut fname:String=String::new();
    let mut lname:String=String::new();
    let mut email:String=String::new();
    let enroll_date:NaiveDate;
    if !get_user_input("First Name:", &mut buf){return;}
    fname.push_str(buf.trim_end());
    buf.clear();
    if !get_user_input("Last Name:", &mut buf){return;}
    lname.push_str(buf.trim_end());
    buf.clear();
    if !get_user_input("Email:", &mut buf){return;}
    email.push_str(buf.trim_end());
    buf.clear();
    if !get_user_input("Enroll Date (yyyy-mm-dd):", &mut buf){return;}
    match NaiveDate::parse_from_str(buf.trim_end(),"%Y-%m-%d"){
        Ok(p_res)=>{
            enroll_date=p_res.to_owned();
        }
        Err(e)=>{
            eprintln!("{e}");
            return;
        }
    }
    buf.clear();
    let  result = Client::configure()
                                         .user(cli.user.as_str())
                                         .host("localhost")
                                         .password(cli.password.as_str())
                                         .dbname("A4").connect(NoTls);
    if let Err(e) = result{//check if we connected, otherwise print error and return
        eprintln!("{e}");
        return;
    }
    let mut client = result.unwrap();
    if let Err(e)=client.execute("INSERT INTO students(first_name,last_name,email,enrollment_date) VALUES($1,$2,$3,$4)", &[&fname,&lname,&email,&enroll_date]){
        eprintln!("{e}");
        return;
    }
}
fn get_user_input(msg:&str,buf:&mut String)-> bool{
    print!("{msg}");
    std::io::stdout().flush().expect("flush failed!");
    if let Err(e)=stdin().read_line(buf){
        eprintln!("{e}");
        return false;
    }
    return true;
}