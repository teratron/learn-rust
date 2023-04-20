# Lesson - minigrep

Первая задача - заставить minigrep принимать два аргумента командной строки:

- путь к файлу,
- строку для поиска.

То есть мы хотим иметь возможность запускать нашу программу через `cargo run`, с использованием двойного дефиса, чтобы
указать, что следующие аргументы предназначены для нашей программы, а не для cargo, строки для поиска и пути к файлу в
котором нужно искать, как описано ниже:

```shell
cd minigrep
cargo run -- body poem.txt
```

`body`     - search string\
`poem.txt` - example filename

    $ IGNORE_CASE=1 cargo run -- to poem.txt
    # или
    PS> $Env:IGNORE_CASE=1; cargo run -- to poem.txt
    PS> Remove-Item Env:IGNORE_CASE

Синтаксис `>` указывает оболочке записывать содержимое стандартного вывода в _output.txt_ вместо экрана.

```shell
cargo run > output.txt
cargo run -- to poem.txt > output.txt
IGNORE_CASE=1 cargo run -- to poem.txt > output.txt
```

