#[doc = "Register `CONF` reader"]
pub type R = crate::R<CONF_SPEC>;
#[doc = "Register `CONF` writer"]
pub type W = crate::W<CONF_SPEC>;
#[doc = "Field `IN_RST` reader - This bit is used to reset crypto DMA in FSM and RX FIFO pointer."]
pub type IN_RST_R = crate::BitReader;
#[doc = "Field `IN_RST` writer - This bit is used to reset crypto DMA in FSM and RX FIFO pointer."]
pub type IN_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_RST` reader - This bit is used to reset crypto DMA out FSM and TX FIFO pointer."]
pub type OUT_RST_R = crate::BitReader;
#[doc = "Field `OUT_RST` writer - This bit is used to reset crypto DMA out FSM and TX FIFO pointer."]
pub type OUT_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AHBM_FIFO_RST` reader - This bit is used to reset crypto DMA AHB master FIFO pointer."]
pub type AHBM_FIFO_RST_R = crate::BitReader;
#[doc = "Field `AHBM_FIFO_RST` writer - This bit is used to reset crypto DMA AHB master FIFO pointer."]
pub type AHBM_FIFO_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AHBM_RST` reader - Reset crypto DMA AHB master."]
pub type AHBM_RST_R = crate::BitReader;
#[doc = "Field `AHBM_RST` writer - Reset crypto DMA AHB master."]
pub type AHBM_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_LOOP_TEST` reader - Reserved"]
pub type IN_LOOP_TEST_R = crate::BitReader;
#[doc = "Field `IN_LOOP_TEST` writer - Reserved"]
pub type IN_LOOP_TEST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_LOOP_TEST` reader - Reserved"]
pub type OUT_LOOP_TEST_R = crate::BitReader;
#[doc = "Field `OUT_LOOP_TEST` writer - Reserved"]
pub type OUT_LOOP_TEST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_AUTO_WRBACK` reader - Set this bit to enable automatic outlink-writeback when all the data in TX Buffer has been transmitted."]
pub type OUT_AUTO_WRBACK_R = crate::BitReader;
#[doc = "Field `OUT_AUTO_WRBACK` writer - Set this bit to enable automatic outlink-writeback when all the data in TX Buffer has been transmitted."]
pub type OUT_AUTO_WRBACK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_NO_RESTART_CLR` reader - Reserved"]
pub type OUT_NO_RESTART_CLR_R = crate::BitReader;
#[doc = "Field `OUT_NO_RESTART_CLR` writer - Reserved"]
pub type OUT_NO_RESTART_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Out EOF flag generation mode of TX FIFO. 1: EOF flag of TX is generated when the last data with EOF would be transmitted has been popped from FIFO of Crypto DMA; 0: EOF flag is generated when the last data with EOF would be transmitted has been pushed into FIFO of Crypto DMA.\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OUT_EOF_MODE {
    #[doc = "0: EOF flag is generated when the last data with EOF would be transmitted has been pushed into FIFO of Crypto DMA"]
    OnPush = 0,
    #[doc = "1: EOF flag is generated when the last data with EOF would be transmitted has been popped from FIFO of Crypto DMA"]
    OnPop = 1,
}
impl From<OUT_EOF_MODE> for bool {
    #[inline(always)]
    fn from(variant: OUT_EOF_MODE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OUT_EOF_MODE` reader - Out EOF flag generation mode of TX FIFO. 1: EOF flag of TX is generated when the last data with EOF would be transmitted has been popped from FIFO of Crypto DMA; 0: EOF flag is generated when the last data with EOF would be transmitted has been pushed into FIFO of Crypto DMA."]
