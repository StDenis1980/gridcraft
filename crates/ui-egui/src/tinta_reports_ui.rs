//! Tinta Office: the first-start question about automatic error reports and their settings in
//! File › Options › General ([`gridcraft_engine::tinta_reports`] keeps the choice; the desktop app
//! sends). Both show only in builds that can send reports: the desktop app sets
//! [`crate::Services::error_reports_dir`] then. The strings are translated here, not in the shared
//! catalogues, so syncing with upstream never conflicts.

use egui::{RichText, Ui};
use serde_json::json;

use crate::SheetApp;

pub(crate) const TITLE: &str = "Send error reports automatically?";
pub(crate) const WHAT: &str = "When the app crashes or stops responding, it can send a report so the problem gets fixed without you writing to us. A report holds the app version, the system, the language, where in the code the error happened and the last lines of the technical log. Never the text of your documents, file names, folders, e-mail addresses or passwords.";
pub(crate) const RIGHTS: &str = "Reports go to Tinta Office (report.tintaoffice.com, stored in the EU) and are deleted after 90 days. You can change this at any time in File › Options › General and delete your reports from the server there. Details: tintaoffice.com/aviso-legal";
pub(crate) const YES: &str = "Yes, send reports";
pub(crate) const NO: &str = "No";
pub(crate) const SECTION: &str = "Error reports";
pub(crate) const CHECKBOX: &str = "Send error reports automatically";
pub(crate) const ID: &str = "Installation id: {id}";
pub(crate) const FORGET: &str = "Delete my reports from the server";
pub(crate) const PENDING: &str = "Deletion requested: it completes the next time the app is online.";
pub(crate) const FOLDER: &str = "Open the reports folder";

const STRINGS: [&str; 11] = [TITLE, WHAT, RIGHTS, YES, NO, SECTION, CHECKBOX, ID, FORGET, PENDING, FOLDER];

