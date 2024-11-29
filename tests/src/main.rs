
#[prsctx_attr::mark]
fn fn1() {
    prsctx::print_context_stack();
}

fn main() {
    prsctx::mark!("main");
    println!("1 --------");
    prsctx::print_context_stack();

    {
        prsctx::mark!("b1");

        {
            prsctx::mark!("b2");
            println!("2 --------");
            prsctx::print_context_stack();
        }
    }

    println!("3. --------");
    prsctx::print_context_stack();

    println!("4 --------");
    fn1();
}
