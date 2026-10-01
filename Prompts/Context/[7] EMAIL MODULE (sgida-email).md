PROYECTO DE INVESTIGACIÓN: Automatización de Flujos de Email para Testing de Seguridad de Autenticación

CONTEXTO:
Desarrollo de gestor de email automatizado para pruebas de flujos de verificación 
en aplicaciones propias. El sistema debe poder recibir, parsear y responder a 
emails de verificación de forma automatizada para validar la robustez de sistemas 
de autenticación.

REQUISITOS TÉCNICOS:

Crate `sgida-email` con capacidades de IMAP/SMTP y parsing.

STACK RUST:
- IMAP: imap (crate) o async-imap
- SMTP: lettre
- Parsing: mailparse (MIME parsing)
- HTML: scraper (selectores CSS) o nipper
- Regex: regex
- HTTP: reqwest (para click de links)

COMPONENTES:

1. Email Receiver:
   - Conexión IMAP a servidores de email temporal
   - Polling o IDLE mode
   - Búsqueda de emails no leídos por remitente/asunto
   
pub struct ImapClient {
    session: imap::Session<TlsStream<TcpStream>>,
    mailbox: String,
}

impl ImapClient {
    pub fn fetch_unread(&mut self, from_pattern: &str) -> Result<Vec<Email>, Error>;
    pub fn wait_for_email(&mut self, timeout: Duration, criteria: SearchCriteria) -> Result<Email, Error>;
}

2. Email Parser:
   - Extracción de texto plano y HTML
   - Parsing de links (href extraction)
   - Extracción de códigos OTP (regex: \b\d{4,8}\b)
   - Manejo de multipart/mixed, multipart/alternative

pub struct ParsedEmail {
    pub subject: String,
    pub from: String,
    pub to: String,
    pub date: DateTime<Utc>,
    pub body_text: String,
    pub body_html: Option<String>,
    pub links: Vec<Url>,
    pub otp_codes: Vec<String>,
    pub attachments: Vec<Attachment>,
}

3. Link Clicker:
   - Extracción de URLs de verificación
   - HTTP GET/POST a links (siguiendo redirects)
   - Manejo de cookies si es necesario
   - Detección de páginas de éxito/error

pub struct LinkClicker {
    client: reqwest::Client,
}

impl LinkClicker {
    pub async fn click_verification_link(&self, url: &Url) -> Result<VerificationResult, Error>;
}

4. SMTP Sender (Opcional):
   - Envío de emails si es necesario para verificación
   - Configuración de SMTP relay

ADAPTADORES DE EMAIL TEMPORAL:

Implementar trait EmailProvider:

pub trait EmailProvider {
    async fn create_inbox(&self) -> Result<Inbox, Error>;
    async fn get_emails(&self, inbox: &Inbox) -> Result<Vec<Email>, Error>;
    async fn wait_for_email(&self, inbox: &Inbox, timeout: Duration) -> Result<Email, Error>;
    async fn destroy_inbox(&self, inbox: &Inbox) -> Result<(), Error>;
}

Implementaciones:
- MailinatorProvider (API REST)
- GuerrillaMailProvider
- TempMailProvider
- CustomImapProvider (para Outlook/Gmail propios)

OTP EXTRACTION:

pub fn extract_otp(text: &str) -> Vec<String> {
    let re = Regex::new(r"\b\d{4,8}\b").unwrap();
    re.find_iter(text)
        .map(|m| m.as_str().to_string())
        .filter(|code| is_likely_otp(code)) // Filtrar años, etc.
        .collect()
}

fn is_likely_otp(code: &str) -> bool {
    // No debe ser año reciente (2020-2030)
    // No debe ser número secuencial (1234, 0000)
    // etc.
}

ENTREGABLES:
1. Cliente IMAP async completo
2. Parser de emails con extracción de OTP
3. Implementación de 2-3 proveedores de email temporal
4. Sistema de click de links con manejo de redirects
5. Tests con emails de ejemplo (fixtures)
6. Ejemplos de uso: crear inbox, esperar email, extraer OTP

CONSIDERACIONES:
- Manejo de errores de red (retry)
- Timeouts en operaciones de polling
- No almacenar contenido de emails (solo extraer OTP/links y descartar)
- Rate limiting respetuoso con APIs de email temporal