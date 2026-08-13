#[doc = "Register `HI` reader"]
pub type R = crate::R<HI_SPEC>;
#[doc = "Field `HI_RO` reader - Represents the actual target value of COMP0, high 20 bits."]
pub type HI_RO_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:19 - Represents the actual target value of COMP0, high 20 bits."]
    #[inline(always)]
    pub fn hi_ro(&self) -> HI_RO_R {
        HI_RO_R::new(self.bits & 0x000f_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HI").field("hi_ro", &self.hi_ro()).finish()
    }
}
#[doc = "Actual target value of COMP0, high 20 bits\n\nYou can [`read`](crate::Reg::read) this register and get [`hi::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HI_SPEC;
impl crate::RegisterSpec for HI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hi::R`](R) reader structure"]
impl crate::Readable for HI_SPEC {}
#[doc = "`reset()` method sets HI to value 0"]
impl crate::Resettable for HI_SPEC {}
