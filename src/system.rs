use byteorder::ByteOrder;
use sys::ETrackingUniverseOrigin;

use crate::errors::ETrackedPropertyError;
use crate::{sys, Context, TrackedDeviceIndex};

use std::ffi::CString;
use std::marker::PhantomData;
use std::pin::Pin;
use std::ptr::null_mut;

pub struct SystemManager<'c> {
    ctx: PhantomData<&'c Context>,
    inner: Pin<&'c mut sys::IVRSystem>,
}

mod private {
    pub trait Sealed {}
}

type PropResult<T> = Result<T, ETrackedPropertyError>;

/// Trait implemented by types that represent storage types of properties.
pub trait TrackedDeviceProperty<'ret>: private::Sealed + Sized {
    fn get<'manager: 'ret>(
        index: TrackedDeviceIndex,
        system: &'manager mut SystemManager,
        prop: sys::ETrackedDeviceProperty,
    ) -> PropResult<Self>;
}

macro_rules! impl_property_type {
    ($ty:ty, $method:ident) => {
        impl private::Sealed for $ty {}
        impl<'ret> TrackedDeviceProperty<'ret> for $ty {
            fn get<'manager: 'ret>(
                index: TrackedDeviceIndex,
                system: &'manager mut SystemManager,
                prop: sys::ETrackedDeviceProperty,
            ) -> PropResult<Self> {
                let mut err = sys::ETrackedPropertyError::TrackedProp_Success;
                let res = unsafe { system.inner.as_mut().$method(index.0, prop, &mut err) };
                ETrackedPropertyError::new(err)?;
                Ok(res)
            }
        }
    };
}

impl_property_type!(bool, GetBoolTrackedDeviceProperty);
impl_property_type!(f32, GetFloatTrackedDeviceProperty);
impl_property_type!(i32, GetInt32TrackedDeviceProperty);
impl_property_type!(u64, GetUint64TrackedDeviceProperty);

impl private::Sealed for String {}
impl<'ret> TrackedDeviceProperty<'ret> for String {
    fn get<'manager: 'ret>(
        index: TrackedDeviceIndex,
        system: &'manager mut SystemManager,
        prop: sys::ETrackedDeviceProperty,
    ) -> PropResult<Self> {
        let mut err = sys::ETrackedPropertyError::TrackedProp_Success;
        let res = unsafe {
            get_string(|ptr, n| {
                system.inner.as_mut().GetStringTrackedDeviceProperty(
                    index.0,
                    prop.clone(),
                    ptr,
                    n,
                    &mut err,
                )
            })
        };
        ETrackedPropertyError::new(err)?;
        match res {
            Some(s) => Ok(s.into_string().unwrap()),
            None => Ok("".to_string()),
        }
    }
}

// TODO: Decide if we want to support matrix types from other libraries, like nalgebra
impl private::Sealed for crate::pose::Matrix3x4 {}
impl<'ret> TrackedDeviceProperty<'ret> for crate::pose::Matrix3x4 {
    fn get<'manager: 'ret>(
        index: TrackedDeviceIndex,
        system: &'manager mut SystemManager,
        prop: sys::ETrackedDeviceProperty,
    ) -> PropResult<Self> {
        let mut err = sys::ETrackedPropertyError::TrackedProp_Success;
        let res = unsafe {
            system
                .inner
                .as_mut()
                .GetMatrix34TrackedDeviceProperty(index.0, prop, &mut err)
        };
        ETrackedPropertyError::new(err)?;
        Ok(res.into())
    }
}

impl private::Sealed for CString {}
impl<'ret> TrackedDeviceProperty<'ret> for CString {
    fn get<'manager: 'ret>(
        index: TrackedDeviceIndex,
        system: &'manager mut SystemManager,
        prop: sys::ETrackedDeviceProperty,
    ) -> PropResult<Self> {
        let mut err = sys::ETrackedPropertyError::TrackedProp_Success;
        let len = unsafe {
            system.inner.as_mut().GetStringTrackedDeviceProperty(
                index.0,
                prop.clone(),
                null_mut(),
                0,
                &mut err,
            )
        };
        ETrackedPropertyError::new(err.clone())?;
        let mut data = vec![0; len as usize];
        let _len = unsafe {
            system.inner.as_mut().GetStringTrackedDeviceProperty(
                index.0,
                prop.clone(),
                data.as_mut_ptr() as *mut i8,
                len,
                &mut err,
            )
        };
        ETrackedPropertyError::new(err)?;

        Ok(CString::from_vec_with_nul(data).expect("missing nul byte from openvr!"))
    }
}

