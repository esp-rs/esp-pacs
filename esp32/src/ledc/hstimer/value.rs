#[doc = "Register `VALUE` reader"]
pub type R = crate::R<VALUE_SPEC>;
#[doc = "Field `CNT` reader - software can read this register to get the current counter value in high speed timer0"]
pub type CNT_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:19 - software can read this register to get the current counter value in high speed timer0"]
    #[inline(always)]
    pub fn cnt(&self) -> CNT_R {
        CNT_R::new(self.bits & 0x000f_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VALUE").field("cnt", &self.cnt()).finish()
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`value::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VALUE_SPEC;
impl crate::RegisterSpec for VALUE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`value::R`](R) reader structure"]
impl crate::Readable for VALUE_SPEC {}
#[doc = "`reset()` method sets VALUE to value 0"]
impl crate::Resettable for VALUE_SPEC {
    const RESET_VALUE: u32 = 0;
}
