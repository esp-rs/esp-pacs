#[doc = "Register `SEL_AG5_COUNTER6` reader"]
pub type R = crate::R<SEL_AG5_COUNTER6_SPEC>;
#[doc = "Field `RESULT` reader - The result for sel agent metric measured in this counter, \\[15:0\\] is min result, and \\[31:16\\] is max result"]
pub type RESULT_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - The result for sel agent metric measured in this counter, \\[15:0\\] is min result, and \\[31:16\\] is max result"]
    #[inline(always)]
    pub fn result(&self) -> RESULT_R {
        RESULT_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SEL_AG5_COUNTER6")
            .field("result", &self.result())
            .finish()
    }
}
#[doc = "reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag5_counter6::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SEL_AG5_COUNTER6_SPEC;
impl crate::RegisterSpec for SEL_AG5_COUNTER6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sel_ag5_counter6::R`](R) reader structure"]
impl crate::Readable for SEL_AG5_COUNTER6_SPEC {}
#[doc = "`reset()` method sets SEL_AG5_COUNTER6 to value 0"]
impl crate::Resettable for SEL_AG5_COUNTER6_SPEC {}
