Since this is the first stage, we've provided this solution to help you get started. To pass this stage, simply follow the instructions below and submit your changes.

### Step 1: Install Development Tools and Configure Development Environment

Follow the official [Anchor Installation Documentation](https://www.anchor-lang.com/docs/installation) to install Rust programming language, Solana CLI tools, and Anchor framework.

### Step 2: Generate Program ID with `anchor keys sync`

Execute the following command to create a unique program ID for deployment:

```sh
anchor keys sync
```

### Step 3: Verify the Changes

After running `anchor keys sync`, check that the program ID has been updated in your `src/lib.rs` file. The `declare_id!` macro should now contain your unique program identifier instead of the default placeholder.

### Step 4: Submit changes

First, run this command to commit your changes:

```sh
git commit -am "[any message]"
```

The output of the command should look like this:

```text
[main 8bc0189] [any message] 1 file changed, 1 insertion(+), 1 deletion(-)
```

Next, run this command to push your changes:

```sh
git push origin main
```

The output of the command should look like this:

```text
remote: Welcome to StackClass! Your commit was received successfully.
```

Once you run the commands above, the Tests failed message below this card will change to Tests passed.