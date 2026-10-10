# fridays-demo — Scaffold Soroban + Counter activado

Proyecto scaffold de [Soroban](https://soroban.stellar.org/) (Stellar): el contrato `hello` del template más un `CounterContract` que estaba escrito pero **comentado desde el primer día**.

## Estado del revival (2026-10)

- **Counter activado**: el código existía pero estaba comentado (`prev code` en el historial). Ahora es un contrato real con `increment`/`get`, persistido en storage.
- **4 tests** que sí corren (`hello`, `hello` con otro nombre, counter desde 0, counter incrementa y persiste).
- **WASM buildeado**: `cargo build --target wasm32-unknown-unknown --release` → `hello_world.wasm` (~2.3KB).
- **Fix de build**: `ethnum 1.5.2 → 1.5.3` (dep transitiva que no compila con toolchains nuevos).
- **API actualizada**: `Symbol::short` (deprecado en SDK 22) → `symbol_short!()`.

## Estructura

```
contracts/hello-world/src/
├── lib.rs    # Contract (hello) + CounterContract (activado)
└── test.rs   # 4 tests
```

## Uso

```bash
rustup target add wasm32-unknown-unknown

cargo test                                          # 4 tests
cargo build --target wasm32-unknown-unknown --release # hello_world.wasm
```

## Contrato

### `hello(to: String) -> Vec<String>`
Devuelve `["Hello", to]`.

### `increment() -> u32`
Incrementa el contador persistido y devuelve el nuevo valor. Primer incremento → 1.

### `get() -> u32`
Devuelve el contador actual (0 si nunca se incrementó).

## Tests

```bash
cargo test
```

- `test_hello` — devuelve ["Hello", "Dev"]
- `test_hello_with_other_name` — eco del nombre pasado
- `test_counter_starts_at_zero` — estado inicial
- `test_counter_increments_and_persists` — 1, 2, 3 y `get()` devuelve 3

## Nota sobre el repo hermano

`soroban-hello-world` es la evolución de este scaffold: ahí vive **savia**, el contrato real de donaciones. Este repo queda como el punto de partida didáctico.

## Licencia

El workspace no declara licencia.

## Autor

[FernandoMay](https://github.com/FernandoMay)
