#[doc = "Register `I2S_INT_ST` reader"]
pub struct R(crate::R<I2S_INT_ST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2S_INT_ST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2S_INT_ST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2S_INT_ST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `I2S_RX_DONE_INT_ST` reader - The masked interrupt status bit for the i2s_rx_done_int interrupt"]
pub type I2S_RX_DONE_INT_ST_R = crate::BitReader<bool>;
#[doc = "Field `I2S_TX_DONE_INT_ST` reader - The masked interrupt status bit for the i2s_tx_done_int interrupt"]
pub type I2S_TX_DONE_INT_ST_R = crate::BitReader<bool>;
#[doc = "Field `I2S_RX_HUNG_INT_ST` reader - The masked interrupt status bit for the i2s_rx_hung_int interrupt"]
pub type I2S_RX_HUNG_INT_ST_R = crate::BitReader<bool>;
#[doc = "Field `I2S_TX_HUNG_INT_ST` reader - The masked interrupt status bit for the i2s_tx_hung_int interrupt"]
pub type I2S_TX_HUNG_INT_ST_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - The masked interrupt status bit for the i2s_rx_done_int interrupt"]
    #[inline(always)]
    pub fn i2s_rx_done_int_st(&self) -> I2S_RX_DONE_INT_ST_R {
        I2S_RX_DONE_INT_ST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The masked interrupt status bit for the i2s_tx_done_int interrupt"]
    #[inline(always)]
    pub fn i2s_tx_done_int_st(&self) -> I2S_TX_DONE_INT_ST_R {
        I2S_TX_DONE_INT_ST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The masked interrupt status bit for the i2s_rx_hung_int interrupt"]
    #[inline(always)]
    pub fn i2s_rx_hung_int_st(&self) -> I2S_RX_HUNG_INT_ST_R {
        I2S_RX_HUNG_INT_ST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The masked interrupt status bit for the i2s_tx_hung_int interrupt"]
    #[inline(always)]
    pub fn i2s_tx_hung_int_st(&self) -> I2S_TX_HUNG_INT_ST_R {
        I2S_TX_HUNG_INT_ST_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "I2S interrupt status register.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2s_int_st](index.html) module"]
pub struct I2S_INT_ST_SPEC;
impl crate::RegisterSpec for I2S_INT_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2s_int_st::R](R) reader structure"]
impl crate::Readable for I2S_INT_ST_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets I2S_INT_ST to value 0"]
impl crate::Resettable for I2S_INT_ST_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
