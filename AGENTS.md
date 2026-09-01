# java2rust

A Java → Rust transpiler, written in Java. It uses JavaParser (3.28.2) with its symbol solver to convert Java sources — files, directories, or Maven sources jars — into compilable Rust crates. This fork was rewritten from scratch (the original forked code only serves as a feature reminder, it isn't called).

`PLAN.md` contains the active work plan, if any.

## Build system

Maven (`pom.xml`); dependencies and the non-standard layout mirror `java2rust.iml` (`.idea/libraries/*.xml`). IDEA project JDK is 25 (`maven.compiler.release=25`). Layout: single `src/` folder with tests in `tests/` (both configured in the pom).

Pinned dependencies:

| Maven coordinate | Version |
|---|---|
| `com.github.javaparser:javaparser-core` | 3.28.2 |
| `com.github.javaparser:javaparser-symbol-solver-core` | 3.28.2 |
| `org.apache.commons:commons-lang3` | 3.20.0 |
| `commons-io:commons-io` | 2.21.0 |
| `info.picocli:picocli` | 4.7.7 |
| `org.junit.jupiter:junit-jupiter` (tests) | 6.0.0 |

The symbol solver also needs javaparser's transitive deps (javassist, guava, jspecify, checker-qual, error_prone_annotations, j2objc-annotations, failureaccess, listenablefuture).

### Compile

```bash
mvn -q compile
```

### Run the CLI

The pom configures `exec-maven-plugin` with the README's command, so IDEA shows it as an `exec:java` Maven run configuration. From the shell:

```bash
mvn -q compile exec:java
```

(Overrides, e.g. a different output directory: pass your own `java` invocation, or temporarily edit the plugin's `<arguments>`.)

- `<output>` (positional): directory where the generated crates are written.
- `--sources`: Java files or directories (a directory is read through its `src/` subfolder).
- `--maven` `groupId:artifactId:version`: looked up in `~/.m2/repository`; if a `-sources.jar` exists it is parsed as a source crate, otherwise the plain jar is registered for type solving.
- `--language`: parser language level (default `JAVA_25`).

### Run the tests

JUnit 6 via surefire (`junit-jupiter` is `test` scope):

```bash
mvn -q test
```

(Or run any single test class from IDEA.)

> ⚠️ **As of 2026-08-31 (branch `experiments`), 48 of 51 tests fail (47 failures + 1 error).** The expectations are stale relative to current output — e.g. tests expect 4-space indentation while the printer emits tabs, and expect `struct A { }` while the code emits `struct A;`. Before adding or trusting tests, print the actual output first (see Development Conventions). Don't "fix" the generator to match a test without checking which side is intended.

## Architecture

Pipeline: `JavaTranspiler` drives three phases — `preanalyze()` → `analyze()` → `generate()` — over a list of `RustJar` (one per crate).

- **`src/cli/Main.java`** — picocli entry point (`cli.Main`); wires parser config + symbol solver into a `JavaTranspiler` and runs the pipeline.
- **`src/java2rust/JavaTranspiler.java`** — the driver. Owns: the `crates` list, a `CombinedTypeSolver` (Reflection + `TranspilerTypeSolver` + per-jar solvers), the name registry (type id → Rust name; unregistered ids print as `/* Java */`), and the method registry (qualified signature → `RustMethod`, used to share a single `RustMethod` per Java method across units).
- **`src/java2rust/DeclVisitor.java`** — `VoidVisitorAdapter` that walks each `CompilationUnit` and builds the Rust IR tree (`RustJar` → `RustPackage` modules → `RustUnit` → items), registering every item's name with the transpiler.
- **`src/java2rust/RustVisitor.java`** (~2300 lines) — the heart of the transpiler: converts AST statements/expressions into printed Rust. Most of the TODOs in `PLAN.md` live here.
- **IR classes**: `RustJar` (crate, emits `Cargo.toml`), `RustPackage` (module), `RustUnit` (file), `RustClass`/`RustEnum`/`RustInterface`/`RustRecord` (type items), `RustMethod`/`RustConstructor`/`RustField`/`RustParam`/`RustParams`, `RustImpls`, `RustVisibility`, `RustSelf`, `RustStatic`, `RustInitializer`. `IRustFunction` is the common interface (method/lambda context). `RustPrinter` is the indentation-aware output builder.
- **`src/java2rust/TranspilerTypeSolver.java`** — feedback loop: resolves already-transpiled types back into the symbol solver so later analysis sees the Rust names.
- **`src/javaparser/SourceZipTypeSolver.java`** — type solver over a Maven `-sources.jar`.
- **`src/java2rust/IdTracker.java` / `IdTrackerVisitor.java`** — tracks Java element ids for import/name resolution.
- **`src/java2rust/RustCalls.java`** — call-graph tracking of thrown exceptions (analysis exists; generated error-propagation code is still partial).
- **`src/java2rust/Java2Rust.java`** — test harness: `test(String java)` runs the full pipeline on a single source string, `assertConversion(java, rust)` asserts trimmed equality; plus the snake_case helpers used for naming.
- **`tests/`** — JUnit tests, one class per language construct (blocks, if/else, try-catch, for, switch, enum, lambda, arrays, expressions, comments, snake_case, …).
- **`generated/`** — output of the transpiler on this project + commons-lang3 (the "transpiled project itself", used to compare output between changes). Currently empty in this checkout; regenerate with the CLI command above (needs the `-sources.jar`s in `~/.m2`).

### Behavior to know

- Primitive mapping (`JavaTranspiler.describe`): `byte`→`i8`, `short`→`i16`, `char`→`u16`, `int`→`i32`, `long`→`i64`, `boolean`→`bool`, `float`→`f32`, `double`→`f64`, `T[]`→`&[T]`, `void`→`()`.
- Methods take `&mut self` if any field is mutated (directly or indirectly), `&self` otherwise; parameters are by-value for primitives, by-reference for classes.
- Unhandled/unresolvable constructs are emitted as `/* Java */ ...` comments (or `/* <error> */`) rather than aborting — a flood of "Unknown identifier" / "Unsolved symbol" stderr noise during runs is expected, not fatal.
- Roadmap items (README): assign `RustPackage` to its `RustJar` for import tracking, generate a Cargo workspace, ensure printed types use full paths.

## Development conventions

- Java source uses **tabs**; public fields with `///` doc comments (Rust-style) for IR classes; jspecify `@NonNull`/`@Nullable` annotations.
- Naming is snake_case via `Java2Rust.camelCaseToSnakeCase` (crate names: dashes → underscores).
- **Test pattern**: `Java2Rust.assertConversion(java, expectedRust)` with text blocks; expected output is exact (trimmed). When a construct's output is in question, first print reality:
  ```bash
  # drop a throwaway main that calls Java2Rust.test("...") and prints the result
  ```
  Do not hand-write expected output — generate it, review it, then pin it.
- `target/` (Maven) and `out/` (IntelliJ) are build output (both gitignored).
- The `stderr.log` at the repo root is a stale captured log, not a build artifact.
