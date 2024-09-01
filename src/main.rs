use std::{clone, collections::HashMap};

struct User {
    id: String,
    parent: String,
    childs: Vec<String>,
    reports: u16
}

fn build_user(id: &String, parent: &String) -> User {
    User {
        id: id.to_string(),
        parent: parent.to_string(),
        childs: Vec::new(),
        reports: 0,
    }
}

fn build_database()-> HashMap<String, HashMap<String, User>> {

    let database: HashMap<String, HashMap<String, User>> = HashMap::new();

    return database;
}


fn build_certificate_tree () -> HashMap<String, User> {

    let users: HashMap<String, User> = HashMap::new();

    return users
}


fn add_certificate_tree (certificate: &String, database: &mut HashMap<String, HashMap<String, User>>) {

    let users = build_certificate_tree ();

    database.insert(certificate.to_string(), users);   

}

fn add_user(id: &String,
            parent: &String,
            certificate: &String,
            database: &mut HashMap<String, HashMap<String, User>>) {

    let new_user: User = build_user(id, parent);

    // Add User
    database.get_mut(certificate).unwrap().insert(id.to_string(), new_user);

    // Update Parent
    database.get_mut(certificate).unwrap().get_mut(parent).unwrap().childs.push(id.to_string());


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
}


fn add_admin(certificate: &String, database: &mut HashMap<String, HashMap<String, User>>)  {

    let admin_id = String::from("admin");
    let admin: User = User{ id: admin_id.clone(),
                            parent: admin_id.clone(),
                            childs: Vec::new(),
                            reports: 0};

    database.get_mut(certificate).unwrap().insert(admin_id, admin);
}


fn make_user_tree_test(branch: u16,
                        level: u16,
                        certificate: &String, 
                        database: &mut HashMap<String, HashMap<String, User>>){
    
    add_admin(certificate, database);

    let mut parents: Vec<String> = Vec::new();
    parents.push(String::from("admin"));

    for _l in 1..level {

        let mut next_parents: Vec<String> = Vec::new();

        for parent in parents.iter() {

            for b in 1..branch {
    
                let id = format!("{parent}-{b}"); 
                add_user(&id, parent, certificate, database);

                next_parents.push(id)
    
            }

        }

        parents = next_parents.clone();
        next_parents.clear();

    }
}



fn print_user_info(id: &String, certificate: &String, database: &HashMap<String, HashMap<String, User>>) {

    println!("############ User Info ############");
    println!("user id: {}", id);
    println!("user certificate: {}", certificate);


    if let Some(db) = database.get(certificate) {
        if let Some(user) = db.get(id) {
            println!("user parent: {}", user.parent);
            println!("user childs: {:?}", user.childs);
            println!("user reports: {}", user.reports);
        } 
    }        

}


fn report_user(id: &String, certificate: &String, database: &mut HashMap<String, HashMap<String, User>>) {

    let mut current_id = id.clone();

    loop {
        let db = match database.get_mut(certificate) {
            Some(db) => db,
            None => {
                println!("certificate '{certificate}' does not exist!");
                break;
            }
        };

        let user = match db.get_mut(&current_id) {
            Some(user) => user,
            None => {
                println!("user '{current_id}' does not exist in certificate '{certificate}'!");
                break;
            }
        };

        user.reports += 1;

        let next_id = &user.parent;

        if *next_id == current_id {
            break;
        }

        current_id = next_id.to_string();
    }
}


fn remove_user() {
    todo!()
}


fn main() {

    let mut database = build_database();
 
    add_certificate_tree (&String::from("post"), &mut database);
    add_certificate_tree (&String::from("comment"), &mut database);
    add_certificate_tree (&String::from("view"), &mut database);    

    let certificate: String = String::from("post");

    make_user_tree_test(4,4, &certificate, &mut database);

    print_user_info(&String::from("admin"), &certificate, &database);
    print_user_info(&String::from("admin-2"), &certificate, &database);
    print_user_info(&String::from("admin-2-1"), &certificate, &database);

    report_user(&String::from("admin-2-1"), &certificate, &mut database);
    report_user(&String::from("admin-2-1"), &certificate, &mut database);
    report_user(&String::from("admin-2-1-3"), &certificate, &mut database);

    print_user_info(&String::from("admin-2-1"), &certificate, &database);
    print_user_info(&String::from("admin-2-1-3"), &certificate, &database);    

}