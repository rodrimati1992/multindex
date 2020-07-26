/// Ensuring that shared references can't be used in `*mut` indexing macros
///
/// ```compile_fail
/// let mut arr = [0];
/// let _ = multindex::multindex_mut!(&arr; 0 );
/// ```
///
/// ```rust
/// let mut arr = [0];
/// let _ = multindex::multindex_mut!(&mut arr; 0 );
/// ```
///
pub struct SharedMutMixingTest;

///
/// ```compile_fail
/// fn foo<'a, 'b, T>(slice: &'a [T]) -> (&'b T,) {
///     multindex::multindex!(slice; 0 )
/// }
/// ```
///
/// ```
/// fn foo<'a, T>(slice: &'a [T]) -> (&'a T,) {
///     multindex::multindex!(slice; 0 )
/// }
/// ```
pub struct AvoidingLifetimeTransmutes;

///
///
/// ```compile_fail
/// fn foo<'a, 'b, T>(slice: &'a mut [T]) -> (&'b mut T,) {
///     multindex::multindex_mut!(slice; 0 )
/// }
/// ```
///
/// ```
/// fn foo<'a, T>(slice: &'a mut [T]) -> (&'a mut T,) {
///     multindex::multindex_mut!(slice; 0 )
/// }
/// ```
///
pub struct AvoidingLifetimeTransmutesMut;

///
/// ```compile_fail
/// let arr = [(); 100];
/// multindex::multindex!(arr; 0.., ..10 );
/// ```
///
/// ```rust
/// let arr = [(); 100];
/// multindex::multindex!(arr; 0.., 10 );
/// ```
///
pub struct NextStartIsUnboundedError;

///
/// ```compile_fail
/// let arr = [(); 100];
/// multindex::multindex!(arr; 20.., 10 );
/// ```
///
/// ```rust
/// let arr = [(); 100];
/// multindex::multindex!(arr; 20.., 30 );
/// ```
///
pub struct NextStartIsLessThanCurrentError;

///
/// ```compile_fail
/// let arr = [(); 100];
/// multindex::multindex!(arr; 10..=usize::MAX );
/// ```
///
/// ```rust
/// if false {
///     let arr = [(); 100];
///     multindex::multindex!(arr; 10..=usize::MAX-1 );
/// }
/// ```
///
pub struct InclusiveUptoUsizeMaxError;

///
/// ```compile_fail
/// let mut arr = [(); 100];
/// multindex::multindex_mut!(arr; 1, 1 );
/// ```
///
/// ```rust
/// let mut arr = [(); 100];
/// multindex::multindex_mut!(arr; 1, 2 );
/// ```
///
pub struct OverlappingIndexArgsError;
