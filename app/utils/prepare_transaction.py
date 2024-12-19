from solders.pubkey import Pubkey
from solders.transaction import Transaction
from solders.message import Message
from solders.hash import Hash
from solana.rpc.api import Client
from spl.token.constants import TOKEN_PROGRAM_ID
from spl.token.instructions import (
    transfer,
    TransferParams,
    get_associated_token_address,
    create_associated_token_account,
)
import base58
import os
from dotenv import load_dotenv
from app.data.token_data import TOKENS

load_dotenv()
rpc = os.getenv("SOLANA_RPC")

client = Client(rpc)

sender_address = Pubkey.from_string("8AhB8fShc1rSfeJerM8JdUNux534HUuhRSgDBQHL8E4n")
recipient_address = Pubkey.from_string("5oJ9ou71kxCG1DjRx3ZSLnrjufd7h76dnBEXyE6PsyD4")

def prepare_transaction(send_token: str, amount: int):
    token_mint = Pubkey.from_string(TOKENS[send_token]["MINT"])
    decimals = TOKENS[send_token]["DECIMALS"]
    
    amount = amount* 10**decimals
    
    sender_token_account = get_associated_token_address(sender_address, token_mint)
    print("here 3")
    recipient_token_account = get_associated_token_address(recipient_address, token_mint)
    recipient_account_info = client.get_account_info(recipient_token_account)
    
    instructions = []
    
    if not recipient_account_info.value:
        instructions.append(
            create_associated_token_account(
                payer=sender_address, owner=recipient_address, mint=token_mint
            )
        )

    transfer_instruction = transfer(
        TransferParams(
            program_id=TOKEN_PROGRAM_ID,
            source=sender_token_account,
            dest=recipient_token_account,
            owner=sender_address,
            amount=amount,
            signers=[],
        )
    )
    instructions.append(transfer_instruction)


    message = Message.new_with_blockhash(
        instructions,
        Pubkey.from_string("11111111111111111111111111111112"),
        Hash.from_string("11111111111111111111111111111111"),
    )

    transaction = Transaction.new_unsigned(message)

    serialized_transaction = bytes(transaction)
    serialized_transaction_str = base58.b58encode(serialized_transaction).decode("ascii")

    return serialized_transaction_str