pub type OUT_EOF_MODE_R = crate::BitReader<OUT_EOF_MODE>;
impl OUT_EOF_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OUT_EOF_MODE {
        match self.bits {
            false => OUT_EOF_MODE::OnPush,
            true => OUT_EOF_MODE::OnPop,
        }
    }
    #[doc = "EOF flag is generated when the last data with EOF would be transmitted has been pushed into FIFO of Crypto DMA"]
    #[inline(always)]
    pub fn is_on_push(&self) -> bool {
        *self == OUT_EOF_MODE::OnPush
    }
    #[doc = "EOF flag is generated when the last data with EOF would be transmitted has been popped from FIFO of Crypto DMA"]
    #[inline(always)]
    pub fn is_on_pop(&self) -> bool {
        *self == OUT_EOF_MODE::OnPop
    }
}
#[doc = "Field `OUT_EOF_MODE` writer - Out EOF flag generation mode of TX FIFO. 1: EOF flag of TX is generated when the last data with EOF would be transmitted has been popped from FIFO of Crypto DMA; 0: EOF flag is generated when the last data with EOF would be transmitted has been pushed into FIFO of Crypto DMA."]
pub type OUT_EOF_MODE_W<'a, REG> = crate::BitWriter<'a, REG, OUT_EOF_MODE>;
impl<'a, REG> OUT_EOF_MODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "EOF flag is generated when the last data with EOF would be transmitted has been pushed into FIFO of Crypto DMA"]
    #[inline(always)]
    pub fn on_push(self) -> &'a mut crate::W<REG> {
        self.variant(OUT_EOF_MODE::OnPush)
    }
    #[doc = "EOF flag is generated when the last data with EOF would be transmitted has been popped from FIFO of Crypto DMA"]
    #[inline(always)]
    pub fn on_pop(self) -> &'a mut crate::W<REG> {
        self.variant(OUT_EOF_MODE::OnPop)
    }
}
#[doc = "Field `OUTDSCR_BURST_EN` reader - Set this bit to enable INCR burst transfer when TX FIFO reads descriptor from internal RAM."]
pub type OUTDSCR_BURST_EN_R = crate::BitReader;
#[doc = "Field `OUTDSCR_BURST_EN` writer - Set this bit to enable INCR burst transfer when TX FIFO reads descriptor from internal RAM."]
pub type OUTDSCR_BURST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INDSCR_BURST_EN` reader - Set this bit to enable INCR burst transfer when RX FIFO reads descriptor from internal RAM."]
pub type INDSCR_BURST_EN_R = crate::BitReader;
#[doc = "Field `INDSCR_BURST_EN` writer - Set this bit to enable INCR burst transfer when RX FIFO reads descriptor from internal RAM."]
pub type INDSCR_BURST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_DATA_BURST_EN` reader - Set this bit to enable INCR burst transfer when TX FIFO reads data from internal RAM."]
pub type OUT_DATA_BURST_EN_R = crate::BitReader;
#[doc = "Field `OUT_DATA_BURST_EN` writer - Set this bit to enable INCR burst transfer when TX FIFO reads data from internal RAM."]
pub type OUT_DATA_BURST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEM_TRANS_EN` reader - Set this bit to enable automatic transmitting data from memory to memory via DMA."]
pub type MEM_TRANS_EN_R = crate::BitReader;
#[doc = "Field `MEM_TRANS_EN` writer - Set this bit to enable automatic transmitting data from memory to memory via DMA."]
pub type MEM_TRANS_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - This bit is used to reset crypto DMA in FSM and RX FIFO pointer."]
    #[inline(always)]
    pub fn in_rst(&self) -> IN_RST_R {
        IN_RST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This bit is used to reset crypto DMA out FSM and TX FIFO pointer."]
    #[inline(always)]
    pub fn out_rst(&self) -> OUT_RST_R {
        OUT_RST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - This bit is used to reset crypto DMA AHB master FIFO pointer."]
    #[inline(always)]
    pub fn ahbm_fifo_rst(&self) -> AHBM_FIFO_RST_R {
        AHBM_FIFO_RST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Reset crypto DMA AHB master."]
    #[inline(always)]
    pub fn ahbm_rst(&self) -> AHBM_RST_R {
        AHBM_RST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Reserved"]
    #[inline(always)]
    pub fn in_loop_test(&self) -> IN_LOOP_TEST_R {
        IN_LOOP_TEST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Reserved"]
    #[inline(always)]
    pub fn out_loop_test(&self) -> OUT_LOOP_TEST_R {
        OUT_LOOP_TEST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Set this bit to enable automatic outlink-writeback when all the data in TX Buffer has been transmitted."]
    #[inline(always)]
    pub fn out_auto_wrback(&self) -> OUT_AUTO_WRBACK_R {
        OUT_AUTO_WRBACK_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Reserved"]
    #[inline(always)]
    pub fn out_no_restart_clr(&self) -> OUT_NO_RESTART_CLR_R {
        OUT_NO_RESTART_CLR_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Out EOF flag generation mode of TX FIFO. 1: EOF flag of TX is generated when the last data with EOF would be transmitted has been popped from FIFO of Crypto DMA; 0: EOF flag is generated when the last data with EOF would be transmitted has been pushed into FIFO of Crypto DMA."]
    #[inline(always)]
    pub fn out_eof_mode(&self) -> OUT_EOF_MODE_R {
        OUT_EOF_MODE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Set this bit to enable INCR burst transfer when TX FIFO reads descriptor from internal RAM."]
    #[inline(always)]
    pub fn outdscr_burst_en(&self) -> OUTDSCR_BURST_EN_R {
        OUTDSCR_BURST_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Set this bit to enable INCR burst transfer when RX FIFO reads descriptor from internal RAM."]
    #[inline(always)]
    pub fn indscr_burst_en(&self) -> INDSCR_BURST_EN_R {
        INDSCR_BURST_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Set this bit to enable INCR burst transfer when TX FIFO reads data from internal RAM."]
    #[inline(always)]
    pub fn out_data_burst_en(&self) -> OUT_DATA_BURST_EN_R {
        OUT_DATA_BURST_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Set this bit to enable automatic transmitting data from memory to memory via DMA."]
    #[inline(always)]
    pub fn mem_trans_en(&self) -> MEM_TRANS_EN_R {
        MEM_TRANS_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONF")
            .field("in_rst", &self.in_rst())
            .field("out_rst", &self.out_rst())
            .field("ahbm_fifo_rst", &self.ahbm_fifo_rst())
            .field("ahbm_rst", &self.ahbm_rst())
            .field("in_loop_test", &self.in_loop_test())
            .field("out_loop_test", &self.out_loop_test())
            .field("out_auto_wrback", &self.out_auto_wrback())
            .field("out_no_restart_clr", &self.out_no_restart_clr())
            .field("out_eof_mode", &self.out_eof_mode())
            .field("outdscr_burst_en", &self.outdscr_burst_en())
            .field("indscr_burst_en", &self.indscr_burst_en())
            .field("out_data_burst_en", &self.out_data_burst_en())
            .field("mem_trans_en", &self.mem_trans_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - This bit is used to reset crypto DMA in FSM and RX FIFO pointer."]
    #[inline(always)]
    pub fn in_rst(&mut self) -> IN_RST_W<'_, CONF_SPEC> {
        IN_RST_W::new(self, 0)
    }
    #[doc = "Bit 1 - This bit is used to reset crypto DMA out FSM and TX FIFO pointer."]
    #[inline(always)]
    pub fn out_rst(&mut self) -> OUT_RST_W<'_, CONF_SPEC> {
        OUT_RST_W::new(self, 1)
    }
    #[doc = "Bit 2 - This bit is used to reset crypto DMA AHB master FIFO pointer."]
    #[inline(always)]
    pub fn ahbm_fifo_rst(&mut self) -> AHBM_FIFO_RST_W<'_, CONF_SPEC> {
        AHBM_FIFO_RST_W::new(self, 2)
    }
    #[doc = "Bit 3 - Reset crypto DMA AHB master."]
    #[inline(always)]
    pub fn ahbm_rst(&mut self) -> AHBM_RST_W<'_, CONF_SPEC> {
        AHBM_RST_W::new(self, 3)
    }
    #[doc = "Bit 4 - Reserved"]
    #[inline(always)]
    pub fn in_loop_test(&mut self) -> IN_LOOP_TEST_W<'_, CONF_SPEC> {
        IN_LOOP_TEST_W::new(self, 4)
    }
    #[doc = "Bit 5 - Reserved"]
    #[inline(always)]
    pub fn out_loop_test(&mut self) -> OUT_LOOP_TEST_W<'_, CONF_SPEC> {
        OUT_LOOP_TEST_W::new(self, 5)
    }
    #[doc = "Bit 6 - Set this bit to enable automatic outlink-writeback when all the data in TX Buffer has been transmitted."]
    #[inline(always)]
    pub fn out_auto_wrback(&mut self) -> OUT_AUTO_WRBACK_W<'_, CONF_SPEC> {
        OUT_AUTO_WRBACK_W::new(self, 6)
    }
    #[doc = "Bit 7 - Reserved"]
    #[inline(always)]
    pub fn out_no_restart_clr(&mut self) -> OUT_NO_RESTART_CLR_W<'_, CONF_SPEC> {
        OUT_NO_RESTART_CLR_W::new(self, 7)
    }
    #[doc = "Bit 8 - Out EOF flag generation mode of TX FIFO. 1: EOF flag of TX is generated when the last data with EOF would be transmitted has been popped from FIFO of Crypto DMA; 0: EOF flag is generated when the last data with EOF would be transmitted has been pushed into FIFO of Crypto DMA."]
    #[inline(always)]
    pub fn out_eof_mode(&mut self) -> OUT_EOF_MODE_W<'_, CONF_SPEC> {
        OUT_EOF_MODE_W::new(self, 8)
    }
    #[doc = "Bit 9 - Set this bit to enable INCR burst transfer when TX FIFO reads descriptor from internal RAM."]
    #[inline(always)]
    pub fn outdscr_burst_en(&mut self) -> OUTDSCR_BURST_EN_W<'_, CONF_SPEC> {
        OUTDSCR_BURST_EN_W::new(self, 9)
    }
    #[doc = "Bit 10 - Set this bit to enable INCR burst transfer when RX FIFO reads descriptor from internal RAM."]
    #[inline(always)]
    pub fn indscr_burst_en(&mut self) -> INDSCR_BURST_EN_W<'_, CONF_SPEC> {
        INDSCR_BURST_EN_W::new(self, 10)
    }
    #[doc = "Bit 11 - Set this bit to enable INCR burst transfer when TX FIFO reads data from internal RAM."]
    #[inline(always)]
    pub fn out_data_burst_en(&mut self) -> OUT_DATA_BURST_EN_W<'_, CONF_SPEC> {
        OUT_DATA_BURST_EN_W::new(self, 11)
    }
    #[doc = "Bit 12 - Set this bit to enable automatic transmitting data from memory to memory via DMA."]
    #[inline(always)]
    pub fn mem_trans_en(&mut self) -> MEM_TRANS_EN_W<'_, CONF_SPEC> {
        MEM_TRANS_EN_W::new(self, 12)
    }
}
#[doc = "DMA configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CONF_SPEC;
impl crate::RegisterSpec for CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`conf::R`](R) reader structure"]
impl crate::Readable for CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`conf::W`](W) writer structure"]
impl crate::Writable for CONF_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CONF to value 0x0100"]
impl crate::Resettable for CONF_SPEC {
    const RESET_VALUE: u32 = 0x0100;
}