// TODO: arrays. I don't feel like dealing with them right now.

impl<'c> SystemManager<'c> {
    pub(super) fn new(_ctx: &'c Context) -> Self {
        let inner = unsafe { Pin::new_unchecked(sys::VRSystem().as_mut::<'c>().unwrap()) };
        Self {
            ctx: Default::default(),
            inner,
        }
    }

    pub fn get_tracked_device_property<'ret, 'manager: 'ret, T: TrackedDeviceProperty<'ret>>(
        &'manager mut self,
        index: TrackedDeviceIndex,
        prop: sys::ETrackedDeviceProperty,
    ) -> PropResult<T> {
        T::get(index, self, prop)
    }

    pub fn get_controller_role_for_tracked_device_index<'ret, 'manager: 'ret>(
        &'manager mut self,
        index: TrackedDeviceIndex,
    ) -> sys::ETrackedControllerRole {
        let role = unsafe {
            self.inner
                .as_mut()
                .GetControllerRoleForTrackedDeviceIndex(index.0)
        };
        // A runtime newer than these headers can answer with a role they do not name. Hand
        // callers the enum's own invalid member rather than an id it has no variant for.
        sys::ETrackedControllerRole::from_raw(role as i32)
            .unwrap_or(sys::ETrackedControllerRole::TrackedControllerRole_Invalid)
    }

    pub fn get_tracked_device_class<'ret, 'manager: 'ret>(
        &'manager mut self,
        index: TrackedDeviceIndex,
    ) -> sys::ETrackedDeviceClass {
        let class = unsafe { self.inner.as_mut().GetTrackedDeviceClass(index.0) };
        sys::ETrackedDeviceClass::from_raw(class as i32)
            .unwrap_or(sys::ETrackedDeviceClass::TrackedDeviceClass_Invalid)
    }

    pub fn get_tracked_device_activity_level<'ret, 'manager: 'ret>(
        &'manager mut self,
        index: TrackedDeviceIndex,
    ) -> sys::EDeviceActivityLevel {
        let level = unsafe { self.inner.as_mut().GetTrackedDeviceActivityLevel(index.0) };
        sys::EDeviceActivityLevel::from_raw(level as i32)
            .unwrap_or(sys::EDeviceActivityLevel::k_EDeviceActivityLevel_Unknown)
    }

    pub fn get_device_to_absolute_tracking_pose<'ret, 'manager: 'ret>(
        &'manager mut self,
        origin: ETrackingUniverseOrigin,
        predicted_seconds_to_photons_from_now: f32,
    ) -> [sys::TrackedDevicePose_t; sys::k_unMaxTrackedDeviceCount as usize] {
        let mut poses: [sys::TrackedDevicePose_t; sys::k_unMaxTrackedDeviceCount as usize] =
            unsafe { std::mem::zeroed() };
        unsafe {
            self.inner.as_mut().GetDeviceToAbsoluteTrackingPose(
                origin,
                predicted_seconds_to_photons_from_now,
                &mut poses[0],
                sys::k_unMaxTrackedDeviceCount,
            )
        };
        poses
    }

    pub fn poll_next_event<'ret, 'manager: 'ret>(&'manager mut self) -> Option<VREvent> {
        let mut event = std::mem::MaybeUninit::uninit();
        let res = unsafe {
            self.inner.as_mut().PollNextEvent(
                event.as_mut_ptr(),
                std::mem::size_of::<sys::VREvent_t>() as u32,
            )
        };
        if !res {
            return None;
        }
        let event = unsafe { event.assume_init() };
        let event = VREvent::parse(event);
        Some(event)
    }
}

unsafe impl Send for SystemManager<'_> {}
unsafe impl Sync for SystemManager<'_> {}
/// The type of an event, as sent by the runtime.
#[derive(Clone, PartialEq, Eq)]
pub enum EventType {
    /// An event type that the openvr.h we build against names.
    Known(sys::EVREventType),
    /// An id that it does not name, kept as the raw number. A runtime newer than these
    /// headers sends ids that [`sys::EVREventType`] has no variant to hold.
    Unknown(u32),
}

