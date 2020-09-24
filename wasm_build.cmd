cargo install wasm-pack

rem Balls example
wasm-pack build examples/balls --release
xcopy /Y examples\balls\pkg\*.* public\balls\pkg\
del public\balls\pkg\.gitignore
copy /Y examples\balls\index.html public\balls\index.html
