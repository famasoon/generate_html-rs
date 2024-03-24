use clap::{clap_app, crate_version};

fn main() {
    let clap = clap_app!(mdrend =>
        (version: crate_version!())
        (author: "FAMASoon")
        (about: "Renders markdown as you like")
        (@arg input: +required "sets the input file")
    )
    .get_matches();
    println!("Input = {:?}", clap.value_of("input"));
    println!("done")
}
