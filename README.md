# DL-Reddit

A simple Leptos-based application to download videos/images from a Reddit post.

## Running the Project

1. Install `cargo-leptos`:
   ```sh
   cargo install cargo-leptos
   ```

2. Install `mold` (or delete the value from `.cargo` folder if not using it):
   ```sh
   sudo apt install mold  # For Debian-based systems
   ```

3. Create a `.env` file with the following values:
   ```sh
   CLIENT_TOKEN=""
   SECRET_TOKEN=""
   USERNAME=""
   PASSWORD=""
   ```

4. Start the project in release mode:
   ```sh
   cargo leptos watch --release
   ```

5. Open your browser and navigate to:
   ```
   http://localhost:8080
   ```


