## Available Tools - MANDATORY USAGE

### CRITICAL: You MUST use these structured tools for file operations

**The following shell commands are BLOCKED and will NOT work:**
- `cat`, `head`, `tail`, `less`, `more` → Use `open(path)` instead
- `grep`, `find` → Use `search_file()` or `search_dir()` instead  
- `touch` → Use `create_file(path, content)` instead
- `vim`, `vi`, `nano`, `emacs` → Use `replace_text()`, `insert_text()`, etc. instead
- `tree` → Use `filemap()` instead
- `rm`, `mv`, `cp` → Use `delete_path()`, `move_path()`, `copy_path()` instead

### Structured Tools Available (USE THESE):

You must call these tools using JSON function call format in your response. The system will execute them automatically.

#### File Viewing and Navigation (Windowed System)

**IMPORTANT: Files are viewed in 100-line windows.** You can only see 100 lines at a time. Use navigation commands to view different parts of large files.

1. **open** - Opens file for viewing with navigation support
   - Parameters: `{"path": "src/main.rs"}` 
   - Optional: `{"path": "src/main.rs", "line_number": 50}` to open at specific line
   - Shows first 100 lines by default, or window containing the specified line
   - Better than cat/head/tail - supports windowed viewing with navigation

2. **goto** - Navigate to specific line in open file  
   - Parameters: `{"line_number": 100}` to jump to line 100
   - Moves the 100-line window to show the specified line and surrounding context
   - Must have a file open first

3. **scroll_up** - Scroll up one page (100 lines) in open file
   - Parameters: `{}` (no parameters needed)
   - Moves the viewing window up by 100 lines

4. **scroll_down** - Scroll down one page (100 lines) in open file
   - Parameters: `{}` (no parameters needed)
   - Moves the viewing window down by 100 lines

#### File Creation and Editing - New Specialized Tools

5. **create_file** - Create new file with specified content
   - Parameters: `{"path": "src/new_file.rs", "content": "// File content here"}`
   - Simple tool for file creation - no mode complexity

6. **replace_text** - Replace specific text in a file
   - Parameters: `{"path": "src/lib.rs", "old_text": "old function", "new_text": "new function"}`
   - Optional: `{"path": "file.txt", "old_text": "pattern", "new_text": "replacement", "occurrence": 2}` - Replace specific occurrence
   - Simple, reliable text replacement

7. **insert_text** - Insert text at specific line in a file
   - Parameters: `{"path": "src/main.rs", "line_number": 10, "text": "// New comment"}`
   - Optional position: `{"path": "file.txt", "line_number": 5, "text": "new line", "position": "after_line"}`
   - Positions: "before_line", "after_line", "at_end"

8. **delete_text** - Delete specific text from a file (use delete_line for multiline ranges)
   - Parameters: `{"path": "src/lib.rs", "text_to_delete": "deprecated_function();"}`
   - Optional: `{"path": "file.txt", "text_to_delete": "old text", "occurrence": 1}` - Delete specific occurrence

8.5. **delete_line** - Delete a range of lines (inclusive)
    - Parameters: {"path": "src/lib.rs", "start_line": 10, "end_line": 20}
    - Prefer this over delete_text for multi-line deletions


8.6. **delete_function** - Delete a function definition by name (language-aware)
    - Parameters: {"file_name": "src/module.rs", "function_name": "target_fn"}
    - Notes:
      - Currently supports Rust (.rs) only. For other languages, the tool will respond that it's not implemented yet and suggest using delete_line or delete_text.
      - Prefer this over manual delete_line/delete_text when removing full functions in Rust files.


9. **overwrite_file** - Replace entire file content
   - Parameters: `{"path": "docs/readme.md", "content": "# New Content\n\nCompletely new file."}`
   - Use for complete file replacement

#### File Management Tools

10. **delete_path** - Delete files or directories
    - Parameters: `{"path": "old_file.txt"}` for files
    - Parameters: `{"path": "old_dir", "recursive": true}` for directories
    - Safety checks for important directories

11. **move_path** - Move or rename files/directories
    - Parameters: `{"source": "old_name.txt", "destination": "new_name.txt"}`
    - Works for both files and directories

12. **copy_path** - Copy files or directories
    - Parameters: `{"source": "template.txt", "destination": "copy.txt"}`
    - Parameters: `{"source": "src_dir", "destination": "backup_dir", "recursive": true}` for directories

13. **create_directory** - Create new directories
    - Parameters: `{"path": "new_dir/sub_dir"}` 
    - Automatically creates parent directories

#### Search and Discovery

14. **find_file** - Find files by name/pattern
    - Parameters: `{"file_name": "**/*.rs"}` to find all Rust files
    - Optional: `{"file_name": "*.txt", "dir": "docs/"}` to search in specific directory
    - Better than find command - structured results

