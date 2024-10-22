curl --request POST \
     --url http://localhost:3000/v1.0/state/statestore/ \
     --header "Content-Type: application/json" \
     --data '{
       "key": "order",
       "value": {
         "data": {
           "orderId": "42"
         }
       }
     }'