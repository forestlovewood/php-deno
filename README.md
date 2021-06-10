# PHP Deno

[![GitHub license](https://img.shields.io/badge/license-MIT-blue.svg)](https://raw.githubusercontent.com/forestlovewood/php-deno/master/LICENSE)

#### PHP Deno is a Rust-based PHP extension that embeds the Deno JavaScript engine.

The extension enables you to rapidly execute JavaScript and TypeScript code from within PHP applications. An intended use-case is for convenient SSR (Server Side Rendering) support when using frontends made in frameworks like React, Vue and Svelte.

## Requirements

- **PHP >= 8.x** on Linux or MacOS
  
    *Other versions of PHP >= 7.x may be supported, but have not been tested.*

## Progress
- [x] **Embed Deno into library extension**
- [x] **Enable execution of code**
- [ ] **Catch all output and return**
  
    This requires further patching of Deno, including a fairly invasive redirection of `stdout` and `stderr`.
- [ ] **Throw exceptions on errors**
- [ ] **Include example project**
- [ ] *(if possible)* **Include class comments and type hinting**
- [ ] **Refactor to negate need for patching Deno**
  
The options for continued development appear to be a choice between:
    
1. Keeping patching in the same way being done currently. *(easy, ugly)*
2. Extracting the required code from `deno` so that only `deno_core` and `deno_runtime` are required. *(very hard, unmaintainable)*
3. Forking `denoland/deno` and maintaining a variant with limited modifications to provide an external `deno` library and to support customising output handling, preferably through the ability to manipulate or override core extensions. *(hard, prettier)*

## Usage

Simply build the project and include the `libdeno.dylib` file as you would any other extension.

```php
<?php

echo (new \Deno)->execute("console.log('Test');", \Deno::JS);
```