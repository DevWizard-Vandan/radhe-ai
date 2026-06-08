import re

SYSTEM_PROMPT = """
You are Radhe, a quantitative research assistant for the
Indian stock market. You analyze backtest results and explain
them in clear, practical language for a quant researcher.
Be specific. Use numbers from the data. Do not speculate
beyond what the data shows.
"""

def build_prompt(result: dict) -> str:
    """
    Build a concise prompt from the KalpaRunner result dict.
    Include:
    - Instrument, date, sessions run
    - Average PnL, best/worst case
    - Win rate, Sharpe, max drawdown
    - Sessions profitable / total
    - Any risk flags present in the result (check result.get("risk_flags", []))
    Format as a structured text block, not JSON.
    Keep under 400 tokens.
    """
    instrument = result.get("instrument", "Unknown")
    date = result.get("date", "Unknown")
    sessions_run = result.get("sessions_run", 0)
    
    agg = result.get("aggregate", {})
    avg_pnl = agg.get("avg_pnl", 0.0)
    best_pnl = agg.get("best_pnl", 0.0)
    worst_pnl = agg.get("worst_pnl", 0.0)
    avg_win_rate = agg.get("avg_win_rate", 0.0)
    avg_sharpe = agg.get("avg_sharpe", 0.0)
    avg_max_drawdown = agg.get("avg_max_drawdown", 0.0)
    sessions_profitable = agg.get("sessions_profitable", 0)
    
    risk_flags = result.get("risk_flags", [])
    if risk_flags:
        risk_flags_str = "\n".join(f"- {flag}" for flag in risk_flags)
    else:
        risk_flags_str = "None"
        
    prompt = f"""Analyze the following backtest results for the Indian stock market:
Instrument: {instrument}
Date: {date}
Sessions Run: {sessions_run}

Performance Metrics:
- Average PnL: {avg_pnl:.2f}
- Best Case PnL: {best_pnl:.2f}
- Worst Case PnL: {worst_pnl:.2f}
- Win Rate: {avg_win_rate:.4f}
- Sharpe Ratio: {avg_sharpe:.4f}
- Max Drawdown: {avg_max_drawdown:.4f}
- Sessions Profitable: {sessions_profitable} / {sessions_run}

Risk Flags:
{risk_flags_str}
"""
    return prompt

def call_ollama(prompt: str,
                model: str = "llama3.2",
                base_url: str = "http://localhost:11434") -> str:
    """
    Call Ollama's /api/generate endpoint.
    POST { "model": model, "prompt": SYSTEM_PROMPT + "\n\n" + prompt,
           "stream": false }
    Return response["response"] string.
    Timeout: 30 seconds.
    On any error (connection refused, timeout, model not found):
      return a fallback string:
      "Local LLM unavailable. Install Ollama and run: ollama pull llama3.2"
    """
    import requests
    url = f"{base_url.rstrip('/')}/api/generate"
    payload = {
        "model": model,
        "prompt": SYSTEM_PROMPT + "\n\n" + prompt,
        "stream": False
    }
    try:
        response = requests.post(url, json=payload, timeout=30)
        response.raise_for_status()
        data = response.json()
        return data["response"]
    except Exception:
        return "Local LLM unavailable. Install Ollama and run: ollama pull llama3.2"

def parse_response(raw: str) -> dict:
    """
    Parse the LLM response into:
    {
      "narrative": str,        # full explanation paragraph
      "suggestions": list[str] # lines starting with "-" or numbered
    }
    If the response has no bullet points, put the whole text
    in narrative and return suggestions as [].
    """
    lines = raw.split("\n")
    narrative_parts = []
    suggestions = []
    
    for line in lines:
        stripped = line.strip()
        is_bullet = False
        bullet_content = ""
        
        if stripped.startswith("-"):
            is_bullet = True
            bullet_content = stripped[1:].strip()
        else:
            match = re.match(r'^\d+[\.)]\s*(.*)$', stripped)
            if match:
                is_bullet = True
                bullet_content = match.group(1).strip()
                
        if is_bullet:
            suggestions.append(bullet_content)
        else:
            if not suggestions:
                narrative_parts.append(line)
                
    if not suggestions:
        narrative = raw.strip()
    else:
        narrative = "\n".join(narrative_parts).strip()
        
    return {
        "narrative": narrative,
        "suggestions": suggestions
    }

def analyze(result: dict,
            model: str = "llama3.2",
            base_url: str = "http://localhost:11434") -> dict:
    """
    Full pipeline: build_prompt → call_ollama → parse_response
    Returns the parsed dict.
    """
    prompt = build_prompt(result)
    raw_response = call_ollama(prompt, model, base_url)
    return parse_response(raw_response)
