use git2::Repository;

fn main() {
    let repo = match Repository::open(".") {
        Ok(repo) => repo,
        Err(e) => panic!("failed to open: {}", e),
    };
    let head = repo.head().expect("Can't read HEAD.");
    
    println!("{}", head.name().unwrap());

    let current_commit = head.peel_to_commit();
    println!("Message: {}", current_commit.unwrap().message().unwrap());
}
