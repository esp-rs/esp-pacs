#[doc = "Register `I2S_INT_RAW` reader"]
pub struct R(crate::R<I2S_INT_RAW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2S_INT_RAW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2S_INT_RAW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2S_INT_RAW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `I2S_RX_DONE_INT_RAW` reader - The raw interrupt status bit for the i2s_rx_done_int interrupt"]
pub type I2S_RX_DONE_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `I2S_TX_DONE_INT_RAW` reader - The raw interrupt status bit for the i2s_tx_done_int interrupt"]
pub type I2S_TX_DONE_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `I2S_RX_HUNG_INT_RAW` reader - The raw interrupt status bit for the i2s_rx_hung_int interrupt"]
pub type I2S_RX_HUNG_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `I2S_TX_HUNG_INT_RAW` reader - The raw interrupt status bit for the i2s_tx_hung_int interrupt"]
pub type I2S_TX_HUNG_INT_RAW_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - The raw interrupt status bit for the i2s_rx_done_int interrupt"]
    #[inline(always)]
    pub fn i2s_rx_done_int_raw(&self) -> I2S_RX_DONE_INT_RAW_R {
        I2S_RX_DONE_INT_RAW_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The raw interrupt status bit for the i2s_tx_done_int interrupt"]
    #[inline(always)]
    pub fn i2s_tx_done_int_raw(&self) -> I2S_TX_DONE_INT_RAW_R {
        I2S_TX_DONE_INT_RAW_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The raw interrupt status bit for the i2s_rx_hung_int interrupt"]
    #[inline(always)]
    pub fn i2s_rx_hung_int_raw(&self) -> I2S_RX_HUNG_INT_RAW_R {
        I2S_RX_HUNG_INT_RAW_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The raw interrupt status bit for the i2s_tx_hung_int interrupt"]
    #[inline(always)]
    pub fn i2s_tx_hung_int_raw(&self) -> I2S_TX_HUNG_INT_RAW_R {
        I2S_TX_HUNG_INT_RAW_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "I2S interrupt raw register, valid in level.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2s_int_raw](index.html) module"]
pub struct I2S_INT_RAW_SPEC;
impl crate::RegisterSpec for I2S_INT_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2s_int_raw::R](R) reader structure"]
impl crate::Readable for I2S_INT_RAW_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets I2S_INT_RAW to value 0"]
impl crate::Resettable for I2S_INT_RAW_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
