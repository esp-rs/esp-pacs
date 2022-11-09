#[doc = "Register `GSNPSID` reader"]
pub struct R(crate::R<GSNPSID_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GSNPSID_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GSNPSID_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GSNPSID_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SYNOPSYSID` reader - "]
pub type SYNOPSYSID_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn synopsysid(&self) -> SYNOPSYSID_R {
        SYNOPSYSID_R::new(self.bits)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gsnpsid](index.html) module"]
pub struct GSNPSID_SPEC;
impl crate::RegisterSpec for GSNPSID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gsnpsid::R](R) reader structure"]
impl crate::Readable for GSNPSID_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets GSNPSID to value 0x4f54_400a"]
impl crate::Resettable for GSNPSID_SPEC {
    const RESET_VALUE: Self::Ux = 0x4f54_400a;
}
