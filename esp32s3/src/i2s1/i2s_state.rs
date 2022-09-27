#[doc = "Register `I2S_STATE` reader"]
pub struct R(crate::R<I2S_STATE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2S_STATE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2S_STATE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2S_STATE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `I2S_TX_IDLE` reader - 1: i2s_tx is idle state. 0: i2s_tx is working."]
pub type I2S_TX_IDLE_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - 1: i2s_tx is idle state. 0: i2s_tx is working."]
    #[inline(always)]
    pub fn i2s_tx_idle(&self) -> I2S_TX_IDLE_R {
        I2S_TX_IDLE_R::new((self.bits & 1) != 0)
    }
}
#[doc = "I2S TX status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2s_state](index.html) module"]
pub struct I2S_STATE_SPEC;
impl crate::RegisterSpec for I2S_STATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2s_state::R](R) reader structure"]
impl crate::Readable for I2S_STATE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets I2S_STATE to value 0x01"]
impl crate::Resettable for I2S_STATE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
