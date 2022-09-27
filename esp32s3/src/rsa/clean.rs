#[doc = "Register `CLEAN` reader"]
pub struct R(crate::R<CLEAN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLEAN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLEAN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLEAN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CLEAN` reader - The content of this bit is 1 when memories complete initialization."]
pub type CLEAN_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - The content of this bit is 1 when memories complete initialization."]
    #[inline(always)]
    pub fn clean(&self) -> CLEAN_R {
        CLEAN_R::new((self.bits & 1) != 0)
    }
}
#[doc = "RSA clean register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clean](index.html) module"]
pub struct CLEAN_SPEC;
impl crate::RegisterSpec for CLEAN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clean::R](R) reader structure"]
impl crate::Readable for CLEAN_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CLEAN to value 0"]
impl crate::Resettable for CLEAN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
