use handle::Handle;
use waitable::Waitable;

#[derive(Debug)]
pub struct Process(Handle);

handle!(Process);

impl Waitable for Process {}
