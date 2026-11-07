mod reading;

pub fn current() -> reading::Reading {
    reading::Reading { value: 1 }
}
