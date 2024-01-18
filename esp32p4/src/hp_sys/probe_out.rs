#[doc = "Register `PROBE_OUT` reader"]
pub type R = crate::R<PROBE_OUT_SPEC>;
#[doc = "Field `REG_PROBE_TOP_OUT` reader - NA"]
pub type REG_PROBE_TOP_OUT_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - NA"]
    #[inline(always)]
    pub fn reg_probe_top_out(&self) -> REG_PROBE_TOP_OUT_R {
        REG_PROBE_TOP_OUT_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PROBE_OUT")
            .field(
                "reg_probe_top_out",
                &format_args!("{}", self.reg_probe_top_out().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PROBE_OUT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`probe_out::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PROBE_OUT_SPEC;
impl crate::RegisterSpec for PROBE_OUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`probe_out::R`](R) reader structure"]
impl crate::Readable for PROBE_OUT_SPEC {}
#[doc = "`reset()` method sets PROBE_OUT to value 0"]
impl crate::Resettable for PROBE_OUT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
