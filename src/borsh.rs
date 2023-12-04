impl<R> borsh::BorshSerialize for crate::index::BitIdx<R>
where
	R: crate::mem::BitRegister,
{
	#[inline]
	fn serialize<W: borsh::io::Write>(
		&self,
		writer: &mut W,
	) -> borsh::io::Result<()> {
		//  Emit the bit-width of the `R` type.
		<u8 as borsh::BorshSerialize>::serialize(
			&(crate::mem::bits_of::<R>() as u8),
			writer,
		)?;
		//  Emit the actual head-bit index.
		<u8 as borsh::BorshSerialize>::serialize(&self.into_inner(), writer)
	}
}

impl<T, O> borsh::BorshSerialize for crate::domain::Domain<'_, wyz::Const, T, O>
where
	T: crate::store::BitStore,
	O: crate::order::BitOrder,
	<T as crate::store::BitStore>::Mem: borsh::BorshSerialize,
{
	#[inline]
	fn serialize<W: borsh::io::Write>(
		&self,
		writer: &mut W,
	) -> borsh::io::Result<()> {
		//  Domain<T> is functionally equivalent to `[T::Mem]`.
		<usize as borsh::BorshSerialize>::serialize(&self.len(), writer)?;
		for elem in *self {
			<<T as crate::store::BitStore>::Mem as borsh::BorshSerialize>::serialize(&elem, writer)?;
		}
		Ok(())
	}
}

impl<T, O> borsh::BorshSerialize for crate::slice::BitSlice<T, O>
where
	T: crate::store::BitStore,
	O: crate::order::BitOrder,
	<T as crate::store::BitStore>::Mem: borsh::BorshSerialize,
{
	#[inline]
	fn serialize<W: borsh::io::Write>(
		&self,
		writer: &mut W,
	) -> borsh::io::Result<()> {
		// order
		<str as borsh::BorshSerialize>::serialize(
			&core::any::type_name::<O>(),
			writer,
		)?;
		// head
		<crate::index::BitIdx<<T as crate::store::BitStore>::Mem> as borsh::BorshSerialize>::serialize(
			&self.as_bitspan().head(),
			writer,
		)?;
		// bits
		<u64 as borsh::BorshSerialize>::serialize(&(self.len() as u64), writer)?;
		// data
		<crate::domain::Domain<'_, wyz::Const, T, O> as borsh::BorshSerialize>::serialize(
			&self.domain(),
			writer,
		)
	}
}

#[cfg(feature = "alloc")]
#[cfg(feature = "alloc")]
impl<T, O> borsh::BorshSerialize for crate::prelude::BitBox<T, O>
where
	T: crate::store::BitStore,
	O: crate::order::BitOrder,
	crate::slice::BitSlice<T, O>: borsh::BorshSerialize,
{
	#[inline]
	fn serialize<W: borsh::io::Write>(
		&self,
		writer: &mut W,
	) -> borsh::io::Result<()> {
		<crate::slice::BitSlice<T, O> as borsh::BorshSerialize>::serialize(
			&self.as_bitslice(),
			writer,
		)
	}
}

#[cfg(feature = "alloc")]
impl<T, O> borsh::BorshSerialize for crate::prelude::BitVec<T, O>
where
	T: crate::store::BitStore,
	O: crate::order::BitOrder,
	crate::slice::BitSlice<T, O>: borsh::BorshSerialize,
{
	#[inline]
	fn serialize<W: borsh::io::Write>(
		&self,
		writer: &mut W,
	) -> borsh::io::Result<()> {
		<crate::slice::BitSlice<T, O> as borsh::BorshSerialize>::serialize(
			&self.as_bitslice(),
			writer,
		)
	}
}

impl<T, O> borsh::BorshSerialize for crate::array::BitArray<T, O>
where
	T: crate::store::BitStore,
	O: crate::order::BitOrder,
	<T as crate::store::BitStore>::Mem: borsh::BorshSerialize,
{
	#[inline]
	fn serialize<W: borsh::io::Write>(
		&self,
		writer: &mut W,
	) -> borsh::io::Result<()> {
		<str as borsh::BorshSerialize>::serialize(
			&core::any::type_name::<O>(),
			writer,
		)?;
		// head
		<crate::index::BitIdx<<T as crate::store::BitStore>::Mem> as borsh::BorshSerialize>::serialize(
			&crate::index::BitIdx::<T::Mem>::MIN,
			writer,
		)?;
		// bits
		<u64 as borsh::BorshSerialize>::serialize(&(self.len() as u64), writer)?;
		// data
		todo!()
		// <[T; 1] as borsh::BorshSerialize>::serialize(
		// 	core::array::from_ref(&self.data),
		// 	writer,
		// )
	}
}

impl<T, O, const N: usize> borsh::BorshSerialize
	for crate::array::BitArray<[T; N], O>
where
	T: crate::store::BitStore,
	O: crate::order::BitOrder,
	<T as crate::store::BitStore>::Mem: borsh::BorshSerialize,
{
	#[inline]
	fn serialize<W: borsh::io::Write>(
		&self,
		writer: &mut W,
	) -> borsh::io::Result<()> {
		<str as borsh::BorshSerialize>::serialize(
			&core::any::type_name::<O>(),
			writer,
		)?;
		// head
		<crate::index::BitIdx<<T as crate::store::BitStore>::Mem> as borsh::BorshSerialize>::serialize(
			&crate::index::BitIdx::<T::Mem>::MIN,
			writer,
		)?;
		// bits
		<u64 as borsh::BorshSerialize>::serialize(&(self.len() as u64), writer)?;
		// data
		todo!()
		// <[T; N] as borsh::BorshSerialize>::serialize(&self.data, writer)
	}
}
