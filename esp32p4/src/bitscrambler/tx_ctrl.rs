#[doc = "Register `TX_CTRL` reader"]
pub type R = crate::R<TX_CTRL_SPEC>;
#[doc = "Register `TX_CTRL` writer"]
pub type W = crate::W<TX_CTRL_SPEC>;
#[doc = "Field `TX_ENA` reader - write this bit to enable the bitscrambler tx"]
pub type TX_ENA_R = crate::BitReader;
#[doc = "Field `TX_ENA` writer - write this bit to enable the bitscrambler tx"]
pub type TX_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_PAUSE` reader - write this bit to pause the bitscrambler tx core"]
pub type TX_PAUSE_R = crate::BitReader;
#[doc = "Field `TX_PAUSE` writer - write this bit to pause the bitscrambler tx core"]
pub type TX_PAUSE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_HALT` reader - write this bit to halt the bitscrambler tx core"]
pub type TX_HALT_R = crate::BitReader;
#[doc = "Field `TX_HALT` writer - write this bit to halt the bitscrambler tx core"]
pub type TX_HALT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_EOF_MODE` reader - write this bit to ser the bitscrambler tx core EOF signal generating mode which is combined with reg_bitscrambler_tx_tailing_bits, 0: counter by read dma fifo, 0 counter by write peripheral buffer"]
pub type TX_EOF_MODE_R = crate::BitReader;
#[doc = "Field `TX_EOF_MODE` writer - write this bit to ser the bitscrambler tx core EOF signal generating mode which is combined with reg_bitscrambler_tx_tailing_bits, 0: counter by read dma fifo, 0 counter by write peripheral buffer"]
pub type TX_EOF_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_COND_MODE` reader - write this bit to specify the LOOP instruction condition mode of bitscrambler tx core, 0: use the little than operator to get the condition, 1: use not equal operator to get the condition"]
pub type TX_COND_MODE_R = crate::BitReader;
#[doc = "Field `TX_COND_MODE` writer - write this bit to specify the LOOP instruction condition mode of bitscrambler tx core, 0: use the little than operator to get the condition, 1: use not equal operator to get the condition"]
pub type TX_COND_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_FETCH_MODE` reader - write this bit to set the bitscrambler tx core fetch instruction mode, 0: prefetch by reset, 1: fetch by instrutions"]
pub type TX_FETCH_MODE_R = crate::BitReader;
#[doc = "Field `TX_FETCH_MODE` writer - write this bit to set the bitscrambler tx core fetch instruction mode, 0: prefetch by reset, 1: fetch by instrutions"]
pub type TX_FETCH_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_HALT_MODE` reader - write this bit to set the bitscrambler tx core halt mode when tx_halt is set, 0: wait write data back done, , 1: ignore write data back"]
pub type TX_HALT_MODE_R = crate::BitReader;
#[doc = "Field `TX_HALT_MODE` writer - write this bit to set the bitscrambler tx core halt mode when tx_halt is set, 0: wait write data back done, , 1: ignore write data back"]
pub type TX_HALT_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_RD_DUMMY` reader - write this bit to set the bitscrambler tx core read data mode when EOF received.0: wait read data, 1: ignore read data"]
pub type TX_RD_DUMMY_R = crate::BitReader;
#[doc = "Field `TX_RD_DUMMY` writer - write this bit to set the bitscrambler tx core read data mode when EOF received.0: wait read data, 1: ignore read data"]
pub type TX_RD_DUMMY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_FIFO_RST` writer - write this bit to reset the bitscrambler tx fifo"]
pub type TX_FIFO_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - write this bit to enable the bitscrambler tx"]
    #[inline(always)]
    pub fn tx_ena(&self) -> TX_ENA_R {
        TX_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - write this bit to pause the bitscrambler tx core"]
    #[inline(always)]
    pub fn tx_pause(&self) -> TX_PAUSE_R {
        TX_PAUSE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - write this bit to halt the bitscrambler tx core"]
    #[inline(always)]
    pub fn tx_halt(&self) -> TX_HALT_R {
        TX_HALT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - write this bit to ser the bitscrambler tx core EOF signal generating mode which is combined with reg_bitscrambler_tx_tailing_bits, 0: counter by read dma fifo, 0 counter by write peripheral buffer"]
    #[inline(always)]
    pub fn tx_eof_mode(&self) -> TX_EOF_MODE_R {
        TX_EOF_MODE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - write this bit to specify the LOOP instruction condition mode of bitscrambler tx core, 0: use the little than operator to get the condition, 1: use not equal operator to get the condition"]
    #[inline(always)]
    pub fn tx_cond_mode(&self) -> TX_COND_MODE_R {
        TX_COND_MODE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - write this bit to set the bitscrambler tx core fetch instruction mode, 0: prefetch by reset, 1: fetch by instrutions"]
    #[inline(always)]
    pub fn tx_fetch_mode(&self) -> TX_FETCH_MODE_R {
        TX_FETCH_MODE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - write this bit to set the bitscrambler tx core halt mode when tx_halt is set, 0: wait write data back done, , 1: ignore write data back"]
    #[inline(always)]
    pub fn tx_halt_mode(&self) -> TX_HALT_MODE_R {
        TX_HALT_MODE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - write this bit to set the bitscrambler tx core read data mode when EOF received.0: wait read data, 1: ignore read data"]
    #[inline(always)]
    pub fn tx_rd_dummy(&self) -> TX_RD_DUMMY_R {
        TX_RD_DUMMY_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TX_CTRL")
            .field("tx_ena", &format_args!("{}", self.tx_ena().bit()))
            .field("tx_pause", &format_args!("{}", self.tx_pause().bit()))
            .field("tx_halt", &format_args!("{}", self.tx_halt().bit()))
            .field("tx_eof_mode", &format_args!("{}", self.tx_eof_mode().bit()))
            .field(
                "tx_cond_mode",
                &format_args!("{}", self.tx_cond_mode().bit()),
            )
            .field(
                "tx_fetch_mode",
                &format_args!("{}", self.tx_fetch_mode().bit()),
            )
            .field(
                "tx_halt_mode",
                &format_args!("{}", self.tx_halt_mode().bit()),
            )
            .field("tx_rd_dummy", &format_args!("{}", self.tx_rd_dummy().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TX_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - write this bit to enable the bitscrambler tx"]
    #[inline(always)]
    #[must_use]
    pub fn tx_ena(&mut self) -> TX_ENA_W<TX_CTRL_SPEC> {
        TX_ENA_W::new(self, 0)
    }
    #[doc = "Bit 1 - write this bit to pause the bitscrambler tx core"]
    #[inline(always)]
    #[must_use]
    pub fn tx_pause(&mut self) -> TX_PAUSE_W<TX_CTRL_SPEC> {
        TX_PAUSE_W::new(self, 1)
    }
    #[doc = "Bit 2 - write this bit to halt the bitscrambler tx core"]
    #[inline(always)]
    #[must_use]
    pub fn tx_halt(&mut self) -> TX_HALT_W<TX_CTRL_SPEC> {
        TX_HALT_W::new(self, 2)
    }
    #[doc = "Bit 3 - write this bit to ser the bitscrambler tx core EOF signal generating mode which is combined with reg_bitscrambler_tx_tailing_bits, 0: counter by read dma fifo, 0 counter by write peripheral buffer"]
    #[inline(always)]
    #[must_use]
    pub fn tx_eof_mode(&mut self) -> TX_EOF_MODE_W<TX_CTRL_SPEC> {
        TX_EOF_MODE_W::new(self, 3)
    }
    #[doc = "Bit 4 - write this bit to specify the LOOP instruction condition mode of bitscrambler tx core, 0: use the little than operator to get the condition, 1: use not equal operator to get the condition"]
    #[inline(always)]
    #[must_use]
    pub fn tx_cond_mode(&mut self) -> TX_COND_MODE_W<TX_CTRL_SPEC> {
        TX_COND_MODE_W::new(self, 4)
    }
    #[doc = "Bit 5 - write this bit to set the bitscrambler tx core fetch instruction mode, 0: prefetch by reset, 1: fetch by instrutions"]
    #[inline(always)]
    #[must_use]
    pub fn tx_fetch_mode(&mut self) -> TX_FETCH_MODE_W<TX_CTRL_SPEC> {
        TX_FETCH_MODE_W::new(self, 5)
    }
    #[doc = "Bit 6 - write this bit to set the bitscrambler tx core halt mode when tx_halt is set, 0: wait write data back done, , 1: ignore write data back"]
    #[inline(always)]
    #[must_use]
    pub fn tx_halt_mode(&mut self) -> TX_HALT_MODE_W<TX_CTRL_SPEC> {
        TX_HALT_MODE_W::new(self, 6)
    }
    #[doc = "Bit 7 - write this bit to set the bitscrambler tx core read data mode when EOF received.0: wait read data, 1: ignore read data"]
    #[inline(always)]
    #[must_use]
    pub fn tx_rd_dummy(&mut self) -> TX_RD_DUMMY_W<TX_CTRL_SPEC> {
        TX_RD_DUMMY_W::new(self, 7)
    }
    #[doc = "Bit 8 - write this bit to reset the bitscrambler tx fifo"]
    #[inline(always)]
    #[must_use]
    pub fn tx_fifo_rst(&mut self) -> TX_FIFO_RST_W<TX_CTRL_SPEC> {
        TX_FIFO_RST_W::new(self, 8)
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
#[doc = "Control and configuration registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TX_CTRL_SPEC;
impl crate::RegisterSpec for TX_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_ctrl::R`](R) reader structure"]
impl crate::Readable for TX_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tx_ctrl::W`](W) writer structure"]
impl crate::Writable for TX_CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TX_CTRL to value 0x04"]
impl crate::Resettable for TX_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x04;
}
