# BookStore 📄

NEAR_ENV=mainnet ./build.sh

./build.sh

CONTRACT=bookstorebos.near

echo $CONTRACT

Inicializar contrato:

    near call $CONTRACT init_contract '{"owner_id":"'$CONTRACT'"}' --accountId $CONTRACT

Crear un libro:

    near call $CONTRACT create_book '{"title":"Luces de bohemia", "description":"Descripcion 1", "author":"Ramón del Valle-Inclán", "year":2001, "price":5, "stock":5}' --accountId yairnava.testnet

    near call $CONTRACT create_book '{"title":"Crimen y castigo", "description":"Descripcion 2", "author":"Fedor Dostoievski", "year":2005, "price":5, "stock":3}' --accountId yairnava.testnet

    near call $CONTRACT create_book '{"title":"100 años de Soledad", "description":"Descripcion 3", "author":"Gabriel García Márquez", "year":2003, "price":4, "stock":5}' --accountId yairnava.testnet

    near call $CONTRACT create_book '{"title":"La casa de los espíritus", "description":"Descripcion 4", "author":"Isabel Allende", "year":2011, "price":2, "stock":5}' --accountId yairnava.testnet

    near call $CONTRACT create_book '{"title":"El Buscón", "description":"Descripcion 5", "author":"Francisco de Quevedo", "year":1991, "price":5, "stock":1}' --accountId yairnava.testnet

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