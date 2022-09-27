#[doc = "Register `FRAM_NUM` reader"]
pub struct R(crate::R<FRAM_NUM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FRAM_NUM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FRAM_NUM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FRAM_NUM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SOF_FRAME_INDEX` reader - Frame index of received SOF frame."]
pub type SOF_FRAME_INDEX_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:10 - Frame index of received SOF frame."]
    #[inline(always)]
    pub fn sof_frame_index(&self) -> SOF_FRAME_INDEX_R {
        SOF_FRAME_INDEX_R::new((self.bits & 0x07ff) as u16)
    }
}
#[doc = "SOF frame number\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fram_num](index.html) module"]
pub struct FRAM_NUM_SPEC;
impl crate::RegisterSpec for FRAM_NUM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fram_num::R](R) reader structure"]
impl crate::Readable for FRAM_NUM_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FRAM_NUM to value 0"]
impl crate::Resettable for FRAM_NUM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
