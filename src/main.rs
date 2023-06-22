use microxdg::*;

fn main() -> Result<(), XdgError> {
    dbg!(std::mem::size_of::<Xdg>());
    dbg!(std::mem::size_of::<XdgApp>());
    dbg!(std::mem::size_of::<XdgError>());

    Ok(())
}
