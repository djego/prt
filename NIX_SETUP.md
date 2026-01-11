# 🦀 PRT - Nix Flakes Development Environment

## ✅ Setup Completado

Tu proyecto ahora usa **Nix con Flakes** en lugar de devenv. Todo está configurado y funcionando.

## 🚀 Uso Diario

### Con direnv (Automático - Recomendado)

Simplemente entra al directorio del proyecto:

```bash
cd /Users/codiego/code/sideprojects/prt
# El entorno se activa automáticamente! 🎉
```

### Comandos Disponibles

Una vez en el entorno, tienes estos comandos personalizados:

```bash
# Compilación
build              # Compila el proyecto (cargo build)
build-release      # Compila optimizado (cargo build --release)
check              # Verifica rápido sin compilar (cargo check)

# Ejecución
run                # Ejecuta la aplicación (cargo run)

# Testing
test               # Corre todos los tests (cargo test)
test-one <nombre>  # Corre un test específico
                   # Ejemplo: test-one test_app_initialization

# Calidad de Código
fmt                # Formatea el código (cargo fmt)
clippy             # Lint básico (cargo clippy)
clippy-pedantic    # Lint estricto (cargo clippy -- -W clippy::pedantic)
```

### Ejemplos de Uso

```bash
# Desarrollo típico
cd ~/code/sideprojects/prt
check              # Verificación rápida
build              # Compilar
test               # Correr tests
clippy             # Verificar calidad

# Correr un test específico
test-one test_reset_function

# Compilar para producción
build-release
```

## 🔧 Comandos de Nix

### Actualizar Dependencias

```bash
# Actualizar flake.lock
nix flake update

# Actualizar solo rust-overlay
nix flake lock --update-input rust-overlay
```

### Entrar Manualmente (sin direnv)

```bash
nix develop
# Ahora estás en el shell de desarrollo
build
test
exit  # Salir
```

### Limpiar Cache

```bash
# Limpiar builds de Nix
nix-collect-garbage

# Limpiar builds de Cargo
cargo clean
```

## 📁 Archivos Importantes

- **`flake.nix`** - Configuración del entorno de desarrollo
- **`flake.lock`** - Versiones exactas de dependencias (no editar manualmente)
- **`.envrc`** - Configuración de direnv (`use flake`)
- **`.gitignore`** - Ya configurado para Nix y direnv

## 🐛 Solución de Problemas

### "command not found: build" o similar

```bash
# Recargar direnv
direnv allow
cd /tmp && cd /Users/codiego/code/sideprojects/prt

# O forzar recarga
eval "$(direnv export bash)"
```

### Actualizar versión de Rust

```bash
# Editar flake.nix y cambiar:
rustToolchain = pkgs.rust-bin.stable.latest.default.override {
  # ...
};

# Luego
nix flake update
```

### Ver qué versiones están disponibles

```bash
nix flake show github:oxalica/rust-overlay
```

## 📊 Información del Entorno

- **Rust**: 1.92.0 (stable)
- **Componentes**: rustc, cargo, clippy, rustfmt, rust-analyzer, rust-src
- **Git**: 2.52.0
- **Sistema**: aarch64-darwin (macOS Apple Silicon)
- **Nixpkgs**: nixos-unstable

## 🎯 Ventajas de esta Configuración

✅ **Reproducible** - Mismo entorno en cualquier máquina con Nix  
✅ **Limpio** - No contamina tu sistema  
✅ **Rápido** - Scripts personalizados para comandos comunes  
✅ **Actualizable** - `nix flake update` y listo  
✅ **Documentado** - Todo está en flake.nix  

## 🔗 Referencias

- [Nix Flakes](https://nixos.wiki/wiki/Flakes)
- [rust-overlay](https://github.com/oxalica/rust-overlay)
- [direnv](https://direnv.net/)

---

**Tip**: Si trabajas con VS Code, instala la extensión "direnv" para mejor integración.
