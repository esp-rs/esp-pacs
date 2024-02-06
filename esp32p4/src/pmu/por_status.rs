#[doc = "Register `POR_STATUS` reader"]
pub type R = crate::R<POR_STATUS_SPEC>;
#[doc = "Field `POR_DONE` reader - need_des"]
pub type POR_DONE_R = crate::BitReader;
impl R {
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn por_done(&self) -> POR_DONE_R {
        POR_DONE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("POR_STATUS")
            .field("por_done", &format_args!("{}", self.por_done().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<POR_STATUS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`por_status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct POR_STATUS_SPEC;
impl crate::RegisterSpec for POR_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`por_status::R`](R) reader structure"]
impl crate::Readable for POR_STATUS_SPEC {}
#[doc = "`reset()` method sets POR_STATUS to value 0x8000_0000"]
impl crate::Resettable for POR_STATUS_SPEC {
    const RESET_VALUE: u32 = 0x8000_0000;
}
