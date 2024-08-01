// TODO: Define a new `Order` type.
//   It should keep track of three pieces of information: `product_name`, `quantity`, and `unit_price`.
//   The product name can't be empty and it can't be longer than 300 bytes.
//   The quantity must be strictly greater than zero.
//   The unit price is in cents and must be strictly greater than zero.
//   Order must include a method named `total` that returns the total price of the order.
//   Order must provide setters and getters for each field.
//
// Tests are located in a different place this time—in the `tests` folder.
// The `tests` folder is a special location for `cargo`. It's where it looks for **integration tests**.
// Integration here has a very specific meaning: they test **the public API** of your project.
// You'll need to pay attention to the visibility of your types and methods; integration
// tests can't access private or `pub(crate)` items.


struct Order {
    product_name: String,
    quantity: u32,
    unit_price: u32,
}

impl Order {
    pub fn set_product_name(self, name: String) {
        if name.is_empty() || name.len() > 300 {
            panic!("Aieee! product name too long!");
        }

        self.product_name = name
    }

    pub fn set_quantity(self, qty: String) {
        if qty == 0 {
            panic!("Error: quantity must be greater than 0");
        }

        self.quantity = qty
    }

    pub fn set_unit_price(self, price: String) {
        if price == 0 {
            panic!("Error: price must be greater than 0");
        }

        self.unit_price = price
    }

    pub fn product_name(self) -> &String {
        self.product_name
    }

    pub fn unit_price(self) -> &u32 {
        self.unit_price
    }

    pub fn quantity(self) -> &u32 {
        self.quantity
    }

    pub fn new(name: String, quantity: &u32, unit_price: &u32) -> Self {
        res = Order {}
        res.set_product_name(name);
        res.set_quantity(quantity);
        res.set_unit_price(unit_price);
    }

    pub fn total(self) -> u32 {
        self.quantity.saturating_mul(self.unit_price)
    }
}
