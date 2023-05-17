use std::collections::HashMap;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LazyOption, LookupMap, UnorderedMap, UnorderedSet};
use near_sdk::json_types::{Base64VecU8, U128};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{
    env, log, near_bindgen, AccountId, Balance, CryptoHash, PanicOnDefault, Promise, PromiseOrValue,
    PromiseResult, Gas, require, serde_json::json
};

use crate::internal::*;
mod internal;

// Estructura auxiliar para claves de las colecciones persistentes.
#[derive(BorshSerialize)]
pub enum StorageKey {
    BooksPerOwner,
    BooksPerOwnerInner { account_id_hash: CryptoHash }
}

// Estructura de dueño de un libro
#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct BookOwner {
    book_id : u64,
    owner : AccountId
}

// Estructura para un perfil
#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Profile {
    name: String,
    bio: String
}

// Estructura de un libro
#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Book {
    book_id : u64,
    title : String,
    description : String,
    author : String,
    year : u64,
    price : u64,
    stock : u64 
}

// Estructura de libro de un usuario
#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct BookUser {
    book_id : u64,
    title : String,
    description : String,
    author : String,
    year : u64
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    // Dueño del contrato
    pub owner_id: AccountId,

    // Almacenamiento de perfiles
    pub profiles: HashMap<AccountId, Profile>,

    // Almacenamiento de libros
    pub books: UnorderedMap<u64, Book>,

    // Almacenamiento de libros por usuario
    pub books_per_owner: LookupMap<AccountId, UnorderedSet<u64>>,


}

#[near_bindgen]
impl Contract {
    // Método de inicialización
    #[init]
    pub fn init_contract(owner_id: AccountId) -> Self {
        Self::new(
            owner_id
        )
    }

    #[init]
    pub fn new(owner_id: AccountId) -> Self {
        // Inicialización de las variables
        let this = Self {
            owner_id,
            profiles: HashMap::new(),
            books: UnorderedMap::new(b"m"),
            books_per_owner: LookupMap::new(StorageKey::BooksPerOwner.try_to_vec().unwrap()),

        };

        this
    }

    // Método para crear un perfil
    pub fn create_profile(&mut self, name: String, bio: String) -> String {
        let account = env::signer_account_id();

        let profile = self.profiles.get(&account);
        if profile.is_none() {
            let new_profile = Profile {
                name : name,
                bio : bio
            };

            self.profiles.insert(account.clone(),new_profile.clone());

            let formated_content=&json!({   
                "standard": "nep171",
                "version": "1.0.0",
                "event": "create_profile",
                "data":new_profile
            }).to_string(); 
            env::log_str(&format!("EVENT_JSON:{}",formated_content).to_string(),);
    
            return "Perfil creado con éxito".to_string();
        } else {
            return "Ya tienes un perfil creado".to_string();
        }
    }

    // Método pra consultar perfil de una cuenta
    pub fn get_profile(&self,account: AccountId) -> Profile {
        let profile = self.profiles.get(&account);

        if profile.is_none() {
            let null_profile = Profile {
                name : "".to_string(),
                bio : "".to_string()
            };

            return null_profile;

        } else {
            let info_profile = profile.unwrap();

            let profile = Profile {
                name : info_profile.name.to_string(),
                bio : info_profile.bio.to_string()
            };

            return profile;
        }
    }

    // Método para crear libro
    pub fn create_book(&mut self, title:String, description: String, author: String, year: u64, price: u64, stock: u64) -> String {
        // Validamos que solo el dueño del contrato pueda ejecutar el método
        //self.assert_owner();

        let book_id = self.books.len() as u64;

        let book = Book {
            book_id : book_id.clone(),
            title : title,
            description : description,
            author : author,
            year : year,
            price : price,
            stock : stock 
        };

        self.books.insert(&book_id, &book);

        let formated_content=&json!({   
            "standard": "nep171",
            "version": "1.0.0",
            "event": "create_book",
            "data":book
        }).to_string(); 
        env::log_str(&format!("EVENT_JSON:{}",formated_content).to_string(),);

        return "Libro registrado con éxito".to_string();
    }

    // Método para consultar todos los libros
    pub fn all_books(&self, from_index: Option<U128>, limit: Option<u64>) -> Vec<Book> {
        let start = u128::from(from_index.unwrap_or(U128(0)));

        self.books.keys()
            .skip(start as usize) 
            .take(limit.unwrap_or(50) as usize) 
            .map(|book_id| self.get_book(book_id.clone()).unwrap())
            .collect()
    }

