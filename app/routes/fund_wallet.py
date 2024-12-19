from fastapi import APIRouter,HTTPException
import os
from dotenv import load_dotenv
import requests

load_dotenv()
crossmint_api_key = os.getenv("CROSSMINT_API_KEY")
fund_wallet_router = APIRouter()

# using dummy wallet
address = "8AhB8fShc1rSfeJerM8JdUNux534HUuhRSgDBQHL8E4n"
url = f"https://staging.crossmint.com/api/v1-alpha2/wallets/{address}/balances"


@fund_wallet_router.get("/fund")
def fund_wallet():
    headers = {
        "X-API-KEY": crossmint_api_key,
        "Content-Type": "application/json"
    }
    
    payload = {
        "amount": 5,
        "currency": "usdc"
    }
    
    try:
        response = requests.post(url, json=payload, headers=headers)
        response_details = response.json()
        return {"status" : "success", "message" : "fundded 5 usdc to you wallet", "data":{"transaction-signature":response_details["txId"], "error": False}}
    except:
        raise HTTPException(status_code=500,detail="Internal server error")

