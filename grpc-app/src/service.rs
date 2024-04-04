use std::convert::Infallible;

use anyhow::Result;
use app::{
  app_service_server::{AppService, AppServiceServer},
  EchoReq, EchoResp,
};
use hyper::service::make_service_fn;
use tonic::{Request, Response, Status};
use tonic_reflection::server::Builder;

use crate::cli::Args;

/// app is the namespace for the GRPC generated code.
pub mod app {
  tonic::include_proto!("proto.app");

  pub const APP_FILE_DESCRIPTOR_SET: &[u8] = tonic::include_file_descriptor_set!("app_descriptor");
}

/// WorkerService handles the GRPC requests.
#[derive(Debug)]
pub struct AppServerService {}

impl AppServerService {
  #[allow(dead_code)]
  async fn new() -> Self {
    AppServerService {}
  }
}

#[tonic::async_trait]
impl AppService for AppServerService {
  async fn echo(&self, req: Request<EchoReq>) -> Result<Response<EchoResp>, Status> {
    log::info!("Got a req from {:?}, req: {:?}", req.remote_addr(), req);
    let reply = app::EchoResp { message: req.into_inner().message };
    Ok(Response::new(reply))
  }
}

pub async fn service_entry(args: Args) -> Result<()> {
  log::info!("args: {:?}", &args);
  let addr = args.addr.parse().unwrap();

  let reflect_service = Builder::configure()
    .register_encoded_file_descriptor_set(app::APP_FILE_DESCRIPTOR_SET)
    .build()
    .unwrap();
  let grpc_service = tonic::transport::Server::builder()
    .add_service(AppServiceServer::new(AppServerService {}))
    .add_service(reflect_service)
    .into_service();

  let make_grpc_service = make_service_fn(move |_conn| {
    let grpc_service = grpc_service.clone();
    async { Ok::<_, Infallible>(grpc_service) }
  });

  hyper::Server::bind(&addr).serve(make_grpc_service).await?;

  Ok(())
}
