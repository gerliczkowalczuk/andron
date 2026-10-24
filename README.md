# andron

**andron** is an ultra-lightweight, high-performance compatibility runtime and translation layer that executes Android (APK/AAB) applications directly on Linux with near-zero CPU and RAM overhead.

Just as **Proton** translates Windows Win32/DirectX calls directly into native Linux POSIX/Vulkan commands without full VM emulation, **andron** bridges the Android Runtime (ART/Bionic/Binder) directly into Linux subsystems (Wayland, PipeWire, and Mesa DMA-BUF).

---

## Why andron?

Traditional Android emulators (like Android Studio AVD, Genymotion, or Bluestacks on Windows) run heavy QEMU virtual machines simulating entire system hardware, consuming gigabytes of RAM and heavy CPU cycles.

**andron** takes a radically different, ultra-optimized approach:
- **No Hardware Emulation:** Android already uses a Linux kernel. andron executes Android binaries on the host CPU directly.
- **Zero-Copy Display Pipeline:** Direct pass-through from Android's `SurfaceFlinger` buffer queue to native **Wayland** compositors (GNOME, KDE Plasma, Sway, Hyprland) using Linux DMA-BUF.
- **Low-Latency Audio:** Audio is routed straight to **PipeWire** / PulseAudio without resampling overhead.
- **Direct Binder IPC Translation:** Inter-process communication is handled via direct kernel `binderfs` or lightweight Unix domain sockets.
- **Minimal RAM Footprint:** Starts instantly and uses only the memory demanded by the running application itself.

---

## Architectural Comparison

```
Typical Emulator (QEMU / Virtual Machine):
[ App ] -> [ Android OS ] -> [ QEMU Hardware Emulation ] -> [ Host OS ] -> [ GPU/Hardware ]
(Heavy CPU overhead, 2-4 GB idle RAM usage)

andron (Direct Translation / Compatibility Layer):
[ Android App (APK) ]
          |
  [ andron Runtime Layer ]
    * Direct Bionic <-> Glibc/Musl mapping
    * ART JIT/AOT Engine
    * SurfaceFlinger <-> Wayland DMA-BUF
    * Direct PipeWire audio channel
          |
[ Native Linux Kernel & Host Hardware ]
(Near-zero CPU overhead, lightweight native RAM footprint)
```

---

## Installation & Build

### Prerequisites
- Linux kernel 5.10+
- Wayland or X11 environment
- Rust (Cargo) 1.80+

### Building

```bash
git clone https://github.com/gerliczkowalczuk/andron.git
cd andron
cargo build --release
```

The compiled binary will be placed at `target/release/andron`.

---

## Usage

### 1. Check System Compatibility
```bash
andron check
```

### 2. Inspect an Android Package (Bytecode & Native Libs)
```bash
andron inspect path/to/app.apk
```

### 3. Run an Android Application
```bash
andron run path/to/app.apk
```

---

## Integration with F-Droid

`andron` acts as the core low-level execution engine. Higher-level storefronts, package managers, and UI integrations (such as `fdroid-linux`) use `andron` under the hood to launch and run downloaded applications effortlessly.

---

## License

GPL-3.0-or-later © Oskar Gerlicz-Kowalczuk