impl EventType {
    pub fn from_raw(raw: u32) -> Self {
        match sys::EVREventType::from_raw(raw as i32) {
            Some(event_type) => Self::Known(event_type),
            None => Self::Unknown(raw),
        }
    }

    /// The id the runtime sent.
    pub fn raw(&self) -> u32 {
        match self {
            Self::Known(event_type) => event_type.clone() as u32,
            Self::Unknown(raw) => *raw,
        }
    }
}

/// sys::EVREventType carries no Debug, so print the id either way.
impl std::fmt::Debug for EventType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Known(_) => write!(f, "EventType::Known({})", self.raw()),
            Self::Unknown(raw) => write!(f, "EventType::Unknown({raw})"),
        }
    }
}

pub struct VREvent {
    pub event_type: EventType,
    pub tracked_device_index: TrackedDeviceIndex,
    pub event_age_seconds: f32,
    pub data: [u8; 52],
}

impl VREvent {
    /// Whether this event has the given event type.
    pub fn is(&self, event_type: sys::EVREventType) -> bool {
        self.event_type == EventType::Known(event_type)
    }

    fn parse(event: sys::VREvent_t) -> VREvent {
        // autocxx does not expose the fields of VREvent_t, so read the struct as bytes.
        let bytes = unsafe {
            std::slice::from_raw_parts(
                &event as *const sys::VREvent_t as *const u8,
                std::mem::size_of::<sys::VREvent_t>(),
            )
        };
        Self::from_bytes(bytes)
    }

    /// VREvent_t starts with eventType, trackedDeviceIndex and eventAgeSeconds, which take
    /// up 12 bytes. The data union takes up the remainder, however long the runtime makes it.
    fn from_bytes(bytes: &[u8]) -> VREvent {
        let payload = bytes.get(12..).unwrap_or_default();
        let mut data = [0u8; 52];
        let len = payload.len().min(data.len());
        data[..len].copy_from_slice(&payload[..len]);
        VREvent {
            event_type: EventType::from_raw(byteorder::LittleEndian::read_u32(&bytes[0..4])),
            tracked_device_index: TrackedDeviceIndex(byteorder::LittleEndian::read_u32(
                &bytes[4..8],
            )),
            event_age_seconds: byteorder::LittleEndian::read_f32(&bytes[8..12]),
            data,
        }
    }
}

/// Helper to call OpenVR functions that return strings
unsafe fn get_string<F: FnMut(*mut std::os::raw::c_char, u32) -> u32>(
    mut f: F,
) -> Option<std::ffi::CString> {
    let n = f(std::ptr::null_mut(), 0);
    if n == 0 {
        return None;
    }
    let mut storage = Vec::new();
    storage.reserve_exact(n as usize);
    storage.resize_with(n as usize, || std::mem::MaybeUninit::zeroed().assume_init());
    let n_ = f(storage.as_mut_ptr() as *mut _, n);
    assert!(n == n_);
    storage.truncate((n - 1) as usize); // Strip trailing null
    Some(std::ffi::CString::from_vec_unchecked(storage))
}

#[cfg(test)]
mod test {
    use super::*;
    fn _compile_test(mut system: SystemManager) {
        // let _bootloader_version =
        //     system.get_tracked_device_property(TrackedDeviceIndex::HMD, props::DisplayBootloaderVersion);
        let _display_version: u64 = system
            .get_tracked_device_property(
                TrackedDeviceIndex::HMD,
                sys::ETrackedDeviceProperty::Prop_DisplayHardwareVersion_Uint64,
            )
            .unwrap();
        let _gc_image_cstring: String = system
            .get_tracked_device_property(
                TrackedDeviceIndex::HMD,
                sys::ETrackedDeviceProperty::Prop_DisplayGCImage_String,
            )
            .unwrap();
    }

    fn event_bytes(event_type: u32, len: usize) -> Vec<u8> {
        let mut bytes = vec![0u8; len];
        bytes[0..4].copy_from_slice(&event_type.to_le_bytes());
        bytes[4..8].copy_from_slice(&7u32.to_le_bytes());
        bytes[8..12].copy_from_slice(&0.5f32.to_le_bytes());
        bytes
    }

