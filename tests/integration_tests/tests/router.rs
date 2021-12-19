use futures_util::FutureExt;
use integration_tests::pb::*;
use tonic::transport::Server;
use tonic::{Request, Response, Status};

struct Svc;

#[tonic::async_trait]
impl test_server::Test for Svc {
    async fn unary_call(&self, _: Request<Input>) -> Result<Response<Output>, Status> {
        Ok(Response::new(Output {}))
    }
}

#[tonic::async_trait]
impl test2_server::Test2 for Svc {
    async fn unary_call(&self, _: Request<Input>) -> Result<Response<Output>, Status> {
        Ok(Response::new(Output {}))
    }
}

#[tonic::async_trait]
impl test3_server::Test3 for Svc {
    async fn unary_call(&self, _: Request<Input>) -> Result<Response<Output>, Status> {
        Ok(Response::new(Output {}))
    }
}

#[tonic::async_trait]
impl test4_server::Test4 for Svc {
    async fn unary_call(&self, _: Request<Input>) -> Result<Response<Output>, Status> {
        Ok(Response::new(Output {}))
    }
}

#[tonic::async_trait]
impl test5_server::Test5 for Svc {
    async fn unary_call(&self, _: Request<Input>) -> Result<Response<Output>, Status> {
        Ok(Response::new(Output {}))
    }
}

#[tonic::async_trait]
impl test6_server::Test6 for Svc {
    async fn unary_call(&self, _: Request<Input>) -> Result<Response<Output>, Status> {
        Ok(Response::new(Output {}))
    }
}

#[tonic::async_trait]
impl test7_server::Test7 for Svc {
    async fn unary_call(&self, _: Request<Input>) -> Result<Response<Output>, Status> {
        Ok(Response::new(Output {}))
    }
}

#[tonic::async_trait]
impl test8_server::Test8 for Svc {
    async fn unary_call(&self, _: Request<Input>) -> Result<Response<Output>, Status> {
        Ok(Response::new(Output {}))
    }
}

#[tonic::async_trait]
impl test9_server::Test9 for Svc {
    async fn unary_call(&self, _: Request<Input>) -> Result<Response<Output>, Status> {
        Ok(Response::new(Output {}))
    }
}

#[tonic::async_trait]
impl test10_server::Test10 for Svc {
    async fn unary_call(&self, _: Request<Input>) -> Result<Response<Output>, Status> {
        Ok(Response::new(Output {}))
    }
}

#[tonic::async_trait]
impl test11_server::Test11 for Svc {
    async fn unary_call(&self, _: Request<Input>) -> Result<Response<Output>, Status> {
        Ok(Response::new(Output {}))
    }
}

#[tonic::async_trait]
impl test12_server::Test12 for Svc {
    async fn unary_call(&self, _: Request<Input>) -> Result<Response<Output>, Status> {
        Ok(Response::new(Output {}))
    }
}

#[tonic::async_trait]
impl test13_server::Test13 for Svc {
    async fn unary_call(&self, _: Request<Input>) -> Result<Response<Output>, Status> {
        Ok(Response::new(Output {}))
    }
}

#[tonic::async_trait]
impl test14_server::Test14 for Svc {
    async fn unary_call(&self, _: Request<Input>) -> Result<Response<Output>, Status> {
        Ok(Response::new(Output {}))
    }
}

#[tonic::async_trait]
impl test15_server::Test15 for Svc {
    async fn unary_call(&self, _: Request<Input>) -> Result<Response<Output>, Status> {
        Ok(Response::new(Output {}))
    }
}

#[tonic::async_trait]
impl test16_server::Test16 for Svc {
    async fn unary_call(&self, _: Request<Input>) -> Result<Response<Output>, Status> {
        Ok(Response::new(Output {}))
    }
}

#[tonic::async_trait]
impl test17_server::Test17 for Svc {
    async fn unary_call(&self, _: Request<Input>) -> Result<Response<Output>, Status> {
        Ok(Response::new(Output {}))
    }
}

