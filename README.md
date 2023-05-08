# BookStore 📄

CONTRACT=dev-1683517949014-68941680708437

echo $CONTRACT

Compilar y desplegar contrato:

    ./build.sh

Inicializar contrato:

near call $CONTRACT init_contract '{"owner_id":"'$CONTRACT'"}' --accountId $CONTRACT

Crear un libro:

    near call $CONTRACT create_book '{"title":"Libro 1", "description":"Descripcion 1", "author":"Autor 1", "year":2023, "price":5, "stock":5}' --accountId $CONTRACT

    near call $CONTRACT create_book '{"title":"Libro 2", "description":"Descripcion 2", "author":"Autor 2", "year":2023, "price":5, "stock":5}' --accountId $CONTRACT

Consultar todos los libros

    near view $CONTRACT all_books '{"from_index": "0", "limit": 50}'

Consultar todos los libros de una cuenta

    near view $CONTRACT books_for_owner '{"account_id": "yairnava.testnet", "from_index": "0", "limit": 50}'

Consultar libro por id

    near view $CONTRACT get_book '{"book_id": 1}'

Comprar un libro:

    near call $CONTRACT buy_book '{"book_id": 0}' --accountId yairnava.testnet --deposit 5

Crear perfil:

    near call $CONTRACT create_profile '{"name": "Yair N", "bio": "Descripción de perfil"}' --accountId yairnava.testnet

Consultar perfil:

    near view $CONTRACT get_profile '{"account": "yairnava.testnet"}'