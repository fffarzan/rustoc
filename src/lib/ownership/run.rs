pub fn run() {
    let book = super::structs::Book {
        pages: 5,
        rating: 9,
    };
    super::display::page_count(&book);
    super::display::rating(&book);

    let tomato = super::structs::GroceryItem {
        quantity: 3,
        id: 1,
    };
    super::display::quantity(&tomato);
    super::display::id(&tomato);
}