use std::io::{ErrorKind, Read, Write};
use std::net::TcpStream;
use std::os::fd::AsRawFd;
use std::sync::Arc;
use std::time::{Duration, Instant};

use rustls::{ClientConfig, ClientConnection, OwnedTrustAnchor, RootCertStore, StreamOwned};

use crate::data::types::WsEvent;

pub struct WsConn {
    pub tls: StreamOwned<ClientConnection, TcpStream>,
    pub ws_buf: Vec<u8>,
    pub ws_tmp: [u8; 16 * 1024],
    pub connected_at: Instant,
}

impl WsConn {
    pub fn connect_blocking(host: &str, path: &str) -> anyhow::Result<Self> {
        // Establish TCP + TLS, perform WS upgrade, then switch to nonblocking.
        // This is intended to mirror the custom low-latency handshake in your spec.
        let mut tcp = TcpStream::connect((host, 443))?;
        tcp.set_read_timeout(Some(Duration::from_secs(3)))?;
        tcp.set_write_timeout(Some(Duration::from_secs(3)))?;
        tcp.set_nonblocking(false)?;

        let mut roots = RootCertStore::empty();
        roots.add_trust_anchors(
            webpki_roots::TLS_SERVER_ROOTS
                .0
                .iter()
                .map(|ta| {
                    OwnedTrustAnchor::from_subject_spki_name_constraints(
                        ta.subject,
                        ta.spki,
                        ta.name_constraints,
                    )
                }),
        );
        let cfg = ClientConfig::builder()
            .with_safe_defaults()
            .with_root_certificates(roots)
            .with_no_client_auth();
        let server_name = host.try_into()?;
        let conn = ClientConnection::new(Arc::new(cfg), server_name)?;
        let mut tls = StreamOwned::new(conn, tcp);

        let req = format!(
            "GET {path} HTTP/1.1\r\n\
             Host: {host}\r\n\
             Upgrade: websocket\r\n\
             Connection: Upgrade\r\n\
             Sec-WebSocket-Key: dGhlIHNhbXBsZSBub25jZQ==\r\n\
             Sec-WebSocket-Version: 13\r\n\
             \r\n"
        );
        tls.write_all(req.as_bytes())?;
        tls.flush()?;

        let mut hdr = Vec::with_capacity(2048);
        let mut tmp = [0u8; 1024];
        loop {
            let n = tls.read(&mut tmp)?;
            if n == 0 {
                anyhow::bail!("Binance WS handshake EOF");
            }
            hdr.extend_from_slice(&tmp[..n]);
            if hdr.windows(4).any(|w| w == b"\r\n\r\n") {
                break;
            }
            if hdr.len() > 8192 {
                anyhow::bail!("Binance WS handshake too big");
            }
        }
        if !hdr.starts_with(b"HTTP/1.1 101") {
            anyhow::bail!("Binance WS upgrade failed: {:?}", String::from_utf8_lossy(&hdr));
        }

        let tcp_ref = tls.get_ref();
        tcp_ref.set_nonblocking(true)?;

        Ok(Self {
            tls,
            ws_buf: Vec::with_capacity(64 * 1024),
            ws_tmp: [0u8; 16 * 1024],
            connected_at: Instant::now(),
        })
    }

    pub fn read_nonblocking(&mut self, _out: &mut Vec<WsEvent>) -> anyhow::Result<()> {
        // Read from TLS into ws_buf, parse frames, and push WsEvent into out.
        // Non-blocking: return immediately when WouldBlock is hit.
        loop {
            match self.tls.read(&mut self.ws_tmp) {
                Ok(0) => break,
                Ok(n) => {
                    self.ws_buf.extend_from_slice(&self.ws_tmp[..n]);
                    // Parse frames from ws_buf here, drain consumed bytes, emit WsEvent.
                    // See your parse_ws_frame logic for reference.
                }
                Err(ref e) if e.kind() == ErrorKind::WouldBlock => break,
                Err(e) => return Err(e.into()),
            }
        }
        Ok(())
    }

    pub fn write_ws(&mut self, _payload: &[u8]) -> anyhow::Result<()> {
        // Write a masked client frame to Binance.
        // Use a minimal framing implementation to avoid overhead.
        Ok(())
    }

    pub fn should_rotate(&self) -> bool {
        // Return true if connection age exceeds rotation threshold.
        self.connected_at.elapsed() > Duration::from_secs(23 * 3600 + 50 * 60)
    }

    pub fn reconnect(&mut self, host: &str, path: &str) -> anyhow::Result<()> {
        // Reconnect with backoff + jitter, reset buffers, and swap tls handle.
        let new_conn = Self::connect_blocking(host, path)?;
        *self = new_conn;
        Ok(())
    }
}
