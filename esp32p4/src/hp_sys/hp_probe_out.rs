#[doc = "Register `HP_PROBE_OUT` reader"]
pub type R = crate::R<HP_PROBE_OUT_SPEC>;
#[doc = "Field `HP_REG_PROBE_TOP_OUT` reader - NA"]
pub type HP_REG_PROBE_TOP_OUT_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - NA"]
    #[inline(always)]
    pub fn hp_reg_probe_top_out(&self) -> HP_REG_PROBE_TOP_OUT_R {
        HP_REG_PROBE_TOP_OUT_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HP_PROBE_OUT")
            .field(
                "hp_reg_probe_top_out",
                &format_args!("{}", self.hp_reg_probe_top_out().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HP_PROBE_OUT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hp_probe_out::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HP_PROBE_OUT_SPEC;
impl crate::RegisterSpec for HP_PROBE_OUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hp_probe_out::R`](R) reader structure"]
impl crate::Readable for HP_PROBE_OUT_SPEC {}
#[doc = "`reset()` method sets HP_PROBE_OUT to value 0"]
impl crate::Resettable for HP_PROBE_OUT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
