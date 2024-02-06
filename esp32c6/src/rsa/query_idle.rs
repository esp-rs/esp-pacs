#[doc = "Register `QUERY_IDLE` reader"]
pub type R = crate::R<QUERY_IDLE_SPEC>;
#[doc = "Field `QUERY_IDLE` reader - query rsa idle. 1'b0: busy, 1'b1: idle"]
pub type QUERY_IDLE_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - query rsa idle. 1'b0: busy, 1'b1: idle"]
    #[inline(always)]
    pub fn query_idle(&self) -> QUERY_IDLE_R {
        QUERY_IDLE_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("QUERY_IDLE")
            .field("query_idle", &format_args!("{}", self.query_idle().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<QUERY_IDLE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "RSA query idle register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`query_idle::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QUERY_IDLE_SPEC;
impl crate::RegisterSpec for QUERY_IDLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`query_idle::R`](R) reader structure"]
impl crate::Readable for QUERY_IDLE_SPEC {}
#[doc = "`reset()` method sets QUERY_IDLE to value 0"]
impl crate::Resettable for QUERY_IDLE_SPEC {
    const RESET_VALUE: u32 = 0;
}
