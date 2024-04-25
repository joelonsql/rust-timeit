#!/bin/sh
mkdir -p num-bigint-org num-bigint-dev

# Clone and check out the specific commit for num-bigint-org
git clone https://github.com/rust-num/num-bigint.git num-bigint-org
git -C num-bigint-org reset --hard e50a1be13526a05f83edb4eb22039af03efbd9bb

# Modify the version in the Cargo.toml to prevent conflicts:
# In Rust, two crates with the same name and version cannot coexist as dependencies.
# Changing the version ensures that both modified num-bigint crates can be used simultaneously
# in the same Rust project without causing dependency resolution conflicts.
git -C num-bigint-org apply -p1 <<'EOF'
diff --git a/Cargo.toml b/Cargo.toml
index ba14b04..bed0842 100644
--- a/Cargo.toml
+++ b/Cargo.toml
@@ -8,7 +8,7 @@ categories = [ "algorithms", "data-structures", "science" ]
 license = "MIT OR Apache-2.0"
 name = "num-bigint"
 repository = "https://github.com/rust-num/num-bigint"
-version = "0.4.4"
+version = "0.4.4-e50a1be13526a05f83edb4eb22039af03efbd9bb"
 readme = "README.md"
 build = "build.rs"
 exclude = ["/ci/*", "/.github/*"]
EOF

# Clone and check out the specific commit for num-bigint-dev
git clone https://github.com/rust-num/num-bigint.git num-bigint-dev
git -C num-bigint-dev reset --hard 06b61c8138ad8a9959ac54d9773d0a9ebe25b346

# Modify the version in the Cargo.toml to prevent conflicts:
git -C num-bigint-dev apply -p1 <<'EOF'
diff --git a/Cargo.toml b/Cargo.toml
index ba14b04..bed0842 100644
--- a/Cargo.toml
+++ b/Cargo.toml
@@ -8,7 +8,7 @@ categories = [ "algorithms", "data-structures", "science" ]
 license = "MIT OR Apache-2.0"
 name = "num-bigint"
 repository = "https://github.com/rust-num/num-bigint"
-version = "0.4.4"
+version = "0.4.4-06b61c8138ad8a9959ac54d9773d0a9ebe25b346"
 readme = "README.md"
 build = "build.rs"
 exclude = ["/ci/*", "/.github/*"]
EOF
