#[doc = "Register `I2SCONF` reader"]
pub struct R(crate::R<I2SCONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2SCONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2SCONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2SCONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `I2SCONF` writer"]
pub struct W(crate::W<I2SCONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2SCONF_SPEC>;
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
impl From<crate::W<I2SCONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2SCONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `I2S_I2S_TX_RESET` reader - "]
pub type I2S_I2S_TX_RESET_R = crate::BitReader;
#[doc = "Field `I2S_I2S_TX_RESET` writer - "]
pub type I2S_I2S_TX_RESET_W<'a, const O: u8> = crate::BitWriter<'a, I2SCONF_SPEC, O>;
#[doc = "Field `I2S_I2S_RX_RESET` reader - "]
pub type I2S_I2S_RX_RESET_R = crate::BitReader;
#[doc = "Field `I2S_I2S_RX_RESET` writer - "]
pub type I2S_I2S_RX_RESET_W<'a, const O: u8> = crate::BitWriter<'a, I2SCONF_SPEC, O>;
#[doc = "Field `I2S_I2S_TX_FIFO_RESET` reader - "]
pub type I2S_I2S_TX_FIFO_RESET_R = crate::BitReader;
#[doc = "Field `I2S_I2S_TX_FIFO_RESET` writer - "]
pub type I2S_I2S_TX_FIFO_RESET_W<'a, const O: u8> = crate::BitWriter<'a, I2SCONF_SPEC, O>;
#[doc = "Field `I2S_I2S_RX_FIFO_RESET` reader - "]
pub type I2S_I2S_RX_FIFO_RESET_R = crate::BitReader;
#[doc = "Field `I2S_I2S_RX_FIFO_RESET` writer - "]
pub type I2S_I2S_RX_FIFO_RESET_W<'a, const O: u8> = crate::BitWriter<'a, I2SCONF_SPEC, O>;
#[doc = "Field `I2S_TRANS_SLAVE_MOD` reader - "]
pub type I2S_TRANS_SLAVE_MOD_R = crate::BitReader;
#[doc = "Field `I2S_TRANS_SLAVE_MOD` writer - "]
pub type I2S_TRANS_SLAVE_MOD_W<'a, const O: u8> = crate::BitWriter<'a, I2SCONF_SPEC, O>;
#[doc = "Field `I2S_RECE_SLAVE_MOD` reader - "]
pub type I2S_RECE_SLAVE_MOD_R = crate::BitReader;
#[doc = "Field `I2S_RECE_SLAVE_MOD` writer - "]
pub type I2S_RECE_SLAVE_MOD_W<'a, const O: u8> = crate::BitWriter<'a, I2SCONF_SPEC, O>;
#[doc = "Field `I2S_RIGHT_FIRST` reader - "]
pub type I2S_RIGHT_FIRST_R = crate::BitReader;
#[doc = "Field `I2S_RIGHT_FIRST` writer - "]
pub type I2S_RIGHT_FIRST_W<'a, const O: u8> = crate::BitWriter<'a, I2SCONF_SPEC, O>;
#[doc = "Field `I2S_MSB_RIGHT` reader - "]
pub type I2S_MSB_RIGHT_R = crate::BitReader;
#[doc = "Field `I2S_MSB_RIGHT` writer - "]
pub type I2S_MSB_RIGHT_W<'a, const O: u8> = crate::BitWriter<'a, I2SCONF_SPEC, O>;
#[doc = "Field `I2S_I2S_TX_START` reader - "]
pub type I2S_I2S_TX_START_R = crate::BitReader;
#[doc = "Field `I2S_I2S_TX_START` writer - "]
pub type I2S_I2S_TX_START_W<'a, const O: u8> = crate::BitWriter<'a, I2SCONF_SPEC, O>;
#[doc = "Field `I2S_I2S_RX_START` reader - "]
pub type I2S_I2S_RX_START_R = crate::BitReader;
#[doc = "Field `I2S_I2S_RX_START` writer - "]
pub type I2S_I2S_RX_START_W<'a, const O: u8> = crate::BitWriter<'a, I2SCONF_SPEC, O>;
#[doc = "Field `I2S_TRANS_MSB_SHIFT` reader - "]
pub type I2S_TRANS_MSB_SHIFT_R = crate::BitReader;
#[doc = "Field `I2S_TRANS_MSB_SHIFT` writer - "]
pub type I2S_TRANS_MSB_SHIFT_W<'a, const O: u8> = crate::BitWriter<'a, I2SCONF_SPEC, O>;
#[doc = "Field `I2S_RECE_MSB_SHIFT` reader - "]
pub type I2S_RECE_MSB_SHIFT_R = crate::BitReader;
#[doc = "Field `I2S_RECE_MSB_SHIFT` writer - "]
pub type I2S_RECE_MSB_SHIFT_W<'a, const O: u8> = crate::BitWriter<'a, I2SCONF_SPEC, O>;
#[doc = "Field `I2S_BITS_MOD` reader - "]
pub type I2S_BITS_MOD_R = crate::FieldReader;
#[doc = "Field `I2S_BITS_MOD` writer - "]
pub type I2S_BITS_MOD_W<'a, const O: u8> = crate::FieldWriter<'a, I2SCONF_SPEC, 4, O>;
#[doc = "Field `I2S_CLKM_DIV_NUM` reader - "]
pub type I2S_CLKM_DIV_NUM_R = crate::FieldReader;
#[doc = "Field `I2S_CLKM_DIV_NUM` writer - "]
pub type I2S_CLKM_DIV_NUM_W<'a, const O: u8> = crate::FieldWriter<'a, I2SCONF_SPEC, 6, O>;
#[doc = "Field `I2S_BCK_DIV_NUM` reader - "]
pub type I2S_BCK_DIV_NUM_R = crate::FieldReader;
#[doc = "Field `I2S_BCK_DIV_NUM` writer - "]
pub type I2S_BCK_DIV_NUM_W<'a, const O: u8> = crate::FieldWriter<'a, I2SCONF_SPEC, 6, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn i2s_i2s_tx_reset(&self) -> I2S_I2S_TX_RESET_R {
        I2S_I2S_TX_RESET_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn i2s_i2s_rx_reset(&self) -> I2S_I2S_RX_RESET_R {
        I2S_I2S_RX_RESET_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn i2s_i2s_tx_fifo_reset(&self) -> I2S_I2S_TX_FIFO_RESET_R {
        I2S_I2S_TX_FIFO_RESET_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn i2s_i2s_rx_fifo_reset(&self) -> I2S_I2S_RX_FIFO_RESET_R {
        I2S_I2S_RX_FIFO_RESET_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn i2s_trans_slave_mod(&self) -> I2S_TRANS_SLAVE_MOD_R {
        I2S_TRANS_SLAVE_MOD_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn i2s_rece_slave_mod(&self) -> I2S_RECE_SLAVE_MOD_R {
        I2S_RECE_SLAVE_MOD_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn i2s_right_first(&self) -> I2S_RIGHT_FIRST_R {
        I2S_RIGHT_FIRST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn i2s_msb_right(&self) -> I2S_MSB_RIGHT_R {
        I2S_MSB_RIGHT_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn i2s_i2s_tx_start(&self) -> I2S_I2S_TX_START_R {
        I2S_I2S_TX_START_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn i2s_i2s_rx_start(&self) -> I2S_I2S_RX_START_R {
        I2S_I2S_RX_START_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn i2s_trans_msb_shift(&self) -> I2S_TRANS_MSB_SHIFT_R {
        I2S_TRANS_MSB_SHIFT_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn i2s_rece_msb_shift(&self) -> I2S_RECE_MSB_SHIFT_R {
        I2S_RECE_MSB_SHIFT_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn i2s_bits_mod(&self) -> I2S_BITS_MOD_R {
        I2S_BITS_MOD_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:21"]
    #[inline(always)]
    pub fn i2s_clkm_div_num(&self) -> I2S_CLKM_DIV_NUM_R {
        I2S_CLKM_DIV_NUM_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 22:27"]
    #[inline(always)]
    pub fn i2s_bck_div_num(&self) -> I2S_BCK_DIV_NUM_R {
        I2S_BCK_DIV_NUM_R::new(((self.bits >> 22) & 0x3f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I2SCONF")
            .field(
                "i2s_bck_div_num",
                &format_args!("{}", self.i2s_bck_div_num().bits()),
            )
            .field(
                "i2s_clkm_div_num",
                &format_args!("{}", self.i2s_clkm_div_num().bits()),
            )
            .field(
                "i2s_bits_mod",
                &format_args!("{}", self.i2s_bits_mod().bits()),
            )
            .field(
                "i2s_rece_msb_shift",
                &format_args!("{}", self.i2s_rece_msb_shift().bit()),
            )
            .field(
                "i2s_trans_msb_shift",
                &format_args!("{}", self.i2s_trans_msb_shift().bit()),
            )
            .field(
                "i2s_i2s_rx_start",
                &format_args!("{}", self.i2s_i2s_rx_start().bit()),
            )
            .field(
                "i2s_i2s_tx_start",
                &format_args!("{}", self.i2s_i2s_tx_start().bit()),
            )
            .field(
                "i2s_msb_right",
                &format_args!("{}", self.i2s_msb_right().bit()),
            )
            .field(
                "i2s_right_first",
                &format_args!("{}", self.i2s_right_first().bit()),
            )
            .field(
                "i2s_rece_slave_mod",
                &format_args!("{}", self.i2s_rece_slave_mod().bit()),
            )
            .field(
                "i2s_trans_slave_mod",
                &format_args!("{}", self.i2s_trans_slave_mod().bit()),
            )
            .field(
                "i2s_i2s_rx_fifo_reset",
                &format_args!("{}", self.i2s_i2s_rx_fifo_reset().bit()),
            )
            .field(
                "i2s_i2s_tx_fifo_reset",
                &format_args!("{}", self.i2s_i2s_tx_fifo_reset().bit()),
            )
            .field(
                "i2s_i2s_rx_reset",
                &format_args!("{}", self.i2s_i2s_rx_reset().bit()),
            )
            .field(
                "i2s_i2s_tx_reset",
                &format_args!("{}", self.i2s_i2s_tx_reset().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<I2SCONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn i2s_i2s_tx_reset(&mut self) -> I2S_I2S_TX_RESET_W<0> {
        I2S_I2S_TX_RESET_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn i2s_i2s_rx_reset(&mut self) -> I2S_I2S_RX_RESET_W<1> {
        I2S_I2S_RX_RESET_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn i2s_i2s_tx_fifo_reset(&mut self) -> I2S_I2S_TX_FIFO_RESET_W<2> {
        I2S_I2S_TX_FIFO_RESET_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn i2s_i2s_rx_fifo_reset(&mut self) -> I2S_I2S_RX_FIFO_RESET_W<3> {
        I2S_I2S_RX_FIFO_RESET_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn i2s_trans_slave_mod(&mut self) -> I2S_TRANS_SLAVE_MOD_W<4> {
        I2S_TRANS_SLAVE_MOD_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn i2s_rece_slave_mod(&mut self) -> I2S_RECE_SLAVE_MOD_W<5> {
        I2S_RECE_SLAVE_MOD_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn i2s_right_first(&mut self) -> I2S_RIGHT_FIRST_W<6> {
        I2S_RIGHT_FIRST_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn i2s_msb_right(&mut self) -> I2S_MSB_RIGHT_W<7> {
        I2S_MSB_RIGHT_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn i2s_i2s_tx_start(&mut self) -> I2S_I2S_TX_START_W<8> {
        I2S_I2S_TX_START_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn i2s_i2s_rx_start(&mut self) -> I2S_I2S_RX_START_W<9> {
        I2S_I2S_RX_START_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn i2s_trans_msb_shift(&mut self) -> I2S_TRANS_MSB_SHIFT_W<10> {
        I2S_TRANS_MSB_SHIFT_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn i2s_rece_msb_shift(&mut self) -> I2S_RECE_MSB_SHIFT_W<11> {
        I2S_RECE_MSB_SHIFT_W::new(self)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    #[must_use]
    pub fn i2s_bits_mod(&mut self) -> I2S_BITS_MOD_W<12> {
        I2S_BITS_MOD_W::new(self)
    }
    #[doc = "Bits 16:21"]
    #[inline(always)]
    #[must_use]
    pub fn i2s_clkm_div_num(&mut self) -> I2S_CLKM_DIV_NUM_W<16> {
        I2S_CLKM_DIV_NUM_W::new(self)
    }
    #[doc = "Bits 22:27"]
    #[inline(always)]
    #[must_use]
    pub fn i2s_bck_div_num(&mut self) -> I2S_BCK_DIV_NUM_W<22> {
        I2S_BCK_DIV_NUM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2SCONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2sconf](index.html) module"]
pub struct I2SCONF_SPEC;
impl crate::RegisterSpec for I2SCONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2sconf::R](R) reader structure"]
impl crate::Readable for I2SCONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2sconf::W](W) writer structure"]
impl crate::Writable for I2SCONF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets I2SCONF to value 0"]
impl crate::Resettable for I2SCONF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
