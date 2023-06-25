# Receive a struct instance from Rust

import rust_interop

rust_interop.use_shared_object()

print("Now instantiating the struct...")

o = rust_interop.Model("first")
print(o)
