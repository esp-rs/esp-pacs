#[doc = "Register `I2SINT_ST` reader"]
pub type R = crate::R<I2SINT_ST_SPEC>;
#[doc = "Register `I2SINT_ST` writer"]
pub type W = crate::W<I2SINT_ST_SPEC>;
#[doc = "Field `I2S_I2S_RX_TAKE_DATA_INT_ST` reader - "]
pub type I2S_I2S_RX_TAKE_DATA_INT_ST_R = crate::BitReader;
#[doc = "Field `I2S_I2S_RX_TAKE_DATA_INT_ST` writer - "]
pub type I2S_I2S_RX_TAKE_DATA_INT_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2S_I2S_TX_PUT_DATA_INT_ST` reader - "]
pub type I2S_I2S_TX_PUT_DATA_INT_ST_R = crate::BitReader;
#[doc = "Field `I2S_I2S_TX_PUT_DATA_INT_ST` writer - "]
pub type I2S_I2S_TX_PUT_DATA_INT_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2S_I2S_RX_WFULL_INT_ST` reader - "]
pub type I2S_I2S_RX_WFULL_INT_ST_R = crate::BitReader;
#[doc = "Field `I2S_I2S_RX_WFULL_INT_ST` writer - "]
pub type I2S_I2S_RX_WFULL_INT_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2S_I2S_RX_REMPTY_INT_ST` reader - "]
pub type I2S_I2S_RX_REMPTY_INT_ST_R = crate::BitReader;
#[doc = "Field `I2S_I2S_RX_REMPTY_INT_ST` writer - "]
pub type I2S_I2S_RX_REMPTY_INT_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2S_I2S_TX_WFULL_INT_ST` reader - "]
pub type I2S_I2S_TX_WFULL_INT_ST_R = crate::BitReader;
#[doc = "Field `I2S_I2S_TX_WFULL_INT_ST` writer - "]
pub type I2S_I2S_TX_WFULL_INT_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2S_I2S_TX_REMPTY_INT_ST` reader - "]
pub type I2S_I2S_TX_REMPTY_INT_ST_R = crate::BitReader;
#[doc = "Field `I2S_I2S_TX_REMPTY_INT_ST` writer - "]
pub type I2S_I2S_TX_REMPTY_INT_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
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
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn i2s_i2s_rx_take_data_int_st(&mut self) -> I2S_I2S_RX_TAKE_DATA_INT_ST_W<I2SINT_ST_SPEC> {
        I2S_I2S_RX_TAKE_DATA_INT_ST_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn i2s_i2s_tx_put_data_int_st(&mut self) -> I2S_I2S_TX_PUT_DATA_INT_ST_W<I2SINT_ST_SPEC> {
        I2S_I2S_TX_PUT_DATA_INT_ST_W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn i2s_i2s_rx_wfull_int_st(&mut self) -> I2S_I2S_RX_WFULL_INT_ST_W<I2SINT_ST_SPEC> {
        I2S_I2S_RX_WFULL_INT_ST_W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn i2s_i2s_rx_rempty_int_st(&mut self) -> I2S_I2S_RX_REMPTY_INT_ST_W<I2SINT_ST_SPEC> {
        I2S_I2S_RX_REMPTY_INT_ST_W::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn i2s_i2s_tx_wfull_int_st(&mut self) -> I2S_I2S_TX_WFULL_INT_ST_W<I2SINT_ST_SPEC> {
        I2S_I2S_TX_WFULL_INT_ST_W::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn i2s_i2s_tx_rempty_int_st(&mut self) -> I2S_I2S_TX_REMPTY_INT_ST_W<I2SINT_ST_SPEC> {
        I2S_I2S_TX_REMPTY_INT_ST_W::new(self, 5)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "I2SINT_ST\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2sint_st::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2sint_st::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2SINT_ST_SPEC;
impl crate::RegisterSpec for I2SINT_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2sint_st::R`](R) reader structure"]
impl crate::Readable for I2SINT_ST_SPEC {}
#[doc = "`write(|w| ..)` method takes [`i2sint_st::W`](W) writer structure"]
impl crate::Writable for I2SINT_ST_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets I2SINT_ST to value 0"]
impl crate::Resettable for I2SINT_ST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
