# Lifetimes

Предисловие:
Лайфтаймы встречаются постоянно в вашем коде, просто они почти всегда незаметны (то есть компилятор обычно сам за вас всё подставит и сделает). И в принципе, можно жить вообще без знания того, что это и как работает. Но всё-таки, имхо, это знание необходимо для понимания происходящего в языке + оно сделает вас более крутым разрабом)

Я бы выделил 5 основных пунктов:
- Мотивация к появлению
- Синтаксис + использование
- Lifetime elision
- HRTBs (Higher-Rank Trait Bounds)
- Subtyping & Variance

Далее я дам ссылки - куда смотреть + небольшие комментарии

### Мотивация + syntax + elision
*База:*
Первые три пункта являются базой и прям обязательны к пониманию + усвоению

- Начало [презентации](/lectures/presentations/lecture_07_2025_unreleased.pdf)
- Статья в [Rust book](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html) (Напомню, что вся книга **очень** рекомендована к прочтению) 

*Advanced:*
Если вы хотите более подробно погрузиться + посмотреть на примеры того, где и как используется, то:
[Crust of Rust: Lifetime Annotations](https://www.youtube.com/live/rAl-9HwD858)
В принципе вся серия [Crust of Rust](https://www.youtube.com/playlist?list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa) очень рекомендована к просмотру, так как даст более глубокое понимание языка

### HRTBs
Невероятно редко встречается, так что чисто для общего развития

- Есть также в [презентации](/lectures/presentations/lecture_07_2025_unreleased.pdf) 
- В конце [Crust of Rust: Functions, Closures, and Their Traits](https://youtu.be/dHkzSZnYXmk?t=3625) (см таймкод: `for bounds`)
- Небольшая [статья в Rustonomicon](https://doc.rust-lang.org/nomicon/hrtb.html)

### Subtyping & Variance
Это вообще наука, но если хотите стать сеньором-помидором, то welcome)

- Как всегда, когда речь заходит про что-то advanced - смотрим *Crust of Rust*: [Subtyping and Variance](https://www.youtube.com/live/iVYWDIW71jk)
- [Статья в Rustonomicon](https://doc.rust-lang.org/nomicon/subtyping.html). Лучше читать после просмотра видео выше, так будет в разы понятнее (иначе кокнет)
- Конечно есть в презе, но по ней думаю ничего не понять
