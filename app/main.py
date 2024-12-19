from fastapi import FastAPI
from fastapi.middleware.cors import CORSMiddleware
from app.routes.health import health_router
from app.routes.create_wallet import create_wallet_router
from app.routes.fund_wallet import fund_wallet_router
from app.routes.transfer import transfer_router
from app.routes.tx_status import tx_status_router

app = FastAPI()

app.add_middleware(
    CORSMiddleware,
    allow_origins=["*"],
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)


@app.get("/")
def read_root():
    return {"message": "This is Sola AI wallet service"}


# health route
app.include_router(health_router, prefix="/wallet-api", tags=["Health"])

# create wallet route
app.include_router(create_wallet_router, prefix="/wallet-api", tags=["Create Wallet"])

# fund wallet 
app.include_router(fund_wallet_router,prefix="/wallet-api", tags=["Fund Wallet"])

# transfer tokens
app.include_router(transfer_router, prefix="/wallet-api", tags=["Transfer Tokens"])

# tx status
app.include_router(tx_status_router,prefix="/wallet-api",tags=["Transaction Status"])