15. **search_file** - Search within file
    - Parameters: `{"search_term": "pub fn"}` to search current file
    - Optional: `{"search_term": "TODO", "file": "src/main.rs"}` for specific file
    - Better than grep - structured results with pagination

16. **search_dir** - Search in directory
     - Parameters: `{"search_term": "TODO"}` to search current directory  
     - Optional: `{"search_term": "TODO", "dir": "src/"}` for specific directory
     - Better than grep -r - structured recursive search

17. **filemap** - Generate project structure overview
     - Parameters: `{"file_path": "src/"}` to map source directory
     - Better than tree command - structured output

#### System Operations

18. **run_command** - Execute shell commands safely
     - Parameters: `{"command": "cargo build"}` to run build commands
     - Use for: builds, git operations, system commands, process management
     - Replaces all direct bash usage - use this instead of bash code blocks
     - Has timeout protection and dangerous command filtering

19. **submit** - Complete task and submit results
     - Parameters: `{}` (no parameters needed)
     - Use when work is finished

### Tool Usage Examples:

#### File Navigation Examples:
```json
// Open a file (shows first 100 lines)
{"name": "open", "arguments": {"path": "src/main.rs"}}

// Open file at specific line (shows window containing line 500)
{"name": "open", "arguments": {"path": "src/main.rs", "line_number": 500}}

// Jump to line 200 (moves window to show line 200)
{"name": "goto", "arguments": {"line_number": 200}}

// Scroll down 100 lines
{"name": "scroll_down", "arguments": {}}

// Scroll up 100 lines  
{"name": "scroll_up", "arguments": {}}
```

#### Search Examples:
```json
// Find all Rust files
{"name": "find_file", "arguments": {"file_name": "**/*.rs"}}

// Search for TODO comments in src directory
{"name": "search_dir", "arguments": {"search_term": "TODO", "dir": "src/"}}

// Search within a specific file
{"name": "search_file", "arguments": {"search_term": "pub fn", "file": "src/lib.rs"}}
```

#### Enhanced Editing Examples:
```json
// Create a new file
{"name": "create_file", "arguments": {"path": "src/new_module.rs", "content": "// New module\npub fn hello() {}\n"}}

// Replace text in a file
{"name": "replace_text", "arguments": {"path": "src/lib.rs", "old_text": "old function", "new_text": "new function"}}

// Handle multiple matches - specify which occurrence to replace
{"name": "replace_text", "arguments": {"path": "src/config.rs", "old_text": "TODO:", "new_text": "FIXME:", "occurrence": 2}}

// Insert text at specific line
{"name": "insert_text", "arguments": {"path": "src/main.rs", "line_number": 10, "text": "// New comment", "position": "after_line"}}

// Delete specific text
{"name": "delete_text", "arguments": {"path": "src/lib.rs", "text_to_delete": "deprecated_function();"}}

// Delete a Rust function by name (Rust only)
{"name": "delete_function", "arguments": {"file_name": "src/module.rs", "function_name": "target_fn"}}

// Delete another function by name
{"name": "delete_function", "arguments": {"file_name": "src/lib.rs", "function_name": "old_helper"}}


// Overwrite entire file
{"name": "overwrite_file", "arguments": {"path": "docs/report.md", "content": "# New Report\n\n## Summary\n\nThis is a completely new report."}}

// File management operations
{"name": "copy_path", "arguments": {"source": "template.rs", "destination": "new_module.rs"}}

{"name": "move_path", "arguments": {"source": "old_name.txt", "destination": "new_name.txt"}}

{"name": "delete_path", "arguments": {"path": "unused_file.txt"}}

{"name": "create_directory", "arguments": {"path": "new_feature/tests"}}
```

#### System Operations Examples:
```json
// Build the project
{"name": "run_command", "arguments": {"command": "cargo build"}}

// Run tests
{"name": "run_command", "arguments": {"command": "cargo test"}}

// Git operations
{"name": "run_command", "arguments": {"command": "git status"}}

// Install dependencies
{"name": "run_command", "arguments": {"command": "npm install"}}

// Directory listing
{"name": "run_command", "arguments": {"command": "ls -la"}}
```

### File Navigation Tips:
1. **Large files**: Use `open` with `line_number` to jump directly to relevant sections
2. **Sequential reading**: Use `scroll_down` to read through files page by page
3. **Quick jumps**: Use `goto` to move to specific line numbers
4. **Context preservation**: Each navigation command maintains your position in the file
5. **Window size**: All navigation moves in 100-line increments

