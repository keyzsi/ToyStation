use std::{net::SocketAddr, sync::Arc};
use rustls::pki_types::{CertificateDer, PrivateKeyDer, pem::PemObject};
use h3_quinn::quinn::crypto::rustls::QuicServerConfig;
use h3_webtransport::server::WebTransportSession;
use h3_datagram::server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    let cert = CertificateDer::from_pem_file("./tls/localhost+2.pem").unwrap();
    let key = PrivateKeyDer::from_pem_file("./tls/localhost+2-key.pem").unwrap();

    let mut tls_config = rustls::ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(vec![cert], key)?;

    tls_config.max_early_data_size = 0;
    tls_config.alpn_protocols = vec![b"h3".into()];

    let listen: SocketAddr = "127.0.0.1:4433".parse()?;
    let server_config = quinn::ServerConfig::with_crypto(Arc::new(QuicServerConfig::try_from(tls_config)?));
    let endpoint = quinn::Endpoint::server(server_config, listen)?;

    println!("Listening on :4433");

    while let Some(new_conn) = endpoint.accept().await {
        tokio::spawn(async move {
            match new_conn.await {
                Ok(conn) => {
                    let quinn_conn = h3_quinn::Connection::new(conn);
                    let mut server_builder = h3::server::builder();

                    server_builder.enable_webtransport(true);
                    server_builder.enable_extended_connect(true);
                    server_builder.max_webtransport_sessions(128);
                    server_builder.enable_datagram(true);

                    let mut h3_conn: h3::server::Connection<h3_quinn::Connection, bytes::Bytes> = server_builder.build(quinn_conn)
                        .await
                        .unwrap();

                    loop {
                        match h3_conn.accept().await {
                            Ok(Some((req, stream))) => {
                                //match WebTransportSession::accept(req, stream, h3_conn).await {
                                //    Ok(session) => {
                                //        println!("Session established");
                                //    }
                                //    Err(err) => {
                                //        break;
                                //    }
                                //}
                            }
                            Ok(None) => {
                                break;
                            }
                            Err(err) => {
                                break;
                            }
                        }
                    }
                }
                Err(err) => {
                    println!("{:?}", err);
                }
            }
        });
    }

    Ok(())
}