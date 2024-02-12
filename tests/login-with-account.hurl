POST http://localhost:8111/user/signup

{
  "username": "iamatestuser2",
  "email": "testuser2@example.com",
  "password": "meaningless"
}

HTTP 200

[Asserts]
jsonpath "$.username" == "iamatestuser2"

POST http://localhost:8111/user/login

{
  "username": "iamatestuser2",
  "password": "meaningless"
}

HTTP 200

# Bad password
POST http://localhost:8111/user/login

{
  "username": "iamatestuser2",
  "password": "meaningful"
}

HTTP 400

[Asserts]
jsonpath "$.error" == "Wrong password"
