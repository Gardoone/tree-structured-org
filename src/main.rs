use std::collections::HashMap;

struct Statistics {
    user_add: u32,
    user_update: u32,
    user_read: u32,
    user_tree_add: u32,
    user_tree_update: u32
}

impl Default for Statistics {
    fn default() -> Statistics {
        Statistics {
            user_add: 0,
            user_update: 0,
            user_read: 0,
            user_tree_add: 0,
            user_tree_update: 0
        }
    }
}

/// Represents a user in the system.
struct User {
    id: String,
    parent: String,
    children: Vec<String>,
    reports: u16,
    blocked: bool,
}

fn build_user(id: &String, parent: &String) -> User {
    User {
        id: id.to_string(),
        parent: parent.to_string(),
        children: Vec::new(),
        reports: 0,
        blocked: false,
    }
}

fn build_database()-> HashMap<String, HashMap<String, User>> {

    let database: HashMap<String, HashMap<String, User>> = HashMap::new();

    return database;
}

fn build_user_tree () -> HashMap<String, User> {

    let users: HashMap<String, User> = HashMap::new();

    return users
}


/// Adds a new user tree to the database based on certificate
fn add_user_tree (certificate: &String, database: &mut HashMap<String, HashMap<String, User>>) -> Result<(), String> {

    if database.contains_key(certificate) {

        return Err(format!("User tree under certificate '{certificate}' already exists in database."));
    }

    let users = build_user_tree ();

    database.insert(certificate.to_string(), users);   


    // Assign an admin
    let admin_id = String::from("admin");
    let admin: User = User{ id: admin_id.clone(),
                            parent: admin_id.clone(),
                            children: Vec::new(),
                            reports: 0, 
                            blocked: false};


    database.get_mut(certificate).unwrap().insert(admin_id, admin);

    println!("Added user tree under certificate '{certificate}'");

    Ok(())
}


/// Checks if a user has permission in a user tree.
fn check_user_permission(user_id: &String, 
                         certificate: &String, 
                         database: &HashMap<String, HashMap<String, User>>) 
                         -> Result<bool, String> {

    let mut current_id = user_id.clone();

    match database.get(certificate) {

        Some(users) => {

            loop {

                match users.get(&current_id) {

                    Some(user) => {

                        if user.blocked {
                            return Ok(false);
                        }

                        // Only admin has identical user id and parent id
                        if user.parent == current_id {
                            break;
                        }

                        current_id = user.parent.clone();                        

                    },

                    None => {
                        return Err(format!("user '{current_id}' not found in certificate tree '{certificate}'."));
                    }
                };
            }
        },

        None => {
            return Err(format!("certificate '{certificate}' not found in database."));
        }
    };

    // println!("user {user_id} has permission in certificate '{certificate}'");

    return Ok(true);
}


/// Add a user to user tree in the database based on certificate
fn add_user(user_id: &String,
            parent: &String,
            certificate: &String,
            database: &mut HashMap<String, HashMap<String, User>>) 
            -> Result<(), String> {   

    if *user_id == *parent {
        return Err(format!("parent id must not be intentical to user id"));
    }

    match check_user_permission(parent, certificate, &database) {
        Ok(x) => {
            if !x {
                return Err(format!("parent {parent} does not have permission."))
            }
        }

        Err(error) => {
            return Err(format!("Error for parent {parent}: ") + &error)
        }
    }

    match database.get_mut(certificate) {

        Some(users) => {

            if users.contains_key(user_id) {
                return Err(format!("user '{user_id}' already exists in database."));
            }

            // Update Parent
            match users.get_mut(parent) {

                Some(user) => {

                    if user.children.contains(user_id) {
                        return Err(format!("parent '{parent}' already added user '{user_id}' in database."));
                    }

                    user.children.push(user_id.to_string());
                },

                None => {
                    return Err(format!("parent '{parent}' not found in certificate tree '{certificate}'."));
                }
            };

            // Add User
            let new_user: User = build_user(user_id, parent);
            users.insert(user_id.clone(), new_user);


        },

        None => {
            return Err(format!("certificate '{certificate}' not found in database."));
        }
    };

    println!("user '{user_id}' added by parent '{parent}' under certificate '{certificate}' in database");

    Ok(())

}



/// Increments the report count for a user and all its ancestors.
fn report_user(user_id: &String, 
               certificate: &String, 
               database: &mut HashMap<String, HashMap<String, User>>) 
               -> Result<(), String> {

    let mut current_id = user_id.clone();

    match database.get_mut(certificate) {

        Some(users) => {

            loop {

                match users.get_mut(&current_id) {

                    Some(user) => {

                        user.reports += 1;

                        // Only admin has identical user id and parent id
                        if user.parent == current_id {
                            break;
                        }

                        current_id = user.parent.clone();                        

                    },

                    None => {
                        println!("user '{current_id}' not found in certificate tree '{certificate}'.");
                    }
                };
            }
        },

        None => {
            return Err(format!("certificate '{certificate}' not found in database."));
        }
    };

    Ok(())    
}