    // Método para consultar libros de un usuario
    pub fn books_for_owner(&self, account_id: AccountId, from_index: Option<U128>, limit: Option<u64>) -> Vec<BookUser> {
        let books_for_owner_set = self.books_per_owner.get(&account_id);
        let books = if let Some(books_for_owner_set) = books_for_owner_set {
            books_for_owner_set
        } else {
            return vec![];
        };

        let start = u128::from(from_index.unwrap_or(U128(0)));

        books.iter()
            .skip(start as usize) 
            .take(limit.unwrap_or(50) as usize) 
            .map(|books_id| self.get_book_user(books_id.clone()).unwrap())
            .collect()
    }

    // Método para obtener la información de un libro por su id
    pub fn get_book(&self, book_id: u64) -> Option<Book> {
        if let Some(book) = self.books.get(&book_id) {
            let book_data = self.books.get(&book_id).unwrap();
            Some(Book {
                book_id : book_id,
                title : book_data.title,
                description : book_data.description,
                author : book_data.author,
                year : book_data.year,
                price : book_data.price,
                stock : book_data.stock
            })
        } else {
            None
        }
    }

    // Método para consultar la información de un libro ya comprado
    pub fn get_book_user(&self, book_id: u64) -> Option<BookUser> {
        if let Some(book) = self.books.get(&book_id) {
            let book_data = self.books.get(&book_id).unwrap();
            Some(BookUser {
                book_id : book_id,
                title : book_data.title,
                description : book_data.description,
                author : book_data.author,
                year : book_data.year
            })
        } else {
            None
        }
    }

    // Método para comprar un libro
    #[payable]
    pub fn buy_book(&mut self, book_id: u64) -> String {
        let account_id = env::signer_account_id();
        let deposit = env::attached_deposit();
        
        if let Some(book) = self.books.get(&book_id) {
            let book_data = self.books.get(&book_id).unwrap();

    
            log!("deposit {}",deposit);
            log!("price {}",book_data.price);

            
            if book_data.stock == 0 {
                // Regresar dinero al usuario
                Promise::new(account_id.clone()).transfer(deposit);
                return "El libro ya no tiene stock".to_string();
            }

            // Verificar si este libro ya fue comprado
            if self.purchased_book(book_id.clone()) {
                // Regresar dinero al usuario
                Promise::new(account_id.clone()).transfer(deposit);
                return "Ya tienes comprado este libro".to_string();
            }

            // Verificar si el deposito es menor a el costo del libro
            // 5000000000000000000000000/1000000000000000000000000 = 5 < 5
            // 5000000000000000000000/1000000000000000000000 = 5 < 5
            if (deposit/1000000000000000000000) < book_data.price.into() {
                // Regresar dinero al usuario
                Promise::new(account_id.clone()).transfer(deposit);
                return "No depositaste el dinero suficiente".to_string();
            }

            let new_data = Book {
                book_id : book_id.clone(),
                title : book_data.title,
                description : book_data.description,
                author : book_data.author,
                year : book_data.year,
                price : book_data.price,
                stock : book_data.stock-1
            };

            self.books.insert(&book_id, &new_data);
            self.internal_add_book_to_owner(&env::signer_account_id(), &book_id);

            let book_owner = BookOwner {
                book_id : book_id,
                owner : env::signer_account_id(),
            };

            let formated_content=&json!({   
                "standard": "nep171",
                "version": "1.0.0",
                "event": "buy_book",
                "data":book_owner
            }).to_string();

            env::log_str(&format!("EVENT_JSON:{}",formated_content).to_string(),);

            // Transferir tokens
            Promise::new(self.owner_id.clone()).transfer(deposit as u128);

            return "Libro comprado con éxito".to_string();

        } else {
            // Regresar dinero al usuario
            Promise::new(account_id.clone()).transfer(deposit);
            return "No existe el libro ingresado".to_string();
        }
    }

    // Método para verificar si un usuario ya compró un libro
    pub fn purchased_book(&self, book_id: u64) -> bool {
        let account_id = env::signer_account_id();

        let books_for_owner_set = self.books_per_owner.get(&account_id);
        let books = if let Some(books_for_owner_set) = books_for_owner_set {
            books_for_owner_set
        } else {
            return false;
        };

        let exists = books.iter().filter_map(|book| {
            if book == book_id {
                Some(true)
            } else {
                None
            }
        }).any(|book| book);

        return exists;
    }

    // Método para validar si la cuenta que llama al contrato es el dueño
    fn assert_owner(&self) {
        require!(self.signer_is_owner(), "El método solo puede llamarlo el dueño del contrato")
    }

    fn signer_is_owner(&self) -> bool {
        self.is_owner(&env::predecessor_account_id())
    }

    fn is_owner(&self, minter: &AccountId) -> bool {
        minter.as_str() == self.owner_id.as_str()
    }

}


