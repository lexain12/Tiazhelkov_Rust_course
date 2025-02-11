# Как настроить домашки

### Настройка репо
1) Создаём свой приватный репо вида: **ФАМИЛИЯ_Rust_course_2025**. То есть в моём случае это *Barinov_Rust_course_2025*

2) Клонируем репо
```
git clone https://github.com/DBarinovv/Rust_course_2025.git
cd Rust_course_2025
```

3) Устанавливаем связь + пушим
```
git remote remove origin
git remote add origin https://github.com/ЛОГИН/Rust_course_homework.git
git push -u origin main
```

4) Добавляем связь для обновлений
```
git remote add upstream https://github.com/DBarinovv/Rust_course_2025.git
```

5) Даём мне, **DBarinovv**, доступ как коллаборатору

### Получить изменения
```
git fetch upstream
git merge upstream/main
```

Если без мёрджа, но вряд ли понадобится
```
git pull upstream main --allow-unrelated-histories
```

### Запушить изменения
```
git add .
git commit -m "COMMIT MESSAGE"
git push origin main
```
