import os
from typing import List, Dict, Any, Optional
from enum import Enum
from fastapi import FastAPI
from openai import OpenAI
from openai.types.chat import ChatCompletion
from pydantic import BaseModel
import uvicorn
from fastapi.middleware.cors import CORSMiddleware
import json

client: Optional[OpenAI] = None
app = FastAPI()
app.add_middleware(
    CORSMiddleware,
    allow_origins=["*"],  # Allow all origins
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)


def get_open_ai_api_key() -> str:
    config_file_path: str = f"{os.path.dirname(os.path.dirname(os.path.abspath(__file__)))}/data/config.json"
    with open(config_file_path) as f:
        config: Dict[str, Any] = json.load(f)
    return config["api_key"]


def get_reply_from_gpt_4o(messages: List["Message"]) -> str:
    global client
    client = OpenAI(api_key=get_open_ai_api_key()) if client is None else client

    completion: ChatCompletion = client.chat.completions.create(
        model="gpt-4o",
        messages=list(map(lambda m: m.dict(), messages)),  # type: ignore[arg-type, return-value]
        temperature=1.0
    )

    return completion.choices[0].message.content


class RoleEnum(str, Enum):
    user: str = "user"
    assistant: str = "assistant"


class Message(BaseModel):
    role: RoleEnum
    content: str


@app.post("/ask")
def read_root(messages: List[Message]) -> Dict[str, str]:
    return {"reply": get_reply_from_gpt_4o(messages)}


if __name__ == "__main__":
    uvicorn.run(app, host="127.0.0.1", port=1999)
