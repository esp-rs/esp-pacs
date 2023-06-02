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
#[doc = "Field `IN_RST` reader - This bit is used to reset DMA channel 1 Rx FSM and Rx FIFO pointer."]
pub type IN_RST_R = crate::BitReader;
#[doc = "Field `IN_RST` writer - This bit is used to reset DMA channel 1 Rx FSM and Rx FIFO pointer."]
pub type IN_RST_W<'a, const O: u8> = crate::BitWriter<'a, IN_CONF0_CH1_SPEC, O>;
#[doc = "Field `IN_LOOP_TEST` reader - reserved"]
pub type IN_LOOP_TEST_R = crate::BitReader;
#[doc = "Field `IN_LOOP_TEST` writer - reserved"]
pub type IN_LOOP_TEST_W<'a, const O: u8> = crate::BitWriter<'a, IN_CONF0_CH1_SPEC, O>;
#[doc = "Field `INDSCR_BURST_EN` reader - Set this bit to 1 to enable INCR burst transfer for Rx channel 1 reading link descriptor when accessing internal SRAM."]
pub type INDSCR_BURST_EN_R = crate::BitReader;
#[doc = "Field `INDSCR_BURST_EN` writer - Set this bit to 1 to enable INCR burst transfer for Rx channel 1 reading link descriptor when accessing internal SRAM."]
pub type INDSCR_BURST_EN_W<'a, const O: u8> = crate::BitWriter<'a, IN_CONF0_CH1_SPEC, O>;
#[doc = "Field `IN_DATA_BURST_EN` reader - Set this bit to 1 to enable INCR burst transfer for Rx channel 1 receiving data when accessing internal SRAM."]
pub type IN_DATA_BURST_EN_R = crate::BitReader;
#[doc = "Field `IN_DATA_BURST_EN` writer - Set this bit to 1 to enable INCR burst transfer for Rx channel 1 receiving data when accessing internal SRAM."]
pub type IN_DATA_BURST_EN_W<'a, const O: u8> = crate::BitWriter<'a, IN_CONF0_CH1_SPEC, O>;
#[doc = "Field `MEM_TRANS_EN` reader - Set this bit 1 to enable automatic transmitting data from memory to memory via DMA."]
pub type MEM_TRANS_EN_R = crate::BitReader;
#[doc = "Field `MEM_TRANS_EN` writer - Set this bit 1 to enable automatic transmitting data from memory to memory via DMA."]
pub type MEM_TRANS_EN_W<'a, const O: u8> = crate::BitWriter<'a, IN_CONF0_CH1_SPEC, O>;
impl R {
    #[doc = "Bit 0 - This bit is used to reset DMA channel 1 Rx FSM and Rx FIFO pointer."]
    #[inline(always)]
    pub fn in_rst(&self) -> IN_RST_R {
        IN_RST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - reserved"]
    #[inline(always)]
    pub fn in_loop_test(&self) -> IN_LOOP_TEST_R {
        IN_LOOP_TEST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set this bit to 1 to enable INCR burst transfer for Rx channel 1 reading link descriptor when accessing internal SRAM."]
    #[inline(always)]
    pub fn indscr_burst_en(&self) -> INDSCR_BURST_EN_R {
        INDSCR_BURST_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Set this bit to 1 to enable INCR burst transfer for Rx channel 1 receiving data when accessing internal SRAM."]
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
        f.debug_struct("IN_CONF0_CH1")
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
impl core::fmt::Debug for crate::generic::Reg<IN_CONF0_CH1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - This bit is used to reset DMA channel 1 Rx FSM and Rx FIFO pointer."]
    #[inline(always)]
    #[must_use]
    pub fn in_rst(&mut self) -> IN_RST_W<0> {
        IN_RST_W::new(self)
    }
    #[doc = "Bit 1 - reserved"]
    #[inline(always)]
    #[must_use]
    pub fn in_loop_test(&mut self) -> IN_LOOP_TEST_W<1> {
        IN_LOOP_TEST_W::new(self)
    }
    #[doc = "Bit 2 - Set this bit to 1 to enable INCR burst transfer for Rx channel 1 reading link descriptor when accessing internal SRAM."]
    #[inline(always)]
    #[must_use]
    pub fn indscr_burst_en(&mut self) -> INDSCR_BURST_EN_W<2> {
        INDSCR_BURST_EN_W::new(self)
    }
    #[doc = "Bit 3 - Set this bit to 1 to enable INCR burst transfer for Rx channel 1 receiving data when accessing internal SRAM."]
    #[inline(always)]
    #[must_use]
    pub fn in_data_burst_en(&mut self) -> IN_DATA_BURST_EN_W<3> {
        IN_DATA_BURST_EN_W::new(self)
    }
    #[doc = "Bit 4 - Set this bit 1 to enable automatic transmitting data from memory to memory via DMA."]
    #[inline(always)]
    #[must_use]
    pub fn mem_trans_en(&mut self) -> MEM_TRANS_EN_W<4> {
        MEM_TRANS_EN_W::new(self)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IN_CONF0_CH1 to value 0"]
impl crate::Resettable for IN_CONF0_CH1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
