#[doc = "Register `RX_CTRL` reader"]
pub type R = crate::R<RX_CTRL_SPEC>;
#[doc = "Register `RX_CTRL` writer"]
pub type W = crate::W<RX_CTRL_SPEC>;
#[doc = "Field `RX_ENA` reader - write this bit to enable the bitscrambler rx"]
pub type RX_ENA_R = crate::BitReader;
#[doc = "Field `RX_ENA` writer - write this bit to enable the bitscrambler rx"]
pub type RX_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_PAUSE` reader - write this bit to pause the bitscrambler rx core"]
pub type RX_PAUSE_R = crate::BitReader;
#[doc = "Field `RX_PAUSE` writer - write this bit to pause the bitscrambler rx core"]
pub type RX_PAUSE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_HALT` reader - write this bit to halt the bitscrambler rx core"]
pub type RX_HALT_R = crate::BitReader;
#[doc = "Field `RX_HALT` writer - write this bit to halt the bitscrambler rx core"]
pub type RX_HALT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_EOF_MODE` reader - write this bit to ser the bitscrambler rx core EOF signal generating mode which is combined with reg_bitscrambler_rx_tailing_bits, 0: counter by read peripheral buffer, 0 counter by write dma fifo"]
pub type RX_EOF_MODE_R = crate::BitReader;
#[doc = "Field `RX_EOF_MODE` writer - write this bit to ser the bitscrambler rx core EOF signal generating mode which is combined with reg_bitscrambler_rx_tailing_bits, 0: counter by read peripheral buffer, 0 counter by write dma fifo"]
pub type RX_EOF_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_COND_MODE` reader - write this bit to specify the LOOP instruction condition mode of bitscrambler rx core, 0: use the little than operator to get the condition, 1: use not equal operator to get the condition"]
pub type RX_COND_MODE_R = crate::BitReader;
#[doc = "Field `RX_COND_MODE` writer - write this bit to specify the LOOP instruction condition mode of bitscrambler rx core, 0: use the little than operator to get the condition, 1: use not equal operator to get the condition"]
pub type RX_COND_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_FETCH_MODE` reader - write this bit to set the bitscrambler rx core fetch instruction mode, 0: prefetch by reset, 1: fetch by instrutions"]
pub type RX_FETCH_MODE_R = crate::BitReader;
#[doc = "Field `RX_FETCH_MODE` writer - write this bit to set the bitscrambler rx core fetch instruction mode, 0: prefetch by reset, 1: fetch by instrutions"]
pub type RX_FETCH_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_HALT_MODE` reader - write this bit to set the bitscrambler rx core halt mode when rx_halt is set, 0: wait write data back done, , 1: ignore write data back"]
pub type RX_HALT_MODE_R = crate::BitReader;
#[doc = "Field `RX_HALT_MODE` writer - write this bit to set the bitscrambler rx core halt mode when rx_halt is set, 0: wait write data back done, , 1: ignore write data back"]
pub type RX_HALT_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_RD_DUMMY` reader - write this bit to set the bitscrambler rx core read data mode when EOF received.0: wait read data, 1: ignore read data"]
pub type RX_RD_DUMMY_R = crate::BitReader;
#[doc = "Field `RX_RD_DUMMY` writer - write this bit to set the bitscrambler rx core read data mode when EOF received.0: wait read data, 1: ignore read data"]
pub type RX_RD_DUMMY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_FIFO_RST` writer - write this bit to reset the bitscrambler rx fifo"]
pub type RX_FIFO_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - write this bit to enable the bitscrambler rx"]
    #[inline(always)]
    pub fn rx_ena(&self) -> RX_ENA_R {
        RX_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - write this bit to pause the bitscrambler rx core"]
    #[inline(always)]
    pub fn rx_pause(&self) -> RX_PAUSE_R {
        RX_PAUSE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - write this bit to halt the bitscrambler rx core"]
    #[inline(always)]
    pub fn rx_halt(&self) -> RX_HALT_R {
        RX_HALT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - write this bit to ser the bitscrambler rx core EOF signal generating mode which is combined with reg_bitscrambler_rx_tailing_bits, 0: counter by read peripheral buffer, 0 counter by write dma fifo"]
    #[inline(always)]
    pub fn rx_eof_mode(&self) -> RX_EOF_MODE_R {
        RX_EOF_MODE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - write this bit to specify the LOOP instruction condition mode of bitscrambler rx core, 0: use the little than operator to get the condition, 1: use not equal operator to get the condition"]
    #[inline(always)]
    pub fn rx_cond_mode(&self) -> RX_COND_MODE_R {
        RX_COND_MODE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - write this bit to set the bitscrambler rx core fetch instruction mode, 0: prefetch by reset, 1: fetch by instrutions"]
    #[inline(always)]
    pub fn rx_fetch_mode(&self) -> RX_FETCH_MODE_R {
        RX_FETCH_MODE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - write this bit to set the bitscrambler rx core halt mode when rx_halt is set, 0: wait write data back done, , 1: ignore write data back"]
    #[inline(always)]
    pub fn rx_halt_mode(&self) -> RX_HALT_MODE_R {
        RX_HALT_MODE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - write this bit to set the bitscrambler rx core read data mode when EOF received.0: wait read data, 1: ignore read data"]
    #[inline(always)]
    pub fn rx_rd_dummy(&self) -> RX_RD_DUMMY_R {
        RX_RD_DUMMY_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RX_CTRL")
            .field("rx_ena", &self.rx_ena())
            .field("rx_pause", &self.rx_pause())
            .field("rx_halt", &self.rx_halt())
            .field("rx_eof_mode", &self.rx_eof_mode())
            .field("rx_cond_mode", &self.rx_cond_mode())
            .field("rx_fetch_mode", &self.rx_fetch_mode())
            .field("rx_halt_mode", &self.rx_halt_mode())
            .field("rx_rd_dummy", &self.rx_rd_dummy())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - write this bit to enable the bitscrambler rx"]
    #[inline(always)]
    #[must_use]
    pub fn rx_ena(&mut self) -> RX_ENA_W<RX_CTRL_SPEC> {
        RX_ENA_W::new(self, 0)
    }
    #[doc = "Bit 1 - write this bit to pause the bitscrambler rx core"]
    #[inline(always)]
    #[must_use]
    pub fn rx_pause(&mut self) -> RX_PAUSE_W<RX_CTRL_SPEC> {
        RX_PAUSE_W::new(self, 1)
    }
    #[doc = "Bit 2 - write this bit to halt the bitscrambler rx core"]
    #[inline(always)]
    #[must_use]
    pub fn rx_halt(&mut self) -> RX_HALT_W<RX_CTRL_SPEC> {
        RX_HALT_W::new(self, 2)
    }
    #[doc = "Bit 3 - write this bit to ser the bitscrambler rx core EOF signal generating mode which is combined with reg_bitscrambler_rx_tailing_bits, 0: counter by read peripheral buffer, 0 counter by write dma fifo"]
    #[inline(always)]
    #[must_use]
    pub fn rx_eof_mode(&mut self) -> RX_EOF_MODE_W<RX_CTRL_SPEC> {
        RX_EOF_MODE_W::new(self, 3)
    }
    #[doc = "Bit 4 - write this bit to specify the LOOP instruction condition mode of bitscrambler rx core, 0: use the little than operator to get the condition, 1: use not equal operator to get the condition"]
    #[inline(always)]
    #[must_use]
    pub fn rx_cond_mode(&mut self) -> RX_COND_MODE_W<RX_CTRL_SPEC> {
        RX_COND_MODE_W::new(self, 4)
    }
    #[doc = "Bit 5 - write this bit to set the bitscrambler rx core fetch instruction mode, 0: prefetch by reset, 1: fetch by instrutions"]
    #[inline(always)]
    #[must_use]
    pub fn rx_fetch_mode(&mut self) -> RX_FETCH_MODE_W<RX_CTRL_SPEC> {
        RX_FETCH_MODE_W::new(self, 5)
    }
    #[doc = "Bit 6 - write this bit to set the bitscrambler rx core halt mode when rx_halt is set, 0: wait write data back done, , 1: ignore write data back"]
    #[inline(always)]
    #[must_use]
    pub fn rx_halt_mode(&mut self) -> RX_HALT_MODE_W<RX_CTRL_SPEC> {
        RX_HALT_MODE_W::new(self, 6)
    }
    #[doc = "Bit 7 - write this bit to set the bitscrambler rx core read data mode when EOF received.0: wait read data, 1: ignore read data"]
    #[inline(always)]
    #[must_use]
    pub fn rx_rd_dummy(&mut self) -> RX_RD_DUMMY_W<RX_CTRL_SPEC> {
        RX_RD_DUMMY_W::new(self, 7)
    }
    #[doc = "Bit 8 - write this bit to reset the bitscrambler rx fifo"]
    #[inline(always)]
    #[must_use]
    pub fn rx_fifo_rst(&mut self) -> RX_FIFO_RST_W<RX_CTRL_SPEC> {
        RX_FIFO_RST_W::new(self, 8)
    }
}
#[doc = "Control and configuration registers\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RX_CTRL_SPEC;
impl crate::RegisterSpec for RX_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_ctrl::R`](R) reader structure"]
impl crate::Readable for RX_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rx_ctrl::W`](W) writer structure"]
impl crate::Writable for RX_CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RX_CTRL to value 0x04"]
impl crate::Resettable for RX_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x04;
}
