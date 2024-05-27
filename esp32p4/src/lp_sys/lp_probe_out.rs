#[doc = "Register `LP_PROBE_OUT` reader"]
pub type R = crate::R<LP_PROBE_OUT_SPEC>;
#[doc = "Field `PROBE_TOP_OUT` reader - need_des"]
pub type PROBE_TOP_OUT_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - need_des"]
    #[inline(always)]
    pub fn probe_top_out(&self) -> PROBE_TOP_OUT_R {
        PROBE_TOP_OUT_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LP_PROBE_OUT")
            .field("probe_top_out", &self.probe_top_out())
            .finish()
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_probe_out::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LP_PROBE_OUT_SPEC;
impl crate::RegisterSpec for LP_PROBE_OUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lp_probe_out::R`](R) reader structure"]
impl crate::Readable for LP_PROBE_OUT_SPEC {}
#[doc = "`reset()` method sets LP_PROBE_OUT to value 0"]
impl crate::Resettable for LP_PROBE_OUT_SPEC {
    const RESET_VALUE: u32 = 0;
}
