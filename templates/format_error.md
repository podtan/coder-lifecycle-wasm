Please provide your tool calls in proper JSON format. Found {actions_length} tool calls but they may not be properly formatted.

**LLM Response that caused this error:**
```
{llm_response}
```

**You can execute multiple tools efficiently!** Please format your response exactly as follows:

<response_example>
THOUGHT: Here are some thoughts about what you want to accomplish and which tools you'll use.

{"name": "run_command", "arguments": {"command": "ls -la"}}
{"name": "open", "arguments": {"path": "src/main.rs"}}
{"name": "search_file", "arguments": {"search_term": "fn main", "file": "src/main.rs"}}
</response_example>

If you want to end the task, please call the submit tool: {"name": "submit", "arguments": {}}
(submit can be used alongside other tools if you want to complete additional work before ending).
