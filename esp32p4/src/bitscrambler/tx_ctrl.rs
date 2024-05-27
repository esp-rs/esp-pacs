///Register `TX_CTRL` reader
pub type R = crate::R<TX_CTRL_SPEC>;
///Register `TX_CTRL` writer
pub type W = crate::W<TX_CTRL_SPEC>;
///Field `TX_ENA` reader - write this bit to enable the bitscrambler tx
pub type TX_ENA_R = crate::BitReader;
///Field `TX_ENA` writer - write this bit to enable the bitscrambler tx
pub type TX_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TX_PAUSE` reader - write this bit to pause the bitscrambler tx core
pub type TX_PAUSE_R = crate::BitReader;
///Field `TX_PAUSE` writer - write this bit to pause the bitscrambler tx core
pub type TX_PAUSE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TX_HALT` reader - write this bit to halt the bitscrambler tx core
pub type TX_HALT_R = crate::BitReader;
///Field `TX_HALT` writer - write this bit to halt the bitscrambler tx core
pub type TX_HALT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TX_EOF_MODE` reader - write this bit to ser the bitscrambler tx core EOF signal generating mode which is combined with reg_bitscrambler_tx_tailing_bits, 0: counter by read dma fifo, 0 counter by write peripheral buffer
pub type TX_EOF_MODE_R = crate::BitReader;
///Field `TX_EOF_MODE` writer - write this bit to ser the bitscrambler tx core EOF signal generating mode which is combined with reg_bitscrambler_tx_tailing_bits, 0: counter by read dma fifo, 0 counter by write peripheral buffer
pub type TX_EOF_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TX_COND_MODE` reader - write this bit to specify the LOOP instruction condition mode of bitscrambler tx core, 0: use the little than operator to get the condition, 1: use not equal operator to get the condition
pub type TX_COND_MODE_R = crate::BitReader;
///Field `TX_COND_MODE` writer - write this bit to specify the LOOP instruction condition mode of bitscrambler tx core, 0: use the little than operator to get the condition, 1: use not equal operator to get the condition
pub type TX_COND_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TX_FETCH_MODE` reader - write this bit to set the bitscrambler tx core fetch instruction mode, 0: prefetch by reset, 1: fetch by instrutions
pub type TX_FETCH_MODE_R = crate::BitReader;
///Field `TX_FETCH_MODE` writer - write this bit to set the bitscrambler tx core fetch instruction mode, 0: prefetch by reset, 1: fetch by instrutions
pub type TX_FETCH_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TX_HALT_MODE` reader - write this bit to set the bitscrambler tx core halt mode when tx_halt is set, 0: wait write data back done, , 1: ignore write data back
pub type TX_HALT_MODE_R = crate::BitReader;
///Field `TX_HALT_MODE` writer - write this bit to set the bitscrambler tx core halt mode when tx_halt is set, 0: wait write data back done, , 1: ignore write data back
pub type TX_HALT_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TX_RD_DUMMY` reader - write this bit to set the bitscrambler tx core read data mode when EOF received.0: wait read data, 1: ignore read data
pub type TX_RD_DUMMY_R = crate::BitReader;
///Field `TX_RD_DUMMY` writer - write this bit to set the bitscrambler tx core read data mode when EOF received.0: wait read data, 1: ignore read data
pub type TX_RD_DUMMY_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TX_FIFO_RST` writer - write this bit to reset the bitscrambler tx fifo
pub type TX_FIFO_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - write this bit to enable the bitscrambler tx
    #[inline(always)]
    pub fn tx_ena(&self) -> TX_ENA_R {
        TX_ENA_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - write this bit to pause the bitscrambler tx core
    #[inline(always)]
    pub fn tx_pause(&self) -> TX_PAUSE_R {
        TX_PAUSE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - write this bit to halt the bitscrambler tx core
    #[inline(always)]
    pub fn tx_halt(&self) -> TX_HALT_R {
        TX_HALT_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - write this bit to ser the bitscrambler tx core EOF signal generating mode which is combined with reg_bitscrambler_tx_tailing_bits, 0: counter by read dma fifo, 0 counter by write peripheral buffer
    #[inline(always)]
    pub fn tx_eof_mode(&self) -> TX_EOF_MODE_R {
        TX_EOF_MODE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - write this bit to specify the LOOP instruction condition mode of bitscrambler tx core, 0: use the little than operator to get the condition, 1: use not equal operator to get the condition
    #[inline(always)]
    pub fn tx_cond_mode(&self) -> TX_COND_MODE_R {
        TX_COND_MODE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - write this bit to set the bitscrambler tx core fetch instruction mode, 0: prefetch by reset, 1: fetch by instrutions
    #[inline(always)]
    pub fn tx_fetch_mode(&self) -> TX_FETCH_MODE_R {
        TX_FETCH_MODE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - write this bit to set the bitscrambler tx core halt mode when tx_halt is set, 0: wait write data back done, , 1: ignore write data back
    #[inline(always)]
    pub fn tx_halt_mode(&self) -> TX_HALT_MODE_R {
        TX_HALT_MODE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - write this bit to set the bitscrambler tx core read data mode when EOF received.0: wait read data, 1: ignore read data
    #[inline(always)]
    pub fn tx_rd_dummy(&self) -> TX_RD_DUMMY_R {
        TX_RD_DUMMY_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TX_CTRL")
            .field("tx_ena", &self.tx_ena())
            .field("tx_pause", &self.tx_pause())
            .field("tx_halt", &self.tx_halt())
            .field("tx_eof_mode", &self.tx_eof_mode())
            .field("tx_cond_mode", &self.tx_cond_mode())
            .field("tx_fetch_mode", &self.tx_fetch_mode())
            .field("tx_halt_mode", &self.tx_halt_mode())
            .field("tx_rd_dummy", &self.tx_rd_dummy())
            .finish()
    }
}
impl W {
    ///Bit 0 - write this bit to enable the bitscrambler tx
    #[inline(always)]
    #[must_use]
    pub fn tx_ena(&mut self) -> TX_ENA_W<TX_CTRL_SPEC> {
        TX_ENA_W::new(self, 0)
    }
    ///Bit 1 - write this bit to pause the bitscrambler tx core
    #[inline(always)]
    #[must_use]
    pub fn tx_pause(&mut self) -> TX_PAUSE_W<TX_CTRL_SPEC> {
        TX_PAUSE_W::new(self, 1)
    }
    ///Bit 2 - write this bit to halt the bitscrambler tx core
    #[inline(always)]
    #[must_use]
    pub fn tx_halt(&mut self) -> TX_HALT_W<TX_CTRL_SPEC> {
        TX_HALT_W::new(self, 2)
    }
    ///Bit 3 - write this bit to ser the bitscrambler tx core EOF signal generating mode which is combined with reg_bitscrambler_tx_tailing_bits, 0: counter by read dma fifo, 0 counter by write peripheral buffer
    #[inline(always)]
    #[must_use]
    pub fn tx_eof_mode(&mut self) -> TX_EOF_MODE_W<TX_CTRL_SPEC> {
        TX_EOF_MODE_W::new(self, 3)
    }
    ///Bit 4 - write this bit to specify the LOOP instruction condition mode of bitscrambler tx core, 0: use the little than operator to get the condition, 1: use not equal operator to get the condition
    #[inline(always)]
    #[must_use]
    pub fn tx_cond_mode(&mut self) -> TX_COND_MODE_W<TX_CTRL_SPEC> {
        TX_COND_MODE_W::new(self, 4)
    }
    ///Bit 5 - write this bit to set the bitscrambler tx core fetch instruction mode, 0: prefetch by reset, 1: fetch by instrutions
    #[inline(always)]
    #[must_use]
    pub fn tx_fetch_mode(&mut self) -> TX_FETCH_MODE_W<TX_CTRL_SPEC> {
        TX_FETCH_MODE_W::new(self, 5)
    }
    ///Bit 6 - write this bit to set the bitscrambler tx core halt mode when tx_halt is set, 0: wait write data back done, , 1: ignore write data back
    #[inline(always)]
    #[must_use]
    pub fn tx_halt_mode(&mut self) -> TX_HALT_MODE_W<TX_CTRL_SPEC> {
        TX_HALT_MODE_W::new(self, 6)
    }
    ///Bit 7 - write this bit to set the bitscrambler tx core read data mode when EOF received.0: wait read data, 1: ignore read data
    #[inline(always)]
    #[must_use]
    pub fn tx_rd_dummy(&mut self) -> TX_RD_DUMMY_W<TX_CTRL_SPEC> {
        TX_RD_DUMMY_W::new(self, 7)
    }
    ///Bit 8 - write this bit to reset the bitscrambler tx fifo
    #[inline(always)]
    #[must_use]
    pub fn tx_fifo_rst(&mut self) -> TX_FIFO_RST_W<TX_CTRL_SPEC> {
        TX_FIFO_RST_W::new(self, 8)
    }
}
/**Control and configuration registers

You can [`read`](crate::generic::Reg::read) this register and get [`tx_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct TX_CTRL_SPEC;
impl crate::RegisterSpec for TX_CTRL_SPEC {
    type Ux = u32;
}
///`read()` method returns [`tx_ctrl::R`](R) reader structure
impl crate::Readable for TX_CTRL_SPEC {}
///`write(|w| ..)` method takes [`tx_ctrl::W`](W) writer structure
impl crate::Writable for TX_CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets TX_CTRL to value 0x04
impl crate::Resettable for TX_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x04;
}