/// Blocks a user from having permission in a certificate tree.
fn block_user(user_id: &String, 
              blocker: &String, 
              certificate: &String, 
              database: &mut HashMap<String, HashMap<String, User>>) 
              -> Result<(), String> {

    match database.get_mut(certificate) {

        Some(users) => {

            match users.get_mut(user_id) {

                Some(user) => {

                    if user.blocked {
                        return Err(format!("user {user_id} has already been blocked."));
                    }

                    if !(user.parent == *blocker) {

                        if !(*user_id == *blocker) {
                            return Err(format!("Only user's parent or themselves can block the user."));
                        }
    
                    }

                    user.blocked = true;                    

                },

                None => {
                    return Err(format!("user '{user_id}' not found in certificate tree '{certificate}'."));
                }
            };
        },

        None => {
            return Err(format!("certificate '{certificate}' not found in database."));
        }
    };

    println!("user {user_id} blocked under certificate '{certificate}'");

    Ok(())        

}


fn unblock_user(user_id: &String, 
                unblocker: &String, 
                certificate: &String, 
                database: &mut HashMap<String, HashMap<String, User>>) 
                -> Result<(), String> {

    // Check if unblocker has permission
    match check_user_permission(unblocker, certificate, &database) {
        Ok(x) => {
            if !x {
                return Err(format!("unblocker {unblocker} does not have permission."))
            }
        }
        Err(error) => {
            return Err(format!("Error for unblocker {unblocker}: ") + &error)
        }
    }

    // Check if user has already blocked
    match check_user_permission(user_id, certificate, &database) {
        Ok(x) => {
            if x {
                return Err(format!("user {user_id} already has permission."))
            }
        }
        Err(error) => {
            return Err(format!("Error for user {user_id}: ") + &error)
        }
    }

    // Move user from a parent to another
    match database.get_mut(certificate) {
        Some(users) => {

            let previous_parent;

            // Unblock user and update its parent
            match users.get_mut(user_id) {

                Some(user) => {

                    previous_parent = user.parent.clone();

                    user.blocked = false;
                    user.parent = unblocker.to_string();
                },

                None => {
                    return Err(format!("user '{user_id}' not found in certificate tree '{certificate}'."));
                }
            };

            // Remove user from previous parent's children
            match users.get_mut(&previous_parent) {

                Some(user) => {

                    let index = user.children.iter().position(|x| *x == user_id.clone()).unwrap();
                    user.children.remove(index);
                    
                },
                
                None => {
                    return Err(format!("user '{user_id}' not found in certificate tree '{certificate}'."));
                }
            };

            // Update unblocker's children
            match users.get_mut(unblocker) {

                Some(user) => {

                    user.children.push(user_id.to_string());
                    
                },
                
                None => {
                    return Err(format!("user '{user_id}' not found in certificate tree '{certificate}'."));
                }
            };            


        },

        None => {
            return Err(format!("certificate '{certificate}' not found in database."));
        }
    };

    println!("user {user_id} unblocked under certificate '{certificate}'");
    Ok(())

}


/// Creates a hierarchical user tree for testing purposes.
fn make_user_tree_test(branch: u16,
                        level: u16,
                        certificate: &String, 
                        database: &mut HashMap<String, HashMap<String, User>>) {

    let mut parents: Vec<String> = Vec::new();
    parents.push(String::from("admin"));

    for _l in 1..level {

        let mut next_parents: Vec<String> = Vec::new();

        for parent in parents.iter() {

            for b in 1..branch {
    
                let id = format!("{parent}-{b}"); 
                let _ = add_user(&id, parent, certificate, database);

                next_parents.push(id)
            }
            
        }

        parents = next_parents.clone();
        next_parents.clear();

    }
    
}


/// Prints information about a specific user.
fn print_user_info(id: &String, certificate: &String, database: &HashMap<String, HashMap<String, User>>) {

    match database.get(certificate) {
        Some(users) => {
            match users.get(id) {
                Some(user) => {

                    println!("############ User Info ############");
                    println!("user id: {}", id);
                    println!("user certificate: {}", certificate);
                    println!("user parent: {}", user.parent);
                    println!("user children: {:?}", user.children);
                    println!("user reports: {}", user.reports);
                    println!("user blocked: {}", user.blocked);
                },
                None => {
                    println!("user '{id}' not found in certificate tree '{certificate}'.");
                }
            };
        },
        None => {
            println!("certificate '{certificate}' not found in database.");
        }
    };

}


