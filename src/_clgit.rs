#![cfg_attr(external_doc, feature(external_doc))]
#![cfg_attr(external_doc, doc(include = "../Readme.md"))]
#![cfg_attr(external_doc, warn(missing_docs))]
#![forbid(unsafe_code)]

pub mod blob;           pub(crate)  use blob::Blob;
mod branch;             pub         use branch::*;
mod cat_file_reader;    pub(crate)  use cat_file_reader::*;
pub mod commit;         pub         use commit::Commit;
mod file_type;          pub         use file_type::*;
pub mod generic;        pub         use generic::HashParseError; pub(crate) use generic::HashTempStr;
mod name;               pub         use name::*;
mod repository;         pub         use repository::*;
mod repository_cache;   pub         use repository_cache::*;
mod shared_hash_map;    pub(crate)  use shared_hash_map::*;
pub mod tree;           pub         use tree::Tree;
pub mod unknown;



#[cfg(test)] mod tests {
    use super::*;

    #[test] fn basic_repository_tests() {
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
}
