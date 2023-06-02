#[doc = "Register `I2SINT_ST` reader"]
pub struct R(crate::R<I2SINT_ST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2SINT_ST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2SINT_ST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2SINT_ST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `I2SINT_ST` writer"]
pub struct W(crate::W<I2SINT_ST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2SINT_ST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<I2SINT_ST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2SINT_ST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `I2S_I2S_RX_TAKE_DATA_INT_ST` reader - "]
pub type I2S_I2S_RX_TAKE_DATA_INT_ST_R = crate::BitReader;
#[doc = "Field `I2S_I2S_RX_TAKE_DATA_INT_ST` writer - "]
pub type I2S_I2S_RX_TAKE_DATA_INT_ST_W<'a, const O: u8> = crate::BitWriter<'a, I2SINT_ST_SPEC, O>;
#[doc = "Field `I2S_I2S_TX_PUT_DATA_INT_ST` reader - "]
pub type I2S_I2S_TX_PUT_DATA_INT_ST_R = crate::BitReader;
#[doc = "Field `I2S_I2S_TX_PUT_DATA_INT_ST` writer - "]
pub type I2S_I2S_TX_PUT_DATA_INT_ST_W<'a, const O: u8> = crate::BitWriter<'a, I2SINT_ST_SPEC, O>;
#[doc = "Field `I2S_I2S_RX_WFULL_INT_ST` reader - "]
pub type I2S_I2S_RX_WFULL_INT_ST_R = crate::BitReader;
#[doc = "Field `I2S_I2S_RX_WFULL_INT_ST` writer - "]
pub type I2S_I2S_RX_WFULL_INT_ST_W<'a, const O: u8> = crate::BitWriter<'a, I2SINT_ST_SPEC, O>;
#[doc = "Field `I2S_I2S_RX_REMPTY_INT_ST` reader - "]
pub type I2S_I2S_RX_REMPTY_INT_ST_R = crate::BitReader;
#[doc = "Field `I2S_I2S_RX_REMPTY_INT_ST` writer - "]
pub type I2S_I2S_RX_REMPTY_INT_ST_W<'a, const O: u8> = crate::BitWriter<'a, I2SINT_ST_SPEC, O>;
#[doc = "Field `I2S_I2S_TX_WFULL_INT_ST` reader - "]
pub type I2S_I2S_TX_WFULL_INT_ST_R = crate::BitReader;
#[doc = "Field `I2S_I2S_TX_WFULL_INT_ST` writer - "]
pub type I2S_I2S_TX_WFULL_INT_ST_W<'a, const O: u8> = crate::BitWriter<'a, I2SINT_ST_SPEC, O>;
#[doc = "Field `I2S_I2S_TX_REMPTY_INT_ST` reader - "]
pub type I2S_I2S_TX_REMPTY_INT_ST_R = crate::BitReader;
#[doc = "Field `I2S_I2S_TX_REMPTY_INT_ST` writer - "]
pub type I2S_I2S_TX_REMPTY_INT_ST_W<'a, const O: u8> = crate::BitWriter<'a, I2SINT_ST_SPEC, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn i2s_i2s_rx_take_data_int_st(&self) -> I2S_I2S_RX_TAKE_DATA_INT_ST_R {
        I2S_I2S_RX_TAKE_DATA_INT_ST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn i2s_i2s_tx_put_data_int_st(&self) -> I2S_I2S_TX_PUT_DATA_INT_ST_R {
        I2S_I2S_TX_PUT_DATA_INT_ST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn i2s_i2s_rx_wfull_int_st(&self) -> I2S_I2S_RX_WFULL_INT_ST_R {
        I2S_I2S_RX_WFULL_INT_ST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn i2s_i2s_rx_rempty_int_st(&self) -> I2S_I2S_RX_REMPTY_INT_ST_R {
        I2S_I2S_RX_REMPTY_INT_ST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn i2s_i2s_tx_wfull_int_st(&self) -> I2S_I2S_TX_WFULL_INT_ST_R {
        I2S_I2S_TX_WFULL_INT_ST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn i2s_i2s_tx_rempty_int_st(&self) -> I2S_I2S_TX_REMPTY_INT_ST_R {
        I2S_I2S_TX_REMPTY_INT_ST_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I2SINT_ST")
            .field(
                "i2s_i2s_tx_rempty_int_st",
                &format_args!("{}", self.i2s_i2s_tx_rempty_int_st().bit()),
            )
            .field(
                "i2s_i2s_tx_wfull_int_st",
                &format_args!("{}", self.i2s_i2s_tx_wfull_int_st().bit()),
            )
            .field(
                "i2s_i2s_rx_rempty_int_st",
                &format_args!("{}", self.i2s_i2s_rx_rempty_int_st().bit()),
            )
            .field(
                "i2s_i2s_rx_wfull_int_st",
                &format_args!("{}", self.i2s_i2s_rx_wfull_int_st().bit()),
            )
            .field(
                "i2s_i2s_tx_put_data_int_st",
                &format_args!("{}", self.i2s_i2s_tx_put_data_int_st().bit()),
            )
            .field(
                "i2s_i2s_rx_take_data_int_st",
                &format_args!("{}", self.i2s_i2s_rx_take_data_int_st().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<I2SINT_ST_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn i2s_i2s_rx_take_data_int_st(&mut self) -> I2S_I2S_RX_TAKE_DATA_INT_ST_W<0> {
        I2S_I2S_RX_TAKE_DATA_INT_ST_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn i2s_i2s_tx_put_data_int_st(&mut self) -> I2S_I2S_TX_PUT_DATA_INT_ST_W<1> {
        I2S_I2S_TX_PUT_DATA_INT_ST_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn i2s_i2s_rx_wfull_int_st(&mut self) -> I2S_I2S_RX_WFULL_INT_ST_W<2> {
        I2S_I2S_RX_WFULL_INT_ST_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn i2s_i2s_rx_rempty_int_st(&mut self) -> I2S_I2S_RX_REMPTY_INT_ST_W<3> {
        I2S_I2S_RX_REMPTY_INT_ST_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn i2s_i2s_tx_wfull_int_st(&mut self) -> I2S_I2S_TX_WFULL_INT_ST_W<4> {
        I2S_I2S_TX_WFULL_INT_ST_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn i2s_i2s_tx_rempty_int_st(&mut self) -> I2S_I2S_TX_REMPTY_INT_ST_W<5> {
        I2S_I2S_TX_REMPTY_INT_ST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2SINT_ST\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2sint_st](index.html) module"]
pub struct I2SINT_ST_SPEC;
impl crate::RegisterSpec for I2SINT_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2sint_st::R](R) reader structure"]
impl crate::Readable for I2SINT_ST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2sint_st::W](W) writer structure"]
impl crate::Writable for I2SINT_ST_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets I2SINT_ST to value 0"]
impl crate::Resettable for I2SINT_ST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