/// For test
fn main() {

    let mut database = build_database();
    let mut statistics: Statistics = Default::default(); 
 
    if let Err(error) = add_user_tree(&String::from("post"), &mut database) {
        println!("{}", error)
    }

    if let Err(error) = add_user_tree(&String::from("comment"), &mut database) {
        println!("{}", error)
    }

    if let Err(error) = add_user_tree(&String::from("view"), &mut database) {
        println!("{}", error)
    }

    // Unsuccessful (user tree already exists)
    if let Err(error) = add_user_tree(&String::from("view"), &mut database) {
        println!("{}", error)
    }

    // Unsuccessful (certificate does not exists)
    if let Err(error) = check_user_permission(&String::from("hassan"), &String::from("jack"), &mut database) {
        println!("{}", error)
    }

    // Unsuccessful (identical parent and user ids)
    if let Err(error) = add_user(&String::from("hassan"), &String::from("hassan"), &String::from("post"),  &mut database) {
        println!("{}", error)
    }


    if let Err(error) = add_user(&String::from("hassan-1"), &String::from("hassan"), &String::from("post"),  &mut database) {
        println!("{}", error)
    }

    // Unsuccessful (user 'admin-1' already exists)
    if let Err(error) = add_user(&String::from("hassan-1"), &String::from("hassan"), &String::from("post"),  &mut database) {
        println!("{}", error)
    }


    // Unsuccessful (user 'admin-1' already exists)
    if let Err(error) = add_user(&String::from("hassan-3-1"), &String::from("hassan-3"), &String::from("post"),  &mut database) {
        println!("{}", error)
    }


    let certificate: String = String::from("post");

    make_user_tree_test(6,6, &certificate, &mut database);

    print_user_info(&String::from("admin"), &certificate, &database);
    print_user_info(&String::from("admin-2"), &certificate, &database);
    print_user_info(&String::from("admin-2-1"), &certificate, &database);

    if let Err(error) = report_user(&String::from("admin-2-1"), &certificate, &mut database) {
        println!("{}", error)
    }

    if let Err(error) = report_user(&String::from("admin-2-1"), &certificate, &mut database) {
        println!("{}", error)
    }

    if let Err(error) = report_user(&String::from("admin-2-1-3"), &certificate, &mut database) {
        println!("{}", error)
    }

    print_user_info(&String::from("admin-2-1"), &certificate, &database);
    print_user_info(&String::from("admin-2-1-3"), &certificate, &database); 


    match check_user_permission(&String::from("admin-2-1-3"), &certificate, &database) {
        Ok(x) => {
            if x {
                println!("user admin-2-1-3 has permission.")
            } else {
                println!("user admin-2-1-3 does not have permission.")
            }
        }

        Err(error) => {
            return println!("{}", error)
        }
    }

    if let Err(error) = block_user(&String::from("admin-2-1-3"), &String::from("admin-2-1-3"), &certificate, &mut database){
        println!("{}", error)
    }

    match check_user_permission(&String::from("admin-2-1-3"), &certificate, &database) {
        Ok(x) => {
            if x {
                println!("user admin-2-1-3 has permission.")
            } else {
                println!("user admin-2-1-3 does not have permission.")
            }
        }

        Err(error) => {
            return println!("{}", error)
        }
    }

    // the same user and blocker
    if let Err(error) = block_user(&String::from("admin-2-1-3"), &String::from("admin-2-1-3"), &certificate, &mut database){
        println!("{}", error)
    }    

    // Unsuccessful (blocker is not user's parent)
    if let Err(error) = block_user(&String::from("admin-3-1-3"), &String::from("admin-2-1-3"), &certificate, &mut database){
        println!("{}", error)
    }    


    //////////// Blocking a user by parent, unblocking again////////////////
    if let Err(error) = block_user(&String::from("admin-3-1-3"), &String::from("admin-3-1"), &certificate, &mut database){
        println!("{}", error)
    }    

    match check_user_permission(&String::from("admin-3-1-3-1-1"), &certificate, &database) {
        Ok(x) => {
            if x {
                println!("user admin-3-1-3-1-1 has permission.")
            } else {
                println!("user admin-3-1-3-1-1 does not have permission.")
            }
        }

        Err(error) => {
            return println!("{}", error)
        }
    }    

    if let Err(error) = unblock_user(&String::from("admin-3-1-3"), &String::from("admin-3-1"), &certificate, &mut database){
        println!("{}", error)
    }        

    match check_user_permission(&String::from("admin-3-1-3-1-1"), &certificate, &database) {
        Ok(x) => {
            if x {
                println!("user admin-3-1-3-1-1 has permission.")
            } else {
                println!("user admin-3-1-3-1-1 does not have permission.")
            }
        }

        Err(error) => {
            return println!("{}", error)
        }
    }       



    //////////// Blocking a user by parent, unblocking by another user////////////////
    if let Err(error) = block_user(&String::from("admin-4-1-3"), &String::from("admin-4-1"), &certificate, &mut database){
        println!("{}", error)
    }    

    match check_user_permission(&String::from("admin-4-1-3"), &certificate, &database) {
        Ok(x) => {
            if x {
                println!("user admin-4-1-3 has permission.")
            } else {
                println!("user admin-4-1-3 does not have permission.")
            }
        }

        Err(error) => {
            return println!("{}", error)
        }
    }    

    if let Err(error) = unblock_user(&String::from("admin-4-1-3"), &String::from("admin-5-1"), &certificate, &mut database){
        println!("{}", error)
    }            

    print_user_info(&String::from("admin-4-1-3"), &certificate, &database);


}