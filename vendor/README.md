# Vendors

  - `glfw` from `https://www.glfw.org`
    - compile normally with `cmake`
    - install to `libs/` dir with `$ DESTDIR="libs/" make install`

  - `glad` from `https://glad.dav1d.de/`
    - compile with `$ gcc -c src/glad.c -Iinclude/ -ldl` and `$ ar rcs libglad.a glad.o`
    - copy `libglad.a` into `libs/usr/local/lib`

  - `clang` from `https://github.com/llvm/llvm-project` or pre-built from `https://releases.llvm.org/download.html`
    - copy `libclang.so` into `libs/usr/local/lib`