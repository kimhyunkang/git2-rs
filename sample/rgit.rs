extern mod git2;

fn main_usage(program: &str) {
    println(fmt!("usage: %s <command> [<args>]", program));
    println("   init: Create an empty git repository");
    println("   clone: Clone a repository into a new directory");
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
    let cmd_args = vec::slice(args, 2, args.len());

    git2::threads_init();

    if cmd == ~"init" {
        cmd_init(cmd_args);
    } else if cmd == ~"clone" {
        cmd_clone(program, cmd_args);
    } else if cmd == ~"status" {
        cmd_status(program, cmd_args);
    } else {
        main_usage(program);
    }
}

fn get_current_repo() -> @git2::Repository {
    match git2::repository::discover(&".", false, &"") {
        Ok(dir) => match git2::repository::open(dir) {
            Ok(repo) => repo,
            Err(e) => fail!(copy e.message),
        },
        Err(e) => fail!(copy e.message),
    }
}

fn cmd_init(args: &[~str]) {
    let path =
    if args.len() == 0 {
        os::getcwd().to_str()
    } else {
        copy args[0]
    };

    match git2::repository::init(path, false) {
        Ok(_) => println(fmt!("Initialized empty Git repository in %s", path)),
        Err(e) => io::stderr().write_line(e.message),
    }
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

        match git2::repository::clone(origin, local_path) {
            Ok(_) => println("done"),
            Err(e) => io::stderr().write_line(e.message),
        }
    }
}

fn cmd_status(_: &str, _: &[~str]) {
    let repo = get_current_repo();

    let mut not_staged: ~[(~str, ~git2::GitStatus)] = ~[];
    let mut staged: ~[(~str, ~git2::GitStatus)] = ~[];

    match repo.status() {
        Ok(status) => {
            let head = repo.head();
            match head.get_ref().branch_name() {
                Some(branch) => println(fmt!("On branch %s", branch)),
                None => println("Not currently on any branch"),
            }

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
        },
        Err(e) => fail!(copy e.message),
    }
}
