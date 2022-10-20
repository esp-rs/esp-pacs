#[doc = "Register `GNPTXSTS` reader"]
pub struct R(crate::R<GNPTXSTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GNPTXSTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GNPTXSTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GNPTXSTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `NPTXFSPCAVAIL` reader - "]
pub type NPTXFSPCAVAIL_R = crate::FieldReader<u16, u16>;
#[doc = "Field `NPTXQSPCAVAIL` reader - "]
pub type NPTXQSPCAVAIL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NPTXQTOP` reader - "]
pub type NPTXQTOP_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn nptxfspcavail(&self) -> NPTXFSPCAVAIL_R {
        NPTXFSPCAVAIL_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn nptxqspcavail(&self) -> NPTXQSPCAVAIL_R {
        NPTXQSPCAVAIL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:30"]
    #[inline(always)]
    pub fn nptxqtop(&self) -> NPTXQTOP_R {
        NPTXQTOP_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gnptxsts](index.html) module"]
pub struct GNPTXSTS_SPEC;
impl crate::RegisterSpec for GNPTXSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gnptxsts::R](R) reader structure"]
impl crate::Readable for GNPTXSTS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets GNPTXSTS to value 0x0004_0100"]
impl crate::Resettable for GNPTXSTS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0004_0100
    }
}
