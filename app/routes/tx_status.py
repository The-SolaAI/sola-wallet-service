from fastapi import APIRouter, HTTPException
import os
from dotenv import load_dotenv
import requests
from pydantic import BaseModel


class StatusRequest(BaseModel):
    tx_id: str


load_dotenv()
crossmint_api_key = os.getenv("CROSSMINT_API_KEY")
tx_status_router = APIRouter()

# using dummy wallet
address = "8AhB8fShc1rSfeJerM8JdUNux534HUuhRSgDBQHL8E4n"

#TODO : add checks
@tx_status_router.post("/tx-status")
def trasfer(request: StatusRequest):
    headers = {"X-API-KEY": crossmint_api_key}

    try:
        tx_id = request.tx_id
        status_url = f"https://staging.crossmint.com/api/v1-alpha2/wallets/{address}/transactions/{tx_id}"

        status_response = requests.get(status_url, headers=headers)
        status_details = status_response.json()
        
        
        return {
            "status": "success",
            "message": "this is the status of the transaction",
            "data": {"transaction-status": status_details["status"], "error": False},
        }
    except:
        raise HTTPException(status_code=500, detail="Internal server error")
