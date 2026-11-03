# rrgb 🦀

Высокопроизводительный драйвер и CLI на **Rust** с открытым исходным кодом (**MIT / Apache-2.0**) для управления подсветкой клавиатур, световых полос (Lightbar) и логотипов **SteelSeries / MSI Gaming Laptops** под Linux.

> **OS:** Исключительно Linux (работает с ядром через `/dev/hidraw`, `ioctl` и подсистему `systemd`).  
> **Скорость:** Отклик на команды **2–5 миллисекунд** (в 30–50 раз быстрее Python-аналогов).  
> **Нагрузка:** **0% CPU** после применения — все анимации аппаратно воспроизводятся контроллерами ноутбука.

---

## 💻 Совместимость с оборудованием

Утилита автоматически распознает контроллеры и синхронизирует все зоны ноутбука:
- **SteelSeries Per-Key RGB (KLC):** Клавиатуры с индивидуальной подсветкой каждой клавиши (`VID: 0x1038, PID: 0x1122`, `0x113A` и др.).
- **SteelSeries Aurora Lightbar & Logo (ALC):** Передняя световая полоса и светящийся дракон на крышке (`VID: 0x1038, PID: 0x1161`).
- **MSI 3-Zone Gaming Keyboards:** Клавиатуры со старыми 3-зонными контроллерами (`VID: 0x1770, PID: 0xFF00`).
- **Универсальный эвристический поиск:** Если у вашей модели ноутбука другой Product ID, команда `rrgb check` мгновенно опросит все hidraw-ноды системы и подскажет точные `VID:PID`.

---

## 📦 Установка на различные дистрибутивы Linux

### 1. Arch Linux / CachyOS / Manjaro
В репозитории есть готовый `PKGBUILD`:
```bash
git clone https://github.com/MikhaelCat/r-steelseries.git
cd r-steelseries
makepkg -si
```
Либо установка через AUR-хелперы (yay / paru), когда пакет опубликован:
```bash
yay -S rrgb-bin
```

---

### 2. Ubuntu / Debian / Linux Mint / Pop!_OS
Установите компилятор Rust и выполните автоматический установщик:
```bash
sudo apt update && sudo apt install -y git curl build-essential
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

git clone https://github.com/MikhaelCat/r-steelseries.git
cd r-steelseries
sudo ./install.sh
```

---

### 3. Fedora / RHEL / AlmaLinux
```bash
sudo dnf install -y git gcc rust cargo
git clone https://github.com/MikhaelCat/r-steelseries.git
cd r-steelseries
sudo ./install.sh
```

---

### 4. openSUSE (Tumbleweed / Leap)
```bash
sudo zypper install -y git gcc rust cargo
git clone https://github.com/MikhaelCat/r-steelseries.git
cd r-steelseries
sudo ./install.sh
```

---

### 5. Ручная сборка (Любой Linux)
```bash
git clone https://github.com/MikhaelCat/r-steelseries.git
cd r-steelseries
cargo build --release
sudo cp target/release/rrgb /usr/local/bin/
sudo cp packaging/50-rrgb.rules /etc/udev/rules.d/
sudo udevadm control --reload && sudo udevadm trigger
```

---

## 🔍 Аппаратная диагностика (`rrgb check`)

Для мгновенной проверки видит ли программа вашу клавиатуру и светодиодные модули:
```bash
rrgb check
```

**Пример вывода:**
```text
=== rrgb Hardware Diagnostics ===
[+] Successfully matched 2 RGB controller(s):
    -> SteelSeries KLC (MSI Keyboard) | VID: 0x1038, PID: 0x1122 | Path: /dev/hidraw3
    -> SteelSeries ALC (MSI Lightbar / Logo) | VID: 0x1038, PID: 0x1161 | Path: /dev/hidraw5

=== All System HID Candidates (/dev/hidraw*) ===
[MATCHED] SteelSeries SteelSeries KLC | ID: 1038:1122 at /dev/hidraw3
[MATCHED] SteelSeries SteelSeries ALC | ID: 1038:1161 at /dev/hidraw5
[  SKIP  ] Logitech M350              | ID: 046d:4080 at /dev/hidraw2
```

---

