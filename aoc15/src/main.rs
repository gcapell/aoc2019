mod interpreter;
mod prog;

fn main() {
    let mut i = interpreter::new_channel(prog::PROG);
    i.send(1);
    i.send(1);
    i.send(1);
    i.send(1);
    i.send(1);
    i.send(1);
}
