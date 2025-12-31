Please solve this bug fix issue:

{task_description}

This appears to be a **bug fix** task. Follow this workflow carefully:

## Bug Fix Workflow

**Complete each step thoroughly before proceeding to the next:**

1. **Understand the project** - Use `filemap` to generate a quick directory overview; check helper files (e.g. .rules, .cursorrules, AGENTS.md) and read them if present
2. **Analyze the bug** - Find and read relevant files to understand the issue
3. **Reproduce the bug** - Create a script to reproduce the issue consistently  
4. **Identify the root cause** - Debug and understand why the bug occurs
5. **Create a minimal fix** - Edit source code with the smallest possible change
6. **Commit changes** - Commit changes with a short, concise message following Angular convention
7. **Verify the fix** - Run your reproduction script to confirm it works
8. **Test edge cases** - Ensure the fix is robust
9. **Run existing tests** - Confirm no regressions are introduced
10. **Clean up** - Remove any temporary debugging files

11. **Commit changes** - Commit all changes using Angular convention

**When you have completed ALL steps above and verified your solution works**, then use:

```json
{"name": "submit", "arguments": {}}
```

## Important Rules

- **Execute multiple tools efficiently** - You can and should use multiple tools in a single response to work more efficiently
- Always include a THOUGHT section explaining your reasoning and planned tool usage
- Work methodically through each step
- Don't skip verification steps
- Only declare completion when ALL work is actually done

## Current Context

Working in: {working_dir}
Key files: {key_files}

## Success Criteria

{success_criteria}

## Additional Instructions

{additional_instructions}

{useful_commands}