use once_cell::sync::Lazy;

static MODELS: Lazy<native_db::Models> = Lazy::new(|| {
    let mut models = native_db::Models::new();
    models.define::<data::v1::Person>().unwrap();
    models.define::<data::v1::Robot>().unwrap();
    models.define::<data::v2::Person>().unwrap();
    models.define::<data::v2::Robot>().unwrap();
    models
});

pub fn main() {
    println!("Opening database...");
    let db = native_db::Builder::new().create(&MODELS, "./db").unwrap();

    {
        println!("Running database migrations...");
        let rw = db.rw_transaction().unwrap();
        rw.migrate::<data::v2::Person>().unwrap();
        rw.migrate::<data::v2::Robot>().unwrap();
        rw.commit().unwrap();
        println!("Migrations successful! 😎");
    }
}

pub mod data {
    use native_db::{ToKey, native_db};
    use native_model::{Model, native_model};
    use serde::{Deserialize, Serialize};

    pub mod v1 {

        use super::*;

        #[derive(Clone, Serialize, Deserialize, Debug)]
        #[native_model(id = 1, version = 1)]
        #[native_db]
        pub struct Person {
            #[primary_key]
            pub name: String,
        }
        impl From<v2::Person> for Person {
            fn from(value: v2::Person) -> Self {
                Self { name: value.name }
            }
        }

        #[derive(Clone, Serialize, Deserialize, Debug)]
        #[native_model(id = 2, version = 1)]
        #[native_db]
        pub struct Robot {
            #[primary_key]
            pub number: String,
        }
        impl From<v2::Robot> for Robot {
            fn from(value: v2::Robot) -> Self {
                Self {
                    number: value.number,
                }
            }
        }
    }

    pub mod v2 {
        use super::*;

        #[derive(Clone, Serialize, Deserialize, Debug)]
        #[native_model(id = 1, version = 2, from = v1::Person)]
        #[native_db]
        pub struct Person {
            #[primary_key]
            pub name: String,
        }
        impl From<v1::Person> for Person {
            fn from(value: v1::Person) -> Self {
                Self { name: value.name }
            }
        }

        #[derive(Clone, Serialize, Deserialize, Debug)]
        #[native_model(id = 2, version = 2, from = v1::Robot)]
        #[native_db]
        pub struct Robot {
            #[primary_key]
            pub number: String,
        }
        impl From<v1::Robot> for Robot {
            fn from(value: v1::Robot) -> Self {
                Self {
                    number: value.number,
                }
            }
        }
    }
}
