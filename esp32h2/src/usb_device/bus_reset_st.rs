#[doc = "Register `BUS_RESET_ST` reader"]
pub struct R(crate::R<BUS_RESET_ST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BUS_RESET_ST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BUS_RESET_ST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BUS_RESET_ST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `USB_BUS_RESET_ST` reader - USB bus reset status. 0: USB-Serial-JTAG is in usb bus reset status. 1: USB bus reset is released."]
pub type USB_BUS_RESET_ST_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - USB bus reset status. 0: USB-Serial-JTAG is in usb bus reset status. 1: USB bus reset is released."]
    #[inline(always)]
    pub fn usb_bus_reset_st(&self) -> USB_BUS_RESET_ST_R {
        USB_BUS_RESET_ST_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BUS_RESET_ST")
            .field(
                "usb_bus_reset_st",
                &format_args!("{}", self.usb_bus_reset_st().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<BUS_RESET_ST_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "USB Bus reset status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bus_reset_st](index.html) module"]
pub struct BUS_RESET_ST_SPEC;
impl crate::RegisterSpec for BUS_RESET_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bus_reset_st::R](R) reader structure"]
impl crate::Readable for BUS_RESET_ST_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets BUS_RESET_ST to value 0x01"]
impl crate::Resettable for BUS_RESET_ST_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
