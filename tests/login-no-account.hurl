POST http://localhost:8111/user/login

{
  "username": "idontexistyet",
  "password": "meaningless"
}

HTTP 404

[Asserts]
jsonpath "$.error" == "Could not find person"
