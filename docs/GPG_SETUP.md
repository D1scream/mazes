# Настройка GPG ключа для GitHub

## 1. Создание ключа

```bash
gpg --full-generate-key
```

- Тип: `1` (RSA and RSA)
- Размер: `4096`
- Срок: `0`
- Имя и email (тот же email, что в Git)
- Пароль

## 2. Получить ID ключа

```bash
gpg --list-secret-keys --keyid-format LONG
```

Скопируйте короткий ID (16 символов после `/`)

## 3. Экспорт публичного ключа

```bash
gpg --armor --export YOUR_KEY_ID
```

## 4. Настроить Git

```bash
git config --global user.signingkey YOUR_KEY_ID
git config --global commit.gpgsign true
```

---

## Проверка

```bash
git log --show-signature
```

Должно быть: `gpg: Good signature`

тест gpg 4