extern mod git2;

fn print_usage(program: &str) {
    println(fmt!("usage: %s <command> [<args>]", program));
    println("   init: Create an empty git repository");
}

fn main() {
    let args = os::args();

    let program = copy args[0];

    if args.len() < 2 {
        print_usage(program);
        return;
    }

    let cmd = copy args[1];

    if cmd == ~"init" {
        cmd_init(vec::slice(args, 2, args.len()));
    } else {
        print_usage(program);
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
