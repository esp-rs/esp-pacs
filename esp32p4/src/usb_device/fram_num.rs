#[doc = "Register `FRAM_NUM` reader"]
pub type R = crate::R<FRAM_NUM_SPEC>;
#[doc = "Field `USB_SERIAL_JTAG_SOF_FRAME_INDEX` reader - Frame index of received SOF frame."]
pub type USB_SERIAL_JTAG_SOF_FRAME_INDEX_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:10 - Frame index of received SOF frame."]
    #[inline(always)]
    pub fn usb_serial_jtag_sof_frame_index(&self) -> USB_SERIAL_JTAG_SOF_FRAME_INDEX_R {
        USB_SERIAL_JTAG_SOF_FRAME_INDEX_R::new((self.bits & 0x07ff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FRAM_NUM")
            .field(
                "usb_serial_jtag_sof_frame_index",
                &format_args!("{}", self.usb_serial_jtag_sof_frame_index().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<FRAM_NUM_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Last received SOF frame index register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fram_num::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FRAM_NUM_SPEC;
impl crate::RegisterSpec for FRAM_NUM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fram_num::R`](R) reader structure"]
impl crate::Readable for FRAM_NUM_SPEC {}
#[doc = "`reset()` method sets FRAM_NUM to value 0"]
impl crate::Resettable for FRAM_NUM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
