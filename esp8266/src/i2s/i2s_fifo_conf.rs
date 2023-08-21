#[doc = "Register `I2S_FIFO_CONF` reader"]
pub type R = crate::R<I2S_FIFO_CONF_SPEC>;
#[doc = "Register `I2S_FIFO_CONF` writer"]
pub type W = crate::W<I2S_FIFO_CONF_SPEC>;
#[doc = "Field `I2S_I2S_RX_DATA_NUM` reader - "]
pub type I2S_I2S_RX_DATA_NUM_R = crate::FieldReader;
#[doc = "Field `I2S_I2S_RX_DATA_NUM` writer - "]
pub type I2S_I2S_RX_DATA_NUM_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O>;
#[doc = "Field `I2S_I2S_TX_DATA_NUM` reader - "]
pub type I2S_I2S_TX_DATA_NUM_R = crate::FieldReader;
#[doc = "Field `I2S_I2S_TX_DATA_NUM` writer - "]
pub type I2S_I2S_TX_DATA_NUM_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O>;
#[doc = "Field `I2S_I2S_DSCR_EN` reader - "]
pub type I2S_I2S_DSCR_EN_R = crate::BitReader;
#[doc = "Field `I2S_I2S_DSCR_EN` writer - "]
pub type I2S_I2S_DSCR_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `I2S_I2S_TX_FIFO_MOD` reader - "]
pub type I2S_I2S_TX_FIFO_MOD_R = crate::FieldReader;
#[doc = "Field `I2S_I2S_TX_FIFO_MOD` writer - "]
pub type I2S_I2S_TX_FIFO_MOD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `I2S_I2S_RX_FIFO_MOD` reader - "]
pub type I2S_I2S_RX_FIFO_MOD_R = crate::FieldReader;
#[doc = "Field `I2S_I2S_RX_FIFO_MOD` writer - "]
pub type I2S_I2S_RX_FIFO_MOD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
impl R {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn i2s_i2s_rx_data_num(&self) -> I2S_I2S_RX_DATA_NUM_R {
        I2S_I2S_RX_DATA_NUM_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:11"]
    #[inline(always)]
    pub fn i2s_i2s_tx_data_num(&self) -> I2S_I2S_TX_DATA_NUM_R {
        I2S_I2S_TX_DATA_NUM_R::new(((self.bits >> 6) & 0x3f) as u8)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn i2s_i2s_dscr_en(&self) -> I2S_I2S_DSCR_EN_R {
        I2S_I2S_DSCR_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:15"]
    #[inline(always)]
    pub fn i2s_i2s_tx_fifo_mod(&self) -> I2S_I2S_TX_FIFO_MOD_R {
        I2S_I2S_TX_FIFO_MOD_R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn i2s_i2s_rx_fifo_mod(&self) -> I2S_I2S_RX_FIFO_MOD_R {
        I2S_I2S_RX_FIFO_MOD_R::new(((self.bits >> 16) & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I2S_FIFO_CONF")
            .field(
                "i2s_i2s_rx_fifo_mod",
                &format_args!("{}", self.i2s_i2s_rx_fifo_mod().bits()),
            )
            .field(
                "i2s_i2s_tx_fifo_mod",
                &format_args!("{}", self.i2s_i2s_tx_fifo_mod().bits()),
            )
            .field(
                "i2s_i2s_dscr_en",
                &format_args!("{}", self.i2s_i2s_dscr_en().bit()),
            )
            .field(
                "i2s_i2s_tx_data_num",
                &format_args!("{}", self.i2s_i2s_tx_data_num().bits()),
            )
            .field(
                "i2s_i2s_rx_data_num",
                &format_args!("{}", self.i2s_i2s_rx_data_num().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<I2S_FIFO_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    #[must_use]
    pub fn i2s_i2s_rx_data_num(&mut self) -> I2S_I2S_RX_DATA_NUM_W<I2S_FIFO_CONF_SPEC, 0> {
        I2S_I2S_RX_DATA_NUM_W::new(self)
    }
    #[doc = "Bits 6:11"]
    #[inline(always)]
    #[must_use]
    pub fn i2s_i2s_tx_data_num(&mut self) -> I2S_I2S_TX_DATA_NUM_W<I2S_FIFO_CONF_SPEC, 6> {
        I2S_I2S_TX_DATA_NUM_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn i2s_i2s_dscr_en(&mut self) -> I2S_I2S_DSCR_EN_W<I2S_FIFO_CONF_SPEC, 12> {
        I2S_I2S_DSCR_EN_W::new(self)
    }
    #[doc = "Bits 13:15"]
    #[inline(always)]
    #[must_use]
    pub fn i2s_i2s_tx_fifo_mod(&mut self) -> I2S_I2S_TX_FIFO_MOD_W<I2S_FIFO_CONF_SPEC, 13> {
        I2S_I2S_TX_FIFO_MOD_W::new(self)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    #[must_use]
    pub fn i2s_i2s_rx_fifo_mod(&mut self) -> I2S_I2S_RX_FIFO_MOD_W<I2S_FIFO_CONF_SPEC, 16> {
        I2S_I2S_RX_FIFO_MOD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "I2S_FIFO_CONF\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2s_fifo_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2s_fifo_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2S_FIFO_CONF_SPEC;
impl crate::RegisterSpec for I2S_FIFO_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2s_fifo_conf::R`](R) reader structure"]
impl crate::Readable for I2S_FIFO_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`i2s_fifo_conf::W`](W) writer structure"]
impl crate::Writable for I2S_FIFO_CONF_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets I2S_FIFO_CONF to value 0"]
impl crate::Resettable for I2S_FIFO_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