    #[test]
    fn parses_an_event_id_that_openvr_h_does_not_name() {
        // 564 is one such id: openvr.h stops at VREvent_Reserved_0563.
        assert!(sys::EVREventType::from_raw(564).is_none());
        let event = VREvent::from_bytes(&event_bytes(564, 64));
        assert_eq!(event.event_type, EventType::Unknown(564));
        assert_eq!(event.tracked_device_index.0, 7);
        assert_eq!(event.event_age_seconds, 0.5);
        assert!(!event.is(sys::EVREventType::VREvent_Quit));
    }

    #[test]
    fn parses_an_event_id_that_openvr_h_names() {
        let quit = sys::EVREventType::VREvent_Quit;
        let event = VREvent::from_bytes(&event_bytes(quit.clone() as u32, 64));
        assert_eq!(event.event_type, EventType::Known(quit));
        assert!(event.is(sys::EVREventType::VREvent_Quit));
        assert!(!event.is(sys::EVREventType::VREvent_TrackedDeviceActivated));
    }

    #[test]
    fn the_generated_lookup_covers_the_event_enum() {
        // Spread over the ranges openvr.h uses, including the last reserved id before 564.
        for event_type in [
            sys::EVREventType::VREvent_None,
            sys::EVREventType::VREvent_TrackedDeviceActivated,
            sys::EVREventType::VREvent_PropertyChanged,
            sys::EVREventType::VREvent_Quit,
            sys::EVREventType::VREvent_Reserved_0563,
            sys::EVREventType::VREvent_VendorSpecific_Reserved_End,
        ] {
            let raw = event_type.clone() as i32;
            assert!(
                sys::EVREventType::from_raw(raw) == Some(event_type),
                "openvr.h names event id {raw}, so from_raw must too"
            );
        }
    }

    #[test]
    fn the_generated_lookups_cover_the_device_enums() {
        assert!(
            sys::ETrackedDeviceClass::from_raw(
                sys::ETrackedDeviceClass::TrackedDeviceClass_DisplayRedirect as i32
            ) == Some(sys::ETrackedDeviceClass::TrackedDeviceClass_DisplayRedirect)
        );
        assert!(
            sys::ETrackedControllerRole::from_raw(
                sys::ETrackedControllerRole::TrackedControllerRole_Stylus as i32
            ) == Some(sys::ETrackedControllerRole::TrackedControllerRole_Stylus)
        );
        // This one states a negative id, so the lookup has to carry it.
        assert!(
            sys::EDeviceActivityLevel::from_raw(-1)
                == Some(sys::EDeviceActivityLevel::k_EDeviceActivityLevel_Unknown)
        );
        assert!(
            sys::EDeviceActivityLevel::from_raw(
                sys::EDeviceActivityLevel::k_EDeviceActivityLevel_Idle_Timeout as i32
            ) == Some(sys::EDeviceActivityLevel::k_EDeviceActivityLevel_Idle_Timeout)
        );

        // Ids beyond what openvr.h names, which is what a newer runtime can answer with.
        assert!(sys::ETrackedDeviceClass::from_raw(99).is_none());
        assert!(sys::ETrackedControllerRole::from_raw(99).is_none());
        assert!(sys::EDeviceActivityLevel::from_raw(99).is_none());
        assert!(sys::EDeviceActivityLevel::from_raw(-2).is_none());
    }

    #[test]
    fn copies_the_data_union_whatever_its_size() {
        let mut bytes = event_bytes(sys::EVREventType::VREvent_PropertyChanged as u32, 64);
        bytes[12] = 0xAB;
        bytes[63] = 0xCD;
        let event = VREvent::from_bytes(&bytes);
        assert_eq!(event.data[0], 0xAB);
        assert_eq!(event.data[51], 0xCD);

        // A shorter struct zero pads, a longer one truncates. Neither panics.
        assert_eq!(VREvent::from_bytes(&event_bytes(0, 12)).data, [0u8; 52]);
        let long = VREvent::from_bytes(&event_bytes(0, 96));
        assert_eq!(long.data, [0u8; 52]);
    }
}

impl std::fmt::Debug for ETrackedPropertyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ETrackedPropertyError").finish()
    }
}
