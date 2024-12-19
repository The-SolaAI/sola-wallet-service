from fastapi import APIRouter,HTTPException
import os
from dotenv import load_dotenv
import requests

create_wallet_router = APIRouter()

load_dotenv()
crossmint_api_key = os.getenv("CROSSMINT_API_KEY")
url = "https://staging.crossmint.com/api/v1-alpha2/wallets"

#TODO : perform checks and add database mapping
@create_wallet_router.get("/create-wallet")
def create_wallet():
    headers = {"X-API-KEY": crossmint_api_key, "Content-Type": "application/json"}
    data = {
        "type": "solana-mpc-wallet",
        "linkedUser":"email:dummy@gmail.com"
    }
    try:
        response = requests.post(url, headers=headers, json=data)
        wallet_details = response.json()
        return {"status":"success", "message": "wallet created", "data":{"address":wallet_details["address"]},"error": False}
    except:
        raise HTTPException(status_code=500)
    
    
