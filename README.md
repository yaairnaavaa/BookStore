# BookStore 📄

CONTRACT=dev-1682014826037-50515912441276

echo $CONTRACT

Inicializar contrato:

near call $CONTRACT init_contract '{"owner_id":"'$CONTRACT'"}' --accountId $CONTRACT

Crear un libro:

    near call $CONTRACT create_book '{"title":"Libro 5", "description":"Descripcion 5", "author":"Autor 5", "year":2023, "price":5, "stock":5}' --accountId yairnava.testnet

Consultar todos los libros

    near view $CONTRACT all_books '{"from_index": "0", "limit": 50}'

Consultar todos los libros de una cuenta

    near view $CONTRACT books_for_owner '{"account_id": "yairnava.testnet", "from_index": "0", "limit": 50}'

Consultar libro por id

    near view $CONTRACT get_book '{"book_id": 1}'

Comprar un libro:

    near call $CONTRACT buy_book '{"book_id": 3}' --accountId yairnava.testnet

Crear perfil:

    near call $CONTRACT create_profile '{"name": "Yair N", "bio": "Descripción de perfil"}' --accountId yairnava.testnet

Consultar perfil:

    near view $CONTRACT get_profile '{"account": "yairnava.testnet"}'