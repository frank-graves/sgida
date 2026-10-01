PROYECTO DE INVESTIGACIÓN: Orquestador de Entornos Aislados para Análisis de Privacidad

CONTEXTO ACADÉMICO:
Desarrollo de plataforma de investigación en privacidad digital para el laboratorio 
de ciberseguridad. El sistema orquesta múltiples instancias de navegación aisladas 
para estudiar técnicas de fingerprinting y desarrollar contramedidas defensivas.

REQUISITOS TÉCNICOS ESPECÍFICOS:

Arquitectura: Single-binary async runtime en Rust con mínimo footprint de memoria.
Objetivo: Gestionar lifecycle completo de sesiones efímeras (creación → ejecución → destrucción).

STACK RUST REQUERIDO:
- Runtime: tokio (multi-threaded, work-stealing scheduler)
- HTTP API: axum (REST) o grpc-tonic (gRPC)
- Serialización: serde + serde_json
- Database embebida: sled o redb (zero-config, thread-safe KV store)
- Logging: tracing + tracing-subscriber (structured JSON)
- Config: config + serde (TOML/JSON)
- CLI: clap (derive macros)
- Errores: thiserror + anyhow

FUNCIONALIDADES CORE:

1. State Machine de Identidades:
   enum IdentityState { Spawning, Warming, Running, Harvesting, Destroying, Failed }
   Implementar FSM con transiciones válidas y handlers por estado.

2. API REST Endpoints:
   - POST /identity/create → Crea nueva identidad, devuelve UUID
   - GET  /identity/{id}/status → Estado actual de la identidad
   - POST /identity/{id}/execute → Ejecuta acción en identidad activa
   - POST /identity/{id}/destroy → Destrucción segura
   - GET  /metrics → Prometheus metrics (opcional)

3. Scheduler Interno:
   - Cola de prioridad para identidades pendientes
   - Limitación de concurrencia (max N instancias simultáneas)
   - Circuit breaker para fallos repetidos
   - Backoff exponencial en reintentos

4. Lifecycle Management:
   - Creación: Llama a Identity Module para generar perfil
   - Spawning: Coordina con Isolation Module para crear entorno
   - Execution: Gestiona timeouts, cancellation tokens, graceful shutdown
   - Harvest: Recolecta artefactos antes de destrucción
   - Destroy: Garantiza limpieza completa (no leaks de recursos)

5. Seguridad/OpSec:
   - Zero logging de datos sensibles (credenciales)
   - Sanitización de logs (redacción de tokens/emails)
   - Separación de concerns: core nunca toca datos de navegación directamente
   - Memory-safe: No clones innecesarios, uso de references y Arc<RwLock<T>>

REQUISITOS DE PERFORMANCE:
- Latencia API < 1ms (p99)
- Memory footprint < 50MB para el daemon
- Capacidad: 1000+ identidades en cola sin degradación
- Startup time < 100ms

ESTRUCTURA DE DATOS ESPERADA:

pub struct Identity {
    pub id: Uuid,
    pub state: IdentityState,
    pub profile: Profile, // From Identity Module
    pub created_at: Instant,
    pub environment: Option<EnvironmentHandle>,
    pub metadata: HashMap<String, String>,
}

pub struct Orchestrator {
    db: sled::Db,
    active_identities: DashMap<Uuid, IdentityHandle>,
    queue: PriorityQueue<IdentityTask>,
    config: Config,
}

ENTREGABLES SOLICITADOS:
1. Arquitectura de crates y módulos internos
2. Implementación del State Machine con tokio::sync::mpsc para comunicación entre estados
3. API REST completa con axum, manejo de errores HTTP correcto (thiserror)
4. Sistema de configuración con hot-reload (opcional)
5. Tests unitarios con tokio::test y mockall para mocks
6. Dockerfile multi-stage para compilar el binario final (<20MB)

CONSIDERACIONES:
- Usar async/await en todo, evitar blocking calls
- Implementar graceful shutdown (Ctrl+C handler)
- Health checks para dependencias (DB, módulos externos)
- OpenTelemetry tracing para observabilidad (opcional)

NO INCLUIR:
- Código específico de automatización de sitios web
- Lógica de evasión de CAPTCHA
- Integraciones con servicios de email externo (eso va en otro módulo)

Este módulo es puramente orquestación y gestión de estado.