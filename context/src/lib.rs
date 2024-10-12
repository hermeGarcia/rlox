pub mod src_library;

/// Contains data that may be relevant at any point during
/// the compiler's execution.
pub struct Context {
    pub src_library: src_library::SrcLibrary,
}
