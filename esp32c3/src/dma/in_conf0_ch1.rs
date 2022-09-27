#[doc = "Register `IN_CONF0_CH1` reader"]
pub struct R(crate::R<IN_CONF0_CH1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IN_CONF0_CH1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IN_CONF0_CH1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IN_CONF0_CH1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IN_CONF0_CH1` writer"]
pub struct W(crate::W<IN_CONF0_CH1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IN_CONF0_CH1_SPEC>;
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
impl From<crate::W<IN_CONF0_CH1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IN_CONF0_CH1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IN_RST_CH1` reader - This bit is used to reset DMA channel 1 Rx FSM and Rx FIFO pointer."]
pub type IN_RST_CH1_R = crate::BitReader<bool>;
#[doc = "Field `IN_RST_CH1` writer - This bit is used to reset DMA channel 1 Rx FSM and Rx FIFO pointer."]
pub type IN_RST_CH1_W<'a, const O: u8> = crate::BitWriter<'a, u32, IN_CONF0_CH1_SPEC, bool, O>;
#[doc = "Field `IN_LOOP_TEST_CH1` reader - reserved"]
pub type IN_LOOP_TEST_CH1_R = crate::BitReader<bool>;
#[doc = "Field `IN_LOOP_TEST_CH1` writer - reserved"]
pub type IN_LOOP_TEST_CH1_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, IN_CONF0_CH1_SPEC, bool, O>;
#[doc = "Field `INDSCR_BURST_EN_CH1` reader - Set this bit to 1 to enable INCR burst transfer for Rx channel 1 reading link descriptor when accessing internal SRAM."]
pub type INDSCR_BURST_EN_CH1_R = crate::BitReader<bool>;
#[doc = "Field `INDSCR_BURST_EN_CH1` writer - Set this bit to 1 to enable INCR burst transfer for Rx channel 1 reading link descriptor when accessing internal SRAM."]
pub type INDSCR_BURST_EN_CH1_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, IN_CONF0_CH1_SPEC, bool, O>;
#[doc = "Field `IN_DATA_BURST_EN_CH1` reader - Set this bit to 1 to enable INCR burst transfer for Rx channel 1 receiving data when accessing internal SRAM."]
pub type IN_DATA_BURST_EN_CH1_R = crate::BitReader<bool>;
#[doc = "Field `IN_DATA_BURST_EN_CH1` writer - Set this bit to 1 to enable INCR burst transfer for Rx channel 1 receiving data when accessing internal SRAM."]
pub type IN_DATA_BURST_EN_CH1_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, IN_CONF0_CH1_SPEC, bool, O>;
#[doc = "Field `MEM_TRANS_EN_CH1` reader - Set this bit 1 to enable automatic transmitting data from memory to memory via DMA."]
pub type MEM_TRANS_EN_CH1_R = crate::BitReader<bool>;
#[doc = "Field `MEM_TRANS_EN_CH1` writer - Set this bit 1 to enable automatic transmitting data from memory to memory via DMA."]
pub type MEM_TRANS_EN_CH1_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, IN_CONF0_CH1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - This bit is used to reset DMA channel 1 Rx FSM and Rx FIFO pointer."]
    #[inline(always)]
    pub fn in_rst_ch1(&self) -> IN_RST_CH1_R {
        IN_RST_CH1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - reserved"]
    #[inline(always)]
    pub fn in_loop_test_ch1(&self) -> IN_LOOP_TEST_CH1_R {
        IN_LOOP_TEST_CH1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set this bit to 1 to enable INCR burst transfer for Rx channel 1 reading link descriptor when accessing internal SRAM."]
    #[inline(always)]
    pub fn indscr_burst_en_ch1(&self) -> INDSCR_BURST_EN_CH1_R {
        INDSCR_BURST_EN_CH1_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Set this bit to 1 to enable INCR burst transfer for Rx channel 1 receiving data when accessing internal SRAM."]
    #[inline(always)]
    pub fn in_data_burst_en_ch1(&self) -> IN_DATA_BURST_EN_CH1_R {
        IN_DATA_BURST_EN_CH1_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Set this bit 1 to enable automatic transmitting data from memory to memory via DMA."]
    #[inline(always)]
    pub fn mem_trans_en_ch1(&self) -> MEM_TRANS_EN_CH1_R {
        MEM_TRANS_EN_CH1_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This bit is used to reset DMA channel 1 Rx FSM and Rx FIFO pointer."]
    #[inline(always)]
    pub fn in_rst_ch1(&mut self) -> IN_RST_CH1_W<0> {
        IN_RST_CH1_W::new(self)
    }
    #[doc = "Bit 1 - reserved"]
    #[inline(always)]
    pub fn in_loop_test_ch1(&mut self) -> IN_LOOP_TEST_CH1_W<1> {
        IN_LOOP_TEST_CH1_W::new(self)
    }
    #[doc = "Bit 2 - Set this bit to 1 to enable INCR burst transfer for Rx channel 1 reading link descriptor when accessing internal SRAM."]
    #[inline(always)]
    pub fn indscr_burst_en_ch1(&mut self) -> INDSCR_BURST_EN_CH1_W<2> {
        INDSCR_BURST_EN_CH1_W::new(self)
    }
    #[doc = "Bit 3 - Set this bit to 1 to enable INCR burst transfer for Rx channel 1 receiving data when accessing internal SRAM."]
    #[inline(always)]
    pub fn in_data_burst_en_ch1(&mut self) -> IN_DATA_BURST_EN_CH1_W<3> {
        IN_DATA_BURST_EN_CH1_W::new(self)
    }
    #[doc = "Bit 4 - Set this bit 1 to enable automatic transmitting data from memory to memory via DMA."]
    #[inline(always)]
    pub fn mem_trans_en_ch1(&mut self) -> MEM_TRANS_EN_CH1_W<4> {
        MEM_TRANS_EN_CH1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA_IN_CONF0_CH1_REG.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [in_conf0_ch1](index.html) module"]
pub struct IN_CONF0_CH1_SPEC;
impl crate::RegisterSpec for IN_CONF0_CH1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [in_conf0_ch1::R](R) reader structure"]
impl crate::Readable for IN_CONF0_CH1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [in_conf0_ch1::W](W) writer structure"]
impl crate::Writable for IN_CONF0_CH1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IN_CONF0_CH1 to value 0"]
impl crate::Resettable for IN_CONF0_CH1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
