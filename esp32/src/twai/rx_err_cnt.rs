#[doc = "Register `RX_ERR_CNT` reader"]
pub struct R(crate::R<RX_ERR_CNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RX_ERR_CNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RX_ERR_CNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RX_ERR_CNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RX_ERR_CNT` reader - The RX error counter register, reflects value changes under reception status."]
pub type RX_ERR_CNT_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - The RX error counter register, reflects value changes under reception status."]
    #[inline(always)]
    pub fn rx_err_cnt(&self) -> RX_ERR_CNT_R {
        RX_ERR_CNT_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Receive Error Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_err_cnt](index.html) module"]
pub struct RX_ERR_CNT_SPEC;
impl crate::RegisterSpec for RX_ERR_CNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rx_err_cnt::R](R) reader structure"]
impl crate::Readable for RX_ERR_CNT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RX_ERR_CNT to value 0"]
impl crate::Resettable for RX_ERR_CNT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
