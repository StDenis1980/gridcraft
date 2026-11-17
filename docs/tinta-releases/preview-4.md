# Tinta Calculo — Preview 4

> **Status:** 2026-11-17 · draft for the Preview 4 release (10 December 2026), version 0.19.1, with
> the 16 November upstream sync · **Revision history:** 2026-11-16 created (changes from Preview 3,
> 12 November, to #52); 2026-11-17 added the Tinta Office look (#55); regional currency, Книга1/Аркуш1 and Edited, format samples (#54). Updated at each weekly sync until the freeze on 7 December.

Release notes for the GitHub release and the site, in Spanish, Ukrainian and English. Changes since
Preview 3 (v0.16.0, 12 November 2026). Before publishing: build the Windows installers with
`packaging/windows/tinta-build-release.ps1` (in pdfcraft) and run the smoke test on them.

## Español

**Tinta Calculo — vista previa 4.** Versión de prueba: úsela con copias de sus hojas de cálculo.

Novedades:

- **Formatos regionales más completos:** números, fechas, horas y moneda se escriben y se muestran como
  en su país, ahora también con los nombres de los meses y de los días; la hora de las fechas lleva
  dos cifras (09:05). Cubre España, Ucrania, Polonia, Rumanía, Lituania, Chequia, Italia, Francia y
  Alemania; con la interfaz en catalán, gallego o euskera se usa el formato de España.
- **Nombres de funciones en español** también con la interfaz en catalán, gallego o euskera, como en
  Excel en España, y una opción para escribir las funciones en inglés.
- **IMPORTARDATOSDINAMICOS (GETPIVOTDATA)**: leer un valor de una tabla dinámica.
- **Texto decorativo** (WordArt) en la pestaña Insertar.
- **Libros de Excel 97-2003 (.xls)** se abren mejor, con sus validaciones de datos; las fórmulas
  antiguas calculan como en Excel.
- **Libros con contraseña** se abren y se guardan con el mismo cifrado que Excel.
- **Asistentes y automatización:** cada comando indica qué puede hacer, y una sesión admite varias
  carpetas permitidas.
- **Linux:** los botones de la ventana están en la barra de título, como en Excel.
- **Aspecto de Tinta Office:** los colores y la forma de los controles de Tinta Correo (gris suave
  con un toque verde oliva, esquinas más redondas), en claro y oscuro; una opción en Archivo › Opciones › General vuelve al aspecto clásico.
- **La moneda de su país** en los formatos Moneda y Contabilidad, el botón de contabilidad y
  Ctrl+Mayús+$: euros en España, grivnas en Ucrania (1 234,50 ₴), con el símbolo donde se escribe.
- **Nombres en su idioma:** los libros y hojas nuevos se llaman como en Excel (Libro1, Hoja1) y la
  barra de título dice «Editado».
- **Formatos de número con vista previa:** en la lista, junto a cada formato se ve cómo quedará la
  celda, también mientras escribe; un formato elegido mientras escribe se aplica al instante.
- Del proyecto original (GridCraft): muchas correcciones de cálculo, rendimiento y seguridad.

Limitaciones conocidas: sin tablas dinámicas avanzadas ni libros muy grandes todavía; los nombres de
funciones en checo, polaco, italiano, francés y alemán son una primera versión.

## Українська

**Tinta Calculo — попередня версія 4.** Тестова версія: працюйте з копіями таблиць.

Що нового:

- **Повніші регіональні формати:** числа, дати, час і гроші вводяться й показуються так, як прийнято
  у вашій країні, тепер і з назвами місяців та днів тижня; година в датах має дві цифри (09:05).
  Підтримано Україну, Іспанію, Польщу, Румунію, Литву, Чехію, Італію, Францію й Німеччину; з
  інтерфейсом каталанською, галісійською чи баскською — формат Іспанії.
- **Іспанські назви функцій** і з інтерфейсом каталанською, галісійською чи баскською, як в Excel в
  Іспанії, та опція писати функції англійською.
- **GETPIVOTDATA**: прочитати значення зі зведеної таблиці.
- **Декоративний текст** (WordArt) на вкладці «Вставлення».
- **Книги Excel 97-2003 (.xls)** відкриваються краще, з перевіркою даних; старі формули рахуються як
  в Excel.
- **Книги з паролем** відкриваються й зберігаються з тим самим шифруванням, що й в Excel.
- **Асистенти й автоматизація:** кожна команда повідомляє, що вона може робити, а сеанс приймає кілька
  дозволених папок.
- **Linux:** кнопки вікна в заголовку, як в Excel.
- **Вигляд Tinta Office:** кольори й форма елементів як у Tinta Correo (м'який сірий з оливково-зеленим
  відтінком, заокругленіші кути), у світлій і темній темі; перемикач у «Файл › Параметри › Загальні» повертає класичний вигляд.
- **Гроші у валюті вашої країни** у форматах «Грошовий» і «Фінансовий», на кнопці фінансового
  формату й Ctrl+Shift+$: 1 234,50 ₴ в Україні, євро в Іспанії, із символом там, де його пишуть.
- **Назви вашою мовою:** нові книги й аркуші називаються як в Excel (Книга1, Аркуш1), а в заголовку
  вікна написано «Змінено».
- **Числові формати зі зразками:** у списку біля кожного формату видно, як виглядатиме комірка, навіть
  поки ви ще вводите число; формат, вибраний під час введення, застосовується одразу.
- З основного проєкту (GridCraft): багато виправлень обчислень, швидкодії й безпеки.

Відомі обмеження: ще немає розширених зведених таблиць і дуже великих книг; назви функцій
чеською, польською, італійською, французькою й німецькою — перша версія.

## English

**Tinta Calculo — Preview 4.** A test version: work on copies of your spreadsheets.

What's new:

- **Fuller regional formats:** numbers, dates, times and money are typed and shown the way your
  country writes them, now with month and weekday names too; times in dates have two-digit hours
  (09:05). Covers Spain, Ukraine, Poland, Romania, Lithuania, Czechia, Italy, France and Germany;
  with a Catalan, Galician or Basque interface, Spain's formats apply.
- **Spanish function names** with a Catalan, Galician or Basque interface too, as Excel in Spain
  does, and an option to type functions in English.
- **GETPIVOTDATA** (IMPORTARDATOSDINAMICOS): read a value from a PivotTable.
- **Decorative Text** (WordArt) on the Insert tab.
- **Excel 97-2003 workbooks (.xls)** open better, with their data validation; older formulas
  calculate as in Excel.
- **Password-protected workbooks** open and save with the same encryption as Excel.
- **Assistants and automation:** every command says what it may do, and a session takes several
  allowed folders.
- **Linux:** the window buttons are in the title bar, as in Excel.
- **Tinta Office look:** Tinta Correo's colours and control shapes (soft grey with a touch of olive green,
  rounder corners), light and dark; an option in File › Options › General switches back to the classic look.
- **Your country's currency** in the Currency and Accounting formats, the Accounting button and
  Ctrl+Shift+$: hryvnias in Ukraine (1 234,50 ₴), euros in Spain, with the symbol where it is written.
- **Names in your language:** new workbooks and sheets are named as in Excel (Libro1/Hoja1,
  Книга1/Аркуш1), and the title bar says Edited in your language.
- **Number formats with samples:** the list shows how the cell will look in each format, also while
  you are still typing; a format picked while typing applies at once.
- From the original project (GridCraft): many calculation, performance and security fixes.

Known limits: no advanced PivotTables or very large workbooks yet; the Czech, Polish, Italian,
French and German function names are a first version.
