# Guide: Cargo.toml Version Specifiers

**Date:** 2025-11-29

## Question

> I see `tracing = "0.1"` and version 0.1.43 is available. I thought specifying only 0.1 would use 0.1.43.

## Short Answer

✅ **YES, you're right!** When you write `tracing = "0.1"`, Cargo uses the latest compatible version available (currently **0.1.41** in your project).

The confusion comes from **two different things**:
1. **What you write** in Cargo.toml → `"0.1"` (version constraint)
2. **What Cargo resolves** → `0.1.41` (concrete installed version)

---

## Semantic Versioning (SemVer) - Basics

### Format: `MAJOR.MINOR.PATCH`

- **MAJOR** (0): Incompatible changes (breaking changes)
- **MINOR** (1): New compatible features
- **PATCH** (41): Compatible bug fixes

Example: `tracing 0.1.41`
- Major: 0 (pre-1.0, unstable API)
- Minor: 1
- Patch: 41

---

## Version Operators in Cargo

### 1. **Caret (^) - Default Operator**

```toml
tracing = "0.1"
# Equivalent to:
tracing = "^0.1"
# Equivalent to:
tracing = "^0.1.0"
```

**Meaning:** "At least this version, up to next incompatible version"

**Rule:** Update allowed if it doesn't modify **leftmost non-zero digit**.

#### Caret Examples:

| Specifier | Means | Accepts |
|-----------|-------|---------|
| `^1.2.3` | `>=1.2.3 <2.0.0` | 1.2.3, 1.2.4, 1.3.0, 1.9.9 |
| `^1.2` | `>=1.2.0 <2.0.0` | 1.2.0, 1.2.5, 1.9.9 |
| `^1` | `>=1.0.0 <2.0.0` | 1.0.0, 1.9.9 |
| `^0.2.3` | `>=0.2.3 <0.3.0` | 0.2.3, 0.2.4, 0.2.99 ❌ 0.3.0 |
| `^0.2` | `>=0.2.0 <0.3.0` | 0.2.0, 0.2.99 |
| `^0.1` | `>=0.1.0 <0.2.0` | 0.1.0, **0.1.41**, 0.1.99 |
| `^0.0.3` | `>=0.0.3 <0.0.4` | 0.0.3 only! |
| `^0.0` | `>=0.0.0 <0.1.0` | 0.0.0 → 0.0.99 |

**Your case:** `tracing = "0.1"` = `^0.1` = accepts `>=0.1.0 <0.2.0`
- ✅ Accepts: 0.1.0, 0.1.41, 0.1.43, 0.1.999
- ❌ Rejects: 0.2.0, 1.0.0

### 2. **Tilde (~) - Patch Updates Only**

```toml
tracing = "~0.1.41"
```

**Meaning:** "At least this version, but only patches"

#### Tilde Examples:

| Specifier | Means | Accepts |
|-----------|-------|---------|
| `~1.2.3` | `>=1.2.3 <1.3.0` | 1.2.3, 1.2.4, 1.2.999 ❌ 1.3.0 |
| `~1.2` | `>=1.2.0 <1.3.0` | 1.2.0 → 1.2.999 |
| `~1` | `>=1.0.0 <2.0.0` | 1.0.0 → 1.999.999 |

### 3. **Other Operators**

```toml
# Exact version (not recommended - too restrictive)
tracing = "=0.1.41"

# Version range
axum = ">=0.7, <0.8"

# Wildcard (equivalent to caret)
serde = "1.*"  # Equivalent to "^1"
```

---

## How Cargo Resolves Versions

### 1. Cargo.toml (Constraints)

```toml
tracing = "0.1"  # You specify a CONSTRAINT
```

### 2. Cargo.lock (Resolved Versions)

Cargo generates/updates `Cargo.lock` with **exact version** chosen:

```toml
[[package]]
name = "tracing"
version = "0.1.41"  # CONCRETE installed version
```

### 3. Resolution Process

1. **First time** (`cargo build`):
   - Cargo finds **latest version** satisfying `"0.1"` → `0.1.41`
   - Writes `0.1.41` to `Cargo.lock`

2. **Subsequent builds**:
   - Cargo uses `0.1.41` (locked in `Cargo.lock`)
   - **Even if 0.1.43 exists**, Cargo keeps 0.1.41 for stability

3. **Manual update** (`cargo update`):
   - Cargo updates to `0.1.43` (still compatible with `"0.1"`)
   - Updates `Cargo.lock`

---

## Checking Resolved Versions

### Useful Commands

```bash
# View full dependency tree
cargo tree

# See which version of 'tracing' is used
cargo tree | findstr "tracing v"

# See users of a dependency
cargo tree -i tracing

# Update to latest compatible versions
cargo update

# Update single dependency
cargo update -p tracing
```

### Example Output

```
tracing v0.1.41  ← RESOLVED version (in Cargo.lock)
```

Even though you wrote `"0.1"`, Cargo resolved to `0.1.41`.

---

## Practical Case: Your Project

### In Cargo.toml

```toml
tracing = "0.1"
```

### Resolved Version (Cargo.lock)

```toml
tracing v0.1.41
```

### Why not 0.1.43?

