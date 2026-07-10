#[doc = "Register `HP2LP_INTR_GROUP1_ST` reader"]
pub type R = crate::R<HP2LP_INTR_GROUP1_ST_SPEC>;
#[doc = "Field `H2LP_INTR_GROUP1_ST` reader - Each bit indicates the status of corresponding peripheral interrupt to LP CPU."]
pub type H2LP_INTR_GROUP1_ST_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Each bit indicates the status of corresponding peripheral interrupt to LP CPU."]
    #[inline(always)]
    pub fn h2lp_intr_group1_st(&self) -> H2LP_INTR_GROUP1_ST_R {
        H2LP_INTR_GROUP1_ST_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HP2LP_INTR_GROUP1_ST")
            .field("h2lp_intr_group1_st", &self.h2lp_intr_group1_st())
            .finish()
    }
}
#[doc = "HpP2LP Interrupt Enable Register Group1\n\nYou can [`read`](crate::Reg::read) this register and get [`hp2lp_intr_group1_st::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HP2LP_INTR_GROUP1_ST_SPEC;
impl crate::RegisterSpec for HP2LP_INTR_GROUP1_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hp2lp_intr_group1_st::R`](R) reader structure"]
impl crate::Readable for HP2LP_INTR_GROUP1_ST_SPEC {}
#[doc = "`reset()` method sets HP2LP_INTR_GROUP1_ST to value 0"]
impl crate::Resettable for HP2LP_INTR_GROUP1_ST_SPEC {}
