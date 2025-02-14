# Как настроить домашки

### Настройка репо
1) Создаём свой приватный репо вида: **ФАМИЛИЯ_Rust_course_2025**. То есть в моём случае это *Barinov_Rust_course_2025*

2) Клонируем репо
```bash
git clone https://github.com/DBarinovv/Rust_course_2025.git &&
cd Rust_course_2025
```

3) Устанавливаем связь + пушим
```bash
git remote remove origin
git remote add origin https://github.com/<ЛОГИН>/<ФАМИЛИЯ>_Rust_course.git
git push -u origin main
```

4) Добавляем связь для обновлений
```bash
git remote add upstream https://github.com/DBarinovv/Rust_course_2025.git
```

5) Даём мне, **DBarinovv**, и ассистенту **ek-krupnik** доступ как коллаборатору

### Получить изменения
```bash
git fetch upstream
git merge upstream/main
```

Если без мёрджа, но вряд ли понадобится
```bash
git pull upstream main --allow-unrelated-histories
```

### Запушить изменения
```bash
git add .
git commit -m "COMMIT MESSAGE"
git push origin main
```
