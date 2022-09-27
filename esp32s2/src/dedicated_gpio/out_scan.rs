#[doc = "Register `OUT_SCAN` reader"]
pub struct R(crate::R<OUT_SCAN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OUT_SCAN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OUT_SCAN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OUT_SCAN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `OUT_STATUS` reader - GPIO out value configured by DEDIC_GPIO_OUT_DRT_REG, DEDIC_GPIO_OUT_MSK_REG, DEDIC_GPIO_OUT_IDV_REG."]
pub type OUT_STATUS_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - GPIO out value configured by DEDIC_GPIO_OUT_DRT_REG, DEDIC_GPIO_OUT_MSK_REG, DEDIC_GPIO_OUT_IDV_REG."]
    #[inline(always)]
    pub fn out_status(&self) -> OUT_STATUS_R {
        OUT_STATUS_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Dedicated GPIO output status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [out_scan](index.html) module"]
pub struct OUT_SCAN_SPEC;
impl crate::RegisterSpec for OUT_SCAN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [out_scan::R](R) reader structure"]
impl crate::Readable for OUT_SCAN_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets OUT_SCAN to value 0"]
impl crate::Resettable for OUT_SCAN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
