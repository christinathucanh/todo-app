// Import the fmt module from the standard library.
// std = Rust's standard library.
// :: = Access a module or item inside another module.
use std::fmt;

// Import io and Write from std::io.
// { } allows importing multiple items from the same module.
//
// io      -> Input/output functionality.
// Write   -> Trait that provides methods like flush().
use std::io::{self, Write};

// ─────────────────────────────────────────
// Data Model
// ─────────────────────────────────────────

// Automatically implement the Debug trait.
// This allows printing the struct with {:?}.
#[derive(Debug)]
struct Todo {
    // usize is an unsigned integer whose size depends
    // on the computer architecture (64-bit on most PCs).
    id: usize,

    // String is an owned, growable UTF-8 string.
    title: String,

    // bool stores true or false.
    done: bool,
}

impl Todo {
    // Associated function (similar to a constructor).
    //
    // fn      = define a function.
    //
    // new     = function name.
    //
    // (id: usize, title: &str)
    //
    // & means "borrow" instead of taking ownership.
    // Borrowing allows reading the string without copying it.
    //
    // str is a string slice.
    // &str means "reference to a string slice."
    //
    // -> Self
    // Returns an instance of the current type (Todo).
    fn new(id: usize, title: &str) -> Self {

        // Construct and return a Todo.
        Todo {

            // Field shorthand.
            // id: id
            id,

            // Convert borrowed string slice into an owned String.
            title: title.to_string(),

            // New todos always start incomplete.
            done: false,
        }
    }
}

// Implement the Display trait.
//
// Display controls how "{}" prints an object.
impl fmt::Display for Todo {

    // &self
    //
    // Borrow the Todo.
    // Ownership stays with the caller.
    //
    // f: &mut fmt::Formatter<'_>
    //
    // &mut
    // Mutable borrow.
    //
    // Formatter writes formatted text.
    //
    // <'_>
    // Lifetime annotation.
    // '_ tells Rust to infer the lifetime automatically.
    //
    // -> fmt::Result
    //
    // Returns either:
    // Ok(())
    // or
    // Err(...)
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        // if is an expression in Rust.
        // The entire if returns one value.
        let status =
            if self.done {
                "✅"
            } else {
                "⬜"
            };

        // write! writes formatted text into Formatter.
        //
        // f is the destination.
        //
        // {} placeholders are replaced by the following values.
        write!(
            f,
            "[{}] {} — {}",
            self.id,
            status,
            self.title
        )
    }
}

// ─────────────────────────────────────────
// App State
// ─────────────────────────────────────────

struct App {

    // Vec<Todo>
    //
    // Vec = growable array.
    //
    // <Todo>
    // Generic type parameter.
    // This vector stores Todo values.
    todos: Vec<Todo>,

    // Next available ID.
    next_id: usize,
}

impl App {

    // Create an empty application.
    fn new() -> Self {
        App {

            // Create an empty vector.
            //
            // Vec::new()
            //
            // :: accesses the associated function new().
            todos: Vec::new(),

            next_id: 1,
        }
    }

    // &mut self
    //
    // Mutable borrow of App.
    // Required because todos will change.
    //
    // title is borrowed because only reading is needed.
    fn add(&mut self, title: &str) {

        // trim()
        // Removes leading and trailing whitespace.
        //
        // is_empty()
        // Returns true if the resulting string has length 0.
        if title.trim().is_empty() {

            println!("  ⚠️  Title cannot be empty.");

            // Exit this function immediately.
            return;
        }

        // Create a new Todo.
        let todo = Todo::new(
            self.next_id,
            title.trim(),
        );

        println!(
            "  ➕ Added: \"{}\" (id={})",
            todo.title,
            todo.id
        );

        // push()
        // Append an element to the end of the vector.
        self.todos.push(todo);

        // Increment ID for the next todo.
        self.next_id += 1;
    }

    // &self
    //
    // Immutable borrow because nothing changes.
    fn list(&self) {

        if self.todos.is_empty() {
            println!("  📭 No todos yet. Add one!");
            return;
        }

        println!("  📋 Todo List:");

        // for loops over each element.
        //
        // &self.todos
        //
        // Borrow the vector.
        //
        // Without &, ownership would move.
        for todo in &self.todos {

            // Display implementation is automatically used.
            println!("     {}", todo);
        }
    }

