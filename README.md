# Toroidal Pathfinding

Находит пути на карте (тороидальной).

Введите карту через stdin с символами `#` (стена), ` ` (пусто), `i` (начало), `O` (конец). Путь отмечается точками `.`.

### Способы ввода карты

1. **Интерактивный ввод:**
```bash
cargo run
```
Введите карту построчно, затем `Ctrl+Z` (Windows) или `Ctrl+D` (Linux/Mac)

2. **Из файла:**
```bash
# Linux/Mac:
cargo run < maps/simple.txt

# Windows PowerShell:
Get-Content maps/simple.txt | cargo run
```

3. **Пайп:**
```bash
echo "##    #
#  #i #
#  O## 
   #   " | cargo run
```

## Тестирование

```bash
cargo test
```