Remember:
1. **ALWAYS use structured tools for file operations**
2. **Use run_command for all system operations (NO bash code blocks)**
3. **Execute multiple tools efficiently** - You can and should use multiple tools in a single response to work more efficiently
4. Analyze thoroughly before making changes
5. Use tools for all file viewing, searching, and editing
6. Submit when complete using `submit()`

## Efficient Multi-Tool Usage

**You can execute multiple tools in a single response!** This is encouraged for efficiency:

```json
// Example: Comprehensive file analysis
{"name": "filemap", "arguments": {"file_path": "src/"}}
{"name": "open", "arguments": {"path": "src/main.rs"}}
{"name": "search_dir", "arguments": {"search_term": "TODO", "dir": "src/"}}
{"name": "run_command", "arguments": {"command": "cargo check"}}
```

This approach allows you to:
- Explore project structure AND examine specific files
- Make multiple edits in sequence  
- Run tests AND check results
- Complete complex tasks more efficiently

## Function Call Format Requirements

**CRITICAL**: Tool calls MUST be valid JSON objects with exactly these keys:
- `"name"`: The tool name (string)
- `"arguments"`: Object containing parameters (object)

### ✅ Correct Format Examples:
```json
{"name": "replace_text", "arguments": {"path": "file.txt", "old_text": "old", "new_text": "new"}}

{"name": "create_file", "arguments": {"path": "src/new.rs", "content": "// New file content"}}

{"name": "insert_text", "arguments": {"path": "src/main.rs", "line_number": 10, "text": "// Comment"}}

{"name": "open", "arguments": {"path": "src/main.rs"}}

{"name": "run_command", "arguments": {"command": "cargo build"}}
```

### ❌ Incorrect Formats (DO NOT USE):
```
replace_text("file.txt", "old", "new")  // Function-style calls are NOT supported
create_file file.txt "content"          // Shell-style commands are NOT supported
delete_path old_file.txt                // Plain commands are NOT supported
```

**Models must emit exact JSON format for tools to work.**

## Special Guidance for Grok-Code-Fast-1 Users

**IMPORTANT**: Grok models require extra attention to parameter completeness and format precision.

### New Specialized Tools - ALWAYS Include All Required Parameters:

#### ✅ Correct File Creation:
```json
{"name": "create_file", "arguments": {"path": "new_file.rs", "content": "// New file content"}}
```

#### ✅ Correct Text Replacement:
```json
{"name": "replace_text", "arguments": {"path": "src/main.rs", "old_text": "println!(\"Hello\");", "new_text": "println!(\"Hello, World!\");"}}
```

#### ✅ Correct Text Insertion:
```json
{"name": "insert_text", "arguments": {"path": "src/lib.rs", "line_number": 10, "text": "// New comment"}}
```

#### ✅ Correct File Management:
```json
{"name": "delete_path", "arguments": {"path": "old_file.txt"}}
{"name": "move_path", "arguments": {"source": "old.txt", "destination": "new.txt"}}
{"name": "copy_path", "arguments": {"source": "template.rs", "destination": "module.rs"}}
```

#### ❌ Common Grok Mistakes - DO NOT USE:
```json
// Missing required content parameter
{"name": "create_file", "arguments": {"path": "file.txt"}}

// Missing required parameters  
{"name": "replace_text", "arguments": {"path": "file.txt"}}

// Incomplete parameter sets
{"name": "insert_text", "arguments": {"path": "file.txt", "text": "content"}}
```

### Grok-Specific Examples:

#### Simple File Creation:
```json
{"name": "create_file", "arguments": {"path": "src/lib.rs", "content": "// Library module\n\npub fn main() {\n    println!(\"Hello\");\n}\n"}}
```

#### Simple Text Replace:
```json
{"name": "replace_text", "arguments": {"path": "src/main.rs", "old_text": "println!(\"Hello\");", "new_text": "println!(\"Hello, World!\");"}}
```

#### Multi-line Replace:
```json
{"name": "replace_text", "arguments": {"path": "README.md", "old_text": "# Old Title\n\nOld description", "new_text": "# New Title\n\nNew description"}}
```

#### Insert at Specific Line:
```json
{"name": "insert_text", "arguments": {"path": "src/main.rs", "line_number": 5, "text": "// This is a new comment", "position": "after_line"}}
```

### Critical Rules for Grok Users:

1. **ALWAYS provide ALL required parameters for each tool**
2. **Use specific tools for specific operations - no complex modes**
3. **create_file for new files, replace_text for text changes**  
4. **Include exact text to match, not partial matches**
5. **Double-check JSON syntax before submitting**

**Benefits of New Tools:**
- ✅ Simple, clear parameter requirements
- ✅ No confusing mode-dependent parameters
- ✅ Each tool does exactly one thing
- ✅ Better error messages when parameters are missing

If you get "Missing required parameter" errors, check that you have included ALL required parameters for the operation you want to perform.
