# PHP Deno

[![GitHub license](https://img.shields.io/badge/license-MIT-blue.svg)](https://raw.githubusercontent.com/forestlovewood/php-deno/master/LICENSE)
![Status](https://img.shields.io/static/v1?label=status&message=Incomplete&color=red)

#### PHP Deno is a Composer package that uses FFI to provide access to the Deno JavaScript and TypeScript engine.

The package enables you to rapidly execute JavaScript and TypeScript code from within PHP applications. An intended use-case is for convenient SSR (Server Side Rendering) support when using frontends made in frameworks like React, Vue and Svelte.

## Requirements

- **Linux** (including Gnu and Musl) or **macOS**
- **x86_64** or **Arm64** (including Apple Silicon)
- **PHP** >= **8.0**

>If you have **Rust >= 1.52.1** installed, it will be used to build the included `libdeno` library, but if you don't, a temporary environment will be downloaded and used automatically as part of the Composer installation process. Don't worry, this environment won't conflict with anything else, and it doesn't require `sudo` privileges.

## Progress
- [x] **Create embeddable Deno environment**
- [ ] **Enable script execution**
- [ ] **Enable output return**
- [ ] **Throw appropriate exceptions**
- [ ] **Include example project**

## Usage

Simply include the Composer package in your project. During this development phase, you will need to manually add this repository before you can use `composer require forestlovewood/php-deno`.

```php
<?php

echo (new \Deno\Deno)->execute("console.log('Test');", \Deno\Type::JS);
```

### Preloading

A preloading helper is included named `preload.php`. For optimal performance, you should preload this file, or `require` it from your existing preload file, if you have one.

#### Example
In this example, a development server is started on `:8080` with the class preloaded. This dramatically reduces the time required to load the library.
```
php -d opcache.enable_cli=true -d opcache.preload=vendor/forestlovewood/php-deno/preload.php -S localhost:8080 index.php
```