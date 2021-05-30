#[cfg(test)]
mod tests {
    use markdown_uidl::parse;

    #[test]
    fn reads_input() {
        parse(" #Heading ");
    }
}
