#[doc = "Register `IN_CONF0_CH%s` reader"]
pub type R = crate::R<IN_CONF0_CH_SPEC>;
#[doc = "Register `IN_CONF0_CH%s` writer"]
pub type W = crate::W<IN_CONF0_CH_SPEC>;
#[doc = "Field `IN_RST` reader - This bit is used to reset DMA channel 0 Rx FSM and Rx FIFO pointer."]
pub type IN_RST_R = crate::BitReader;
#[doc = "Field `IN_RST` writer - This bit is used to reset DMA channel 0 Rx FSM and Rx FIFO pointer."]
pub type IN_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_LOOP_TEST` reader - reserved"]
pub type IN_LOOP_TEST_R = crate::BitReader;
#[doc = "Field `IN_LOOP_TEST` writer - reserved"]
pub type IN_LOOP_TEST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INDSCR_BURST_EN` reader - Set this bit to 1 to enable INCR burst transfer for Rx channel 0 reading link descriptor when accessing internal SRAM."]
pub type INDSCR_BURST_EN_R = crate::BitReader;
#[doc = "Field `INDSCR_BURST_EN` writer - Set this bit to 1 to enable INCR burst transfer for Rx channel 0 reading link descriptor when accessing internal SRAM."]
pub type INDSCR_BURST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_DATA_BURST_EN` reader - Set this bit to 1 to enable INCR burst transfer for Rx channel 0 receiving data when accessing internal SRAM."]
pub type IN_DATA_BURST_EN_R = crate::BitReader;
#[doc = "Field `IN_DATA_BURST_EN` writer - Set this bit to 1 to enable INCR burst transfer for Rx channel 0 receiving data when accessing internal SRAM."]
pub type IN_DATA_BURST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEM_TRANS_EN` reader - Set this bit 1 to enable automatic transmitting data from memory to memory via DMA."]
pub type MEM_TRANS_EN_R = crate::BitReader;
#[doc = "Field `MEM_TRANS_EN` writer - Set this bit 1 to enable automatic transmitting data from memory to memory via DMA."]
pub type MEM_TRANS_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - This bit is used to reset DMA channel 0 Rx FSM and Rx FIFO pointer."]
    #[inline(always)]
    pub fn in_rst(&self) -> IN_RST_R {
        IN_RST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - reserved"]
    #[inline(always)]
    pub fn in_loop_test(&self) -> IN_LOOP_TEST_R {
        IN_LOOP_TEST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set this bit to 1 to enable INCR burst transfer for Rx channel 0 reading link descriptor when accessing internal SRAM."]
    #[inline(always)]
    pub fn indscr_burst_en(&self) -> INDSCR_BURST_EN_R {
        INDSCR_BURST_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Set this bit to 1 to enable INCR burst transfer for Rx channel 0 receiving data when accessing internal SRAM."]
    #[inline(always)]
    pub fn in_data_burst_en(&self) -> IN_DATA_BURST_EN_R {
        IN_DATA_BURST_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Set this bit 1 to enable automatic transmitting data from memory to memory via DMA."]
    #[inline(always)]
    pub fn mem_trans_en(&self) -> MEM_TRANS_EN_R {
        MEM_TRANS_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IN_CONF0_CH")
            .field("in_rst", &format_args!("{}", self.in_rst().bit()))
            .field(
                "in_loop_test",
                &format_args!("{}", self.in_loop_test().bit()),
            )
            .field(
                "indscr_burst_en",
                &format_args!("{}", self.indscr_burst_en().bit()),
            )
            .field(
                "in_data_burst_en",
                &format_args!("{}", self.in_data_burst_en().bit()),
            )
            .field(
                "mem_trans_en",
                &format_args!("{}", self.mem_trans_en().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<IN_CONF0_CH_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - This bit is used to reset DMA channel 0 Rx FSM and Rx FIFO pointer."]
    #[inline(always)]
    #[must_use]
    pub fn in_rst(&mut self) -> IN_RST_W<IN_CONF0_CH_SPEC> {
        IN_RST_W::new(self, 0)
    }
    #[doc = "Bit 1 - reserved"]
    #[inline(always)]
    #[must_use]
    pub fn in_loop_test(&mut self) -> IN_LOOP_TEST_W<IN_CONF0_CH_SPEC> {
        IN_LOOP_TEST_W::new(self, 1)
    }
    #[doc = "Bit 2 - Set this bit to 1 to enable INCR burst transfer for Rx channel 0 reading link descriptor when accessing internal SRAM."]
    #[inline(always)]
    #[must_use]
    pub fn indscr_burst_en(&mut self) -> INDSCR_BURST_EN_W<IN_CONF0_CH_SPEC> {
        INDSCR_BURST_EN_W::new(self, 2)
    }
    #[doc = "Bit 3 - Set this bit to 1 to enable INCR burst transfer for Rx channel 0 receiving data when accessing internal SRAM."]
    #[inline(always)]
    #[must_use]
    pub fn in_data_burst_en(&mut self) -> IN_DATA_BURST_EN_W<IN_CONF0_CH_SPEC> {
        IN_DATA_BURST_EN_W::new(self, 3)
    }
    #[doc = "Bit 4 - Set this bit 1 to enable automatic transmitting data from memory to memory via DMA."]
    #[inline(always)]
    #[must_use]
    pub fn mem_trans_en(&mut self) -> MEM_TRANS_EN_W<IN_CONF0_CH_SPEC> {
        MEM_TRANS_EN_W::new(self, 4)
    }
}
#[doc = "DMA_IN_CONF%s_CH%s_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_conf0_ch::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_conf0_ch::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IN_CONF0_CH_SPEC;
impl crate::RegisterSpec for IN_CONF0_CH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`in_conf0_ch::R`](R) reader structure"]
impl crate::Readable for IN_CONF0_CH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`in_conf0_ch::W`](W) writer structure"]
impl crate::Writable for IN_CONF0_CH_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IN_CONF0_CH%s to value 0"]
impl crate::Resettable for IN_CONF0_CH_SPEC {
    const RESET_VALUE: u32 = 0;
}
