use std::io;
use std::io::Write;
use std::mem;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::PoisonError;

#[derive(Clone, Default)]
pub struct CapturedOutput {
    pub bytes: Arc<Mutex<Vec<u8>>>,
}

impl CapturedOutput {
    pub fn take(&self) -> Vec<u8> {
        mem::take(&mut *self.bytes.lock().unwrap_or_else(PoisonError::into_inner))
    }
}

impl Write for CapturedOutput {
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
    fn write(&mut self, written: &[u8]) -> io::Result<usize> {
        self.bytes
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .extend_from_slice(written);

        Ok(written.len())
    }
}
