PROYECTO DE INVESTIGACIÓN: Gestión de Identidad de Red para Análisis de Privacidad de Conexiones

CONTEXTO:
Desarrollo de gestor de networking en Rust para aislamiento completo de tráfico 
de red por sesión. Cada identidad debe tener configuración DNS, proxy y TLS 
independiente para estudios de correlación de tráfico.

REQUISITOS TÉCNICOS:

Crate `sgida-network` que gestione conectividad aislada.

STACK RUST:
- Async: tokio
- HTTP client: hyper o reqwest (para health checks)
- DNS: trust-dns-resolver o hickory-resolver
- Proxy: tokio-socks (SOCKS5)
- VPN: wireguard-rs o boringtun (userspace WireGuard)
- Config: wireguard-config
- IP: ipnet (manipulación de rangos IP)

COMPONENTES:

1. Proxy Rotator:
   - Gestión de pool de proxies SOCKS5/HTTP
   - Rotación por identidad (cada identidad tiene proxy asignado)
   - Health checks periódicos (latencia, disponibilidad)
   - Blacklisting de proxies fallidos

pub struct ProxyPool {
    proxies: Vec<Proxy>,
    health_checker: HealthChecker,
    blacklist: DashSet<Proxy>,
}

pub struct Proxy {
    pub url: String, // socks5://user:pass@host:port
    pub protocol: ProxyProtocol,
    pub location: GeoLocation,
    pub latency_ms: Option<u64>,
}

2. DNS Resolver Aislado:
   - Cada identidad usa resolv.conf independiente
   - O usar trust-dns con configuración per-query
   - DNS over HTTPS (DoH) para privacidad
   - Caché DNS aislada por identidad

pub struct DnsConfig {
    pub nameservers: Vec<SocketAddr>,
    pub doh_enabled: bool,
    pub hosts_overrides: HashMap<String, IpAddr>,
}

3. WireGuard Manager:
   - Crear interfaz wg0-{uuid} por identidad
   - Configurar con claves privadas efímeras
   - Routing table separada (namespace)
   - Kill switch (si VPN cae, no hay tráfico)

pub struct WireGuardTunnel {
    pub interface: String,
    pub config: WireGuardConfig,
    pub namespace: Option<String>, // Network namespace
}

4. Traffic Shaping (Opcional):
   - Limitación de ancho de banda (tc en Linux)
   - Simulación de latencia variable
   - Packet loss artificial (para simular mala conexión)

NETWORK NAMESPACE SETUP:

Implementar función que configure:
- Crear namespace: ip netns add {uuid}
- Crear veth pair: ip link add veth0 type veth peer name veth1
- Mover veth1 a namespace: ip link set veth1 netns {uuid}
- Configurar IPs: ip addr add ...
- Levantar interfaces: ip link set ...
- Configurar routing: ip route add default via ...
- Configurar DNS: mount bind /etc/resolv.conf en namespace

O usar approach de containers (Podman hace esto automáticamente).

WEBRTC LEAK PROTECTION:

- Configurar browser para deshabilitar WebRTC
- O modificar políticas: media.peerconnection.enabled = false
- Enmascarar IPs locales si WebRTC está activo

TLS/SSL:

- Opción de usar certificados propios para MITM local (solo para debugging)
- Verificación de certificados configurable
- OCSP stapling checks

ENTREGABLES:
1. Implementación de ProxyPool con health checks
2. Configurador de network namespaces (funciones auxiliares)
3. Integración opcional con WireGuard
4. Tests de aislamiento (verificar que tráfico sale por IP correcta)
5. Ejemplo de uso: crear identidad, asignar proxy, verificar IP pública

CONSIDERACIONES:
- Async/await para todas las operaciones de red
- Timeouts en health checks
- Manejo de errores de red (retry con backoff)
- No logging de URLs visitadas (solo metadata de conexión)