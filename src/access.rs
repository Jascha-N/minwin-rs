use std::ops::BitOr;
use winapi as w;



pub trait Access {
    fn mask(&self) -> u32;
}

pub trait CombinableAccess: Access {
    fn combine<T: CombinableAccess>(&self, other: T) -> CustomAccess {
        CustomAccess(self.mask() | other.mask())
    }
}



pub struct CustomAccess(pub u32);

impl Access for CustomAccess {
    fn mask(&self) -> u32 {
        self.0
    }
}

impl CombinableAccess for CustomAccess {}

impl<T: CombinableAccess> BitOr<T> for CustomAccess {
    type Output = CustomAccess;

    fn bitor(self, other: T) -> CustomAccess {
        self.combine(other)
    }
}



pub struct MaximumAccess;

impl Access for MaximumAccess {
    fn mask(&self) -> u32 {
        w::MAXIMUM_ALLOWED
    }
}



pub struct SystemSecurityAccess;

impl Access for SystemSecurityAccess {
    fn mask(&self) -> u32 {
        w::ACCESS_SYSTEM_SECURITY
    }
}

impl CombinableAccess for SystemSecurityAccess {}

impl<T: CombinableAccess> BitOr<T> for SystemSecurityAccess {
    type Output = CustomAccess;

    fn bitor(self, other: T) -> CustomAccess {
        self.combine(other)
    }
}



access! { GenericAccess,
    All => w::GENERIC_ALL,
    Read => w::GENERIC_READ,
    Write => w::GENERIC_WRITE,
    Execute => w::GENERIC_EXECUTE;
}

access! { StandardAccess,
    Delete => w::DELETE,
    ReadControl => w::READ_CONTROL,
    WriteDac => w::WRITE_DAC,
    WriteOwner => w::WRITE_OWNER,
    Synchronize => w::SYNCHRONIZE;

    all => w::STANDARD_RIGHTS_ALL,
    read => w::STANDARD_RIGHTS_READ,
    write => w::STANDARD_RIGHTS_WRITE,
    execute => w::STANDARD_RIGHTS_EXECUTE,
    required => w::STANDARD_RIGHTS_REQUIRED
}
