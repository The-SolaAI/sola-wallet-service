# Sola AI Wallet Service

## Endpoints

### Lulo Endpoints

POST /api/wallet/lulo/deposit

POST /api/wallet/lulo/withdraw

POST /api/wallet/lulo/update

GET /api/wallet/lulo/balance


### Jupiter Endpoints

POST /api/wallet/jupiter/swap

POST /api/wallet/jupiter/limit (TBA)

## To use

`Clone the Repo`

`cd sola-wallet-service`

create a `.env` file with the following params:

**->** LULO_API_KEY=""
  
`cargo run`
