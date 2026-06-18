use simplex::provider::SimplicityNetwork;

#[derive(Debug, Clone)]
pub struct Session<P, S> {
    provider: P,
    signer: S,
    network: SimplicityNetwork,
}

impl<P, S> Session<P, S> {
    pub fn new(provider: P, signer: S, network: SimplicityNetwork) -> Self {
        Self {
            provider,
            signer,
            network,
        }
    }

    pub fn provider(&self) -> &P {
        &self.provider
    }

    pub fn signer(&self) -> &S {
        &self.signer
    }

    pub fn network(&self) -> &SimplicityNetwork {
        &self.network
    }

    pub fn into_parts(self) -> (P, S, SimplicityNetwork) {
        (self.provider, self.signer, self.network)
    }
}

#[cfg(test)]
mod tests {
    use super::Session;
    use simplex::provider::SimplicityNetwork;

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct DummyProvider;

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct DummySigner;

    #[test]
    fn stores_dependencies() {
        let session = Session::new(DummyProvider, DummySigner, SimplicityNetwork::LiquidTestnet);

        assert_eq!(*session.provider(), DummyProvider);
        assert_eq!(*session.signer(), DummySigner);
        assert_eq!(*session.network(), SimplicityNetwork::LiquidTestnet);
    }

    #[test]
    fn can_destructure_session() {
        let session = Session::new(DummyProvider, DummySigner, SimplicityNetwork::LiquidTestnet);
        let (provider, signer, network) = session.into_parts();

        assert_eq!(provider, DummyProvider);
        assert_eq!(signer, DummySigner);
        assert_eq!(network, SimplicityNetwork::LiquidTestnet);
    }
}
