fn parse(text: &str) -> Result<(), &'static str > {
    let table_name = text.split(" ").last();
    if table_name.is_none() {
        return Err("invalid command");
    }

    Ok(())
}