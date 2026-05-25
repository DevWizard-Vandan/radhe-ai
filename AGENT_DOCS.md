# Radhe AI - Agent Integration Guidelines

This document provides developer guidelines and integration steps for other **AI coding agents**, automation tools, editor integrations, and command-line shell scripts looking to programmatically interface with Radhe AI.

---

## 1. Output Normalization & Scrape Readiness

Unlike generic conversational LLM APIs that output verbose preambles, greetings, or markdown blocks, Radhe AI features an active **output post-processing pipeline** that guarantees predictable and clean command returns. This makes scraping and command interpolation extremely reliable.

* **Code-Only Output (`--code` and `--fix` modes)**:
  - Generates raw code blocks without surrounding markdown code blocks (e.g., ```rust ... ```).
  - Automatically strips comments containing `// bug:` or `# bug:`.
  - Terminates generation immediately upon encountering trailing markdown blocks or standard code explanation headers.
* **Consistent Token Output**:
  - Max token boundaries are hard-coded programmatically to avoid run-away context sizes.
* **Diagnostic Exit Codes**:
  - On success, `radhe` exits with code `0`.
  - On path configuration or filesystem errors (such as a missing file during `--fix`), it exits with code `1` and prints the error message strictly to `stderr`.

---

## 2. Programmatic Execution Wrapper

AI agents and tools can invoke `radhe` directly as a subprocess.

### Example in Node.js / TypeScript
```typescript
import { exec } from "child_process";

/**
 * Invokes Radhe AI to write compilable code.
 * @param prompt Natural language description of the coding task.
 */
function generateCode(prompt: string): Promise<string> {
  return new Promise((resolve, reject) => {
    exec(`radhe --code "${prompt}"`, (error, stdout, stderr) => {
      if (error) {
        return reject(new Error(stderr.trim()));
      }
      resolve(stdout.trim());
    });
  });
}
```

### Example in Python
```python
import subprocess
import sys

def fix_code_file(file_path: str) -> str:
    """Invokes Radhe AI to automatically fix errors inside a source file."""
    try:
        result = subprocess.run(
            ["radhe", "--fix", file_path],
            capture_output=True,
            text=True,
            check=True
        )
        return result.stdout.strip()
    except subprocess.CalledProcessError as e:
        print(f"Error: {e.stderr.strip()}", file=sys.stderr)
        raise
```

---

## 3. Delimiter-Based Parsing Strategy

If you are developing custom frontend shells or terminal dashboards wrapping Radhe, please note that the underlying inference engine uses structural delimiters to segment prompt context from model output:

* **General/Explain Delimiter**: `### RESPONSE:`
* **Fix Mode Delimiter**: `FIXED CODE:`

When capturing raw stdout of the subprocess, Radhe AI automatically splits the text at these delimiters and strips standard warning logs. If you need raw un-stripped model output, configure your script to access the underlying model execution folder directly.
