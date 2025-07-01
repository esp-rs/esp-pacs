#[doc = "Register `STATUS_INT` reader"]
pub type R = crate::R<STATUS_INT_SPEC>;
#[doc = "Field `NEXT` reader - need des"]
pub type NEXT_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - need des"]
    #[inline(always)]
    pub fn next(&self) -> NEXT_R {
        NEXT_R::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STATUS_INT")
            .field("next", &self.next())
            .finish()
    }
}
#[doc = "need des\n\nYou can [`read`](crate::Reg::read) this register and get [`status_int::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STATUS_INT_SPEC;
impl crate::RegisterSpec for STATUS_INT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status_int::R`](R) reader structure"]
impl crate::Readable for STATUS_INT_SPEC {}
#[doc = "`reset()` method sets STATUS_INT to value 0"]
impl crate::Resettable for STATUS_INT_SPEC {}
