use std::collections::HashMap;

#[derive(Debug, Clone)]
struct User {
    id: u32,
    name: String,
}

fn find_in_vec(users: &[User], id: u32) -> Option<&User> {
    users.iter().find(|user| user.id == id)
}

fn main() {
    let users = vec![
        User {
            id: 10,
            name: "Ada".to_string(),
        },
        User {
            id: 20,
            name: "Grace".to_string(),
        },
        User {
            id: 30,
            name: "Linus".to_string(),
        },
    ];

    let users_by_id: HashMap<u32, User> =
        users.iter().cloned().map(|user| (user.id, user)).collect();

    println!("vec lookup: {:?}", find_in_vec(&users, 20));
    println!("hashmap lookup: {:?}", users_by_id.get(&20));

    let mut sorted = users.clone();
    sorted.sort_by(|left, right| left.name.cmp(&right.name));
    println!("sorted names:");
    for user in sorted {
        println!("{} {}", user.id, user.name);
    }
}