/// Each language's translations, in the order of [`STRINGS`].
const TRANSLATIONS: &[(&str, [&str; 11])] = &[
    (
        "es",
        [
            "¿Enviar informes de errores automáticamente?",
            "Cuando el programa falla o deja de responder, puede enviar un informe para que el problema se corrija sin que tengas que escribirnos. El informe contiene la versión del programa, el sistema, el idioma, el lugar del código donde ocurrió el error y las últimas líneas del registro técnico. Nunca el texto de tus documentos, nombres de archivos, carpetas, direcciones de correo ni contraseñas.",
            "Los informes van a Tinta Office (report.tintaoffice.com, almacenados en la UE) y se borran a los 90 días. Puedes cambiarlo cuando quieras en Archivo › Opciones › General y borrar allí tus informes del servidor. Detalles: tintaoffice.com/aviso-legal",
            "Sí, enviar informes",
            "No",
            "Informes de errores",
            "Enviar informes de errores automáticamente",
            "Identificador de la instalación: {id}",
            "Borrar mis informes del servidor",
            "Borrado solicitado: se completará la próxima vez que el programa tenga conexión.",
            "Abrir la carpeta de informes",
        ],
    ),
    (
        "ca",
        [
            "Voleu enviar informes d'errors automàticament?",
            "Quan el programa falla o deixa de respondre, pot enviar un informe perquè el problema es corregeixi sense que ens hàgiu d'escriure. L'informe conté la versió del programa, el sistema, l'idioma, el lloc del codi on s'ha produït l'error i les últimes línies del registre tècnic. Mai el text dels vostres documents, noms de fitxers, carpetes, adreces electròniques ni contrasenyes.",
            "Els informes van a Tinta Office (report.tintaoffice.com, emmagatzemats a la UE) i s'esborren als 90 dies. Podeu canviar-ho quan vulgueu a Fitxer › Opcions › General i esborrar-hi els vostres informes del servidor. Detalls: tintaoffice.com/aviso-legal",
            "Sí, envia els informes",
            "No",
            "Informes d'errors",
            "Envia els informes d'errors automàticament",
            "Identificador de la instal·lació: {id}",
            "Esborra els meus informes del servidor",
            "Esborrament sol·licitat: es completarà la propera vegada que el programa tingui connexió.",
            "Obre la carpeta d'informes",
        ],
    ),
    (
        "gl",
        [
            "Enviar informes de erros automaticamente?",
            "Cando o programa falla ou deixa de responder, pode enviar un informe para que o problema se corrixa sen que teñas que escribirnos. O informe contén a versión do programa, o sistema, o idioma, o lugar do código onde se produciu o erro e as últimas liñas do rexistro técnico. Nunca o texto dos teus documentos, nomes de ficheiros, cartafoles, enderezos de correo nin contrasinais.",
            "Os informes van a Tinta Office (report.tintaoffice.com, almacenados na UE) e bórranse aos 90 días. Podes cambialo cando queiras en Ficheiro › Opcións › Xeral e borrar alí os teus informes do servidor. Detalles: tintaoffice.com/aviso-legal",
            "Si, enviar informes",
            "Non",
            "Informes de erros",
            "Enviar informes de erros automaticamente",
            "Identificador da instalación: {id}",
            "Borrar os meus informes do servidor",
            "Borrado solicitado: completarase a próxima vez que o programa teña conexión.",
            "Abrir o cartafol de informes",
        ],
    ),
    (
        "eu",
        [
            "Errore-txostenak automatikoki bidali?",
            "Programak huts egiten duenean edo erantzuteari uzten dionean, txosten bat bidal dezake arazoa konpontzeko, guri idatzi behar izan gabe. Txostenak programaren bertsioa, sistema, hizkuntza, errorea kodearen zein lekutan gertatu den eta erregistro teknikoaren azken lerroak ditu. Inoiz ez zure dokumentuen testua, fitxategi-izenak, karpetak, helbide elektronikoak edo pasahitzak.",
            "Txostenak Tinta Officera doaz (report.tintaoffice.com, EBn gordeta) eta 90 egunen buruan ezabatzen dira. Nahi duzunean alda dezakezu Fitxategia › Aukerak › Orokorra atalean, eta zure txostenak zerbitzaritik ezabatu bertan. Xehetasunak: tintaoffice.com/aviso-legal",
            "Bai, bidali txostenak",
            "Ez",
            "Errore-txostenak",
            "Bidali errore-txostenak automatikoki",
            "Instalazioaren identifikatzailea: {id}",
            "Ezabatu nire txostenak zerbitzaritik",
            "Ezabatzea eskatuta: programak konexioa duen hurrengo aldian osatuko da.",
            "Ireki txostenen karpeta",
        ],
    ),
    (
        "el",
        [
            "Αυτόματη αποστολή αναφορών σφαλμάτων;",
            "Όταν η εφαρμογή κλείνει απρόσμενα ή σταματά να αποκρίνεται, μπορεί να στείλει μια αναφορά ώστε το πρόβλημα να διορθωθεί χωρίς να μας γράψετε. Η αναφορά περιέχει την έκδοση της εφαρμογής, το σύστημα, τη γλώσσα, το σημείο του κώδικα όπου συνέβη το σφάλμα και τις τελευταίες γραμμές του τεχνικού αρχείου καταγραφής. Ποτέ το κείμενο των εγγράφων σας, ονόματα αρχείων, φακέλους, διευθύνσεις email ή κωδικούς πρόσβασης.",
            "Οι αναφορές πηγαίνουν στο Tinta Office (report.tintaoffice.com, αποθήκευση στην ΕΕ) και διαγράφονται μετά από 90 ημέρες. Μπορείτε να το αλλάξετε οποιαδήποτε στιγμή στο Αρχείο › Επιλογές › Γενικά και να διαγράψετε εκεί τις αναφορές σας από τον διακομιστή. Λεπτομέρειες: tintaoffice.com/en/legal",
            "Ναι, αποστολή αναφορών",
            "Όχι",
            "Αναφορές σφαλμάτων",
            "Αυτόματη αποστολή αναφορών σφαλμάτων",
            "Αναγνωριστικό εγκατάστασης: {id}",
            "Διαγραφή των αναφορών μου από τον διακομιστή",
            "Ζητήθηκε διαγραφή: θα ολοκληρωθεί την επόμενη φορά που η εφαρμογή θα είναι συνδεδεμένη.",
            "Άνοιγμα του φακέλου αναφορών",
        ],
    ),
    (
        "zh",
        [
            "自动发送错误报告？",
            "当应用崩溃或停止响应时，它可以发送一份报告，让问题无需你写信给我们即可得到修复。报告包含应用版本、系统、语言、代码中发生错误的位置以及技术日志的最后几行。绝不包含你的文档文本、文件名、文件夹、电子邮件地址或密码。",
            "报告发送至 Tinta Office（report.tintaoffice.com，存储在欧盟），90 天后删除。你可以随时在“文件 › 选项 › 常规”中更改此设置，并在那里从服务器删除你的报告。详情：tintaoffice.com/en/legal",
            "是，发送报告",
            "否",
            "错误报告",
            "自动发送错误报告",
            "安装 ID：{id}",
            "从服务器删除我的报告",
            "已请求删除：将在应用下次联网时完成。",
            "打开报告文件夹",
        ],
    ),
    (
        "ja",
        [
            "エラー レポートを自動的に送信しますか?",
            "アプリがクラッシュしたり応答しなくなったりしたときに、レポートを送信して、ご連絡いただかなくても問題を修正できるようにします。レポートには、アプリのバージョン、システム、言語、エラーが発生したコード内の場所、技術ログの最後の数行が含まれます。文書の本文、ファイル名、フォルダー、メール アドレス、パスワードは決して含まれません。",
            "レポートは Tinta Office (report.tintaoffice.com、EU 内に保存) に送られ、90 日後に削除されます。[ファイル] › [オプション] › [全般] でいつでも変更でき、そこでサーバーからレポートを削除することもできます。詳細: tintaoffice.com/en/legal",
            "はい、送信します",
            "いいえ",
            "エラー レポート",
            "エラー レポートを自動的に送信する",
            "インストール ID: {id}",
            "サーバーから自分のレポートを削除",
            "削除を要求しました: 次にアプリがオンラインになったときに完了します。",
            "レポート フォルダーを開く",
        ],
    ),
    (
        "ko",
        [
            "오류 보고서를 자동으로 보낼까요?",
            "앱이 충돌하거나 응답하지 않을 때 보고서를 보내 문의하지 않아도 문제가 해결되도록 할 수 있습니다. 보고서에는 앱 버전, 시스템, 언어, 오류가 발생한 코드 위치, 기술 로그의 마지막 몇 줄이 포함됩니다. 문서 내용, 파일 이름, 폴더, 이메일 주소, 암호는 절대 포함되지 않습니다.",
            "보고서는 Tinta Office(report.tintaoffice.com, EU에 저장)로 전송되며 90일 후 삭제됩니다. 파일 › 옵션 › 일반에서 언제든지 변경할 수 있고, 그곳에서 서버의 보고서를 삭제할 수 있습니다. 자세히: tintaoffice.com/en/legal",
            "예, 보고서 보내기",
            "아니요",
            "오류 보고서",
            "오류 보고서 자동으로 보내기",
            "설치 ID: {id}",
            "서버에서 내 보고서 삭제",
            "삭제 요청됨: 다음에 앱이 온라인일 때 완료됩니다.",
            "보고서 폴더 열기",
        ],
    ),
    (
        "ru",
        [
            "Отправлять отчёты об ошибках автоматически?",
            "Когда программа аварийно завершается или перестаёт отвечать, она может отправить отчёт, чтобы проблему исправили без вашего письма нам. В отчёте есть версия программы, система, язык, место в коде, где произошла ошибка, и последние строки технического журнала. Никогда — текст ваших документов, имена файлов, папки, адреса email или пароли.",
            "Отчёты поступают в Tinta Office (report.tintaoffice.com, хранятся в ЕС) и удаляются через 90 дней. Изменить это можно в любое время в меню Файл › Параметры › Общие, там же можно удалить свои отчёты с сервера. Подробнее: tintaoffice.com/en/legal",
            "Да, отправлять отчёты",
            "Нет",
            "Отчёты об ошибках",
            "Отправлять отчёты об ошибках автоматически",
            "Идентификатор установки: {id}",
            "Удалить мои отчёты с сервера",
            "Удаление запрошено: оно завершится, когда программа в следующий раз будет в сети.",
            "Открыть папку отчётов",
        ],
    ),
    (
        "pt",
        [
            "Enviar relatórios de erro automaticamente?",
            "Quando o aplicativo trava ou para de responder, ele pode enviar um relatório para que o problema seja corrigido sem que você precise nos escrever. O relatório contém a versão do aplicativo, o sistema, o idioma, o ponto do código onde o erro ocorreu e as últimas linhas do registro técnico. Nunca o texto dos seus documentos, nomes de arquivos, pastas, endereços de e-mail ou senhas.",
            "Os relatórios vão para a Tinta Office (report.tintaoffice.com, armazenados na UE) e são excluídos após 90 dias. Você pode mudar isso a qualquer momento em Arquivo › Opções › Geral e excluir lá seus relatórios do servidor. Detalhes: tintaoffice.com/en/legal",
            "Sim, enviar relatórios",
            "Não",
            "Relatórios de erro",
            "Enviar relatórios de erro automaticamente",
            "ID da instalação: {id}",
            "Excluir meus relatórios do servidor",
            "Exclusão solicitada: será concluída na próxima vez que o aplicativo estiver online.",
            "Abrir a pasta de relatórios",
        ],
    ),
    (
        "de",
        [
            "Fehlerberichte automatisch senden?",
            "Wenn die App abstürzt oder nicht mehr reagiert, kann sie einen Bericht senden, damit das Problem behoben wird, ohne dass Sie uns schreiben müssen. Der Bericht enthält die App-Version, das System, die Sprache, die Stelle im Code, an der der Fehler auftrat, und die letzten Zeilen des technischen Protokolls. Niemals den Text Ihrer Dokumente, Dateinamen, Ordner, E-Mail-Adressen oder Passwörter.",
            "Berichte gehen an Tinta Office (report.tintaoffice.com, Speicherung in der EU) und werden nach 90 Tagen gelöscht. Sie können das jederzeit unter Datei › Optionen › Allgemein ändern und dort Ihre Berichte vom Server löschen. Details: tintaoffice.com/en/legal",
            "Ja, Berichte senden",
            "Nein",
            "Fehlerberichte",
            "Fehlerberichte automatisch senden",
            "Installations-ID: {id}",
            "Meine Berichte vom Server löschen",
            "Löschung angefordert: Sie wird abgeschlossen, sobald die App das nächste Mal online ist.",
            "Berichtsordner öffnen",
        ],
    ),
    (
        "cs",
        [
            "Odesílat zprávy o chybách automaticky?",
            "Když aplikace spadne nebo přestane reagovat, může odeslat zprávu, aby se problém opravil, aniž byste nám museli psát. Zpráva obsahuje verzi aplikace, systém, jazyk, místo v kódu, kde k chybě došlo, a poslední řádky technického protokolu. Nikdy text vašich dokumentů, názvy souborů, složky, e-mailové adresy ani hesla.",
            "Zprávy jdou do Tinta Office (report.tintaoffice.com, uloženy v EU) a po 90 dnech se mažou. Kdykoli to můžete změnit v nabídce Soubor › Možnosti › Obecné a tam také smazat své zprávy ze serveru. Podrobnosti: tintaoffice.com/en/legal",
            "Ano, odesílat zprávy",
            "Ne",
            "Zprávy o chybách",
            "Odesílat zprávy o chybách automaticky",
            "ID instalace: {id}",
            "Smazat mé zprávy ze serveru",
            "Smazání vyžádáno: dokončí se, až bude aplikace příště online.",
            "Otevřít složku se zprávami",
        ],
    ),
    (
        "fr",
        [
            "Envoyer automatiquement les rapports d'erreur ?",
            "Quand l'application plante ou ne répond plus, elle peut envoyer un rapport pour que le problème soit corrigé sans que vous ayez à nous écrire. Le rapport contient la version de l'application, le système, la langue, l'endroit du code où l'erreur s'est produite et les dernières lignes du journal technique. Jamais le texte de vos documents, les noms de fichiers, les dossiers, les adresses e-mail ni les mots de passe.",
            "Les rapports sont envoyés à Tinta Office (report.tintaoffice.com, stockés dans l'UE) et supprimés après 90 jours. Vous pouvez changer ce choix à tout moment dans Fichier › Options › Général et y supprimer vos rapports du serveur. Détails : tintaoffice.com/en/legal",
            "Oui, envoyer les rapports",
            "Non",
            "Rapports d'erreur",
            "Envoyer automatiquement les rapports d'erreur",
            "Identifiant de l'installation : {id}",
            "Supprimer mes rapports du serveur",
            "Suppression demandée : elle aura lieu la prochaine fois que l'application sera en ligne.",
            "Ouvrir le dossier des rapports",
        ],
    ),
    (
        "sr",
        [
            "Automatski slati izveštaje o greškama?",
            "Kada se aplikacija sruši ili prestane da odgovara, može da pošalje izveštaj kako bi se problem ispravio bez vašeg pisanja nama. Izveštaj sadrži verziju aplikacije, sistem, jezik, mesto u kodu gde je greška nastala i poslednje redove tehničkog dnevnika. Nikada tekst vaših dokumenata, imena datoteka, fascikle, adrese e-pošte ili lozinke.",
            "Izveštaji idu u Tinta Office (report.tintaoffice.com, čuvaju se u EU) i brišu se posle 90 dana. To možete promeniti bilo kada u Datoteka › Opcije › Opšte i tamo obrisati svoje izveštaje sa servera. Detalji: tintaoffice.com/en/legal",
            "Da, šalji izveštaje",
            "Ne",
            "Izveštaji o greškama",
            "Automatski šalji izveštaje o greškama",
            "ID instalacije: {id}",
            "Obriši moje izveštaje sa servera",
            "Brisanje je zatraženo: završiće se sledeći put kada aplikacija bude na mreži.",
            "Otvori fasciklu izveštaja",
        ],
    ),
    (
        "tr",
        [
            "Hata raporları otomatik olarak gönderilsin mi?",
            "Uygulama çöktüğünde veya yanıt vermediğinde, bize yazmanıza gerek kalmadan sorunun düzeltilmesi için bir rapor gönderebilir. Rapor; uygulama sürümünü, sistemi, dili, hatanın koddaki yerini ve teknik günlüğün son satırlarını içerir. Belgelerinizin metnini, dosya adlarını, klasörleri, e-posta adreslerini veya parolaları asla içermez.",
            "Raporlar Tinta Office'e (report.tintaoffice.com, AB'de saklanır) gider ve 90 gün sonra silinir. Bunu istediğiniz zaman Dosya › Seçenekler › Genel bölümünden değiştirebilir ve raporlarınızı oradan sunucudan silebilirsiniz. Ayrıntılar: tintaoffice.com/en/legal",
            "Evet, raporları gönder",
            "Hayır",
            "Hata raporları",
            "Hata raporlarını otomatik olarak gönder",
            "Kurulum kimliği: {id}",
            "Raporlarımı sunucudan sil",
            "Silme istendi: uygulama bir sonraki çevrimiçi olduğunda tamamlanacak.",
            "Rapor klasörünü aç",
        ],
    ),
    (
        "pl",
        [
            "Wysyłać raporty o błędach automatycznie?",
            "Gdy aplikacja ulegnie awarii lub przestanie odpowiadać, może wysłać raport, aby problem został naprawiony bez pisania do nas. Raport zawiera wersję aplikacji, system, język, miejsce w kodzie, w którym wystąpił błąd, i ostatnie wiersze dziennika technicznego. Nigdy treści Twoich dokumentów, nazw plików, folderów, adresów e-mail ani haseł.",
            "Raporty trafiają do Tinta Office (report.tintaoffice.com, przechowywane w UE) i są usuwane po 90 dniach. Możesz to zmienić w dowolnej chwili w Plik › Opcje › Ogólne i tam usunąć swoje raporty z serwera. Szczegóły: tintaoffice.com/en/legal",
            "Tak, wysyłaj raporty",
            "Nie",
            "Raporty o błędach",
            "Wysyłaj raporty o błędach automatycznie",
            "Identyfikator instalacji: {id}",
            "Usuń moje raporty z serwera",
            "Usunięcie zlecone: zakończy się, gdy aplikacja następnym razem będzie online.",
            "Otwórz folder raportów",
        ],
    ),
    (
        "it",
        [
            "Inviare automaticamente le segnalazioni di errore?",
            "Quando l'app si blocca o smette di rispondere, può inviare una segnalazione perché il problema venga corretto senza che tu debba scriverci. La segnalazione contiene la versione dell'app, il sistema, la lingua, il punto del codice in cui si è verificato l'errore e le ultime righe del registro tecnico. Mai il testo dei tuoi documenti, nomi di file, cartelle, indirizzi e-mail o password.",
            "Le segnalazioni vanno a Tinta Office (report.tintaoffice.com, archiviate nell'UE) e vengono eliminate dopo 90 giorni. Puoi cambiare la scelta in qualsiasi momento in File › Opzioni › Generali ed eliminare lì le tue segnalazioni dal server. Dettagli: tintaoffice.com/en/legal",
            "Sì, invia le segnalazioni",
            "No",
            "Segnalazioni di errore",
            "Invia automaticamente le segnalazioni di errore",
            "ID dell'installazione: {id}",
            "Elimina le mie segnalazioni dal server",
            "Eliminazione richiesta: verrà completata la prossima volta che l'app sarà online.",
            "Apri la cartella delle segnalazioni",
        ],
    ),
    (
        "uk",
        [
            "Надсилати звіти про помилки автоматично?",
            "Коли програма аварійно завершується або перестає відповідати, вона може надіслати звіт, щоб проблему виправили без вашого листа нам. У звіті є версія програми, система, мова, місце в коді, де сталася помилка, і останні рядки технічного журналу. Ніколи — текст ваших документів, назви файлів, папки, адреси email чи паролі.",
            "Звіти надходять до Tinta Office (report.tintaoffice.com, зберігаються в ЄС) і видаляються через 90 днів. Змінити це можна будь-коли в меню Файл › Параметри › Загальні, там само можна видалити свої звіти із сервера. Докладніше: tintaoffice.com/uk/pravova-informatsiia",
            "Так, надсилати звіти",
            "Ні",
            "Звіти про помилки",
            "Надсилати звіти про помилки автоматично",
            "Ідентифікатор встановлення: {id}",
            "Видалити мої звіти із сервера",
            "Видалення запитано: воно завершиться, коли програма наступного разу буде в мережі.",
            "Відкрити папку звітів",
        ],
    ),
    (
        "vi",
        [
            "Tự động gửi báo cáo lỗi?",
            "Khi ứng dụng bị lỗi hoặc ngừng phản hồi, nó có thể gửi báo cáo để sự cố được khắc phục mà bạn không cần viết cho chúng tôi. Báo cáo gồm phiên bản ứng dụng, hệ thống, ngôn ngữ, vị trí trong mã nơi xảy ra lỗi và vài dòng cuối của nhật ký kỹ thuật. Không bao giờ có nội dung tài liệu, tên tệp, thư mục, địa chỉ email hay mật khẩu của bạn.",
            "Báo cáo được gửi đến Tinta Office (report.tintaoffice.com, lưu trữ tại EU) và bị xóa sau 90 ngày. Bạn có thể thay đổi bất cứ lúc nào trong Tệp › Tùy chọn › Chung và xóa báo cáo của mình khỏi máy chủ tại đó. Chi tiết: tintaoffice.com/en/legal",
            "Có, gửi báo cáo",
            "Không",
            "Báo cáo lỗi",
            "Tự động gửi báo cáo lỗi",
            "ID cài đặt: {id}",
            "Xóa báo cáo của tôi khỏi máy chủ",
            "Đã yêu cầu xóa: sẽ hoàn tất vào lần tới ứng dụng trực tuyến.",
            "Mở thư mục báo cáo",
        ],
    ),
    (
        "nl",
        [
            "Foutrapporten automatisch versturen?",
            "Als de app vastloopt of niet meer reageert, kan hij een rapport sturen zodat het probleem wordt opgelost zonder dat u ons hoeft te schrijven. Het rapport bevat de appversie, het systeem, de taal, de plek in de code waar de fout optrad en de laatste regels van het technische logboek. Nooit de tekst van uw documenten, bestandsnamen, mappen, e-mailadressen of wachtwoorden.",
            "Rapporten gaan naar Tinta Office (report.tintaoffice.com, opgeslagen in de EU) en worden na 90 dagen verwijderd. U kunt dit altijd wijzigen via Bestand › Opties › Algemeen en daar uw rapporten van de server verwijderen. Details: tintaoffice.com/en/legal",
            "Ja, rapporten versturen",
            "Nee",
            "Foutrapporten",
            "Foutrapporten automatisch versturen",
            "Installatie-ID: {id}",
            "Mijn rapporten van de server verwijderen",
            "Verwijdering aangevraagd: deze wordt voltooid zodra de app weer online is.",
            "Map met rapporten openen",
        ],
    ),
    (
        "ro",
        [
            "Trimiteți automat rapoarte de eroare?",
            "Când aplicația se blochează sau nu mai răspunde, poate trimite un raport pentru ca problema să fie rezolvată fără să ne scrieți. Raportul conține versiunea aplicației, sistemul, limba, locul din cod unde a apărut eroarea și ultimele rânduri din jurnalul tehnic. Niciodată textul documentelor, numele fișierelor, folderele, adresele de e-mail sau parolele.",
            "Rapoartele ajung la Tinta Office (report.tintaoffice.com, stocate în UE) și sunt șterse după 90 de zile. Puteți schimba oricând aceasta în Fișier › Opțiuni › General și tot acolo vă puteți șterge rapoartele de pe server. Detalii: tintaoffice.com/en/legal",
            "Da, trimite rapoarte",
            "Nu",
            "Rapoarte de eroare",
            "Trimite automat rapoarte de eroare",
            "ID-ul instalării: {id}",
            "Șterge rapoartele mele de pe server",
            "Ștergere solicitată: se va finaliza data viitoare când aplicația este online.",
            "Deschide folderul cu rapoarte",
        ],
    ),
];

