#[doc = "Register `SEL_AG4_RANGE4` reader"]
pub type R = crate::R<SEL_AG4_RANGE4_SPEC>;
#[doc = "Field `RESULT` reader - The interval statistics results in this counter for sel agent"]
pub type RESULT_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - The interval statistics results in this counter for sel agent"]
    #[inline(always)]
    pub fn result(&self) -> RESULT_R {
        RESULT_R::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SEL_AG4_RANGE4")
            .field("result", &self.result())
            .finish()
    }
}
#[doc = "reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag4_range4::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SEL_AG4_RANGE4_SPEC;
impl crate::RegisterSpec for SEL_AG4_RANGE4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sel_ag4_range4::R`](R) reader structure"]
impl crate::Readable for SEL_AG4_RANGE4_SPEC {}
#[doc = "`reset()` method sets SEL_AG4_RANGE4 to value 0"]
impl crate::Resettable for SEL_AG4_RANGE4_SPEC {}
