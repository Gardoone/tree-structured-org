use std::collections::HashMap;

/// Represents a user in the system.
struct User {
    id: String,
    parent: String,
    childs: Vec<String>,
    reports: u16
}

/// Creates a new `User` with the given `id` and `parent`.
///
/// # Arguments
/// * `id` - A reference to the user's ID.
/// * `parent` - A reference to the user's parent ID.
///
/// # Returns
/// A `User` struct with the specified ID and parent, an empty list of children, and zero reports.
fn build_user(id: &String, parent: &String) -> User {
    User {
        id: id.to_string(),
        parent: parent.to_string(),
        childs: Vec::new(),
        reports: 0,
    }
}

/// Initializes an empty database.
///
/// # Returns
/// An empty `HashMap` that maps certificates to `HashMap` of users.
fn build_database()-> HashMap<String, HashMap<String, User>> {

    let database: HashMap<String, HashMap<String, User>> = HashMap::new();

    return database;
}


/// Initializes an empty user certificate tree.
///
/// # Returns
/// An empty `HashMap` that maps user IDs to `User` structs.
fn build_certificate_tree () -> HashMap<String, User> {

    let users: HashMap<String, User> = HashMap::new();

    return users
}


/// Adds a new certificate tree to the database.
///
/// # Arguments
/// * `certificate` - A reference to the certificate name.
/// * `database` - A mutable reference to the database where the certificate tree will be added.
fn add_certificate_tree (certificate: &String, database: &mut HashMap<String, HashMap<String, User>>) {

    let users = build_certificate_tree ();

    database.insert(certificate.to_string(), users);   

}


/// Adds a new user to a specific certificate tree in the database.
///
/// # Arguments
/// * `id` - A reference to the new user's ID.
/// * `parent` - A reference to the parent user's ID.
/// * `certificate` - A reference to the certificate under which the user is added.
/// * `database` - A mutable reference to the database where the user will be added.
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


// Adds an admin user to a specific certificate tree.
///
/// # Arguments
/// * `certificate` - A reference to the certificate under which the admin will be added.
/// * `database` - A mutable reference to the database where the admin will be added.
fn add_admin(certificate: &String, database: &mut HashMap<String, HashMap<String, User>>)  {

    let admin_id = String::from("admin");
    let admin: User = User{ id: admin_id.clone(),
                            parent: admin_id.clone(),
                            childs: Vec::new(),
                            reports: 0};

    database.get_mut(certificate).unwrap().insert(admin_id, admin);
}


/// Creates a hierarchical user tree for testing purposes.
///
/// # Arguments
/// * `branch` - The number of child users each parent user will have at each level.
/// * `level` - The number of levels in the user hierarchy.
/// * `certificate` - A reference to the certificate under which the users will be created.
/// * `database` - A mutable reference to the database where the user tree will be added.
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


/// Prints information about a specific user.
///
/// # Arguments
/// * `id` - A reference to the user's ID.
/// * `certificate` - A reference to the certificate under which the user resides.
/// * `database` - A reference to the database from which user information will be retrieved.
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

/// Increments the report count for a user and all its ancestors.
///
/// # Arguments
/// * `id` - A reference to the user's ID.
/// * `certificate` - A reference to the certificate under which the user resides.
/// * `database` - A mutable reference to the database where the user's report count will be updated.
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