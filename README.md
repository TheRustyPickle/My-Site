# About

The source code of my site, built with Leptos and hosted on [https://render.com/](Render)

## How to run

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
   CLIENT_TOKEN="REDDIT CLIENT TOKEN"
   SECRET_TOKEN="REDDIT SECRET TOKEN"
   USERNAME="REDDIT USERNAME"
   PASSWORD="REDDIT PASSWORD"
   ADDRESS="127.0.0.1"
   ```

4. Start the project in release mode:

   ```sh
   cargo leptos serve --release
   ```

5. Open your browser and navigate to:

   ```
   http://localhost:8080
   ```
