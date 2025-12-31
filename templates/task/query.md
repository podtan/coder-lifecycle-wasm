Please help with this query:

{task_description}

## Task Analysis

**IMPORTANT: First determine if this is a simple command execution or a complex exploration task.**

### For Simple Commands (like "echo something and task is done"):
- Execute the requested command directly using `run_command`
- Verify the result meets the requirements  
- Call `submit` immediately when done

### For Complex Exploration Tasks:
Follow this workflow carefully:

1. **Understand the project** - Use `filemap` to generate a quick directory overview; check helper files (e.g. .rules, .cursorrules, AGENTS.md) and read them if present
2. **Understand the question** - Clarify what information is being requested
3. **Explore the codebase** - Navigate and examine relevant files to gather information
4. **Analyze findings** - Review discoveries and organize the information
5. **Investigate dependencies** - Look at related code, configs, and documentation
6. **Research thoroughly** - Ensure comprehensive understanding of the topic
7. **Create detailed report** - Write a Markdown file with your findings and answer
8. **Provide examples** - Include relevant code snippets and explanations

**When your task is complete (either simple command execution or full exploration)**, then use:

```json
{"name": "submit", "arguments": {}}
```

## Important Rules

- **Execute multiple tools efficiently** - You can and should use multiple tools in a single response to work more efficiently
- Always include a THOUGHT section explaining your reasoning and planned tool usage
- Work methodically through each step
- Create a comprehensive Markdown report before completing
- Only declare completion when ALL work is actually done

## Current Context

Working in: {working_dir}
Key files: {key_files}

## Success Criteria

{success_criteria}

## Additional Instructions

{additional_instructions}

{useful_commands}
