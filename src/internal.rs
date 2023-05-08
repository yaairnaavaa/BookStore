use crate::*;
use near_sdk::{CryptoHash};
use std::mem::size_of;

// Usado para generar un prefijo único en nuestras colecciones de almacenamiento (esto es para evitar colisiones de datos)
pub(crate) fn hash_account_id(account_id: &AccountId) -> CryptoHash {
    // Obtener el hash predeterminado
    let mut hash = CryptoHash::default();
    // Hacemos hash del ID de la cuenta y lo devolvemos
    hash.copy_from_slice(&env::sha256(account_id.as_bytes()));
    hash
}

impl Contract {
    // Añadimos un libro a un usuario
    pub(crate) fn internal_add_book_to_owner(
        &mut self,
        account_id: &AccountId,
        book_id: &u64,
    ) {
        // Obtener el conjunto de libros para la cuenta dada
        let mut book_set = self.books_per_owner.get(account_id).unwrap_or_else(|| {
            // Si la cuenta no tiene libros, creamos un nuevo UnorderedSet
            UnorderedSet::new(
                StorageKey::BooksPerOwnerInner {
                    account_id_hash: hash_account_id(&account_id),
                }
                .try_to_vec()
                .unwrap(),
            )
        });

        book_set.insert(book_id);

        self.books_per_owner.insert(account_id, &book_set);
    }

} 