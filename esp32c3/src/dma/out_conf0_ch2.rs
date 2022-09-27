#[doc = "Register `OUT_CONF0_CH2` reader"]
pub struct R(crate::R<OUT_CONF0_CH2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OUT_CONF0_CH2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OUT_CONF0_CH2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OUT_CONF0_CH2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OUT_CONF0_CH2` writer"]
pub struct W(crate::W<OUT_CONF0_CH2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OUT_CONF0_CH2_SPEC>;
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
impl From<crate::W<OUT_CONF0_CH2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OUT_CONF0_CH2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OUT_RST_CH2` reader - This bit is used to reset DMA channel 2 Tx FSM and Tx FIFO pointer."]
pub type OUT_RST_CH2_R = crate::BitReader<bool>;
#[doc = "Field `OUT_RST_CH2` writer - This bit is used to reset DMA channel 2 Tx FSM and Tx FIFO pointer."]
pub type OUT_RST_CH2_W<'a, const O: u8> = crate::BitWriter<'a, u32, OUT_CONF0_CH2_SPEC, bool, O>;
#[doc = "Field `OUT_LOOP_TEST_CH2` reader - reserved"]
pub type OUT_LOOP_TEST_CH2_R = crate::BitReader<bool>;
#[doc = "Field `OUT_LOOP_TEST_CH2` writer - reserved"]
pub type OUT_LOOP_TEST_CH2_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, OUT_CONF0_CH2_SPEC, bool, O>;
#[doc = "Field `OUT_AUTO_WRBACK_CH2` reader - Set this bit to enable automatic outlink-writeback when all the data in tx buffer has been transmitted."]
pub type OUT_AUTO_WRBACK_CH2_R = crate::BitReader<bool>;
#[doc = "Field `OUT_AUTO_WRBACK_CH2` writer - Set this bit to enable automatic outlink-writeback when all the data in tx buffer has been transmitted."]
pub type OUT_AUTO_WRBACK_CH2_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, OUT_CONF0_CH2_SPEC, bool, O>;
#[doc = "Field `OUT_EOF_MODE_CH2` reader - EOF flag generation mode when transmitting data. 1: EOF flag for Tx channel 2 is generated when data need to transmit has been popped from FIFO in DMA"]
pub type OUT_EOF_MODE_CH2_R = crate::BitReader<bool>;
#[doc = "Field `OUT_EOF_MODE_CH2` writer - EOF flag generation mode when transmitting data. 1: EOF flag for Tx channel 2 is generated when data need to transmit has been popped from FIFO in DMA"]
pub type OUT_EOF_MODE_CH2_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, OUT_CONF0_CH2_SPEC, bool, O>;
#[doc = "Field `OUTDSCR_BURST_EN_CH2` reader - Set this bit to 1 to enable INCR burst transfer for Tx channel 2 reading link descriptor when accessing internal SRAM."]
pub type OUTDSCR_BURST_EN_CH2_R = crate::BitReader<bool>;
#[doc = "Field `OUTDSCR_BURST_EN_CH2` writer - Set this bit to 1 to enable INCR burst transfer for Tx channel 2 reading link descriptor when accessing internal SRAM."]
pub type OUTDSCR_BURST_EN_CH2_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, OUT_CONF0_CH2_SPEC, bool, O>;
#[doc = "Field `OUT_DATA_BURST_EN_CH2` reader - Set this bit to 1 to enable INCR burst transfer for Tx channel 2 transmitting data when accessing internal SRAM."]
pub type OUT_DATA_BURST_EN_CH2_R = crate::BitReader<bool>;
#[doc = "Field `OUT_DATA_BURST_EN_CH2` writer - Set this bit to 1 to enable INCR burst transfer for Tx channel 2 transmitting data when accessing internal SRAM."]
pub type OUT_DATA_BURST_EN_CH2_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, OUT_CONF0_CH2_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - This bit is used to reset DMA channel 2 Tx FSM and Tx FIFO pointer."]
    #[inline(always)]
    pub fn out_rst_ch2(&self) -> OUT_RST_CH2_R {
        OUT_RST_CH2_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - reserved"]
    #[inline(always)]
    pub fn out_loop_test_ch2(&self) -> OUT_LOOP_TEST_CH2_R {
        OUT_LOOP_TEST_CH2_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set this bit to enable automatic outlink-writeback when all the data in tx buffer has been transmitted."]
    #[inline(always)]
    pub fn out_auto_wrback_ch2(&self) -> OUT_AUTO_WRBACK_CH2_R {
        OUT_AUTO_WRBACK_CH2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - EOF flag generation mode when transmitting data. 1: EOF flag for Tx channel 2 is generated when data need to transmit has been popped from FIFO in DMA"]
    #[inline(always)]
    pub fn out_eof_mode_ch2(&self) -> OUT_EOF_MODE_CH2_R {
        OUT_EOF_MODE_CH2_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Set this bit to 1 to enable INCR burst transfer for Tx channel 2 reading link descriptor when accessing internal SRAM."]
    #[inline(always)]
    pub fn outdscr_burst_en_ch2(&self) -> OUTDSCR_BURST_EN_CH2_R {
        OUTDSCR_BURST_EN_CH2_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Set this bit to 1 to enable INCR burst transfer for Tx channel 2 transmitting data when accessing internal SRAM."]
    #[inline(always)]
    pub fn out_data_burst_en_ch2(&self) -> OUT_DATA_BURST_EN_CH2_R {
        OUT_DATA_BURST_EN_CH2_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This bit is used to reset DMA channel 2 Tx FSM and Tx FIFO pointer."]
    #[inline(always)]
    pub fn out_rst_ch2(&mut self) -> OUT_RST_CH2_W<0> {
        OUT_RST_CH2_W::new(self)
    }
    #[doc = "Bit 1 - reserved"]
    #[inline(always)]
    pub fn out_loop_test_ch2(&mut self) -> OUT_LOOP_TEST_CH2_W<1> {
        OUT_LOOP_TEST_CH2_W::new(self)
    }
    #[doc = "Bit 2 - Set this bit to enable automatic outlink-writeback when all the data in tx buffer has been transmitted."]
    #[inline(always)]
    pub fn out_auto_wrback_ch2(&mut self) -> OUT_AUTO_WRBACK_CH2_W<2> {
        OUT_AUTO_WRBACK_CH2_W::new(self)
    }
    #[doc = "Bit 3 - EOF flag generation mode when transmitting data. 1: EOF flag for Tx channel 2 is generated when data need to transmit has been popped from FIFO in DMA"]
    #[inline(always)]
    pub fn out_eof_mode_ch2(&mut self) -> OUT_EOF_MODE_CH2_W<3> {
        OUT_EOF_MODE_CH2_W::new(self)
    }
    #[doc = "Bit 4 - Set this bit to 1 to enable INCR burst transfer for Tx channel 2 reading link descriptor when accessing internal SRAM."]
    #[inline(always)]
    pub fn outdscr_burst_en_ch2(&mut self) -> OUTDSCR_BURST_EN_CH2_W<4> {
        OUTDSCR_BURST_EN_CH2_W::new(self)
    }
    #[doc = "Bit 5 - Set this bit to 1 to enable INCR burst transfer for Tx channel 2 transmitting data when accessing internal SRAM."]
    #[inline(always)]
    pub fn out_data_burst_en_ch2(&mut self) -> OUT_DATA_BURST_EN_CH2_W<5> {
        OUT_DATA_BURST_EN_CH2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA_OUT_CONF0_CH2_REG.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [out_conf0_ch2](index.html) module"]
pub struct OUT_CONF0_CH2_SPEC;
impl crate::RegisterSpec for OUT_CONF0_CH2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [out_conf0_ch2::R](R) reader structure"]
impl crate::Readable for OUT_CONF0_CH2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [out_conf0_ch2::W](W) writer structure"]
impl crate::Writable for OUT_CONF0_CH2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OUT_CONF0_CH2 to value 0x08"]
impl crate::Resettable for OUT_CONF0_CH2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x08
    }
}