    fn complete(&mut self, id: usize) {

        // iter_mut()
        //
        // Iterate through mutable references.
        //
        // find(...)
        //
        // Returns:
        // Some(item)
        // or
        // None
        //
        // |t|
        //
        // Closure parameter.
        //
        // Equivalent to a tiny anonymous function.
        //
        // t.id == id
        // Compare IDs.
        match self
            .todos
            .iter_mut()
            .find(|t| t.id == id)
        {

            // Some(todo)
            //
            // Pattern matching extracts the value.
            //
            // if !todo.done
            //
            // Guard.
            // Only matches if not already complete.
            Some(todo) if !todo.done => {

                todo.done = true;

                println!(
                    "  ✅ Marked done: \"{}\"",
                    todo.title
                );
            }

            // Todo exists but was already complete.
            Some(_) =>

                println!(
                    "  ℹ️ Todo #{} is already done.",
                    id
                ),

            // No matching todo.
            None =>

                println!(
                    "  ❌ No todo found with id={}.",
                    id
                ),
        }
    }

    fn delete(&mut self, id: usize) {

        // Store original length.
        let before = self.todos.len();

        // retain()
        //
        // Keep only elements for which
        // the closure returns true.
        //
        // |t|
        // Closure parameter.
        //
        // t.id != id
        // Keep every todo except the one
        // with the matching ID.
        self.todos.retain(|t| t.id != id);

        // Compare lengths.
        if self.todos.len() < before {

            println!("  🗑️ Deleted todo #{}.", id);

        } else {

            println!(
                "  ❌ No todo found with id={}.",
                id
            );
        }
    }

    fn stats(&self) {

        // Total number of todos.
        let total = self.todos.len();

        // iter()
        // Iterate through immutable references.
        //
        // filter(...)
        // Keep only matching items.
        //
        // count()
        // Count remaining elements.
        let done =
            self.todos
                .iter()
                .filter(|t| t.done)
                .count();

        let pending = total - done;

        println!(
            "  📊 Stats: {} total | {} done | {} pending",
            total,
            done,
            pending
        );
    }
}

// ─────────────────────────────────────────
// Helper Functions
// ─────────────────────────────────────────

fn prompt(label: &str) -> String {

    // print! does NOT automatically print a newline.
    print!("{}", label);

    // stdout()
    // Access standard output.
    //
    // flush()
    // Force text to appear immediately.
    //
    // unwrap()
    //
    // Result<T,E>
    //
    // If Ok(value), extract it.
    //
    // If Err(...), panic and terminate.
    io::stdout().flush().unwrap();

    // Empty String.
    let mut buf = String::new();

    // mut
    //
    // Variable can be modified.
    io::stdin()
        .read_line(&mut buf)
        .unwrap();

    // Remove trailing newline.
    buf.trim().to_string()
}

fn print_help() {

    println!();

    println!("  Commands:");
    println!("    add <title>      Add a new todo");
    println!("    list             List all todos");
    println!("    done <id>        Mark a todo complete");
    println!("    delete <id>      Delete a todo");
    println!("    stats            Show statistics");
    println!("    help             Show help");
    println!("    quit             Exit");

    println!();
}

// ─────────────────────────────────────────
// Entry Point
// ─────────────────────────────────────────

fn main() {

    println!("╔══════════════════════════════╗");
    println!("║   🦀 Rust Todo App  v0.1.0   ║");
    println!("╚══════════════════════════════╝");

    print_help();

    // mut
    //
    // The App will change over time.
    let mut app = App::new();

    // Infinite loop.
    loop {

        let input = prompt("todo> ");

        // input.find(' ')
        //
        // Returns:
        // Some(position)
        // or
        // None
        //
        // match handles both cases.
        let (cmd, rest) =
            match input.find(' ') {

                Some(pos) => (

                    // &input[..pos]
                    //
                    // Borrow a slice from the beginning
                    // to pos (exclusive).
                    &input[..pos],

                    // Slice after the first space.
                    input[pos + 1..].trim(),
                ),

                None => (

                    // Convert String into &str.
                    input.as_str(),

                    "",
                ),
            };

        // Match command.
        match cmd {

            "add" =>
                app.add(rest),

            "list" =>
                app.list(),

            "done" =>

                // parse::<usize>()
                //
                // Convert text into usize.
                //
                // ::
                // Specify the target type.
                //
                // <usize>
                // Generic type parameter.
                match rest.parse::<usize>() {

                    Ok(id) =>
                        app.complete(id),

                    Err(_) =>
                        println!(
                            "  ⚠️ Usage: done <id>"
                        ),
                },

            "delete" =>

                match rest.parse::<usize>() {

                    Ok(id) =>
                        app.delete(id),

                    Err(_) =>
                        println!(
                            "  ⚠️ Usage: delete <id>"
                        ),
                },

            "stats" =>
                app.stats(),

            "help" =>
                print_help(),

            // Multiple patterns separated by |
            // match any of these strings.
            "quit" | "exit" | "q" => {

                println!("  👋 Bye!");

                // Exit the loop.
                break;
            }

            // Ignore empty input.
            "" => {}

            // Wildcard pattern.
            // Matches every remaining case.
            _ =>

                println!(
                    "  ❓ Unknown command '{}'. Type 'help'.",
                    cmd
                ),
        }
    }
}