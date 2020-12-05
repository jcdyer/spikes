#![feature(generic_associated_types)]

use std::net;

trait Monad /* : Applicative (for pure/return, doesn't matter for this example) */ {
    // Self is like the "f a" in haskell

    /// extract the "a" from "f a"
    type Unplug;

    /// exchange the "a" in "f a" in the type of Self with B
    type Plug<B>: Monad<Unplug=B>;

    fn bind<B, F>(self, f: F) -> Self::Plug<B>
    where
        F: Fn(Self::Unplug) -> Self::Plug<B>;

    fn wrap(val: Self::Unplug) -> Self;
}

impl<A> Monad for Option<A> {
    type Unplug = A;
    type Plug<B> = Option<B>;

    fn bind<B, F>(self, f: F) -> Option<B>
    where
        F: Fn(A) -> Option<B>,
    {
        self.and_then(f)
    }

    fn wrap(val: A) -> Option<A> {
        Some(val)
    }
}

impl<A, E> Monad for Result<A, E> {
    type Unplug = A;
    type Plug<B> = Result<B, E>;

    fn bind<B, F>(self, f: F) -> Result<B, E>
    where
        F: Fn(A) -> Result<B, E>
    {
        self.and_then(f)
    }

    fn wrap(val: A) -> Result<A, E> {
        Ok(val)
    }
}

impl<A> Monad for Vec<A>  {
    type Unplug = A;
    type Plug<B> = Vec<B>;

    fn wrap(val: A) -> Vec<A> {
        vec![val]
    }

    fn bind<B, F>(self, f: F) -> Vec<B>
    where
        F: Fn(A) -> Vec<B>
    {
            self.into_iter()
                .map(f)
                .flatten()
                .collect()

    }
}


fn bind_port<M>(monad: M) -> M::Plug<u16>
where
    M: Monad<Unplug=net::SocketAddr>,
{
    monad.bind(|socketaddr| Monad::wrap(socketaddr.port()))
}


fn main() {
    let opt = Some(net::SocketAddr::from(([192, 168, 0, 4], 127)));
    let res = "[::1]:80".parse::<net::SocketAddr>();
    let v = vec![
        net::SocketAddr::from((net::Ipv6Addr::LOCALHOST, 25)),
        net::SocketAddr::from(([192, 168, 0, 4], 587)),
    ];

    println!("{:?}", bind_port(opt));
    println!("{:?}", bind_port(res));
    println!("{:?}", bind_port(v));
}
