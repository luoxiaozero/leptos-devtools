# Leptos Developer Tools

in development

## Used

The current library is very unstable.

### 1. Clone current repository

```
git clone git@github.com:luoxiaozero/leptos-devtools.git
```

### 2. Build extension

Execute the following command in the extension directory.

```
pnpm install

pnpm build
```

### 3. Install the chrome extension

1. Open the extension page in google chrome.

- chrome://extensions in the url bar and press enter.

- click on the tree dots in the top right of the browser, then click "More tools" then click "Extensions".

2. Activate developer mode.

Turn on the switch on the top right of the page that says "Developer mode".

3. Load unpacked extension.
   
Click on the button on the top left of the page that says "Load unpacked".

Then select a folder that `extension/dist` directory.

### 4. Reference current library

Add current library to Cargo.toml file.

```
leptos_devtools = { git = "https://github.com/luoxiaozero/leptos-devtools" }
```

Then add `lepton_devtools::devtools()` before `mount_to_body` function in `main` function;

### 5. Run the Project and play with the Developer Tools.

The current library is very unstable.

When there is a problem, refresh the page or reopen Developer Tools.

## Resources

[Solid Devtools](https://github.com/thetarnav/solid-devtools)