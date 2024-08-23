use std::{collections::HashMap, time::TryFromFloatSecsError};

struct Certificates {
    post: bool,
    comment: bool,
    view: bool
}

impl Default for Certificates {
    fn default() -> Certificates {
        Certificates {
            post: false,
            comment: false,
            view: true,
        }
    }
}

struct User {
    id: String,
    parent: String,
    childs: Vec<String>,
    certificates: Certificates,
    reports: u16
}

// TODO: add certificates to parameters
fn build_user(id: String, parent: String) -> User {
    User {
        id: id,
        parent: parent,
        childs: Vec::new(),
        certificates: Default::default(),
        reports: 0,
    }
}

fn add_user(id: String,
            parent: String,
            mut users: HashMap<String, User>) 
            -> HashMap<String, User> {

    let new_user: User = build_user(id.clone(), parent.clone());
    users.insert(id.clone(), new_user);    

    if let Some(x) = users.get_mut(&parent) {
        x.childs.push(id.clone());
    } 
    
    return users
}


fn add_admin (mut users: HashMap<String, User>)
-> HashMap<String, User> {

    let admin_id = String::from("admin");
    let admin: User = User{ id: admin_id.clone(),
                            parent: admin_id.clone(),
                            childs: Vec::new(),
                            certificates: Default::default(),
                            reports: 0};
    users.insert(admin_id.clone(), admin);

    return users;
}

fn make_user_tree_test(branch: u16,
                        level: u16,
                        mut users: HashMap<String, User>)
                        -> HashMap<String, User> {
    
    
    users = add_admin(users);

    let mut parents: Vec<String> = Vec::new();
    parents.push(String::from("admin"));

    for l in 1..level {

        let mut next_parents: Vec<String> = Vec::new();

        for parent in parents.iter() {

            for b in 1..branch {
    
                let id = format!("{parent}-{b}"); 
                users = add_user(id.clone(), parent.clone(), users);

                next_parents.push(id)
    
            }

        }

        parents = next_parents.clone();
        next_parents.clear();

    }

    return users;
}



fn print_user_info(id: String, users: &HashMap<String, User>) {

    println!("############ User Info ############");
    println!("user id: {}", id);

    if let Some(value) = users.get(&id) {
        println!("user parent: {}", value.parent);
        println!("user childs: {:?}", value.childs);
        println!("user reports: {}", value.reports);
        println!("user certificates:");
        println!("post: {}", value.certificates.post);
        println!("comment: {}", value.certificates.comment);
        println!("view: {}", value.certificates.view);
    } else {
        println!("The user does not exist at dataset")
    }
}


fn report_user (id: String, mut users: HashMap<String, User>) -> HashMap<String, User> {

    if let Some(x) = users.get_mut(&id) {
        x.reports += 1;
    } else {
        println!("id does not exist!")
    }

    return users
}


fn calculate_reports(id: String, users: HashMap<String, User>) -> u32 {

    let mut reports: u32 = 0;

    let mut current_childs: Vec<String> = Vec::new();
    let mut next_childs: Vec<String> = Vec::new();

    current_childs.push(id);

    loop {

        // println!("{:?}", current_childs);
        
        for child in current_childs.iter() {

            if let Some(x) = users.get(child) {
                reports += x.reports as u32;
                next_childs.append(&mut x.childs.clone());
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

    let mut users: HashMap<String, User> = HashMap::new();

    users = make_user_tree_test(4,4, users);

    print_user_info(String::from("admin"), &users);
    print_user_info(String::from("admin-2"), &users);
    print_user_info(String::from("admin-2-1"), &users);

    users = report_user(String::from("admin-2-1"), users);
    users = report_user(String::from("admin-2-1"), users);
    users = report_user(String::from("admin-2-1-3"), users);

    print_user_info(String::from("admin-2-1"), &users);

    let reports = calculate_reports(String::from("admin"), users);
    

}
