PROYECTO DE INVESTIGACIÓN: Simulación de Comportamiento de Usuario para Calibración de Sistemas Anti-Bot

CONTEXTO:
Desarrollo de biblioteca de generación de comportamiento humano sintético 
para pruebas de usabilidad y calibración de thresholds de detección de bots. 
El objetivo es crear patrones de interacción indistinguibles de usuarios reales 
usando modelos matemáticos de interacción humana.

REQUISITOS TÉCNICOS:

Crate `sgida-behavior` con algoritmos de comportamiento.

STACK RUST:
- Matemáticas: nalgebra (vectores, matrices)
- Curvas: kurbo (curvas de Bézier) o implementación propia
- Random: rand_distr (distribuciones estadísticas)
- Time: tokio::time
- Serialización: serde

ALGORITMOS REQUERIDOS:

1. Curvas de Bézier para Movimiento de Mouse:
   - Implementar curvas cúbicas de Bézier: B(t) = (1-t)³P₀ + 3(1-t)²tP₁ + 3(1-t)t²P₂ + t³P₃
   - P₀: posición inicial
   - P₃: posición final (objetivo con offset aleatorio)
   - P₁, P₂: puntos de control generados aleatoriamente pero suaves
   
   - Velocidad variable: Perlin noise o distribución log-normal
   - Errores humanos: overshoot ocasional, corrección, micro-tremor

pub fn generate_mouse_path(
    start: Point2<f64>,
    end: Point2<f64>,
    duration_ms: u64,
    seed: u64,
) -> Vec<(Point2<f64>, u64)> { // (posición, timestamp)

2. Patrones de Scroll:
   - Scroll no lineal: aceleración/desaceleración
   - Pausas de lectura: distribución exponencial (tiempo entre scrolls)
   - Scroll hacia atrás: ocasionalmente releer contenido
   - Velocidad dependiente de densidad de texto

pub struct ScrollPattern {
    pub segments: Vec<ScrollSegment>,
    pub total_duration: Duration,
}

pub struct ScrollSegment {
    pub direction: ScrollDirection,
    pub distance: f64,
    pub duration: Duration,
    pub pause_after: Duration,
}

3. Timing de Acciones:
   - Distribución log-normal para tiempos de reacción
   - Media: 250ms, sigma: 0.5 (ajustable)
   - Outliers ocasionales (pensamiento profundo: 2-5s)

pub fn reaction_time() -> Duration {
    let dist = LogNormal::new(250.0_f64.ln(), 0.5).unwrap();
    Duration::from_millis(dist.sample(&mut rng) as u64)
}

4. Patrones de Tipeo:
   - Velocidad de tipeo variable: 200-600 CPM (caracteres por minuto)
   - Pausas entre palabras: distribución normal
   - Errores de tipeo ocasionales con corrección (backspace)
   - Velocidad dependiente de complejidad de palabra

pub struct TypingPattern {
    pub text: String,
    pub events: Vec<TypingEvent>, // KeyDown, KeyUp, Delay
}

pub enum TypingEvent {
    KeyPress(char, Duration), // tecla, delay hasta siguiente
    Backspace(Duration),
    Pause(Duration),
}

5. Warm-up Traffic:
   - Generar secuencias de navegación realista
   - Sitios de referencia: Google, Wikipedia, Reddit, noticias
   - Click patterns: 70% de clicks en resultados de búsqueda
   - Dwell time: distribución gamma en páginas visitadas

pub struct WarmupScenario {
    pub sites: Vec<SiteVisit>,
    pub total_duration: Range<Duration>,
}

impl WarmupScenario {
    pub fn generate(seed: u64, locale: &str) -> Self {
        // Generar secuencia coherente basada en locale
        // Ej: locale es-ES → visitar El País, Marca, etc.
    }
}

ACTION PLANNER:

Implementar Behavior Tree o FSM para acciones complejas:

pub enum Action {
    Navigate(String),
    Click(ElementSelector),
    Type(ElementSelector, String),
    Scroll(ScrollDirection, f64),
    Wait(Duration),
    ReadContent, // Simula lectura (espera basada en densidad de texto)
}

pub struct ActionSequence {
    actions: Vec<Action>,
    current: usize,
}

impl ActionSequence {
    pub async fn execute(&mut self, controller: &BrowserController) -> Result<(), Error> {
        // Ejecutar cada acción con delays humanos entre ellas
    }
}

ENTREGABLES:
1. Implementación de curvas de Bézier para mouse
2. Generador de patrones de scroll realistas
3. Simulador de tipeo con errores ocasionales
4. Generador de escenarios de warm-up
5. Ejemplos de uso integrados con Playwright (o trait genérico)
6. Benchmarks: latencia de generación de paths

CONSIDERACIONES:
- Seed-based para reproducibilidad
- Paralelizable (generar paths en paralelo)
- No bloquear async runtime (usar tokio::task::spawn_blocking para cálculo intensivo)
- Documentación de modelos matemáticos usados (Fitts's Law, etc.)