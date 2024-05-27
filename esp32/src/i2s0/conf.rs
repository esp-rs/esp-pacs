#[doc = "Register `CONF` reader"]
pub type R = crate::R<CONF_SPEC>;
#[doc = "Register `CONF` writer"]
pub type W = crate::W<CONF_SPEC>;
#[doc = "Field `TX_RESET` reader - "]
pub type TX_RESET_R = crate::BitReader;
#[doc = "Field `TX_RESET` writer - "]
pub type TX_RESET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_RESET` reader - "]
pub type RX_RESET_R = crate::BitReader;
#[doc = "Field `RX_RESET` writer - "]
pub type RX_RESET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_FIFO_RESET` reader - "]
pub type TX_FIFO_RESET_R = crate::BitReader;
#[doc = "Field `TX_FIFO_RESET` writer - "]
pub type TX_FIFO_RESET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_FIFO_RESET` reader - "]
pub type RX_FIFO_RESET_R = crate::BitReader;
#[doc = "Field `RX_FIFO_RESET` writer - "]
pub type RX_FIFO_RESET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_START` reader - "]
pub type TX_START_R = crate::BitReader;
#[doc = "Field `TX_START` writer - "]
pub type TX_START_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_START` reader - "]
pub type RX_START_R = crate::BitReader;
#[doc = "Field `RX_START` writer - "]
pub type RX_START_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_SLAVE_MOD` reader - "]
pub type TX_SLAVE_MOD_R = crate::BitReader;
#[doc = "Field `TX_SLAVE_MOD` writer - "]
pub type TX_SLAVE_MOD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_SLAVE_MOD` reader - "]
pub type RX_SLAVE_MOD_R = crate::BitReader;
#[doc = "Field `RX_SLAVE_MOD` writer - "]
pub type RX_SLAVE_MOD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_RIGHT_FIRST` reader - "]
pub type TX_RIGHT_FIRST_R = crate::BitReader;
#[doc = "Field `TX_RIGHT_FIRST` writer - "]
pub type TX_RIGHT_FIRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_RIGHT_FIRST` reader - "]
pub type RX_RIGHT_FIRST_R = crate::BitReader;
#[doc = "Field `RX_RIGHT_FIRST` writer - "]
pub type RX_RIGHT_FIRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_MSB_SHIFT` reader - "]
pub type TX_MSB_SHIFT_R = crate::BitReader;
#[doc = "Field `TX_MSB_SHIFT` writer - "]
pub type TX_MSB_SHIFT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_MSB_SHIFT` reader - "]
pub type RX_MSB_SHIFT_R = crate::BitReader;
#[doc = "Field `RX_MSB_SHIFT` writer - "]
pub type RX_MSB_SHIFT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_SHORT_SYNC` reader - "]
pub type TX_SHORT_SYNC_R = crate::BitReader;
#[doc = "Field `TX_SHORT_SYNC` writer - "]
pub type TX_SHORT_SYNC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_SHORT_SYNC` reader - "]
pub type RX_SHORT_SYNC_R = crate::BitReader;
#[doc = "Field `RX_SHORT_SYNC` writer - "]
pub type RX_SHORT_SYNC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_MONO` reader - "]
pub type TX_MONO_R = crate::BitReader;
#[doc = "Field `TX_MONO` writer - "]
pub type TX_MONO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_MONO` reader - "]
pub type RX_MONO_R = crate::BitReader;
#[doc = "Field `RX_MONO` writer - "]
pub type RX_MONO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_MSB_RIGHT` reader - "]
pub type TX_MSB_RIGHT_R = crate::BitReader;
#[doc = "Field `TX_MSB_RIGHT` writer - "]
pub type TX_MSB_RIGHT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_MSB_RIGHT` reader - "]
pub type RX_MSB_RIGHT_R = crate::BitReader;
#[doc = "Field `RX_MSB_RIGHT` writer - "]
pub type RX_MSB_RIGHT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SIG_LOOPBACK` reader - "]
pub type SIG_LOOPBACK_R = crate::BitReader;
#[doc = "Field `SIG_LOOPBACK` writer - "]
pub type SIG_LOOPBACK_W<'a, REG> = crate::BitWriter<'a, REG>;
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
            .field("tx_reset", &self.tx_reset())
            .field("rx_reset", &self.rx_reset())
            .field("tx_fifo_reset", &self.tx_fifo_reset())
            .field("rx_fifo_reset", &self.rx_fifo_reset())
            .field("tx_start", &self.tx_start())
            .field("rx_start", &self.rx_start())
            .field("tx_slave_mod", &self.tx_slave_mod())
            .field("rx_slave_mod", &self.rx_slave_mod())
            .field("tx_right_first", &self.tx_right_first())
            .field("rx_right_first", &self.rx_right_first())
            .field("tx_msb_shift", &self.tx_msb_shift())
            .field("rx_msb_shift", &self.rx_msb_shift())
            .field("tx_short_sync", &self.tx_short_sync())
            .field("rx_short_sync", &self.rx_short_sync())
            .field("tx_mono", &self.tx_mono())
            .field("rx_mono", &self.rx_mono())
            .field("tx_msb_right", &self.tx_msb_right())
            .field("rx_msb_right", &self.rx_msb_right())
            .field("sig_loopback", &self.sig_loopback())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn tx_reset(&mut self) -> TX_RESET_W<CONF_SPEC> {
        TX_RESET_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn rx_reset(&mut self) -> RX_RESET_W<CONF_SPEC> {
        RX_RESET_W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn tx_fifo_reset(&mut self) -> TX_FIFO_RESET_W<CONF_SPEC> {
        TX_FIFO_RESET_W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn rx_fifo_reset(&mut self) -> RX_FIFO_RESET_W<CONF_SPEC> {
        RX_FIFO_RESET_W::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn tx_start(&mut self) -> TX_START_W<CONF_SPEC> {
        TX_START_W::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn rx_start(&mut self) -> RX_START_W<CONF_SPEC> {
        RX_START_W::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn tx_slave_mod(&mut self) -> TX_SLAVE_MOD_W<CONF_SPEC> {
        TX_SLAVE_MOD_W::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn rx_slave_mod(&mut self) -> RX_SLAVE_MOD_W<CONF_SPEC> {
        RX_SLAVE_MOD_W::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn tx_right_first(&mut self) -> TX_RIGHT_FIRST_W<CONF_SPEC> {
        TX_RIGHT_FIRST_W::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn rx_right_first(&mut self) -> RX_RIGHT_FIRST_W<CONF_SPEC> {
        RX_RIGHT_FIRST_W::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn tx_msb_shift(&mut self) -> TX_MSB_SHIFT_W<CONF_SPEC> {
        TX_MSB_SHIFT_W::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn rx_msb_shift(&mut self) -> RX_MSB_SHIFT_W<CONF_SPEC> {
        RX_MSB_SHIFT_W::new(self, 11)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn tx_short_sync(&mut self) -> TX_SHORT_SYNC_W<CONF_SPEC> {
        TX_SHORT_SYNC_W::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn rx_short_sync(&mut self) -> RX_SHORT_SYNC_W<CONF_SPEC> {
        RX_SHORT_SYNC_W::new(self, 13)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn tx_mono(&mut self) -> TX_MONO_W<CONF_SPEC> {
        TX_MONO_W::new(self, 14)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn rx_mono(&mut self) -> RX_MONO_W<CONF_SPEC> {
        RX_MONO_W::new(self, 15)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn tx_msb_right(&mut self) -> TX_MSB_RIGHT_W<CONF_SPEC> {
        TX_MSB_RIGHT_W::new(self, 16)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    #[must_use]
    pub fn rx_msb_right(&mut self) -> RX_MSB_RIGHT_W<CONF_SPEC> {
        RX_MSB_RIGHT_W::new(self, 17)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    #[must_use]
    pub fn sig_loopback(&mut self) -> SIG_LOOPBACK_W<CONF_SPEC> {
        SIG_LOOPBACK_W::new(self, 18)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CONF_SPEC;
impl crate::RegisterSpec for CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`conf::R`](R) reader structure"]
impl crate::Readable for CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`conf::W`](W) writer structure"]
impl crate::Writable for CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CONF to value 0x0003_0300"]
impl crate::Resettable for CONF_SPEC {
    const RESET_VALUE: u32 = 0x0003_0300;
}
