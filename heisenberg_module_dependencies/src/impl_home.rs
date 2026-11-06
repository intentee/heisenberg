use crate::local_impl_home::LocalImplHome;

pub enum ImplHome {
    Foreign,
    Local { home: LocalImplHome },
}
