pub trait ASTItem {
    fn generate_assembly(&self) -> String;
    fn pretty_print(&self) -> String;
}