Please implement this new feature:

{task_description}

This appears to be a **feature development** task. Follow this workflow carefully:

## Feature Development Workflow

**Complete each step thoroughly before proceeding to the next:**

1. **Understand the project** - Use `filemap` to generate a quick directory overview; check helper files (e.g. .rules, .cursorrules, AGENTS.md) and read them if present
2. **Analyze requirements** - Understand exactly what the feature should do
3. **Design the solution** - Plan how to integrate with existing architecture
4. **Identify integration points** - Find where the feature should be added
5. **Implement core functionality** - Write the main feature code following project patterns
6. **Add configuration support** - Update config files if needed
7. **Create/update tests** - Ensure proper test coverage
8. **Update documentation** - Add or modify docs for the new feature
9. **Test end-to-end** - Verify complete functionality
10. **Run existing tests** - Confirm no regressions

11. **Commit changes** - Follow Angular commit convention for the feature
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