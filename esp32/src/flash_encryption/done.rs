#[doc = "Register `DONE` reader"]
pub struct R(crate::R<DONE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DONE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DONE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DONE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `FLASH_DONE` reader - Set this bit when encryption operation is complete."]
pub type FLASH_DONE_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Set this bit when encryption operation is complete."]
    #[inline(always)]
    pub fn flash_done(&self) -> FLASH_DONE_R {
        FLASH_DONE_R::new((self.bits & 1) != 0)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [done](index.html) module"]
pub struct DONE_SPEC;
impl crate::RegisterSpec for DONE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [done::R](R) reader structure"]
impl crate::Readable for DONE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DONE to value 0"]
impl crate::Resettable for DONE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
