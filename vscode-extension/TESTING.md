# Testing the BackLang VS Code Extension

This guide provides step-by-step instructions to test all features of the BackLang Debug Extension.

## Preparation

1. **Navigate to the extension directory**

   ```bash
   cd vscode-extension
   npm install
   npm run compile
   ```

2. **Open in VS Code**

   ```bash
   code .
   ```

3. **Launch Debug Instance**
   - Press `F5` to start a new VS Code window with the extension loaded
   - You should see "BackLang Debug" extension in the extensions sidebar

## Test Cases

### Test 1: Syntax Highlighting

**Steps:**

1. Create a new file with `.bl` extension (e.g., `test.bl`)
2. Copy the following code:
   ```backlang
   let x = 10
   let name = "test"
   function add(a, b) { return a + b }
   print("Hello " + name)
   ```

**Expected Result:**

- Keywords (`let`, `function`, `print`) are highlighted in purple
- Strings are highlighted in green
- Numbers are highlighted in orange
- Comments are highlighted in gray

### Test 2: Syntax Validation

**Steps:**

1. Create a file `syntax_test.bl`
2. Add this code with missing closing brace:
   ```backlang
   if true {
       print("test")
   ```

**Expected Result:**

- Red squiggly line under the code
- Error message in Problems panel: "Unmatched braces"

### Test 3: Run Command

**Steps:**

1. Create a file `run_test.bl`:

   ```backlang
   print("Hello from BackLang")
   let sum = 5 + 3
   print("5 + 3 = " + sum)
   ```

2. With the file open, press `Ctrl+Shift+R`

**Expected Result:**

- A terminal opens and displays the output
- Shows "Hello from BackLang" and "5 + 3 = 8"

### Test 4: Debug with Breakpoints

**Steps:**

1. Create `debug_test.bl`:

   ```backlang
   let x = 10
   let y = 20
   let z = x + y
   print("Result: " + z)
   ```

2. Click on the line number (left margin) at line 3 to set a breakpoint
3. Press `Ctrl+Shift+D` to start debugging
4. The debugger should pause at the breakpoint

**Expected Result:**

- Red circle appears on the breakpoint line
- Debugger toolbar appears at top
- Execution pauses at the breakpoint
- You can inspect variables in the Debug panel

### Test 5: Debug Navigation

**Steps:**

1. With debugger paused:
   - Press `F10` (Step Over)
   - Press `F11` (Step Into)
   - Press `Shift+F11` (Step Out)
   - Press `F5` (Continue)

**Expected Result:**

- Debugger advances one line at a time with Step Over
- Step Into goes into functions
- Continue runs to next breakpoint
- All controls work smoothly

### Test 6: Variable Inspection

**Steps:**

1. In debugging session with code paused
2. Look at the "Variables" panel in the Debug sidebar

**Expected Result:**

- Local variables are visible
- Variable values are displayed
- Can expand objects to see properties

### Test 7: Create HTML5 Base

**Steps:**

1. Press `Ctrl+Shift+P` to open Command Palette
2. Type `!bl` or search for "Create HTML5 Base"
3. Enter a file name (e.g., "myapp")

**Expected Result:**

- A file `myapp.html` is created
- It opens automatically
- Contains a modern, responsive HTML5 template
- Has CSS styling with gradient background
- Includes JavaScript for interactivity
- Has buttons that work

### Test 8: HTML5 Template Functionality

**Steps:**

1. Open the generated `myapp.html` in a browser
2. Click the "Get Started" button
3. Click the "Documentation" button

**Expected Result:**

- Alert box appears with "BackLang Extension is active! 🎉"
- Documentation button navigates to GitHub
- Page is responsive and looks good

### Test 9: Hover Information

**Steps:**

1. Open a `.bl` file with variables and functions:

   ```backlang
   let myVar = 42
   function myFunc() { return myVar }
   myFunc()
   ```

2. Hover your mouse over `myVar` and `myFunc`

**Expected Result:**

- Tooltip appears showing "**Variable**: myVar" or "**Function**: myFunc"
- Information is contextually relevant

### Test 10: Code Actions

**Steps:**

1. Open any `.bl` file
2. Right-click in the editor
3. Look for Quick Fix or Code Actions

**Expected Result:**

- "Create HTML5 Base" option appears in context menu
- Selecting it opens the HTML5 generation dialog

### Test 11: Multiple Files

**Steps:**

1. Create multiple `.bl` files
2. Switch between them
3. Set breakpoints in different files
4. Debug across files

**Expected Result:**

- Extension works smoothly with multiple files
- Each file maintains its own breakpoints
- Debugging context switches correctly

### Test 12: Error Handling

**Steps:**

1. Create a file with syntax errors:

   ```backlang
   let 123var = 10  // Invalid variable name
   function test) {  // Missing opening paren
   ```

2. Save the file

**Expected Result:**

- Errors appear in Problems panel
- Specific error messages are shown
- Red squiggly lines indicate problem locations

## Performance Testing

### Test 13: Large File Performance

**Steps:**

1. Create a `.bl` file with 100+ lines
2. Add breakpoints throughout
3. Run debugging

**Expected Result:**

- Extension remains responsive
- No lag or freezing
- Highlighting works smoothly

## Platform Testing

### Test 14: Windows, Mac, Linux

Test on each platform:

1. Install extension on each OS
2. Run all above tests
3. Check keyboard shortcuts work per OS

**Expected Result:**

- All features work on all platforms
- Shortcuts adapt to OS (Cmd on Mac, Ctrl on Windows/Linux)

## Integration Testing

### Test 15: Integration with BackLang Compiler

**Steps:**

1. Ensure the `bl` binary is accessible in PATH
2. Run the "Run" command with a `.bl` file
3. Check output matches compiler behavior

**Expected Result:**

- Extension correctly invokes the BackLang compiler
- Output matches direct compiler execution

## Final Checklist

- [ ] All test cases passed
- [ ] No console errors in extension
- [ ] Performance is acceptable
- [ ] Documentation is clear
- [ ] All features are accessible
- [ ] Extension loads without warnings
- [ ] No memory leaks during extended use

## Reporting Issues

If you find any issues during testing:

1. Document the exact steps to reproduce
2. Include the error message
3. Provide the `.bl` file content
4. Note your OS and VS Code version
5. Create a GitHub issue with this information

## Feedback

If you have suggestions or feedback:

- Open a GitHub discussion
- Create a feature request issue
- Contact the team at support@backlang.dev

---

Thank you for testing the BackLang VS Code Extension! 🚀
