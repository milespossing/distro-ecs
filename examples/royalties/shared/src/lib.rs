use datetime::{Instant};
use uuid::{Uuid};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

pub struct Transaction {
    pub timestamp: Instant,
    pub transaction_id: Uuid,
}

pub struct ProductTransaction {
    pub product_id: Uuid,
}

pub struct Quantity {
    pub quantity: f64,
}

pub struct Price {
    pub price: f64,
}
