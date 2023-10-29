use subprocess::{Exec, Redirection};

fn main() {
    let out_and_err = Exec::cmd("ls")
        .stdout(Redirection::Pipe)
        .stderr(Redirection::Merge)
        .capture()?
        .stdout_str();
}
