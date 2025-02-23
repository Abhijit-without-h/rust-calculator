---
# Rust CLI Calculator

A simple command-line calculator built in Rust. This project is designed to help beginners learn Rust by building a practical, interactive tool.

## Features

- Supports basic arithmetic operations: `+`, `-`, `*`, `/`
- Handles invalid inputs gracefully
- Allows continuous calculations until the user types "exit"
- Supports floating-point numbers (e.g., `3.14 + 2.5`)
- Simple and easy to use

## How to Use

1. **Clone the repository**:
   ```bash
   git clone https://github.com/your-username/rust-calculator.git
   cd rust-calculator
   ```

2. **Run the calculator**:
   ```bash
   cargo run
   ```

3. **Enter calculations**:
   - Use the format: `number operator number`
   - Example: `5 + 3` or `10.5 * 2`
   - Type `exit` to quit the program.

   Example session:
   ```
   Welcome to Rust Calculator!
   Enter calculations like '2 + 3' or 'exit' to quit

   5 + 3
   Result: 8

   10 / 2
   Result: 5

   exit
   Goodbye!
   ```

## How It Works

The program reads user input as a string, splits it into components, and processes the input step by step:

1. **Input Handling**:
   - Reads the entire line as a `String`.
   - Trims whitespace and converts to lowercase.
   - Checks for the "exit" command.

2. **Parsing**:
   - Splits the input into three parts: `number1`, `operator`, and `number2`.
   - Converts the numbers to `f64` (floating-point) for calculations.

3. **Calculation**:
   - Uses a `match` statement to perform the correct operation based on the operator.
   - Handles division by zero and invalid operators gracefully.

4. **Output**:
   - Displays the result or an error message if the input is invalid.

## Code Structure

- **Main Loop**: The program runs in a loop until the user types "exit".
- **Error Handling**: Uses `match` and `Result` to handle invalid inputs without crashing.
- **Modular Design**: Easy to extend with new features (e.g., more operators, complex expressions).

## Dependencies

This project uses only the Rust standard library (`std`). No external dependencies are required.

## How to Contribute

1. Fork the repository.
2. Add new features or fix bugs.
3. Submit a pull request.

Some ideas for improvements:
- Add support for more operators (e.g., `%`, `^`).
- Handle complex expressions (e.g., `3 + 4 * 2`).
- Add unit tests.
- Improve error messages and user experience.

## License

This project is open-source and available under the [MIT License](LICENSE).

---