## 🎨 Справочник команд и 14 режимов подсветки

### Базовые команды
```bash
rrgb status    # Показать активный режим и устройства
rrgb effects   # Список всех режимов
rrgb off       # Выключить подсветку
rrgb on        # Включить подсветку с последними настройками
rrgb gui       # Запустить графический интерфейс с ползунками
```

### Фирменные пресеты SteelSeries Engine:

1. **`fire` — Огонь / Пламя:**
   Мерцающие переливы пламени (красный, оранжевый, янтарный, золотистый).
   ```bash
   rrgb set fire -s 50
   ```

2. **`aurora` — Северное сияние:**
   Глубокие перетекания бирюзового (Cyan), изумрудного и фиолетового цветов.
   ```bash
   rrgb set aurora -s 40 -d right
   ```

3. **`neon` — Киберпанк / Synthwave:**
   Неоновый розовый ↔ электро-бирюзовый ↔ фиолетовый.
   ```bash
   rrgb set neon -s 50 -d right
   ```

4. **`matrix` — Матрица (Digital Rain):**
   Падающий сверху вниз зеленый цифровой дождь.
   ```bash
   rrgb set matrix -s 60
   ```

5. **`police` — Стробоскопы:**
   Двойные вспышки: красный слева, синий справа.
   ```bash
   rrgb set police -s 60
   ```

6. **`pulse` — Сердцебиение / Кардиограмма:**
   Ритмичный двойной удар выбранного цвета с последующим затуханием:
   ```bash
   rrgb set pulse -c ff0000 -s 40
   ```

7. **`disco` — Клубные переливы:**
   Динамичные вспышки ярких цветов по клавиатуре и световой полосе:
   ```bash
   rrgb set disco -s 50
   ```

8. **`rainbow` — Радужная волна:**
   ```bash
   rrgb set rainbow -s 50 -d right
   ```
   *Направления (`-d`): `right`, `left`, `up`, `down`, `out` (из центра), `in` (к центру).*

9. **`spectrum` — Смена спектра:**
   Синхронный мягкий перелив всех цветов радуги:
   ```bash
   rrgb set spectrum -s 40
   ```

10. **`breathing` — Дыхание:**
    ```bash
    rrgb set breathing -c 00ffff -s 30
    ```

11. **`starlight` — Звездное небо:**
    ```bash
    rrgb set starlight -c ffffff -s 50
    ```

12. **`reactive` — Реакция на нажатия (вспышки под пальцами):**
    ```bash
    rrgb set reactive -c 001133 --highlight ffffff
    ```

13. **`wave` — Одиночная волна цвета:**
    ```bash
    rrgb set wave -c ff0055 -s 50 -d right
    ```

14. **`static` — Статичный цвет:**
    ```bash
    rrgb set static -c ff0000   # Красный
    rrgb set static -c 00ffff   # Бирюзовый
    rrgb set static -c ffffff   # Белый
    rrgb set static -c 8000ff   # Фиолетовый
    ```

---

## ⚙️ Флаги команд

| Флаг | Назначение | Значения | Пример |
|---|---|---|---|
| `-c`, `--color` | Основной цвет | HEX-код | `-c 00ff88` |
| `--highlight` | Цвет вспышки (для `reactive`) | HEX-код | `--highlight ffffff` |
| `-s`, `--speed` | Скорость эффекта | от `0` до `100` | `-s 50` |
| `-b`, `--brightness` | Яркость | от `0` до `100` | `-b 80` |
| `-d`, `--direction` | Направление волны | `right`, `left`, `up`, `down`, `out`, `in` | `-d out` |
| `--wave-size` | Ширина волны | от `0` до `100` | `--wave-size 50` |

---

## 🧪 Тестирование и CI

- Запуск unit-тестов:
  ```bash
  cargo test
  ```
- Тест стабильности памяти (20 000 циклов):
  ```bash
  cargo test --test memory_leak_test
  ```
- Запуск тестов скорости (Criterion):
  ```bash
  cargo bench
  ```

---

## 📜 Лицензия

Двойная открытая лицензия **MIT OR Apache-2.0** по вашему выбору (см. `LICENSE-MIT` и `LICENSE-APACHE`).
