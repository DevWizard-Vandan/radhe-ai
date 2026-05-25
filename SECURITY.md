# Security Policy

We take the security and privacy of Radhe AI seriously. Because Radhe AI is designed as a **100% offline, local-first terminal AI assistant**, it does not upload any of your code, prompts, or files to external APIs, servers, or trackers. 

All computation happens strictly on your local CPU.

## Privacy & Offline Boundary

* **No Network Calls**: The core Rust application does not establish outbound network connections for inference. The local LLM engine (`llama-completion.exe`) executes locally in a separate subprocess.
* **No Telemetry**: There is no analytical tracking or telemetry built into Radhe AI.
* **No Cache Retention**: Your queries are passed to `llama-completion.exe` in-memory. Output files and source files used with `--fix` are only read, not copied or cached.

## Supported Versions

Security updates are actively applied to the following versions:

| Version | Supported          |
| ------- | ------------------ |
| v0.1.x  | :white_check_mark: |
| < v0.1  | :x:                |

## Reporting a Vulnerability

If you discover a security vulnerability (such as a local privilege escalation, command injection, or directory traversal risk in path resolution), please do not open a public GitHub Issue. Instead, report it privately to:

* **Contact Email**: [vandansharma@example.com](mailto:vandansharma@example.com)

Please include:
1. A detailed description of the vulnerability.
2. Steps or a proof-of-concept (PoC) script to reproduce the issue.
3. The operating system and version of Radhe AI used.

We will acknowledge your report within 48 hours and work with you to release a security patch before public disclosure.
