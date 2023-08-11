use crate::create_command::create_command;

pub fn push() {
    let mut git = create_command("git");
    git.arg("branch");

    let status = git.output().unwrap();

    let mut branch_take = String::new();

    let list_branch = String::from_utf8(status.stdout).unwrap();
    for line_index in list_branch.lines() {
        if line_index.contains("*") {
            let v: Vec<&str> = line_index.split(' ').collect();
            branch_take.push_str(v[1]);
        }
    }

    let mut git_push = create_command("git");
    git_push.args(["push", "origin", branch_take.as_str()]);

    let status = git_push.output().unwrap();
    println!("{}", String::from_utf8(status.stdout).unwrap());
}
