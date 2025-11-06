pub fn page_count(book: &super::structs::Book) {
    println!("pages = {:?}", book.pages);
}

pub fn rating(book: &super::structs::Book) {
    println!("rating = {:?}", book.rating);
}

pub fn quantity(item: &super::structs::GroceryItem) {
    println!("quantity: {:?}", item.quantity);
}

pub fn id(item: &super::structs::GroceryItem) {
    println!("id: {:?}", item.id);
}