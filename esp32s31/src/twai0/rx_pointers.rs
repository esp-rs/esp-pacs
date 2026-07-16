#[doc = "Register `RX_POINTERS` reader"]
pub type R = crate::R<RX_POINTERS_SPEC>;
#[doc = "Field `RX_WPP` reader - Write pointer position in RX buffer. Upon store of received frame write pointer is updated."]
pub type RX_WPP_R = crate::FieldReader<u16>;
#[doc = "Field `RX_RPP` reader - Read pointer position in RX buffer. Upon read of received frame read pointer is updated."]
pub type RX_RPP_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:11 - Write pointer position in RX buffer. Upon store of received frame write pointer is updated."]
    #[inline(always)]
    pub fn rx_wpp(&self) -> RX_WPP_R {
        RX_WPP_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - Read pointer position in RX buffer. Upon read of received frame read pointer is updated."]
    #[inline(always)]
    pub fn rx_rpp(&self) -> RX_RPP_R {
        RX_RPP_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RX_POINTERS")
            .field("rx_wpp", &self.rx_wpp())
            .field("rx_rpp", &self.rx_rpp())
            .finish()
    }
}
#[doc = "TWAI FD rx memory pointer information register\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_pointers::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RX_POINTERS_SPEC;
impl crate::RegisterSpec for RX_POINTERS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_pointers::R`](R) reader structure"]
impl crate::Readable for RX_POINTERS_SPEC {}
#[doc = "`reset()` method sets RX_POINTERS to value 0"]
impl crate::Resettable for RX_POINTERS_SPEC {}
