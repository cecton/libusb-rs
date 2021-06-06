use libusb::*;
use std::fmt;

pub struct DfuFuncDescriptor {
    descriptor: libusb_dfu_func_descriptor,
}

impl DfuFuncDescriptor {
    fn length(&self) -> u8 {
        self.descriptor.bLength
    }

    fn descriptor_type(&self) -> u8 {
        self.descriptor.bDescriptorType
    }

    fn attributes(&self) -> u8 {
        self.descriptor.bmAttributes
    }

    fn detach_timeout(&self) -> u16 {
        self.descriptor.wDetachTimeOut
    }

    fn transfer_size(&self) -> u16 {
        self.descriptor.wTransferSize
    }

    fn dfu_version(&self) -> u16 {
        self.descriptor.bcdDFUVersion
    }

    fn can_download(&self) -> bool {
        self.descriptor.bmAttributes & (1 << 0) == 1
    }

    fn can_upload(&self) -> bool {
        self.descriptor.bmAttributes & (1 << 1) == 1
    }

    fn manifest_tolerant(&self) -> bool {
        self.descriptor.bmAttributes & (1 << 2) == 1
    }

    fn will_detach(&self) -> bool {
        self.descriptor.bmAttributes & (1 << 3) == 1
    }
}

impl fmt::Debug for DfuFuncDescriptor {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let mut debug = fmt.debug_struct("DfuFuncDescriptor");

        let descriptor = &self.descriptor;

        debug.field("bLength", &descriptor.bLength);
        debug.field("bDescriptorType", &descriptor.bDescriptorType);
        debug.field("bmAttributes", &descriptor.bmAttributes);
        debug.field("wDetachTimeOut", &descriptor.wDetachTimeOut);
        debug.field("wTransferSize", &descriptor.wTransferSize);
        debug.field("bcdDFUVersion", &descriptor.bcdDFUVersion);

        debug.finish()
    }
}

#[doc(hidden)]
pub unsafe fn from_libusb(descriptor: libusb_dfu_func_descriptor) -> DfuFuncDescriptor {
    DfuFuncDescriptor { descriptor }
}
