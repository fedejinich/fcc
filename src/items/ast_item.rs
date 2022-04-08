pub trait ASTItem {
    fn generate_assembly(&self) -> String;
}