1. **Cargo.lock already exists** → Cargo uses locked version (0.1.41)
2. To update: `cargo update -p tracing`
3. After update: Cargo resolves to 0.1.43 (or latest available)

---

## Best Practices

### ✅ Recommended: Caret (implicit)

```toml
# Best practice - flexible but safe
serde = "1.0"        # Accepts 1.0.x, 1.1.x, etc. (not 2.0)
axum = "0.7"         # Accepts 0.7.x (not 0.8)
rand = "0.9"         # Accepts 0.9.x (not 0.10)
```

**Advantages:**
- Maximum flexibility for Cargo
- Automatic security updates (via `cargo update`)
- SemVer compatibility guaranteed

### ⚠️ Avoid: Exact Version

```toml
# Too restrictive - blocks security patches
serde = "=1.0.197"
```

**Problems:**
- Blocks bug/security fixes
- Dependency conflicts with other crates

### 🔧 Advanced Usage: Tilde

```toml
# Useful to avoid new features (rare)
some-crate = "~1.2.3"  # Only patches: 1.2.x
```

---

## Special Cases: 0.x Versions

### Version 0.x.y (Pre-1.0)

Versions `0.x` considered **unstable**:

```toml
axum = "0.7"  # = ^0.7 = >=0.7.0 <0.8.0
```

**Important:** In `0.x`, **MINOR** change can be breaking!
- `0.7 → 0.8` can break your code
- That's why `"0.7"` does NOT accept `0.8`

### Version 1.x.y (Stable)

```toml
serde = "1.0"  # = ^1.0 = >=1.0.0 <2.0.0
```

**SemVer Guarantee:**
- `1.0 → 1.999` = always compatible
- `1.x → 2.0` = breaking changes

---

## Complete Example: Typical Cargo.toml

```toml
[dependencies]
# Stable crates (1.x) - very permissive
serde = "1.0"              # Accepts 1.0.x → 1.999.x
tokio = "1"                # Accepts 1.x.y
anyhow = "1"               # Accepts 1.x.y

# Pre-1.0 crates (0.x) - more restrictive automatically
axum = "0.7"               # Accepts 0.7.x only
pulldown-cmark = "0.13"    # Accepts 0.13.x only

# Exact version (rare, for reproducible lockfile)
some-tool = "=2.1.0"       # EXACTLY 2.1.0

# Explicit range (rare)
legacy-lib = ">=1.0, <1.5" # Between 1.0 and 1.4.x
```

---

## Recommended Workflow

### 1. Initial Development

```bash
cargo build  # Resolves and locks versions
```

### 2. Periodic Updates

```bash
# Update all dependencies (respects constraints)
cargo update

# Test
cargo test

# Commit Cargo.lock
git add Cargo.lock
git commit -m "Update dependencies"
```

### 3. Update Specific Crate

```bash
# Example: update only tracing
cargo update -p tracing

# Verify new version
cargo tree -i tracing
```

### 4. Major Upgrade (Breaking Changes)

```bash
# 1. Edit Cargo.toml manually
# axum = "0.7" → axum = "0.8"

# 2. Rebuild (resolves 0.8.x)
cargo build

# 3. Fix breaking changes in code
# 4. Test
cargo test
```

---

## FAQ

### Q: Should I commit Cargo.lock?

**Yes for:**
- Applications / Binaries
- Guarantees reproducible builds

**No for:**
- Libraries
- Let consumers choose versions

### Q: How to know if new version available?

```bash
# Option 1: cargo-outdated (extension)
cargo install cargo-outdated
cargo outdated

# Option 2: cargo-edit (extension)
cargo install cargo-edit
cargo upgrade --dry-run
```

### Q: Difference between Cargo.toml and Cargo.lock?

| Cargo.toml | Cargo.lock |
|------------|------------|
| Version constraints | Exact resolved versions |
| Manually edited | Auto-generated |
| `serde = "1.0"` | `serde v1.0.197` |
| Specifies WHAT to install | Specifies EXACTLY what |

---

## Summary

### Your Initial Question

> `tracing = "0.1"` → does it use 0.1.43?

**Answer:**
1. ✅ `"0.1"` **CAN** use 0.1.43 (compatible)
2. 🔒 But your `Cargo.lock` **LOCKS** probably earlier version (0.1.41)
3. 🔄 To get 0.1.43: `cargo update -p tracing`

### Key Points

1. `"0.1"` = **constraint** (accepts `>=0.1.0 <0.2.0`)
2. Cargo **resolves** to exact version (ex: 0.1.41)
3. `Cargo.lock` **locks** this version
4. `cargo update` updates to latest compatible

### Best Practice

```toml
# Simple, clear, flexible
tracing = "0.1"

# Cargo does the rest!
```

---

## References

- [Specifying Dependencies - The Cargo Book](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html)
- [SemVer Specification](https://semver.org/)
- [Understanding SemVer Constraints](https://david-garcia.medium.com/understanding-the-semantic-version-semver-constraints-caret-vs-tilde-82c659339637)
- [Cargo FAQ - Stack Overflow](https://stackoverflow.com/questions/30826513/what-is-the-syntax-for-specifying-dependency-versions-in-cargo)

---

**Conclusion:** Your intuition was correct! `"0.1"` uses latest compatible version available. Exact version depends on what's resolved in `Cargo.lock`.
