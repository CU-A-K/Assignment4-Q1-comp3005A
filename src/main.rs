use std::io::{Read,stdin, Write};
use postgres::{*};
use chrono::NaiveDate;
struct Credentials{
    user:String,
    password:String
}
fn main() {
    let mut buf:String=String::new();//buffer for user input
    let mut cred=Credentials{user:String::new(),password:String::new()};//stored database credentials

    //get username
    if !get_user_input("Username:", &mut buf){return;}
    cred.user.push_str(buf.trim_end());
    buf.clear();

    //get password
    if !get_user_input("password:", &mut buf){return;}
    cred.password.push_str(buf.trim_end());
    buf.clear();

    //menu
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
            3=> update_student_email(&cred),
            4=> delete_student(&cred),
            5=>{
                println!("exiting program.");
                break;
            }
            _=>println!("invalid selection!"),
        }
        buf.clear();
    }
        
    
}
/*
*Function to get user input
* in:
*   msg: message prompt for user
* out:
*   buf: buffer for input
* return: bool, false if we get an error. true otherwise
*/
fn get_user_input(msg:&str,buf:&mut String)-> bool{
    print!("{msg}");
    std::io::stdout().flush().expect("flush failed!");
    
    if let Err(e)=stdin().read_line(buf){//tru to read user input. print error and return false if we fail.
        eprintln!("{e}");
        return false;
    }

    return true;
}
/*
* get students from database and print to stdout.
* credentials are used as input to open connection
*/
fn get_all_students(cred:&Credentials){
    //connect
    let  result = Client::configure()
                                         .user(cred.user.as_str())
                                         .host("localhost")
                                         .password(cred.password.as_str())
                                         .dbname("A4").connect(NoTls);//wrapped result of connection attempt
    
    let mut client;
    let mut buf:Vec<u8>=Vec::new();

    if let Err(e) = result{//check if we connected, otherwise print error and return
        eprintln!("{e}");
        return;
    }
    client = result.unwrap();

    //copy table to stdout
    match client.copy_out("COPY students TO stdout"){//check if table was copied correctly, otherwise print error and return
        Ok(mut r)=>{
            if let Err(e)=r.read_to_end(&mut buf){//try to read input, otherwise print error and return
                eprintln!("{e}");
                return;
            }
        }
        Err(e)=>eprintln!("{e}"),
    }

    //print table
    println!("ID      fname   lname   email                   enrollDate");
    for c in buf{ 
        print!("{}",c as char);
    }

    if let Err(e)=client.close(){//try to close connection, print error if we cant
        eprintln!("{e}")
    }
}
/*
* function to add new student
*/
fn add_student(cred:&Credentials){
    println!("New student setup");
    let mut buf:String=String::new();
    let mut fname:String=String::new();
    let mut lname:String=String::new();
    let mut email:String=String::new();
    let enroll_date:NaiveDate;

    //get first name
    if !get_user_input("First Name:", &mut buf){return;}//if we get a false return.
    fname.push_str(buf.trim_end());
    buf.clear();

    //get last name
    if !get_user_input("Last Name:", &mut buf){return;}
    lname.push_str(buf.trim_end());
    buf.clear();

    //get email
    if !get_user_input("Email:", &mut buf){return;}
    email.push_str(buf.trim_end());
    buf.clear();

    //get enroll date
    if !get_user_input("Enroll Date (yyyy-mm-dd):", &mut buf){return;}
    match NaiveDate::parse_from_str(buf.trim_end(),"%Y-%m-%d"){//try to parse date, if we cant print error and return.
        Ok(p_res)=>{
            enroll_date=p_res.to_owned();
        }
        Err(e)=>{
            eprintln!("{e}");
            return;
        }
    }
    buf.clear();

    //connect
    let  result = Client::configure()
                                         .user(cred.user.as_str())
                                         .host("localhost")
                                         .password(cred.password.as_str())
                                         .dbname("A4").connect(NoTls);//wrapped result of connection attempt
    if let Err(e) = result{//check if we connected, otherwise print error and return
        eprintln!("{e}");
        return;
    }

    let mut client = result.unwrap();
    match client.query("SELECT * FROM students WHERE email=$1", &[&email]){//need to manually check here because otherwise serial will increment without creating a new row
        Ok(result)=>{
            if !result.is_empty(){
                eprintln!("That email already exists!");
                return;
            }
        }
        Err(e)=>{
            eprintln!("{e}");
            return;
        }
    }

    //insert new student
    if let Err(e)=client.execute(
                  "INSERT INTO students(first_name,last_name,email,enrollment_date) VALUES($1,$2,$3,$4)",
                 &[&fname,&lname,&email,&enroll_date]){//try to insert student. print error and return otherwise
        eprintln!("{e}");
        return;
    }
    
    if let Err(e)=client.close(){
        eprintln!("{e}")
    }
}
fn update_student_email(cred:&Credentials){
    let id:i32;
    let mut email:String=String::new();
    let mut buf:String=String::new();

    //get student id
    if !get_user_input("Student ID:", &mut buf){return;}
    id=buf.trim_end().parse().unwrap_or(-1);
    buf.clear();
    if id==-1{
        println!("Error Invalid ID!");
        return;
    }

    //get new email
    if !get_user_input("New Email:", &mut buf){return;}
    email.push_str(buf.trim_end());
    buf.clear();

    //connect
    let  result = Client::configure()
                                         .user(cred.user.as_str())
                                         .host("localhost")
                                         .password(cred.password.as_str())
                                         .dbname("A4").connect(NoTls);//wrapped result of connection attempt
    if let Err(e) = result{//check if we connected, otherwise print error and return
        eprintln!("{e}");
        return;
    }
    let mut client = result.unwrap();

    //update student email
    if let Err(e)=client.execute(
                        "UPDATE students SET email=$1 WHERE student_id=$2", 
                        &[&email,&id]){//try to update email. print error and return if failed
        eprintln!("{e}");
        return;
    }

    if let Err(e)=client.close(){//try to close connection, print error if failed
        eprintln!("{e}")
    }
}

fn delete_student(cred:&Credentials){
    let mut buf:String=String::new();
    let id:i32;

    //get student id
    if !get_user_input("Student ID:", &mut buf){return;}
    id=buf.trim_end().parse().unwrap_or(-1);
    buf.clear();
    if id==-1{
        println!("Error Invalid ID!");
        return;
    }

    //connect
    let  result = Client::configure()
                                         .user(cred.user.as_str())
                                         .host("localhost")
                                         .password(cred.password.as_str())
                                         .dbname("A4").connect(NoTls);//wrapped result of connection attempt
    if let Err(e) = result{//check if we connected, otherwise print error and return
        eprintln!("{e}");
        return;
    }
    let mut client = result.unwrap();

    //delete student
    if let Err(e)=client.execute(
                        "DELETE FROM students WHERE student_id=$1",
                        &[&id]){
        eprintln!("{e}");
        return;
    }

    if let Err(e)=client.close(){
        eprintln!("{e}")
    }
}