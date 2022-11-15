use std::io;
struct Employee {
    name: String,
    employee_type: String,
    id: u32,
    salary: u32,
   
}
fn main() {

let mut auto_id = 1;


loop {
    
    let input_name = loop {
        let mut name =  String::new();
        println!("Employee Name");
         io::stdin()
            .read_line(&mut name)
            .expect("Failed to read line");
            match name.trim() {
                ""=> println!("Add valid Name"),
                _ => break name.to_owned().trim().to_string(),
            }
    };

    let input_type = loop {
        let mut e_type =  String::new();
        println!("Employee Type: a - Junior Engineer, b - Senior Engineer");
         io::stdin()
            .read_line(&mut e_type)
            .expect("Failed to read line");
            match e_type.trim() {
                "a"=> break "Junior Engineer".to_string(),
                "b" => break "Senior Engineer".to_string(),
                _ => println!("Please select valid employe type"),
            }
    };

    fn calculate_salary(e_type: &String) -> u32{
        println!("{}", e_type);
        match e_type.trim(){
            "Junior Engineer" => 50000,
            "Senior Engineer" => 60000,
            _ => 0,
        }
    }

    let salary = calculate_salary(&input_type);

    let new_employee = Employee{
        name: input_name,
        employee_type: input_type,
        salary,
        id: auto_id  
    };

    println!("====== Employee Details ======");
    println!("ID: {}", new_employee.id);
    println!("Name: {}", new_employee.name);
    println!("Position: {}", new_employee.employee_type);
    println!("Salary: {}", new_employee.salary);
    println!("====== Employee Details ======");

    auto_id += 1;
    let mut key = String::new();
    println!("Enter y to add more, enter any key or press enter to exit!");
    io::stdin()
            .read_line(&mut key)
            .expect("Failed to read line");
            match key.trim() {
                "y" => continue,
                _ => return,
     }
    }
}

