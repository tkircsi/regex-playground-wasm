# Regex Playground WASM

A web-based regular expression testing tool built with Rust and WebAssembly. This playground allows you to test regular expressions in real-time with a clean, modern interface.

## Features

- Real-time regex matching
- Shows matched text and position information
- Clean, responsive user interface
- Built with Rust and WebAssembly for performance
- No server-side processing required
- Interactive examples with one-click testing

## User Interface

The interface consists of three main components:

1. **Regular Expression Input**
   - A text field where you can enter your regex pattern
   - Updates results in real-time as you type
   - Includes helpful tooltips and examples

2. **Test Text Input**
   - A textarea where you can enter the text to test against
   - Pre-filled with a sample text for quick testing

3. **Results Display**
   - Shows all matches found in the text
   - For each match, displays:
     - The matched text in quotes
     - The start and end positions in the text
   - Updates automatically as you modify either the pattern or test text

4. **Interactive Examples**
   - Collapsible section with common regex patterns
   - One-click testing of examples
   - Covers various use cases like word matching, email extraction, date finding, and number detection

## Setup

1. Clone the repository:
   ```bash
   git clone https://github.com/tkircsi/regex-playground-wasm.git
   cd regex-playground-wasm
   ```

2. Build the WebAssembly module:
   ```bash
   ./build.sh
   ```

3. Serve the frontend:
   ```bash
   cd frontend
   python3 -m http.server
   ```

4. Open your browser to `http://localhost:8000`

## Examples

Here are some example regex patterns you can try:

1. **Find words starting with 't'**
   ```
   Pattern: \bt\w*\b
   Test Text: "The quick brown fox jumps over the lazy dog"
   Output:
   "The" (position 0-3)
   ```

2. **Find email addresses**
   ```
   Pattern: \b[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,}\b
   Test Text: "Contact me at john.doe@example.com or support@company.org"
   Output:
   "john.doe@example.com" (position 13-33)
   "support@company.org" (position 37-55)
   ```

3. **Find dates in YYYY-MM-DD format**
   ```
   Pattern: \b\d{4}-\d{2}-\d{2}\b
   Test Text: "The event is on 2024-04-23 and ends on 2024-04-25"
   Output:
   "2024-04-23" (position 15-25)
   "2024-04-25" (position 40-50)
   ```

4. **Find all numbers**
   ```
   Pattern: \b\d+\b
   Test Text: "There are 42 apples and 7 oranges in 3 baskets"
   Output:
   "42" (position 10-12)
   "7" (position 22-23)
   "3" (position 35-36)
   ```

## Technical Details

- **Backend**: Rust with WebAssembly
- **Frontend**: Vanilla JavaScript
- **Build Tools**: wasm-pack
- **Dependencies**:
  - regex (Rust)
  - wasm-bindgen
  - serde-wasm-bindgen

## License

MIT License - see LICENSE file for details
