use clgit::*;

fn main() {
    let repository = Repository::from_path(".").expect("Unable to open clgit repository");

    println!("Local Branches:");
    for branch in repository.local_branches().unwrap() {
        let branch = branch.unwrap();
        println!("* {} => {}", branch.name().to_string_lossy(), branch.commit());
    }
    println!();

    println!("Remote Branches:");
    for branch in repository.remote_branches().unwrap() {
        let branch = branch.unwrap();
        println!("* {} => {}", branch.name().to_string_lossy(), branch.commit());
    }
    println!();
}
