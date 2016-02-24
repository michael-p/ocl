// use std::convert::Into;
use error::{Result as OclResult};
use core::{self, DeviceId as DeviceIdCore, PlatformId as PlatformIdCore, DeviceType};
use standard::Device;

/// Specifies [what boils down to] a list of devices.
///
/// The variants: `All`, `Index`, and `Indices` are context-specific, not robust, 
/// and may lead to a stack unwind if the context changes. They are useful for
/// convenience only [NOTE: This may change and they may soon round-robin by default, 
/// making them robust and sexy... well robust anyway][UPDATE: this will probably remain as is].
///
/// The `TypeFlags` variant is useful for specifying a list of devices using a bitfield
/// (`DeviceType`) and is the most robust / portable.
///
///
///
/// [FIXME: Add some links to the SDK]
/// [FIXME: Figure out what we're doing as far as round-robin/moduloing by default]
/// - UPDATE: Leave this to the builder or whatever else to determine and leave this
///   enum an exact index which panics.
///
#[derive(Debug, Clone)]
pub enum DeviceSpecifier {
    All,
    Index(usize),
    Indices(Vec<usize>),
    Single(Device),
    List(Vec<Device>),
    TypeFlags(DeviceType),
}

impl DeviceSpecifier {
    /// Returns the list of devices matching the parameters specified by this `DeviceSpecifier`
    ///
    /// ### Panics
    ///
    /// Any device indices within the `Index` and `Indices` variants must be within the range of the number of devices for the platform specified by `Platform`. If no `platform` has been specified, this behaviour is undefined and could end up using any platform at all.
    ///
    pub fn to_device_list(&self, platform: Option<PlatformIdCore>) -> OclResult<Vec<DeviceIdCore>> {
        let platform = match platform {
            Some(plat) => Some(plat),
            None => None,
        };

        let device_list_all = try!(core::get_device_ids(platform.clone(), Some(core::DEVICE_TYPE_ALL)));

        Ok(match self {
            &DeviceSpecifier::All => {
                device_list_all
            },
            &DeviceSpecifier::Single(ref device) => {
                vec![device.as_core().clone()]
            },
            &DeviceSpecifier::List(ref devices) => {
                devices.iter().map(|d| d.as_core().clone()).collect() 
            },
            &DeviceSpecifier::Index(idx) => {
                assert!(idx < device_list_all.len(), "ocl::Context::new: DeviceSpecifier::Index: \
                    Device index out of range.");
                vec![device_list_all[idx].clone()]
            },
            &DeviceSpecifier::Indices(ref idx_list) => {
                idx_list.iter().map(|&idx| {
                        assert!(idx < device_list_all.len(), "ocl::Context::new: \
                            DeviceSpecifier::Indices: Device index out of range.");
                        device_list_all[idx].clone()
                    } ).collect()
            },
            &DeviceSpecifier::TypeFlags(flags) => {
                try!(core::get_device_ids(platform.clone(), Some(flags)))
            },
        })
    }
}