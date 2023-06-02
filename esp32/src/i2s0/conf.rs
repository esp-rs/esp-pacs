#[doc = "Register `CONF` reader"]
pub struct R(crate::R<CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONF` writer"]
pub struct W(crate::W<CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONF_SPEC>;
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
impl From<crate::W<CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TX_RESET` reader - "]
pub type TX_RESET_R = crate::BitReader;
#[doc = "Field `TX_RESET` writer - "]
pub type TX_RESET_W<'a, const O: u8> = crate::BitWriter<'a, CONF_SPEC, O>;
#[doc = "Field `RX_RESET` reader - "]
pub type RX_RESET_R = crate::BitReader;
#[doc = "Field `RX_RESET` writer - "]
pub type RX_RESET_W<'a, const O: u8> = crate::BitWriter<'a, CONF_SPEC, O>;
#[doc = "Field `TX_FIFO_RESET` reader - "]
pub type TX_FIFO_RESET_R = crate::BitReader;
#[doc = "Field `TX_FIFO_RESET` writer - "]
pub type TX_FIFO_RESET_W<'a, const O: u8> = crate::BitWriter<'a, CONF_SPEC, O>;
#[doc = "Field `RX_FIFO_RESET` reader - "]
pub type RX_FIFO_RESET_R = crate::BitReader;
#[doc = "Field `RX_FIFO_RESET` writer - "]
pub type RX_FIFO_RESET_W<'a, const O: u8> = crate::BitWriter<'a, CONF_SPEC, O>;
#[doc = "Field `TX_START` reader - "]
pub type TX_START_R = crate::BitReader;
#[doc = "Field `TX_START` writer - "]
pub type TX_START_W<'a, const O: u8> = crate::BitWriter<'a, CONF_SPEC, O>;
#[doc = "Field `RX_START` reader - "]
pub type RX_START_R = crate::BitReader;
#[doc = "Field `RX_START` writer - "]
pub type RX_START_W<'a, const O: u8> = crate::BitWriter<'a, CONF_SPEC, O>;
#[doc = "Field `TX_SLAVE_MOD` reader - "]
pub type TX_SLAVE_MOD_R = crate::BitReader;
#[doc = "Field `TX_SLAVE_MOD` writer - "]
pub type TX_SLAVE_MOD_W<'a, const O: u8> = crate::BitWriter<'a, CONF_SPEC, O>;
#[doc = "Field `RX_SLAVE_MOD` reader - "]
pub type RX_SLAVE_MOD_R = crate::BitReader;
#[doc = "Field `RX_SLAVE_MOD` writer - "]
pub type RX_SLAVE_MOD_W<'a, const O: u8> = crate::BitWriter<'a, CONF_SPEC, O>;
#[doc = "Field `TX_RIGHT_FIRST` reader - "]
pub type TX_RIGHT_FIRST_R = crate::BitReader;
#[doc = "Field `TX_RIGHT_FIRST` writer - "]
pub type TX_RIGHT_FIRST_W<'a, const O: u8> = crate::BitWriter<'a, CONF_SPEC, O>;
#[doc = "Field `RX_RIGHT_FIRST` reader - "]
pub type RX_RIGHT_FIRST_R = crate::BitReader;
#[doc = "Field `RX_RIGHT_FIRST` writer - "]
pub type RX_RIGHT_FIRST_W<'a, const O: u8> = crate::BitWriter<'a, CONF_SPEC, O>;
#[doc = "Field `TX_MSB_SHIFT` reader - "]
pub type TX_MSB_SHIFT_R = crate::BitReader;
#[doc = "Field `TX_MSB_SHIFT` writer - "]
pub type TX_MSB_SHIFT_W<'a, const O: u8> = crate::BitWriter<'a, CONF_SPEC, O>;
#[doc = "Field `RX_MSB_SHIFT` reader - "]
pub type RX_MSB_SHIFT_R = crate::BitReader;
#[doc = "Field `RX_MSB_SHIFT` writer - "]
pub type RX_MSB_SHIFT_W<'a, const O: u8> = crate::BitWriter<'a, CONF_SPEC, O>;
#[doc = "Field `TX_SHORT_SYNC` reader - "]
pub type TX_SHORT_SYNC_R = crate::BitReader;
#[doc = "Field `TX_SHORT_SYNC` writer - "]
pub type TX_SHORT_SYNC_W<'a, const O: u8> = crate::BitWriter<'a, CONF_SPEC, O>;
#[doc = "Field `RX_SHORT_SYNC` reader - "]
pub type RX_SHORT_SYNC_R = crate::BitReader;
#[doc = "Field `RX_SHORT_SYNC` writer - "]
pub type RX_SHORT_SYNC_W<'a, const O: u8> = crate::BitWriter<'a, CONF_SPEC, O>;
#[doc = "Field `TX_MONO` reader - "]
pub type TX_MONO_R = crate::BitReader;
#[doc = "Field `TX_MONO` writer - "]
pub type TX_MONO_W<'a, const O: u8> = crate::BitWriter<'a, CONF_SPEC, O>;
#[doc = "Field `RX_MONO` reader - "]
pub type RX_MONO_R = crate::BitReader;
#[doc = "Field `RX_MONO` writer - "]
pub type RX_MONO_W<'a, const O: u8> = crate::BitWriter<'a, CONF_SPEC, O>;
#[doc = "Field `TX_MSB_RIGHT` reader - "]
pub type TX_MSB_RIGHT_R = crate::BitReader;
#[doc = "Field `TX_MSB_RIGHT` writer - "]
pub type TX_MSB_RIGHT_W<'a, const O: u8> = crate::BitWriter<'a, CONF_SPEC, O>;
#[doc = "Field `RX_MSB_RIGHT` reader - "]
pub type RX_MSB_RIGHT_R = crate::BitReader;
#[doc = "Field `RX_MSB_RIGHT` writer - "]
pub type RX_MSB_RIGHT_W<'a, const O: u8> = crate::BitWriter<'a, CONF_SPEC, O>;
#[doc = "Field `SIG_LOOPBACK` reader - "]
pub type SIG_LOOPBACK_R = crate::BitReader;
#[doc = "Field `SIG_LOOPBACK` writer - "]
pub type SIG_LOOPBACK_W<'a, const O: u8> = crate::BitWriter<'a, CONF_SPEC, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn tx_reset(&self) -> TX_RESET_R {
        TX_RESET_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rx_reset(&self) -> RX_RESET_R {
        RX_RESET_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn tx_fifo_reset(&self) -> TX_FIFO_RESET_R {
        TX_FIFO_RESET_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rx_fifo_reset(&self) -> RX_FIFO_RESET_R {
        RX_FIFO_RESET_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn tx_start(&self) -> TX_START_R {
        TX_START_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn rx_start(&self) -> RX_START_R {
        RX_START_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn tx_slave_mod(&self) -> TX_SLAVE_MOD_R {
        TX_SLAVE_MOD_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn rx_slave_mod(&self) -> RX_SLAVE_MOD_R {
        RX_SLAVE_MOD_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn tx_right_first(&self) -> TX_RIGHT_FIRST_R {
        TX_RIGHT_FIRST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn rx_right_first(&self) -> RX_RIGHT_FIRST_R {
        RX_RIGHT_FIRST_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn tx_msb_shift(&self) -> TX_MSB_SHIFT_R {
        TX_MSB_SHIFT_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn rx_msb_shift(&self) -> RX_MSB_SHIFT_R {
        RX_MSB_SHIFT_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn tx_short_sync(&self) -> TX_SHORT_SYNC_R {
        TX_SHORT_SYNC_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn rx_short_sync(&self) -> RX_SHORT_SYNC_R {
        RX_SHORT_SYNC_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn tx_mono(&self) -> TX_MONO_R {
        TX_MONO_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn rx_mono(&self) -> RX_MONO_R {
        RX_MONO_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn tx_msb_right(&self) -> TX_MSB_RIGHT_R {
        TX_MSB_RIGHT_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn rx_msb_right(&self) -> RX_MSB_RIGHT_R {
        RX_MSB_RIGHT_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn sig_loopback(&self) -> SIG_LOOPBACK_R {
        SIG_LOOPBACK_R::new(((self.bits >> 18) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONF")
            .field("tx_reset", &format_args!("{}", self.tx_reset().bit()))
            .field("rx_reset", &format_args!("{}", self.rx_reset().bit()))
            .field(
                "tx_fifo_reset",
                &format_args!("{}", self.tx_fifo_reset().bit()),
            )
            .field(
                "rx_fifo_reset",
                &format_args!("{}", self.rx_fifo_reset().bit()),
            )
            .field("tx_start", &format_args!("{}", self.tx_start().bit()))
            .field("rx_start", &format_args!("{}", self.rx_start().bit()))
            .field(
                "tx_slave_mod",
                &format_args!("{}", self.tx_slave_mod().bit()),
            )
            .field(
                "rx_slave_mod",
                &format_args!("{}", self.rx_slave_mod().bit()),
            )
            .field(
                "tx_right_first",
                &format_args!("{}", self.tx_right_first().bit()),
            )
            .field(
                "rx_right_first",
                &format_args!("{}", self.rx_right_first().bit()),
            )
            .field(
                "tx_msb_shift",
                &format_args!("{}", self.tx_msb_shift().bit()),
            )
            .field(
                "rx_msb_shift",
                &format_args!("{}", self.rx_msb_shift().bit()),
            )
            .field(
                "tx_short_sync",
                &format_args!("{}", self.tx_short_sync().bit()),
            )
            .field(
                "rx_short_sync",
                &format_args!("{}", self.rx_short_sync().bit()),
            )
            .field("tx_mono", &format_args!("{}", self.tx_mono().bit()))
            .field("rx_mono", &format_args!("{}", self.rx_mono().bit()))
            .field(
                "tx_msb_right",
                &format_args!("{}", self.tx_msb_right().bit()),
            )
            .field(
                "rx_msb_right",
                &format_args!("{}", self.rx_msb_right().bit()),
            )
            .field(
                "sig_loopback",
                &format_args!("{}", self.sig_loopback().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn tx_reset(&mut self) -> TX_RESET_W<0> {
        TX_RESET_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn rx_reset(&mut self) -> RX_RESET_W<1> {
        RX_RESET_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn tx_fifo_reset(&mut self) -> TX_FIFO_RESET_W<2> {
        TX_FIFO_RESET_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn rx_fifo_reset(&mut self) -> RX_FIFO_RESET_W<3> {
        RX_FIFO_RESET_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn tx_start(&mut self) -> TX_START_W<4> {
        TX_START_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn rx_start(&mut self) -> RX_START_W<5> {
        RX_START_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn tx_slave_mod(&mut self) -> TX_SLAVE_MOD_W<6> {
        TX_SLAVE_MOD_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn rx_slave_mod(&mut self) -> RX_SLAVE_MOD_W<7> {
        RX_SLAVE_MOD_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn tx_right_first(&mut self) -> TX_RIGHT_FIRST_W<8> {
        TX_RIGHT_FIRST_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn rx_right_first(&mut self) -> RX_RIGHT_FIRST_W<9> {
        RX_RIGHT_FIRST_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn tx_msb_shift(&mut self) -> TX_MSB_SHIFT_W<10> {
        TX_MSB_SHIFT_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn rx_msb_shift(&mut self) -> RX_MSB_SHIFT_W<11> {
        RX_MSB_SHIFT_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn tx_short_sync(&mut self) -> TX_SHORT_SYNC_W<12> {
        TX_SHORT_SYNC_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn rx_short_sync(&mut self) -> RX_SHORT_SYNC_W<13> {
        RX_SHORT_SYNC_W::new(self)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn tx_mono(&mut self) -> TX_MONO_W<14> {
        TX_MONO_W::new(self)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn rx_mono(&mut self) -> RX_MONO_W<15> {
        RX_MONO_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn tx_msb_right(&mut self) -> TX_MSB_RIGHT_W<16> {
        TX_MSB_RIGHT_W::new(self)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    #[must_use]
    pub fn rx_msb_right(&mut self) -> RX_MSB_RIGHT_W<17> {
        RX_MSB_RIGHT_W::new(self)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    #[must_use]
    pub fn sig_loopback(&mut self) -> SIG_LOOPBACK_W<18> {
        SIG_LOOPBACK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conf](index.html) module"]
pub struct CONF_SPEC;
impl crate::RegisterSpec for CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [conf::R](R) reader structure"]
impl crate::Readable for CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [conf::W](W) writer structure"]
impl crate::Writable for CONF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CONF to value 0x0003_0300"]
impl crate::Resettable for CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0x0003_0300;
}
