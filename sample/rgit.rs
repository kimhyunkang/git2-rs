extern mod git2;

fn main_usage(program: &str) {
    println(fmt!("usage: %s <command> [<args>]", program));
    println("   init: Create an empty git repository");
    println("   clone: Clone a repository into a new directory");
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

    if cmd == ~"init" {
        cmd_init(cmd_args);
    } else if cmd == ~"clone" {
        cmd_clone(program, cmd_args);
    } else {
        main_usage(program);
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
