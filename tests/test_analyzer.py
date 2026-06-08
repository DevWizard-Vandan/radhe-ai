import pytest
from radhe.analyzer import build_prompt, parse_response, analyze

def test_build_prompt_contains_key_fields():
    sample_result = {
        "date": "2026-06-08",
        "instrument": "NIFTY50",
        "sessions_run": 5,
        "aggregate": {
            "avg_pnl": 1500.50,
            "best_pnl": 3000.00,
            "worst_pnl": -500.00,
            "avg_win_rate": 0.60,
            "avg_sharpe": 1.5,
            "avg_max_drawdown": 0.02,
            "sessions_profitable": 3
        },
        "risk_flags": ["Low win rate"]
    }
    prompt = build_prompt(sample_result)
    assert "NIFTY50" in prompt
    assert "1500.50" in prompt
    assert len(prompt) < 2000

def test_parse_response_with_bullets():
    raw_response = "The strategy performed well.\n- Add spread filter\n- Reduce size"
    parsed = parse_response(raw_response)
    assert parsed["narrative"] == "The strategy performed well."
    assert len(parsed["suggestions"]) == 2
    assert parsed["suggestions"] == ["Add spread filter", "Reduce size"]

def test_parse_response_no_bullets():
    raw_response = "No clear patterns found in this regime."
    parsed = parse_response(raw_response)
    assert parsed["narrative"] == "No clear patterns found in this regime."
    assert parsed["suggestions"] == []

def test_analyze_ollama_offline():
    sample_result = {
        "date": "2026-06-08",
        "instrument": "NIFTY50",
        "sessions_run": 5,
        "aggregate": {
            "avg_pnl": 1500.50,
            "best_pnl": 3000.00,
            "worst_pnl": -500.00,
            "avg_win_rate": 0.60,
            "avg_sharpe": 1.5,
            "avg_max_drawdown": 0.02,
            "sessions_profitable": 3
        }
    }
    result = analyze(sample_result, base_url="http://localhost:19999")
    assert isinstance(result, dict)
    assert "narrative" in result
    assert "suggestions" in result
    narrative = result["narrative"]
    assert "unavailable" in narrative or "Ollama" in narrative
