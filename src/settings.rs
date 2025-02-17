use sys::EVRSettingsError;

use crate::{sys, Context};
use std::{ffi::CStr, marker::PhantomData, mem::MaybeUninit, pin::Pin};

pub struct SettingsManager<'c> {
	ctx: PhantomData<&'c Context>,
	inner: Pin<&'c mut sys::IVRSettings>,
}

impl<'c> SettingsManager<'c> {
	pub(super) fn new(_ctx: &'c Context) -> Self {
		let inner =
			unsafe { Pin::new_unchecked(sys::VRSettings().as_mut::<'c>().unwrap()) };
		Self {
			ctx: Default::default(),
			inner,
		}
	}

	pub fn remove_section<'ret, 'manager: 'ret>(
		&'manager mut self,
		pch_section: &CStr,
	) -> Result<(), EVRSettingsError> {
		unsafe {
			let mut error: MaybeUninit<EVRSettingsError> = MaybeUninit::uninit();
			self.inner.as_mut().RemoveSection(
				pch_section.as_ptr() as *mut _,
				error.as_mut_ptr() as *mut EVRSettingsError,
			);
			let error = error.assume_init();
			if error != EVRSettingsError::VRSettingsError_None {
				return Err(error);
			}
			return Ok(());
		}
	}

	pub fn remove_key_in_section<'ret, 'manager: 'ret>(
		&'manager mut self,
		pch_section: &CStr,
		pch_settings_key: &CStr,
	) -> Result<(), EVRSettingsError> {
		unsafe {
			let mut error: MaybeUninit<EVRSettingsError> = MaybeUninit::uninit();
			self.inner.as_mut().RemoveKeyInSection(
				pch_section.as_ptr() as *mut _,
				pch_settings_key.as_ptr() as *mut _,
				error.as_mut_ptr() as *mut EVRSettingsError,
			);
			let error = error.assume_init();
			if error != EVRSettingsError::VRSettingsError_None {
				return Err(error);
			}
			return Ok(());
		}
	}

	pub fn get_int32<'ret, 'manager: 'ret>(
		&'manager mut self,
		pch_section: &CStr,
		pch_settings_key: &CStr,
	) -> Result<i32, EVRSettingsError> {
		unsafe {
			let mut error: MaybeUninit<EVRSettingsError> = MaybeUninit::uninit();
			let result = self.inner.as_mut().GetInt32(
				pch_section.as_ptr() as *mut _,
				pch_settings_key.as_ptr() as *mut _,
				error.as_mut_ptr() as *mut EVRSettingsError,
			);
			let error = error.assume_init();
			if error != EVRSettingsError::VRSettingsError_None {
				return Err(error);
			}
			return Ok(result);
		};
	}

	pub fn set_int32<'ret, 'manager: 'ret>(
		&'manager mut self,
		pch_section: &CStr,
		pch_settings_key: &CStr,
		value: i32,
	) -> Result<(), EVRSettingsError> {
		unsafe {
			let mut error: MaybeUninit<EVRSettingsError> = MaybeUninit::uninit();
			self.inner.as_mut().SetInt32(
				pch_section.as_ptr() as *mut _,
				pch_settings_key.as_ptr() as *mut _,
				value,
				error.as_mut_ptr() as *mut EVRSettingsError,
			);
			let error = error.assume_init();
			if error != EVRSettingsError::VRSettingsError_None {
				return Err(error);
			}
			return Ok(());
		};
	}

	pub fn get_float<'ret, 'manager: 'ret>(
		&'manager mut self,
		pch_section: &CStr,
		pch_settings_key: &CStr,
	) -> Result<f32, EVRSettingsError> {
		unsafe {
			let mut error: MaybeUninit<EVRSettingsError> = MaybeUninit::uninit();
			let result = self.inner.as_mut().GetFloat(
				pch_section.as_ptr() as *mut _,
				pch_settings_key.as_ptr() as *mut _,
				error.as_mut_ptr() as *mut EVRSettingsError,
			);
			let error = error.assume_init();
			if error != EVRSettingsError::VRSettingsError_None {
				return Err(error);
			}
			return Ok(result);
		};
	}

	pub fn set_float<'ret, 'manager: 'ret>(
		&'manager mut self,
		pch_section: &CStr,
		pch_settings_key: &CStr,
		value: f32,
	) -> Result<(), EVRSettingsError> {
		unsafe {
			let mut error: MaybeUninit<EVRSettingsError> = MaybeUninit::uninit();
			self.inner.as_mut().SetFloat(
				pch_section.as_ptr() as *mut _,
				pch_settings_key.as_ptr() as *mut _,
				value,
				error.as_mut_ptr() as *mut EVRSettingsError,
			);
			let error = error.assume_init();
			if error != EVRSettingsError::VRSettingsError_None {
				return Err(error);
			}
			return Ok(());
		};
	}

	pub fn get_bool<'ret, 'manager: 'ret>(
		&'manager mut self,
		pch_section: &CStr,
		pch_settings_key: &CStr,
	) -> Result<bool, EVRSettingsError> {
		unsafe {
			let mut error: MaybeUninit<EVRSettingsError> = MaybeUninit::uninit();
			let result = self.inner.as_mut().GetBool(
				pch_section.as_ptr() as *mut _,
				pch_settings_key.as_ptr() as *mut _,
				error.as_mut_ptr() as *mut EVRSettingsError,
			);
			let error = error.assume_init();
			if error != EVRSettingsError::VRSettingsError_None {
				return Err(error);
			}
			return Ok(result);
		};
	}

	pub fn set_bool<'ret, 'manager: 'ret>(
		&'manager mut self,
		pch_section: &CStr,
		pch_settings_key: &CStr,
		value: bool,
	) -> Result<(), EVRSettingsError> {
		unsafe {
			let mut error: MaybeUninit<EVRSettingsError> = MaybeUninit::uninit();
			self.inner.as_mut().SetBool(
				pch_section.as_ptr() as *mut _,
				pch_settings_key.as_ptr() as *mut _,
				value,
				error.as_mut_ptr() as *mut EVRSettingsError,
			);
			let error = error.assume_init();
			if error != EVRSettingsError::VRSettingsError_None {
				return Err(error);
			}
			return Ok(());
		};
	}
}
