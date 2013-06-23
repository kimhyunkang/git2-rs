extern mod git2;
use std::os;

fn main_usage(program: &str) {
    println(fmt!("usage: %s <command> [<args>]", program));
    println("   add: Add file contents to the index");
    println("   init: Create an empty git repository");
    println("   clone: Clone a repository into a new directory");
    println("   rm: Remove files from the working tree and from the index");
    println("   status: Show the working tree status");
}

fn main() {
    let args = os::args();

    let program = copy args[0];

    if args.len() < 2 {
        main_usage(program);
        return;
    }

    let cmd = copy args[1];
    let cmd_args = args.slice(2, args.len());

    git2::threads_init();

    if cmd == ~"init" {
        cmd_init(cmd_args);
    } else if cmd == ~"clone" {
        cmd_clone(program, cmd_args);
    } else if cmd == ~"status" {
        cmd_status(program, cmd_args);
    } else if cmd == ~"add" {
        cmd_add(program, cmd_args);
    } else if cmd == ~"rm" {
        cmd_rm(program, cmd_args);
    } else {
        main_usage(program);
    }
}

fn get_current_repo() -> @mut git2::Repository {
    let dir = git2::repository::discover(&".", false, &"").get();
    git2::repository::open(dir).unwrap()
}

fn cmd_init(args: &[~str]) {
    let path =
    if args.len() == 0 {
        os::getcwd().to_str()
    } else {
        copy args[0]
    };

    git2::repository::init(path, false);
    println(fmt!("Initialized empty Git repository in %s", path));
}

fn clone_usage(program: &str) {
    println(fmt!("usage: %s clone <repo> [<dir>]", program));
}

fn cmd_clone(program: &str, args: &[~str]) {
    if args.len() == 0 {
        clone_usage(program);
    } else {
        let origin = copy args[0];
        let local_path = if args.len() < 2 {
            os::getcwd().to_str()
        } else {
            copy args[1]
        };

        git2::repository::clone(origin, local_path);
        println("done");
    }
}

fn cmd_status(_: &str, _: &[~str]) {
    let repo = get_current_repo();

    let mut not_staged: ~[(~str, ~git2::Status)] = ~[];
    let mut staged: ~[(~str, ~git2::Status)] = ~[];

    if repo.is_empty() {
        println("Empty repository")
    } else {
        let head = repo.head().unwrap();
        match head.branch_name() {
            Some(branch) => println(fmt!("On branch %s", branch)),
            None => println("Not currently on any branch"),
        }
    }

    let status = repo.status();

    for status.each() |&tup| {
        let (path, stat) = tup;
        if (stat.index_new || stat.index_modified || stat.index_deleted || stat.index_renamed
            || stat.index_typechange)
        {
            staged.push((copy path, copy stat))
        }
        if stat.wt_new || stat.wt_modified || stat.wt_deleted || stat.wt_typechange {
            not_staged.push((copy path, copy stat))
        }
    }

    if staged.is_empty() && not_staged.is_empty() {
        println("nothing to commit (working directory clean)");
        return;
    }

    if !staged.is_empty() {
        println("Changed staged for commit")
    }
    for staged.each() |&tup| {
        let (path, stat) = tup;
        if stat.index_new {
            print("new: ")
        } else if stat.index_modified {
            print("modified: ")
        } else if stat.index_deleted {
            print("deleted: ")
        } else if stat.index_renamed {
            print("renamed: ")
        } else if stat.index_typechange {
            print("typechange: ")
        }

        println(path)
    }

    if !not_staged.is_empty() {
        println("Changed not staged for commit")
    }
    for not_staged.each() |&tup| {
        let (path, stat) = tup;
        if stat.wt_new {
            print("new: ")
        } else if stat.wt_modified {
            print("modified: ")
        } else if stat.wt_deleted {
            print("deleted: ")
        } else if stat.wt_typechange {
            print("typechange: ")
        }

        println(path)
    }
}

fn add_usage(program: &str) {
    println(fmt!("usage: %s add <filename>", program));
}

fn cmd_add(program: &str, args: &[~str]) {
    if args.len() == 0 {
        add_usage(program);
    } else {
        let path = copy args[0];
        let repo = get_current_repo();
        let mut index = repo.index().unwrap();
        index.add_bypath(path);
        index.write();
    }
}

fn rm_usage(program: &str) {
    println(fmt!("usage: %s rm <filename>", program));
}

fn cmd_rm(program: &str, args: &[~str]) {
    if args.len() == 0 {
        rm_usage(program);
    } else {
        let path = copy args[0];
        let repo = get_current_repo();
        let mut index = repo.index().unwrap();
        index.remove_bypath(path);
        index.write();
    }
}
