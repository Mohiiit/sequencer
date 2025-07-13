use std::io::Error as IoError;

use async_trait::async_trait;
use futures::{Sink, Stream};
use libp2p::PeerId;
#[cfg(any(feature = "testing", test))]
use mockall::automock;

pub trait NegotiatorChannel:
    Sink<Vec<u8>, Error = IoError> + Stream<Item = Result<Vec<u8>, IoError>> + Unpin + Send
{
}

// Blanket implementation for any type that implements all the required traits
impl<T> NegotiatorChannel for T where
    T: Sink<Vec<u8>, Error = IoError> + Stream<Item = Result<Vec<u8>, IoError>> + Unpin + Send
{
}

pub enum NegotiatorOutput {
    None,
    /// Returned when the handshake concluded that the currently connecting peer is a duplicate of
    /// the currently connected peer. `PeerId` is the (older) currently connected peer.
    DuplicatePeer(PeerId),
}

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum NegotiatorError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("Authentication failed")]
    AuthenticationFailed,
}

#[async_trait]
#[cfg_attr(any(feature = "testing", test), automock)]
pub trait Negotiator: Send + Clone {
    /// Performs the handshake protocol when we are the incoming connection side.
    /// `connection` is the channel that can be used to communicate with the other peer.
    async fn negotiate_incoming_connection(
        &mut self,
        my_peer_id: PeerId,
        other_peer_id: PeerId,
        connection: &mut dyn NegotiatorChannel,
    ) -> Result<NegotiatorOutput, NegotiatorError>;

    /// Performs the handshake protocol when we are the outgoing connection side.
    /// `connection` is the channel that can be used to communicate with the other peer.
    async fn negotiate_outgoing_connection(
        &mut self,
        my_peer_id: PeerId,
        other_peer_id: PeerId,
        connection: &mut dyn NegotiatorChannel,
    ) -> Result<NegotiatorOutput, NegotiatorError>;

    /// A unique identified for your authentication protocol. For example: "strk_id" or
    /// "strk_id_v2".
    // TODO(guy.f): Consider making this a const.
    fn protocol_name(&self) -> &'static str;
}

// Automock does not implement Clone, so we need to do it manually.
#[cfg(any(feature = "testing", test))]
impl Clone for MockNegotiator {
    fn clone(&self) -> Self {
        MockNegotiator::default()
    }
}
