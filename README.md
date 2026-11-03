# r-steelseries 🦀

Высокопроизводительный драйвер и CLI на **Rust** для управления RGB-подсветкой ноутбуков **MSI Raider 18 HX AI** (клавиатура SteelSeries KLC + передняя световая полоса и логотип дракона SteelSeries ALC).

Полностью переписан с Python (`keyglow` / `msi-perkeyrgb`) на **Rust** с сохранением и расширением функционала оригинального **SteelSeries Engine / GG**.

---

## ⚡ Особенности

- **Сверхбыстрый запуск:** Бинарник стартует и завершает отправку сцены в микроконтроллер за **2-5 мс** (в 30-50 раз быстрее Python-версии).
- **Синхронный контроль двух контроллеров:** Одновременная отправка пакетов на `1038:1122` (клавиатура KLC) и `1038:1161` (полоса Lightbar и логотип дракона ALC).
- **Аппаратные анимации:** Все эффекты рассчитываются и воспроизводятся самими чипами контроллеров SteelSeries — 0% нагрузки на CPU после применения.
- **14 режимов подсветки:** Полный набор оригинальных пресетов SteelSeries (Fire, Aurora, Neon, Matrix, Police, Disco, Pulse и др.).
- **Безопасность памяти:** Строгая типизация пакетов протокола HID, отсутствие segfaults, AddressSanitizer и строгий Clippy в CI.

---

## 🚀 Установка и сборка

```bash
git clone https://github.com/MikhaelCat/r-steelseries.git
cd r-steelseries
cargo build --release
```

Скомпилированный бинарник будет находиться по пути:
`target/release/r-steelseries`

Чтобы установить его в систему:
```bash
sudo cp target/release/r-steelseries /usr/local/bin/
```

---

## 💻 Использование

### Статус и проверка устройств
```bash
r-steelseries status
```

### Список всех эффектов
```bash
r-steelseries effects
```

### Включение / выключение
```bash
r-steelseries off
r-steelseries on
```

### Примеры настройки эффектов

- **Огонь / Пламя (Fire):**
  ```bash
  r-steelseries set fire -s 50
  ```

- **Северное сияние (Aurora Borealis):**
  ```bash
  r-steelseries set aurora -s 40 -d right
  ```

- **Неоновый Synthwave (Neon):**
  ```bash
  r-steelseries set neon -s 50 -d right
  ```

- **Матрица (Matrix Rain):**
  ```bash
  r-steelseries set matrix -s 60
  ```

- **Статичные цвета:**
  ```bash
  r-steelseries set static -c ff0000   # Красный
  r-steelseries set static -c 00ffff   # Бирюзовый
  r-steelseries set static -c 8000ff   # Фиолетовый
  ```

- **Радуга (Rainbow Wave):**
  ```bash
  r-steelseries set rainbow -s 50 -d right
  ```

---

## 🧪 Тестирование, CI и бенчмарки

- **Запуск тестов:**
  ```bash
  cargo test
  ```
- **Запуск бенчмарков:**
  ```bash
  cargo bench
  ```
- **Проверка форматирования и линтера:**
  ```bash
  cargo fmt --all -- --check
  cargo clippy --all-targets -- -D warnings
  ```
