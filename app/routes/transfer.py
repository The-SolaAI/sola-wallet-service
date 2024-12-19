from fastapi import APIRouter, HTTPException
import os
from dotenv import load_dotenv
import requests
from app.utils.prepare_transaction import prepare_transaction
from pydantic import BaseModel


class TransferRequest(BaseModel):
    destination: str
    token: str
    amount: int


load_dotenv()
crossmint_api_key = os.getenv("CROSSMINT_API_KEY")
transfer_router = APIRouter()

# using dummy wallet
address = "8AhB8fShc1rSfeJerM8JdUNux534HUuhRSgDBQHL8E4n"
url = f"https://staging.crossmint.com/api/v1-alpha2/wallets/{address}/transactions"


@transfer_router.post("/transfer")
def trasfer(request: TransferRequest):
    headers = {"X-API-KEY": crossmint_api_key, "Content-Type": "application/json"}

    try:
        serialized_transaction_str = prepare_transaction(request.token, request.amount)

        payload = {
            "params": {
                "transaction": serialized_transaction_str,
            },
        }

        tx_response = requests.post(url, json=payload, headers=headers)
        tx_response_details = tx_response.json()
        tx_id = tx_response_details["id"]

        return {
            "status": "success",
            "message": "the transaction has been sent to blockchain",
            "data": {"transaction-id": tx_id, "error": False},
        }
    except:
        raise HTTPException(status_code=500, detail="Internal server error")
