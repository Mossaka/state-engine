curl --request POST \
     --url http://localhost:3000/v1.0/state/statestore/ \
     --header "Content-Type: application/json" \
     --data '[{
       "key": "order",
       "value": {
         "data": {
           "orderId": "42"
         }
       }
     }]'

curl --request POST \
     --url http://localhost:8080/neworder \
     --header "Content-Type: application/json" \
     --data '{"data":{"orderId":"42"}}'

hey -m POST -H "Content-Type: application/json" -d '{"data":{"orderId":"42"}}' -n 1000 -c 10 http://localhost:8080/neworder