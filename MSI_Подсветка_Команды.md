# Руководство по управлению подсветкой MSI Raider 18 (Keyglow)

Данная инструкция содержит **все** доступные режимы (включая расширенные фирменные пресеты в стиле оригинального **SteelSeries Engine / GG**) для синхронного управления клавиатурой, передней световой полосой (Lightbar) и логотипом дракона на **MSI Raider 18 HX AI**.

Все анимации аппаратно загружаются в чипы контроллеров SteelSeries (`1038:1122` и `1038:1161`), работают плавно и не нагружают процессор.

---

## 🚀 Быстрый запуск

### Графический интерфейс (GUI)
Открыть окно с переключателями режимов, палитрой и ползунками:
```bash
keyglow gui
```

### Основные команды
- **Выключить всю подсветку:**
  ```bash
  keyglow off
  ```
- **Включить подсветку:**
  ```bash
  keyglow on
  ```
- **Посмотреть текущий режим:**
  ```bash
  keyglow status
  ```
- **Список всех режимов:**
  ```bash
  keyglow effects
  ```

---

## 🌈 Фирменные режимы SteelSeries (Расширенный набор)

Синтаксис: `keyglow set <РЕЖИМ> [ПАРАМЕТРЫ]`

### 1. `fire` — Огонь / Пламя (Fire)
Мерцающее пламя костра с переходами через темно-красный, оранжевый, янтарный и золотисто-желтый цвета по всей клавиатуре и передней полосе.
```bash
keyglow set fire -s 50
```

### 2. `aurora` — Северное сияние (Aurora Borealis)
Мистическое перетекание оттенков бирюзового (Cyan), изумрудного и глубокого фиолетового цвета.
```bash
keyglow set aurora -s 40 -d right
```
*(Поддерживает направление волны `-d`: `right`, `left`, `up`, `down`, `out`, `in`)*

### 3. `neon` — Неоновый Synthwave / Cyberpunk
Стильный неоновый перелив: неоновый розовый (Hot Pink) ↔ электро-бирюзовый (Cyan) ↔ фиолетовый.
```bash
keyglow set neon -s 50 -d right
```

### 4. `matrix` — Матрица (Cyber Data Rain)
Эффект падающего зеленого цифрового дождя из «Матрицы» сверху вниз по клавиатуре и передней панели:
```bash
keyglow set matrix -s 60
```

### 5. `disco` — Дискотека (Disco Party)
Динамичная смена ярких танцевальных цветов по секторам клавиатуры и полосы:
```bash
keyglow set disco -s 50
```

### 6. `police` — Полицейский стробоскоп (Police Strobe)
Стробоскопические двойные вспышки: красные с левой стороны, синие с правой стороны (на клавиатуре и полосе):
```bash
keyglow set police -s 60
```

### 7. `pulse` — Пульс / Сердцебиение (Heartbeat Pulse)
Двойной ритмичный удар выбранного цвета с последующим затуханием (как кардиограмма):
- Красный пульс:
  ```bash
  keyglow set pulse -c ff0000 -s 40
  ```
- Неоновый синий пульс:
  ```bash
  keyglow set pulse -c 00ffff -s 40
  ```

---

## 🎨 Классические режимы

### 8. `static` — Статичный цвет
Один ровный цвет на всех зонах без мерцания.
- **Красный:** `keyglow set static -c ff0000`
- **Синий:** `keyglow set static -c 0000ff`
- **Бирюзовый (Cyan):** `keyglow set static -c 00ffff`
- **Зеленый:** `keyglow set static -c 00ff00`
- **Белый:** `keyglow set static -c ffffff`
- **Фиолетовый:** `keyglow set static -c 8000ff`
- **Оранжевый:** `keyglow set static -c ff8000`
- **Золотой:** `keyglow set static -c ffd700`

### 9. `rainbow` — Радужная волна (Rainbow Wave)
Бегущий спектр радуги во всех направлениях:
- Вправо: `keyglow set rainbow -d right -s 50`
- Влево: `keyglow set rainbow -d left -s 50`
- От центра наружу: `keyglow set rainbow -d out -s 50`
- К центру: `keyglow set rainbow -d in -s 50`

### 10. `spectrum` — Смена спектра (Color Cycle)
Все клавиши, логотип и полоса одновременно циклично меняют цвета радуги:
```bash
keyglow set spectrum -s 40
```

### 11. `breathing` — Дыхание
Плавное угасание и загорание выбранного цвета:
```bash
keyglow set breathing -c 00ffff -s 30
keyglow set breathing -c ff0000 -s 25
```

### 12. `wave` — Цветовая волна (Color Wave)
Волна выбранного цвета, пробегающая по клавиатуре:
```bash
keyglow set wave -c 00ffff -s 50 -d right
keyglow set wave -c ff0055 -s 60 -d left
```

### 13. `starlight` — Звездное небо / Мерцание
Случайные клавиши и участки полосы плавно вспыхивают и затухают:
```bash
keyglow set starlight -c ffffff -s 50
keyglow set starlight -c 0088ff -s 60
```

### 14. `reactive` — Реакция на нажатия
Фоновый цвет + яркая подсветка нажатой клавиши при печати:
```bash
keyglow set reactive -c 001133 --highlight ffffff
keyglow set reactive -c 000000 --highlight 00ffff
```

---

## ⚙️ Таблица параметров и флагов

| Флаг | Значение | Диапазон / Значения | Пример |
|---|---|---|---|
| `-c`, `--color` | Основной цвет (HEX) | `ff0000`, `00ffff`, `ffffff` | `-c 00ff88` |
| `--highlight` | Цвет вспышки (для `reactive`) | HEX-код | `--highlight ffffff` |
| `-b`, `--brightness` | Яркость | от `0` до `100` (%) | `-b 70` |
| `-s`, `--speed` | Скорость анимации | от `0` до `100` (%) | `-s 50` |
| `-d`, `--direction` | Направление волны | `right`, `left`, `up`, `down`, `out`, `in` | `-d out` |
| `--wave-size` | Ширина волны | от `0` до `100` (%) | `--wave-size 60` |
