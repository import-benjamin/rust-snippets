use git2::Repository;

fn main() {
    let repo = match Repository::open(".") {
        Ok(repo) => repo,
        Err(e) => panic!("failed to open: {}", e),
    };
    let head = repo.head().expect("Can't read HEAD.");
    
    println!("{}", head.name().unwrap());
}
