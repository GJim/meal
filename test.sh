curl -X POST \
    -H "Content-Type: application/json" \
    -d '{"name":"牛肉便當", "price": 100}' \
    http://localhost:8000/product/add

curl -X GET http://localhost:8000/product/all

curl -X POST \
    -H "Content-Type: application/json" \
    -d '{"name":"週年慶", "initial": "2022-01-01 00:00:00", "finish": "2022-01-03 18:00:00"}' \
    http://localhost:8000/label/add

curl -X GET http://localhost:8000/label/all

curl -X POST \
    -H "Content-Type: application/json" \
    -d '{"name":"飯", "product": 1, "quantity": 1, "unit": "匙"}' \
    http://localhost:8000/ingredient/add

curl -X POST \
    -H "Content-Type: application/json" \
    -d '{"name":"蛋", "product": 1, "quantity": 2, "unit": "個"}' \
    http://localhost:8000/ingredient/add

curl -X GET http://localhost:8000/ingredient/all

curl -X POST \
    -H "Content-Type: application/json" \
    -d '{"product": 1, "price": 100, "labels": "週年慶,牛肉便當"}' \
    http://localhost:8000/sell/add

curl -X GET \
    -H "Content-Type: application/json" \
    -d '{"start": "2022-01-01 00:00:00", "end": "2025-01-01 00:00:00"}' \
    http://localhost:8000/sell/range