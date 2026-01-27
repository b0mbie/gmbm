use super::RawType;

/// Pre-defined type in Garry's Mod Lua.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub enum StdType {
	None = -1,
	Nil,
	Bool,
	LightUserData,
	Number,
	String,
	Table,
	Function,
	UserData,
	Thread,

	// GMod types.
	Entity,
	Vector,
	Angle,
	PhysObj,
	Save,
	Restore,
	DamageInfo,
	EffectData,
	MoveData,
	RecipientFilter,
	UserCmd,
	ScriptedVehicle,
	Material,
	Panel,
	Particle,
	ParticleEmitter,
	Texture,
	UserMsg,
	ConVar,
	IMesh,
	Matrix,
	Sound,
	PixelVisHandle,
	DLight,
	Video,
	File,
	Locomotion,
	Path,
	NavArea,
	SoundHandle,
	NavLadder,
	ParticleSystem,
	ProjectedTexture,
	PhysCollide,
	SurfaceInfo,
}

impl StdType {
	pub const fn to_raw(self) -> RawType {
		self as _
	}
}

/// Type returned by the Garry's Mod Lua API.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Type(pub RawType);

impl Type {
	/// Returns a [`Type`] that represents the specified [`StdType`].
	pub const fn from_std(ty: StdType) -> Self {
		Self(ty.to_raw())
	}

	/// Returns `true` if this type is the specified [`StdType`].
	pub const fn is_std(self, ty: StdType) -> bool {
		self.0 == ty.to_raw()
	}
}

impl From<StdType> for Type {
	fn from(value: StdType) -> Self {
		Self::from_std(value)
	}
}

impl PartialEq<StdType> for Type {
	fn eq(&self, other: &StdType) -> bool {
		self.is_std(*other)
	}
}
impl PartialEq<Type> for StdType {
	fn eq(&self, other: &Type) -> bool {
		other.is_std(*self)
	}
}
