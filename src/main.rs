use std::{collections::HashMap, fs::create_dir, time::TryFromFloatSecsError};

struct User {
    id: String,
    parent: String,
    childs: Vec<String>,
    reports: u16
}

fn build_user(id: String, parent: String) -> User {
    User {
        id: id,
        parent: parent,
        childs: Vec::new(),
        reports: 0,
    }
}

fn build_database()-> HashMap<String, HashMap<String, User>> {

    let mut database: HashMap<String, HashMap<String, User>> = HashMap::new();

    return database;

}


fn build_certificate_tree () -> HashMap<String, User> {

    let mut users: HashMap<String, User> = HashMap::new();

    return users
}


fn add_certificate_tree (certificate: String, mut database: HashMap<String, HashMap<String, User>>) -> HashMap<String, HashMap<String, User>> {

    let mut users = build_certificate_tree ();

    database.insert(certificate.clone(), users);   

    return database
}

fn add_user(id: String,
            parent: String,
            certificate: String,
            mut database: HashMap<String, HashMap<String, User>>) 
            -> HashMap<String, HashMap<String, User>> {

    let new_user: User = build_user(id.clone(), parent.clone());

    // Add User
    database.get_mut(&certificate).unwrap().insert(id.clone(), new_user);

    // Update Parent
    database.get_mut(&certificate).unwrap().get_mut(&parent).unwrap().childs.push(id.clone());


    /* Another implementation
    // Add User
    if let Some(db) = database.get_mut(&certificate) {
        db.insert(id.clone(), new_user);

        // Update Parent
        if let Some(user) = db.get_mut(&parent) {
            user.childs.push(id.clone());
        } 
    }    
    */


    return database
}


fn add_admin(certificate: String, mut database: HashMap<String, HashMap<String, User>>)
-> HashMap<String, HashMap<String, User>> {

    let admin_id = String::from("admin");
    let admin: User = User{ id: admin_id.clone(),
                            parent: admin_id.clone(),
                            childs: Vec::new(),
                            reports: 0};

    database.get_mut(&certificate).unwrap().insert(admin_id.clone(), admin);

    return database
}

fn make_user_tree_test(branch: u16,
                        level: u16,
                        certificate: String, 
                        mut database: HashMap<String, HashMap<String, User>>)
                        -> HashMap<String, HashMap<String, User>> {
    
    
    database = add_admin(certificate.clone(), database);

    let mut parents: Vec<String> = Vec::new();
    parents.push(String::from("admin"));

    for l in 1..level {

        let mut next_parents: Vec<String> = Vec::new();

        for parent in parents.iter() {

            for b in 1..branch {
    
                let id = format!("{parent}-{b}"); 
                database = add_user(id.clone(), parent.clone(), certificate.clone(), database);

                next_parents.push(id)
    
            }

        }

        parents = next_parents.clone();
        next_parents.clear();

    }

    return database;
}



fn print_user_info(id: String, certificate: String, database: &HashMap<String, HashMap<String, User>>) {

    println!("############ User Info ############");
    println!("user id: {}", id);
    println!("user certificate: {}", certificate);


    if let Some(db) = database.get(&certificate) {
        if let Some(user) = db.get(&id) {
            println!("user parent: {}", user.parent);
            println!("user childs: {:?}", user.childs);
            println!("user reports: {}", user.reports);
        } 
    }        

}


fn report_user(id: String, certificate: String, mut database: HashMap<String, HashMap<String, User>>) -> HashMap<String, HashMap<String, User>> {


    if let Some(db) = database.get_mut(&certificate) {

        if let Some(user) = db.get_mut(&id) {
            user.reports += 1;

        } else {
            println!("user does not exist!")
        }

    } else {
        println!("certificate does not exist!")
    }    

    return database
}


fn calculate_reports(id: String, certificate: String, database: HashMap<String, HashMap<String, User>>) -> u32 {

    let mut reports: u32 = 0;

    let mut current_childs: Vec<String> = Vec::new();
    let mut next_childs: Vec<String> = Vec::new();

    current_childs.push(id.clone());

    let mut index: u32 = 0;

    loop {

        for child in current_childs.iter() {

            if let Some(db) = database.get(&certificate) {
                if let Some(user) = db.get(&child.clone()) {
                    reports += user.reports as u32;
                    next_childs.append(&mut user.childs.clone());
                } else {
                    println!("user does not exist!")
                }
        
            } else {
                println!("certificate does not exist!")
            }              
        }

        current_childs = next_childs.clone();
        next_childs.clear();

        if current_childs.is_empty() {
            break;
        }
    }

    println!("number of reports for user branches: {reports}");

    return reports;
}

fn remove_user() {
    todo!()
}

fn main() {

    let mut database = build_database();

    database = add_certificate_tree (String::from("post"), database);
    database = add_certificate_tree (String::from("comment"), database);
    database = add_certificate_tree (String::from("view"), database);

    let certificate: String = String::from("post");

    database = make_user_tree_test(4,4, certificate.clone(), database);

    print_user_info(String::from("admin"), certificate.clone(), &database);
    print_user_info(String::from("admin-2"), certificate.clone(), &database);
    print_user_info(String::from("admin-2-1"), certificate.clone(), &database);

    database = report_user(String::from("admin-2-1"), certificate.clone(), database);
    database = report_user(String::from("admin-2-1"), certificate.clone(), database);
    database = report_user(String::from("admin-2-1-3"), certificate.clone(), database);

    print_user_info(String::from("admin-2-1"), certificate.clone(), &database);

    let reports = calculate_reports(String::from("admin"), certificate.clone(), database);
    

}
