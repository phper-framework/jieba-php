# jieba-php

[Jieba 中文分词](https://github.com/messense/jieba-rs)在 Rust 中实现，并为 PHP 提供绑定。

[The Jieba Chinese Word Segmentation](https://github.com/messense/jieba-rs) Implemented in Rust Bound for PHP.

## Requirements

- rust
- php-dev

## Build

```shell
# Optional, specify if php isn't installed globally.
#
# export PHP_CONFIG=<Your path of php-config>

# Build libjieba.so.
cargo build --release
```

## Run

```shell
php -d "extension=target/debug/libjieba.so" --ri jieba

php -d "extension=target/debug/libjieba.so" -r "print_r((new Jieba())->cut('我们中出了一个叛徒'));"
```

## API

```php
class Jieba {
    public function __construct();

    public function cut(string $sentence, bool $hmm): array;

    public function cutForSearch(string $sentence, bool $hmm): array;

    public function cutAll(string $sentence): array;
}
```

## Examples

```php
<?php

$jieba = new Jieba();

$words = $jieba->cut("我们中出了一个叛徒", true);
print_r($words);

$words = $jieba->cutAll("我们中出了一个叛徒");
print_r($words);

$words = $jieba->cutForSearch("我们中出了一个叛徒");
print_r($words);
```

## License

MulanPSL-2.0
