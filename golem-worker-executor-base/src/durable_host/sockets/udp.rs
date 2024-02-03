use async_trait::async_trait;
use wasmtime::component::Resource;

use crate::durable_host::DurableWorkerCtx;
use crate::metrics::wasm::record_host_function_call;
use crate::workerctx::WorkerCtx;
use wasmtime_wasi::preview2::bindings::wasi::sockets::udp::{
    Host, HostIncomingDatagramStream, HostOutgoingDatagramStream, HostUdpSocket, IncomingDatagram,
    IncomingDatagramStream, IpAddressFamily, IpSocketAddress, Network, OutgoingDatagram,
    OutgoingDatagramStream, Pollable, UdpSocket,
};
use wasmtime_wasi::preview2::SocketError;

impl<Ctx: WorkerCtx> HostUdpSocket for DurableWorkerCtx<Ctx> {
    fn start_bind(
        &mut self,
        self_: Resource<UdpSocket>,
        network: Resource<Network>,
        local_address: IpSocketAddress,
    ) -> Result<(), SocketError> {
        record_host_function_call("sockets::udp", "start_bind");
        HostUdpSocket::start_bind(&mut self.as_wasi_view(), self_, network, local_address)
    }

    fn finish_bind(&mut self, self_: Resource<UdpSocket>) -> Result<(), SocketError> {
        record_host_function_call("sockets::udp", "finish_bind");
        HostUdpSocket::finish_bind(&mut self.as_wasi_view(), self_)
    }

    fn stream(
        &mut self,
        self_: Resource<UdpSocket>,
        remote_address: Option<IpSocketAddress>,
    ) -> Result<
        (
            Resource<IncomingDatagramStream>,
            Resource<OutgoingDatagramStream>,
        ),
        SocketError,
    > {
        record_host_function_call("sockets::udp", "stream");
        HostUdpSocket::stream(&mut self.as_wasi_view(), self_, remote_address)
    }

    fn local_address(
        &mut self,
        self_: Resource<UdpSocket>,
    ) -> Result<IpSocketAddress, SocketError> {
        record_host_function_call("sockets::udp", "local_address");
        HostUdpSocket::local_address(&mut self.as_wasi_view(), self_)
    }

    fn remote_address(
        &mut self,
        self_: Resource<UdpSocket>,
    ) -> Result<IpSocketAddress, SocketError> {
        record_host_function_call("sockets::udp", "remote_address");
        HostUdpSocket::remote_address(&mut self.as_wasi_view(), self_)
    }

    fn address_family(&mut self, self_: Resource<UdpSocket>) -> anyhow::Result<IpAddressFamily> {
        record_host_function_call("sockets::udp", "address_family");
        HostUdpSocket::address_family(&mut self.as_wasi_view(), self_)
    }

    fn unicast_hop_limit(&mut self, self_: Resource<UdpSocket>) -> Result<u8, SocketError> {
        record_host_function_call("sockets::udp", "unicast_hop_limit");
        HostUdpSocket::unicast_hop_limit(&mut self.as_wasi_view(), self_)
    }

    fn set_unicast_hop_limit(
        &mut self,
        self_: Resource<UdpSocket>,
        value: u8,
    ) -> Result<(), SocketError> {
        record_host_function_call("sockets::udp", "set_unicast_hop_limit");
        HostUdpSocket::set_unicast_hop_limit(&mut self.as_wasi_view(), self_, value)
    }

    fn receive_buffer_size(&mut self, self_: Resource<UdpSocket>) -> Result<u64, SocketError> {
        record_host_function_call("sockets::udp", "receive_buffer_size");
        HostUdpSocket::receive_buffer_size(&mut self.as_wasi_view(), self_)
    }

    fn set_receive_buffer_size(
        &mut self,
        self_: Resource<UdpSocket>,
        value: u64,
    ) -> Result<(), SocketError> {
        record_host_function_call("sockets::udp", "set_receive_buffer_size");
        HostUdpSocket::set_receive_buffer_size(&mut self.as_wasi_view(), self_, value)
    }

    fn send_buffer_size(&mut self, self_: Resource<UdpSocket>) -> Result<u64, SocketError> {
        record_host_function_call("sockets::udp", "send_buffer_size");
        HostUdpSocket::send_buffer_size(&mut self.as_wasi_view(), self_)
    }

    fn set_send_buffer_size(
        &mut self,
        self_: Resource<UdpSocket>,
        value: u64,
    ) -> Result<(), SocketError> {
        record_host_function_call("sockets::udp", "set_send_buffer_size");
        HostUdpSocket::set_send_buffer_size(&mut self.as_wasi_view(), self_, value)
    }

    fn subscribe(&mut self, self_: Resource<UdpSocket>) -> anyhow::Result<Resource<Pollable>> {
        record_host_function_call("sockets::udp", "subscribe");
        HostUdpSocket::subscribe(&mut self.as_wasi_view(), self_)
    }

    fn drop(&mut self, rep: Resource<UdpSocket>) -> anyhow::Result<()> {
        record_host_function_call("sockets::udp", "drop");
        HostUdpSocket::drop(&mut self.as_wasi_view(), rep)
    }
}

impl<Ctx: WorkerCtx> HostIncomingDatagramStream for DurableWorkerCtx<Ctx> {
    fn receive(
        &mut self,
        self_: Resource<IncomingDatagramStream>,
        max_results: u64,
    ) -> Result<Vec<IncomingDatagram>, SocketError> {
        record_host_function_call("sockets::udp", "receive");
        HostIncomingDatagramStream::receive(&mut self.as_wasi_view(), self_, max_results)
    }

    fn subscribe(
        &mut self,
        self_: Resource<IncomingDatagramStream>,
    ) -> anyhow::Result<Resource<Pollable>> {
        record_host_function_call("sockets::udp", "subscribe");
        HostIncomingDatagramStream::subscribe(&mut self.as_wasi_view(), self_)
    }

    fn drop(&mut self, rep: Resource<IncomingDatagramStream>) -> anyhow::Result<()> {
        record_host_function_call("sockets::udp", "drop");
        HostIncomingDatagramStream::drop(&mut self.as_wasi_view(), rep)
    }
}

impl<Ctx: WorkerCtx> HostOutgoingDatagramStream for DurableWorkerCtx<Ctx> {
    fn check_send(&mut self, self_: Resource<OutgoingDatagramStream>) -> Result<u64, SocketError> {
        record_host_function_call("sockets::udp", "check_send");
        HostOutgoingDatagramStream::check_send(&mut self.as_wasi_view(), self_)
    }

    fn send(
        &mut self,
        self_: Resource<OutgoingDatagramStream>,
        datagrams: Vec<OutgoingDatagram>,
    ) -> Result<u64, SocketError> {
        record_host_function_call("sockets::udp", "send");
        HostOutgoingDatagramStream::send(&mut self.as_wasi_view(), self_, datagrams)
    }

    fn subscribe(
        &mut self,
        self_: Resource<OutgoingDatagramStream>,
    ) -> anyhow::Result<Resource<Pollable>> {
        record_host_function_call("sockets::udp", "subscribe");
        HostOutgoingDatagramStream::subscribe(&mut self.as_wasi_view(), self_)
    }

    fn drop(&mut self, rep: Resource<OutgoingDatagramStream>) -> anyhow::Result<()> {
        record_host_function_call("sockets::udp", "drop");
        HostOutgoingDatagramStream::drop(&mut self.as_wasi_view(), rep)
    }
}

#[async_trait]
impl<Ctx: WorkerCtx> Host for DurableWorkerCtx<Ctx> {}
