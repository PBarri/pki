use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum CaCommand {
    #[structopt(name = "ca")]
    Foo(Foo),
}

#[derive(Debug, StructOpt)]
pub struct Foo {
    pub bar: Option<String>,
}