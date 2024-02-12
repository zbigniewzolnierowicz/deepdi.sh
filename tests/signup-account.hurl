POST http://localhost:8111/user/signup

{
  "username": "iamatestuser1",
  "email": "testuser1@example.com",
  "password": "meaningless"
}

HTTP 200

[Asserts]
jsonpath "$.username" == "iamatestuser1"
