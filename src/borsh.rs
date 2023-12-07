use crate::view::BitViewSized;

impl<R> borsh::BorshSerialize for crate::index::BitIdx<R>
where
	R: crate::mem::BitRegister,
{
	#[inline]
	fn serialize<W: borsh::io::Write>(
		&self,
		writer: &mut W,
	) -> borsh::io::Result<()> {
		//  Emit the actual head-bit index.
		<u8 as borsh::BorshSerialize>::serialize(&self.into_inner(), writer)
	}
}

impl<T> borsh::BorshDeserialize for crate::index::BitIdx<T>
where
	T: crate::mem::BitRegister,
{
	#[inline]
	fn deserialize_reader<R: borsh::io::Read>(
		reader: &mut R,
	) -> borsh::io::Result<Self> {
		let index = <u8 as borsh::BorshDeserialize>::deserialize_reader(reader)?;
		Self::new(index).map_err(|_| {
			borsh::io::Error::new(
				borsh::io::ErrorKind::InvalidInput,
				"invalid index",
			)
		})
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
		// head
		<crate::index::BitIdx<<T as crate::store::BitStore>::Mem> as borsh::BorshSerialize>::serialize(
			&self.as_bitspan().head(),
			writer,
		)?;
		// bits
		<usize as borsh::BorshSerialize>::serialize(&self.len(), writer)?;
		// data
		<crate::domain::Domain<'_, wyz::Const, T, O> as borsh::BorshSerialize>::serialize(
			&self.domain(),
			writer,
		)
	}
}

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

#[cfg(feature = "alloc")]
impl<T, O> borsh::BorshDeserialize for crate::prelude::BitVec<T, O>
where
	T: crate::store::BitStore,
	O: crate::order::BitOrder,
	T: borsh::BorshDeserialize,
	crate::slice::BitSlice<T, O>: borsh::BorshDeserialize,
{
	#[inline]
	fn deserialize_reader<R: borsh::io::Read>(
		reader: &mut R,
	) -> borsh::io::Result<Self> {
		let head = <crate::index::BitIdx<T::Mem> as borsh::BorshDeserialize>::deserialize_reader(reader)?;
		let bits =
			<usize as borsh::BorshDeserialize>::deserialize_reader(reader)?;
		let vec = {
			let len =
				<usize as borsh::BorshDeserialize>::deserialize_reader(reader)?;
			let mut buffer = Vec::default();
			for _ix in 0..len {
				buffer.push(<T as borsh::BorshDeserialize>::deserialize_reader(
					reader,
				)?);
			}
			buffer
		};

		unsafe {
			let addr = crate::ptr::AddressExt::into_address(vec.as_ptr());
			let mut bv = crate::prelude::BitVec::try_from_vec(vec)
				.map_err(|_| {
					crate::ptr::BitSpan::<wyz::Const, T, O>::new(
						addr, head, bits,
					)
					.unwrap_err()
				})
				.map_err(|_e| {
					borsh::io::Error::new(
						borsh::io::ErrorKind::InvalidData,
						"invalid data",
					)
				})?;
			bv.set_head(head);
			bv.set_len(bits);
			Ok(bv)
		}
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
		// data
		<<T as crate::store::BitStore>::Mem as borsh::BorshSerialize>::serialize(
			&crate::store::BitStore::load_value(&self.data),
			writer,
		)
	}
}

impl<T, O> borsh::BorshDeserialize for crate::array::BitArray<T, O>
where
	T: crate::store::BitStore,
	O: crate::order::BitOrder,
	<T as crate::store::BitStore>::Mem: borsh::BorshDeserialize,
{
	#[inline]
	fn deserialize_reader<R: borsh::io::Read>(
		reader: &mut R,
	) -> borsh::io::Result<Self> {
		let value = <<T as crate::store::BitStore>::Mem as borsh::BorshDeserialize>::deserialize_reader(reader)?;
		Ok(Self::new(crate::store::BitStore::new(value)))
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
		// data
		for value in self
			.data
			.as_raw_slice()
			.iter()
			.map(crate::store::BitStore::load_value)
		{
			<<T as crate::store::BitStore>::Mem as borsh::BorshSerialize>::serialize(&value, writer)?;
		}
		Ok(())
	}
}

impl<T, O, const N: usize> borsh::BorshDeserialize
	for crate::array::BitArray<[T; N], O>
where
	T: crate::store::BitStore,
	O: crate::order::BitOrder,
	<T as crate::store::BitStore>::Mem: borsh::BorshDeserialize,
{
	#[inline]
	fn deserialize_reader<R: borsh::io::Read>(
		reader: &mut R,
	) -> borsh::io::Result<Self> {
		let mut uninit = [core::mem::MaybeUninit::<
			<T as crate::store::BitStore>::Mem,
		>::uninit(); N];
		for (_idx, slot) in uninit.iter_mut().enumerate() {
			slot.write(
				<<T as crate::store::BitStore>::Mem as borsh::BorshDeserialize>::deserialize_reader(reader)?
			);
		}
		let data = uninit
			.map(|elem| unsafe { core::mem::MaybeUninit::assume_init(elem) })
			.map(crate::store::BitStore::new);
		Ok(Self::new(data))
	}
}