/// `text` (one of [`STRINGS`]) in the interface language; English when there is no translation.
fn tr(app: &SheetApp, text: &'static str) -> &'static str {
    let code = app.ui.language.code();
    let Some(i) = STRINGS.iter().position(|s| *s == text) else { return text };
    TRANSLATIONS.iter().find(|(c, _)| *c == code).and_then(|(_, t)| t.get(i).copied()).unwrap_or(text)
}

fn available(app: &SheetApp) -> bool {
    app.services.error_reports_dir.is_some()
}

/// The question, over everything, until it is answered once.
pub(crate) fn first_start(app: &mut SheetApp, ctx: &egui::Context) {
    if !available(app) || app.session.prefs.tinta_reports.consent.is_some() {
        return;
    }
    egui::Modal::new(egui::Id::unique("tinta.errorReports")).show(ctx, |ui| {
        ui.set_max_width(460.0);
        ui.label(RichText::new(tr(app, TITLE)).strong().size(17.0));
        ui.add_space(8.0);
        ui.label(tr(app, WHAT));
        ui.add_space(6.0);
        ui.label(RichText::new(tr(app, RIGHTS)).small().weak());
        ui.add_space(12.0);
        ui.horizontal(|ui| {
            if ui.button(tr(app, YES)).clicked() {
                app.run_or_alert("file.errorReports", json!({"value": true}));
            }
            if ui.button(tr(app, NO)).clicked() {
                app.run_or_alert("file.errorReports", json!({"value": false}));
            }
        });
    });
}

/// The section at the end of File › Options › General.
pub(crate) fn options(app: &mut SheetApp, ui: &mut Ui) {
    if !available(app) {
        return;
    }
    ui.add_space(12.0);
    ui.label(RichText::new(tr(app, SECTION)).strong());
    let mut on = app.session.prefs.tinta_reports.consent == Some(true);
    if ui.checkbox(&mut on, tr(app, CHECKBOX)).changed() {
        app.run_or_alert("file.errorReports", json!({"value": on}));
    }
    ui.label(RichText::new(tr(app, WHAT)).small().weak());
    ui.label(RichText::new(tr(app, RIGHTS)).small().weak());
    let p = app.session.prefs.tinta_reports.clone();
    if !p.install_id.is_empty() {
        ui.label(RichText::new(tr(app, ID).replace("{id}", &p.install_id)).small().monospace());
    }
    ui.horizontal(|ui| {
        if ui.add_enabled(!p.install_id.is_empty(), egui::Button::new(tr(app, FORGET))).clicked() {
            app.run_or_alert("file.errorReports", json!({"forget": true}));
        }
        if let Some(dir) = app.services.error_reports_dir.clone()
            && let Some(open) = &app.services.open_folder
            && ui.button(tr(app, FOLDER)).clicked()
        {
            let _ = std::fs::create_dir_all(&dir);
            open(&dir);
        }
    });
    if p.forget_pending.is_some() {
        ui.label(RichText::new(tr(app, PENDING)).small().weak());
    }
}

#[cfg(test)]
mod tests {
    use egui_kittest::kittest::Queryable;

    use super::*;
    use crate::{Services, SheetApp};

    fn harness(dir: Option<std::path::PathBuf>) -> egui_kittest::Harness<'static, SheetApp> {
        let mut session = gridcraft_engine::Session::new();
        session.new_workbook();
        let services = Services { error_reports_dir: dir, ..Default::default() };
        let app = SheetApp::new(session, services);
        let mut h = egui_kittest::Harness::builder().with_size(egui::vec2(1400.0, 1000.0)).build_ui_state(
            |ui, app: &mut SheetApp| {
                let ctx = ui.ctx().clone();
                app.logic(&ctx);
                app.ui(ui);
            },
            app,
        );
        h.run_steps(4);
        h
    }

    /// A build that can send asks once; the answer closes the question for good.
    #[test]
    fn the_first_start_asks_once() {
        let mut h = harness(Some(std::env::temp_dir().join("tinta-reports-test")));
        let _ = h.get_by_label(TITLE);
        h.get_by_label(YES).click();
        h.run_steps(3);
        assert_eq!(h.state().session.prefs.tinta_reports.consent, Some(true));
        assert!(h.query_by_label(TITLE).is_none());
    }

    /// Without a report address (upstream builds, tests, the web) nothing asks.
    #[test]
    fn builds_that_cannot_send_never_ask() {
        let h = harness(None);
        assert!(h.query_by_label(TITLE).is_none());
        assert_eq!(h.state().session.prefs.tinta_reports.consent, None);
    }

    /// The settings show in File › Options › General, and Delete asks the server to forget.
    #[test]
    fn options_turn_reports_off_and_delete_them() {
        let mut h = harness(Some(std::env::temp_dir().join("tinta-reports-test")));
        h.state_mut().run("file.errorReports", json!({"value": true})).unwrap();
        h.state_mut().session.prefs.tinta_reports.install_id = "0f8fad5b-d9cb-469f-a165-70867728950e".into();
        h.state_mut().open_backstage(crate::backstage::Page::Options);
        h.run_steps(4);
        h.get_by_label(CHECKBOX).click();
        h.run_steps(2);
        assert_eq!(h.state().session.prefs.tinta_reports.consent, Some(false));
        h.get_by_label(FORGET).click();
        h.run_steps(2);
        assert!(h.state().session.prefs.tinta_reports.forget_pending.is_some());
        let _ = h.get_by_label(PENDING);
    }

    /// Every string is translated in every language the app offers, and keeps its `{id}`.
    #[test]
    fn every_string_is_translated() {
        for lang in crate::i18n::Language::ALL {
            if lang.code() == "en" {
                continue;
            }
            let (_, t) = TRANSLATIONS.iter().find(|(c, _)| *c == lang.code()).unwrap_or_else(|| panic!("{} missing", lang.code()));
            for (s, t) in STRINGS.iter().zip(t) {
                assert!(!t.trim().is_empty(), "{}: {s:?}", lang.code());
                assert_eq!(s.contains("{id}"), t.contains("{id}"), "{}: {t:?}", lang.code());
            }
        }
    }
}
