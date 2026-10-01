PROYECTO DE INVESTIGACIÓN: Mitigación de Detección de Automatización para Estudios de Privacidad

CONTEXTO:
Investigación de técnicas utilizadas por sitios web para detectar navegación 
automatizada. Desarrollo de parches y mitigaciones para permitir estudios 
comparativos de efectividad de estas técnicas en entornos controlados.

REQUISITOS TÉCNICOS:

Crate `sgida-stealth` que proporciona capacidades de "stealth" para navegadores 
automatizados. Enfoque en modificación del navegador y del entorno de ejecución.

STACK RUST:
- Inyección: frida-rs (bindings para Frida)
- Patch: diff + patch crates (manipulación de binarios)
- TLS: utls (Golang interop vía FFI o reimplementación)
- JS: quickjs o similar (para ejecutar scripts de stealth)

COMPONENTES:

1. Browser Patcher:
   - Modificar binario de LibreWolf/Firefox para:
     * Eliminar flag `--remote-debugging-port` detection
     * Modificar navigator.webdriver (siempre undefined)
     * Parchear Chrome DevTools Protocol leaks
   
   - O usar approach runtime con LD_PRELOAD:
     * Crear librería .so que intercepte llamadas a getenv, etc.
     * Interceptar lectura de /proc/self/exe para ocultar puppeteer

2. JavaScript Injection Engine:
   - Inyectar scripts anti-detection antes de cargar cualquier página
   - Scripts a implementar:
     * navigator.webdriver = undefined
     * chrome.runtime = undefined
     * Modificación de prototype de CanvasRenderingContext2D (noise)
     * Modificación de WebGLRenderingContext (spoofing de vendor/renderer)
     * Modificación de AudioBuffer (fingerprint consistente)
     * Ocultación de plugins (navigator.plugins)
   
   - Implementar como WebExtension (manifest v2) o inyección CDP

3. TLS Fingerprint Spoofing:
   - Integrar con utls (Golang) o reimplementar en Rust:
     * JA3 fingerprint customization
     * Randomización de cipher suites
     * Modificación de TLS extensions (SNI, ALPN, etc.)
   
   - Crear proxy local SOCKS5 que modifique el handshake TLS

4. WebRTC Leak Prevention:
   - Configurar browser policies para deshabilitar WebRTC
   - O modificar APIs para retornar IPs falsas/privadas

IMPLEMENTACIÓN JS STEALTH (Ejemplo):

Crear archivo `stealth.js` que se inyecte:

```javascript
// WebDriver detection bypass
Object.defineProperty(navigator, 'webdriver', {
    get: () => undefined,
    enumerable: true,
    configurable: true
});

// Chrome object sanitization
window.chrome = {
    runtime: {},
    loadTimes: function() {},
    csi: function() {},
    app: {}
};

// Canvas noise (consistente por sesión)
const originalGetImageData = CanvasRenderingContext2D.prototype.getImageData;
CanvasRenderingContext2D.prototype.getImageData = function(x, y, w, h) {
    const data = originalGetImageData.call(this, x, y, w, h);
    // Añadir noise imperceptible basado en session seed
    return addNoise(data, SESSION_SEED);
};

INYECCIÓN EN PLAYWRIGHT:

Implementar trait StealthInjector:

    fn inject_preload_script(&self, page: &Page, script: &str) -> Result<(), Error>;
    fn apply_stealth_flags(&self, launch_options: &mut LaunchOptions);

FLAGS DE NAVEGADOR:

pub struct StealthFlags {
pub disable_blink_features: Vec<String>, // "AutomationControlled"
pub disable_features: Vec<String>,
pub user_agent_override: Option<String>,
pub window_size: (u32, u32),
pub proxy: Option<ProxyConfig>,
}

Implementar función que genere vector de args para lanzar LibreWolf:

    --no-first-run
    --no-default-browser-check
    --disable-blink-features=AutomationControlled
    --disable-features=IsolateOrigins,site-per-process
    --window-size={w},{h}
    --user-agent={ua}
    --disable-web-security (opcional, para CORS)
    --disable-features=WebRtcHideLocalIpsWithMdns (WebRTC)

ENTREGABLES:

    Sistema de inyección JS (archivos embebidos en binario con include_str!)
    Funciones de generación de flags de navegador
    Implementación de proxy TLS con JA3 spoofing (usar tokio-socks + tls)
    Tests que verifiquen stealth (ej: verificar navigator.webdriver es undefined)
    Documentación de cada técnica de evasión implementada

NOTA:
Este módulo es puramente para investigación de técnicas de detección y
desarrollo de contramedidas defensivas. No incluir exploits, solo mitigaciones
de fingerprinting conocidas.