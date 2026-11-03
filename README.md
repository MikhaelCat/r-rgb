# rrgb 🦀

Высокопроизводительный драйвер и CLI на **Rust** с открытым исходным кодом (**MIT / Apache-2.0**) для управления подсветкой клавиатур, световых полос (Lightbar) и логотипов **SteelSeries / MSI Gaming Laptops** под Linux.

Полная замена Python-утилит (`keyglow`, `msi-perkeyrgb`, `msiklm`) со скоростью отклика **2–5 миллисекунд** и встроенной аппаратной диагностикой.

---

## ⚡ Особенности

- **Двойная открытая лицензия:** Двойное лицензирование **MIT** или **Apache-2.0** — максимально свободный доступ для открытого сообщества и интеграции.
- **Быстрая диагностика `rrgb check`:** Одной командой сканирует все HID-устройства системы, показывает, какие контроллеры распознаны, и выводит подробную информацию для моделей с нестандартными `VID:PID` (например, `1770:ff00` у более старых моделей MSI или `1038:1122`/`1038:1161` у новых).
- **Синхронная работа контроллеров:** Управляет как клавиатурой (KLC), так и полосой/драконом (ALC) одновременно.
- **14 аппаратных режимов:** Fire, Aurora Borealis, Neon Shift, Matrix Rain, Police Strobe, Pulse, Disco, Starlight, Rainbow, Breathing, Wave, Spectrum, Reactive, Static.
- **0% CPU при работе:** Эффекты загружаются в контроллер и исполняются аппаратно.

---

## 🚀 Установка

### 1. Сборка из исходников:
```bash
git clone https://github.com/MikhaelCat/r-steelseries.git
cd r-steelseries
cargo build --release
```

Бинарник появится в `target/release/rrgb`.

### 2. Установка в систему:
```bash
sudo cp target/release/rrgb /usr/local/bin/
```
Теперь команда `rrgb` доступна из любого места в терминале.

---

## 🔍 Диагностика оборудования (`rrgb check`)

Если у вас или другого пользователя ноутбук другой модели (или подсветка не реагирует), выполните:
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
Если ваше устройство отображается со статусом `[ SKIP ]`, его `VID:PID` можно сразу увидеть в списке и добавить в `SUPPORTED_PIDS` в файле `src/device.rs`.

---

## 💻 Справочник команд `rrgb`

### Базовые команды:
- `rrgb check` — Полная диагностика клавиатуры и всех подключенных контроллеров.
- `rrgb status` — Показать текущий режим подсветки и активные порты.
- `rrgb effects` — Вывести список всех 14 доступных режимов с описанием.
- `rrgb off` — Выключить подсветку.
- `rrgb on` — Включить подсветку.

---

### Фирменные эффекты:

1. **Огонь / Пламя:**
   ```bash
   rrgb set fire -s 50
   ```
2. **Северное сияние:**
   ```bash
   rrgb set aurora -s 40 -d right
   ```
3. **Неоновый Киберпанк:**
   ```bash
   rrgb set neon -s 50 -d right
   ```
4. **Матрица:**
   ```bash
   rrgb set matrix -s 60
   ```
5. **Пульс / Кардиограмма:**
   ```bash
   rrgb set pulse -c ff0000 -s 40
   ```
6. **Полицейский стробоскоп:**
   ```bash
   rrgb set police -s 60
   ```
7. **Дискотека:**
   ```bash
   rrgb set disco -s 50
   ```

---

### Классические режимы и цвета:

- **Статичные цвета:**
  ```bash
  rrgb set static -c ff0000   # Красный
  rrgb set static -c 00ffff   # Бирюзовый
  rrgb set static -c 0000ff   # Синий
  rrgb set static -c ffffff   # Белый
  rrgb set static -c 8000ff   # Фиолетовый
  ```
- **Радужная волна:**
  ```bash
  rrgb set rainbow -s 50 -d right
  ```
- **Плавный перелив спектра:**
  ```bash
  rrgb set spectrum -s 40
  ```
- **Дыхание:**
  ```bash
  rrgb set breathing -c 00ffff -s 30
  ```
- **Звёздное небо:**
  ```bash
  rrgb set starlight -c ffffff -s 50
  ```
- **Реакция на нажатия:**
  ```bash
  rrgb set reactive -c 001133 --highlight ffffff
  ```

---

## ⚙️ Флаги и параметры

| Флаг | Значение | Диапазон / Пример |
|---|---|---|
| `-c`, `--color` | Основной цвет | HEX (`ff0000`, `00ffff`) |
| `--highlight` | Цвет нажатия для `reactive` | HEX (`ffffff`) |
| `-s`, `--speed` | Скорость эффекта | от `0` до `100` |
| `-b`, `--brightness` | Яркость | от `0` до `100` |
| `-d`, `--direction` | Направление волны | `right`, `left`, `up`, `down`, `out`, `in` |
| `--wave-size` | Ширина волны | от `0` до `100` |

---

## 📜 Лицензия

Проект распространяется под двойной открытой лицензией **MIT OR Apache-2.0** по вашему выбору (см. файлы `LICENSE-MIT` и `LICENSE-APACHE`).
