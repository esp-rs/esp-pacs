#[doc = "Register `RX_HEAD` reader"]
pub type R = crate::R<RX_HEAD_SPEC>;
#[doc = "Field `RX_HEAD` reader - a"]
pub type RX_HEAD_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - a"]
    #[inline(always)]
    pub fn rx_head(&self) -> RX_HEAD_R {
        RX_HEAD_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RX_HEAD")
            .field("rx_head", &format_args!("{}", self.rx_head().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RX_HEAD_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "a\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_head::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RX_HEAD_SPEC;
impl crate::RegisterSpec for RX_HEAD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_head::R`](R) reader structure"]
impl crate::Readable for RX_HEAD_SPEC {}
#[doc = "`reset()` method sets RX_HEAD to value 0"]
impl crate::Resettable for RX_HEAD_SPEC {
    const RESET_VALUE: u32 = 0;
}
