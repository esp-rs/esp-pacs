#[doc = "Register `SYSTEM_HP2LP_INTR_GROUP%s_ST` reader"]
pub type R = crate::R<SYSTEM_HP2LP_INTR_GROUP_ST_SPEC>;
#[doc = "Field `VAL` reader - "]
pub type VAL_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn val(&self) -> VAL_R {
        VAL_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SYSTEM_HP2LP_INTR_GROUP_ST")
            .field("val", &self.val())
            .finish()
    }
}
#[doc = "HP to LP interrupt group %s status\n\nYou can [`read`](crate::Reg::read) this register and get [`system_hp2lp_intr_group_st::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYSTEM_HP2LP_INTR_GROUP_ST_SPEC;
impl crate::RegisterSpec for SYSTEM_HP2LP_INTR_GROUP_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`system_hp2lp_intr_group_st::R`](R) reader structure"]
impl crate::Readable for SYSTEM_HP2LP_INTR_GROUP_ST_SPEC {}
#[doc = "`reset()` method sets SYSTEM_HP2LP_INTR_GROUP%s_ST to value 0"]
impl crate::Resettable for SYSTEM_HP2LP_INTR_GROUP_ST_SPEC {}
