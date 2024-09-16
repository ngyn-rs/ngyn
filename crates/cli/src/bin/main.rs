#![allow(clippy::unnecessary_wraps)]
#![allow(clippy::missing_const_for_fn)]

mod cmd;

fn main() {
    let mut app = cmd::default::command()
        .subcommand(cmd::generate::command())
        .subcommand(cmd::new::command());

    // let v = app.render_version();
    let matches = app.clone().get_matches();

    // use info! or trace! etc. to log
    cmd::tracing(&matches);

    let res = matches.subcommand().map_or_else(
        || cmd::default::run(&mut app, &matches),
        |tup| match tup {
            ("generate", subcommand_matches) => cmd::generate::run(&matches, &subcommand_matches),
            ("new", subcommand_matches) => cmd::new::run(&matches, &subcommand_matches),
            _ => unreachable!(),
        },
    );

    cmd::result_exit(res);
}
