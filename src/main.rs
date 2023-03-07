use parsed::UnusedParser;
mod parsed;

fn main() {
    let path = "./src/data/test.ts";
    let parser = UnusedParser::new(path);

    dbg!(&parser.module);
}
