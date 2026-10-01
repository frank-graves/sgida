PROYECTO DE INVESTIGACIÓN: Aislamiento de Procesos para Análisis de Contención de Fingerprinting

CONTEXTO:
Desarrollo de driver de aislamiento en Rust para crear entornos de ejecución 
efímeros. Cada entorno debe tener filesystem, network y process namespace 
independientes para garantizar zero cross-contamination entre sesiones de análisis.

REQUISITOS TÉCNICOS:

Crate `sgida-isolation` que abstrae múltiples backends de aislamiento.

STACK RUST:
- Containers: podman-api (cliente Rust) o shiplift (Docker)
- Namespaces: nix (libc bindings)
- Filesystem: tempfile + fs_extra
- Process: tokio::process
- Networking: rtnetlink (netlink bindings) o simple command calls
- Seccomp: seccomp-sys (opcional, para sandboxing)

ABSTRACCIÓN DE BACKENDS:

pub trait IsolationBackend {
    async fn create(&self, spec: ContainerSpec) -> Result<Container, Error>;
    async fn destroy(&self, container: &Container) -> Result<(), Error>;
    async fn exec(&self, container: &Container, cmd: &[String]) -> Result<Output, Error>;
    async fn copy_in(&self, container: &Container, src: &Path, dst: &Path) -> Result<(), Error>;
    async fn copy_out(&self, container: &Container, src: &Path, dst: &Path) -> Result<(), Error>;
}

IMPLEMENTACIONES REQUERIDAS:

1. PodmanBackend (prioridad 1):
   - Usa podman (rootless, daemonless)
   - Comandos: podman run --rm --network none --name {uuid} ...
   - Volumen montado para perfil del navegador
   - Limitar recursos: --memory=1g --cpus=1.0

2. FirecrackerBackend (opcional, investigación):
   - Usar crate `firecracker-rs` o HTTP API
   - MicroVMs con kernel custom
   - Más overhead pero aislamiento real de kernel

ESTRUCTURA DE DATOS:

pub struct ContainerSpec {
    pub id: Uuid,
    pub image: String, // "docker.io/library/alpine:latest" o "localhost/sgida-browser:latest"
    pub env_vars: HashMap<String, String>,
    pub mounts: Vec<Mount>,
    pub network_mode: NetworkMode, // None, Bridge, Container(id)
    pub resources: ResourceLimits,
}

pub struct ResourceLimits {
    pub memory_mb: u64,
    pub cpu_shares: u64,
    pub pids_limit: u64,
}

pub struct Container {
    pub id: Uuid,
    pub backend_id: String, // Container ID de podman/firecracker
    pub ip_address: Option<IpAddr>,
    pub created_at: Instant,
}

AISLAMIENTO DE RED (CRÍTICO):

Implementar NetworkNamespace:

pub struct NetworkNamespace {
    pub name: String,
    pub veth_pair: (String, String),
    pub ip_addr: IpAddr,
    pub gateway: IpAddr,
}

Funciones:
- setup_veth_pair() -> Crea veth0 (host) y veth1 (container)
- move_to_namespace(iface: &str, ns: &str)
- setup_iptables_rules(ns: &str, allow_only: &[IpAddr])

O usar approach más simple:
- Crear network namespace con `ip netns add {uuid}`
- Configurar veth pair
- Usar `podman run --network ns:/var/run/netns/{uuid}`

FILESYSTEM EFÍMERO:

- Crear tmpfs para cada container: /tmp/sgida/{uuid}/
- Estructura:
  /tmp/sgida/{uuid}/
    ├── profile/        # Datos del navegador (cookies, etc)
    ├── downloads/      # Descargas temporales
    └── logs/           # Logs del navegador
- Al destruir: rm -rf completo (o unmount tmpfs)

SANDBOXING ADICIONAL:

Opcional con seccomp:
- Crear perfil seccomp que bloquee syscalls peligrosas
- Permitir solo: read, write, open, close, mmap, mprotect, etc.
- Bloquear: execve (excepto navegador), ptrace, etc.

LIMPIEZA GARANTIZADA:

Implementar Drop trait para Container:
impl Drop for Container {
    fn drop(&mut self) {
        // Forzar destrucción incluso si el programa panics
        // Usar std::sync::Arc para shared ownership
    }
}

O usar scopeguard pattern para cleanup garantizado.

ENTREGABLES:
1. Trait IsolationBackend bien documentado
2. Implementación PodmanBackend completa (más común)
3. Funciones de utilidad para network namespaces
4. Sistema de montaje de tmpfs con cleanup automático
5. Tests de integración que verifiquen aislamiento real
   (crear archivo en container A, verificar que no existe en B)
6. Ejemplos de uso: creación, ejecución de comando, destrucción

CONSIDERACIONES:
- Manejar caso donde podman no esté instalado (graceful degradation)
- Timeouts en todas las operaciones async
- Logging de operaciones (sin datos sensibles)
- Thread-safety (Arc<Mutex<>> donde sea necesario)