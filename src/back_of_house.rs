pub mod ordering{
    pub fn deliver_order() {}

    pub fn fix_incorrect_order() {
        cook_order();
        deliver_order();
    }

    fn cook_order() {}
}