#[tonic::async_trait]
impl test18_server::Test18 for Svc {
    async fn unary_call(&self, _: Request<Input>) -> Result<Response<Output>, Status> {
        Ok(Response::new(Output {}))
    }
}

#[tonic::async_trait]
impl test19_server::Test19 for Svc {
    async fn unary_call(&self, _: Request<Input>) -> Result<Response<Output>, Status> {
        Ok(Response::new(Output {}))
    }
}

#[tonic::async_trait]
impl test20_server::Test20 for Svc {
    async fn unary_call(&self, _: Request<Input>) -> Result<Response<Output>, Status> {
        Ok(Response::new(Output {}))
    }
}

#[tonic::async_trait]
impl test21_server::Test21 for Svc {
    async fn unary_call(&self, _: Request<Input>) -> Result<Response<Output>, Status> {
        Ok(Response::new(Output {}))
    }
}

#[tonic::async_trait]
impl test22_server::Test22 for Svc {
    async fn unary_call(&self, _: Request<Input>) -> Result<Response<Output>, Status> {
        Ok(Response::new(Output {}))
    }
}

#[tonic::async_trait]
impl test23_server::Test23 for Svc {
    async fn unary_call(&self, _: Request<Input>) -> Result<Response<Output>, Status> {
        Ok(Response::new(Output {}))
    }
}

#[tonic::async_trait]
impl test24_server::Test24 for Svc {
    async fn unary_call(&self, _: Request<Input>) -> Result<Response<Output>, Status> {
        Ok(Response::new(Output {}))
    }
}

#[tonic::async_trait]
impl test25_server::Test25 for Svc {
    async fn unary_call(&self, _: Request<Input>) -> Result<Response<Output>, Status> {
        Ok(Response::new(Output {}))
    }
}

#[tonic::async_trait]
impl test26_server::Test26 for Svc {
    async fn unary_call(&self, _: Request<Input>) -> Result<Response<Output>, Status> {
        Ok(Response::new(Output {}))
    }
}

#[tonic::async_trait]
impl test27_server::Test27 for Svc {
    async fn unary_call(&self, _: Request<Input>) -> Result<Response<Output>, Status> {
        Ok(Response::new(Output {}))
    }
}

#[tonic::async_trait]
impl test28_server::Test28 for Svc {
    async fn unary_call(&self, _: Request<Input>) -> Result<Response<Output>, Status> {
        Ok(Response::new(Output {}))
    }
}

#[tonic::async_trait]
impl test29_server::Test29 for Svc {
    async fn unary_call(&self, _: Request<Input>) -> Result<Response<Output>, Status> {
        Ok(Response::new(Output {}))
    }
}

#[tonic::async_trait]
impl test30_server::Test30 for Svc {
    async fn unary_call(&self, _: Request<Input>) -> Result<Response<Output>, Status> {
        Ok(Response::new(Output {}))
    }
}

#[tonic::async_trait]
impl test31_server::Test31 for Svc {
    async fn unary_call(&self, _: Request<Input>) -> Result<Response<Output>, Status> {
        Ok(Response::new(Output {}))
    }
}

#[tonic::async_trait]
impl test32_server::Test32 for Svc {
    async fn unary_call(&self, _: Request<Input>) -> Result<Response<Output>, Status> {
        Ok(Response::new(Output {}))
    }
}

#[tonic::async_trait]
impl test33_server::Test33 for Svc {
    async fn unary_call(&self, _: Request<Input>) -> Result<Response<Output>, Status> {
        Ok(Response::new(Output {}))
    }
}

#[tonic::async_trait]
impl test34_server::Test34 for Svc {
    async fn unary_call(&self, _: Request<Input>) -> Result<Response<Output>, Status> {
        Ok(Response::new(Output {}))
    }
}

