#[doc = "Register `CDETECT` reader"]
pub struct R(crate::R<CDETECT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CDETECT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CDETECT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CDETECT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CARD_DETECT_N` reader - Value on sdhost_card_detect_n input ports (1 bit per card), read-only bits. 0 represents presence of card. Only NUM_CARDS number of bits are implemented."]
pub type CARD_DETECT_N_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:1 - Value on sdhost_card_detect_n input ports (1 bit per card), read-only bits. 0 represents presence of card. Only NUM_CARDS number of bits are implemented."]
    #[inline(always)]
    pub fn card_detect_n(&self) -> CARD_DETECT_N_R {
        CARD_DETECT_N_R::new((self.bits & 3) as u8)
    }
}
#[doc = "Card detect register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cdetect](index.html) module"]
pub struct CDETECT_SPEC;
impl crate::RegisterSpec for CDETECT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cdetect::R](R) reader structure"]
impl crate::Readable for CDETECT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CDETECT to value 0"]
impl crate::Resettable for CDETECT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
