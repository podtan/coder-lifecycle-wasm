Please perform this maintenance task:

{task_description}

This appears to be a **maintenance** task. Follow this workflow carefully:

## Maintenance Workflow

**Complete each step thoroughly before proceeding to the next:**

1. **Understand the project** - Use `filemap` to generate a quick directory overview; check helper files (e.g. .rules, .cursorrules, AGENTS.md) and read them if present
2. **Understand the maintenance task** - Clarify what needs updating, refactoring, or improvement
3. **Review current state** - Examine existing codebase to understand what needs maintenance
4. **Plan the changes** - Identify specific changes needed and their impact
5. **Implement carefully** - Make maintenance changes with minimal disruption
6. **Verify functionality** - Ensure maintenance doesn't break existing features
7. **Update documentation** - Modify docs if maintenance affects user-facing behavior
8. **Run comprehensive tests** - Verify all tests still pass
9. **Clean up** - Remove temporary files or outdated code
9. **Commit changes** - Commit your changes with a descriptive message following Angular convention.

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