from fastapi import FastAPI, Request
import requests
from .analyzer import analyze

app = FastAPI(title="Radhe API")

@app.post("/analyze")
async def post_analyze(request: Request):
    result = await request.json()
    analysis = analyze(result)
    return analysis

@app.get("/health")
async def get_health():
    ollama_available = False
    try:
        response = requests.get("http://localhost:11434/api/tags", timeout=2)
        if response.status_code == 200:
            ollama_available = True
    except Exception:
        pass
        
    return {
        "status": "ok",
        "ollama_available": ollama_available
    }