#[tonic::async_trait]
impl test35_server::Test35 for Svc {
    async fn unary_call(&self, _: Request<Input>) -> Result<Response<Output>, Status> {
        Ok(Response::new(Output {}))
    }
}

#[tokio::test]
async fn router_with_5_services() {
    Server::builder()
        .add_service(test_server::TestServer::new(Svc))
        .add_service(test2_server::Test2Server::new(Svc))
        .add_service(test3_server::Test3Server::new(Svc))
        .add_service(test4_server::Test4Server::new(Svc))
        .add_service(test5_server::Test5Server::new(Svc))
        .serve("127.0.0.1:1339".parse().unwrap())
        .now_or_never();
}

#[tokio::test]
async fn router_with_14_services() {
    Server::builder()
        .add_service(test_server::TestServer::new(Svc))
        .add_service(test2_server::Test2Server::new(Svc))
        .add_service(test3_server::Test3Server::new(Svc))
        .add_service(test4_server::Test4Server::new(Svc))
        .add_service(test5_server::Test5Server::new(Svc))
        .add_service(test6_server::Test6Server::new(Svc))
        .add_service(test7_server::Test7Server::new(Svc))
        .add_service(test8_server::Test8Server::new(Svc))
        .add_service(test9_server::Test9Server::new(Svc))
        .add_service(test10_server::Test10Server::new(Svc))
        .add_service(test11_server::Test11Server::new(Svc))
        .add_service(test12_server::Test12Server::new(Svc))
        .add_service(test13_server::Test13Server::new(Svc))
        .add_service(test14_server::Test14Server::new(Svc))
        .serve("127.0.0.1:1339".parse().unwrap())
        .now_or_never();
}

#[tokio::test]
async fn router_with_35_services() {
    Server::builder()
        .add_service(test_server::TestServer::new(Svc))
        .add_service(test2_server::Test2Server::new(Svc))
        .add_service(test3_server::Test3Server::new(Svc))
        .add_service(test4_server::Test4Server::new(Svc))
        .add_service(test5_server::Test5Server::new(Svc))
        .add_service(test6_server::Test6Server::new(Svc))
        .add_service(test7_server::Test7Server::new(Svc))
        .add_service(test8_server::Test8Server::new(Svc))
        .add_service(test9_server::Test9Server::new(Svc))
        .add_service(test10_server::Test10Server::new(Svc))
        .add_service(test11_server::Test11Server::new(Svc))
        .add_service(test12_server::Test12Server::new(Svc))
        .add_service(test13_server::Test13Server::new(Svc))
        .add_service(test14_server::Test14Server::new(Svc))
        .add_service(test15_server::Test15Server::new(Svc))
        .add_service(test16_server::Test16Server::new(Svc))
        .add_service(test17_server::Test17Server::new(Svc))
        .add_service(test18_server::Test18Server::new(Svc))
        .add_service(test19_server::Test19Server::new(Svc))
        .add_service(test20_server::Test20Server::new(Svc))
        .add_service(test21_server::Test21Server::new(Svc))
        .add_service(test22_server::Test22Server::new(Svc))
        .add_service(test23_server::Test23Server::new(Svc))
        .add_service(test24_server::Test24Server::new(Svc))
        .add_service(test25_server::Test25Server::new(Svc))
        .add_service(test26_server::Test26Server::new(Svc))
        .add_service(test27_server::Test27Server::new(Svc))
        .add_service(test28_server::Test28Server::new(Svc))
        .add_service(test29_server::Test29Server::new(Svc))
        .add_service(test30_server::Test30Server::new(Svc))
        .add_service(test31_server::Test31Server::new(Svc))
        .add_service(test32_server::Test32Server::new(Svc))
        .add_service(test33_server::Test33Server::new(Svc))
        .add_service(test34_server::Test34Server::new(Svc))
        .add_service(test35_server::Test35Server::new(Svc))
        .serve("127.0.0.1:1339".parse().unwrap())
        .now_or_never();
}
