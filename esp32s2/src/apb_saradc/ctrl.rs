#[doc = "Register `CTRL` reader"]
pub struct R(crate::R<CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL` writer"]
pub struct W(crate::W<CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_SPEC>;
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
impl From<crate::W<CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `START_FORCE` reader - 0: select FSM to start SAR ADC. 1: select software to start SAR ADC."]
pub type START_FORCE_R = crate::BitReader<bool>;
#[doc = "Field `START_FORCE` writer - 0: select FSM to start SAR ADC. 1: select software to start SAR ADC."]
pub type START_FORCE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `START` reader - Start SAR ADC by software."]
pub type START_R = crate::BitReader<bool>;
#[doc = "Field `START` writer - Start SAR ADC by software."]
pub type START_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `WORK_MODE` reader - 0: single-channel scan mode. 1: double-channel scan mode. 2: alternate-channel scan mode."]
pub type WORK_MODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WORK_MODE` writer - 0: single-channel scan mode. 1: double-channel scan mode. 2: alternate-channel scan mode."]
pub type WORK_MODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, u8, 2, O>;
#[doc = "Field `SAR_SEL` reader - 0: select SAR ADC1. 1: select SAR ADC2, only work for single-channel scan mode."]
pub type SAR_SEL_R = crate::BitReader<bool>;
#[doc = "Field `SAR_SEL` writer - 0: select SAR ADC1. 1: select SAR ADC2, only work for single-channel scan mode."]
pub type SAR_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `SAR_CLK_GATED` reader - SAR clock gate enable bit."]
pub type SAR_CLK_GATED_R = crate::BitReader<bool>;
#[doc = "Field `SAR_CLK_GATED` writer - SAR clock gate enable bit."]
pub type SAR_CLK_GATED_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `SAR_CLK_DIV` reader - SAR clock divider"]
pub type SAR_CLK_DIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SAR_CLK_DIV` writer - SAR clock divider"]
pub type SAR_CLK_DIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, u8, 8, O>;
#[doc = "Field `SAR1_PATT_LEN` reader - 0 ~ 15 means length 1 ~ 16"]
pub type SAR1_PATT_LEN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SAR1_PATT_LEN` writer - 0 ~ 15 means length 1 ~ 16"]
pub type SAR1_PATT_LEN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, u8, 4, O>;
#[doc = "Field `SAR2_PATT_LEN` reader - 0 ~ 15 means length 1 ~ 16"]
pub type SAR2_PATT_LEN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SAR2_PATT_LEN` writer - 0 ~ 15 means length 1 ~ 16"]
pub type SAR2_PATT_LEN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, u8, 4, O>;
#[doc = "Field `SAR1_PATT_P_CLEAR` reader - Clear the pointer of pattern table for DIG ADC1 CTRL."]
pub type SAR1_PATT_P_CLEAR_R = crate::BitReader<bool>;
#[doc = "Field `SAR1_PATT_P_CLEAR` writer - Clear the pointer of pattern table for DIG ADC1 CTRL."]
pub type SAR1_PATT_P_CLEAR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `SAR2_PATT_P_CLEAR` reader - Clear the pointer of pattern table for DIG ADC2 CTRL."]
pub type SAR2_PATT_P_CLEAR_R = crate::BitReader<bool>;
#[doc = "Field `SAR2_PATT_P_CLEAR` writer - Clear the pointer of pattern table for DIG ADC2 CTRL."]
pub type SAR2_PATT_P_CLEAR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `DATA_SAR_SEL` reader - 1: sar_sel will be coded to the MSB of the 16-bit output data, in this case the resolution should not be larger than 11 bits."]
pub type DATA_SAR_SEL_R = crate::BitReader<bool>;
#[doc = "Field `DATA_SAR_SEL` writer - 1: sar_sel will be coded to the MSB of the 16-bit output data, in this case the resolution should not be larger than 11 bits."]
pub type DATA_SAR_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `DATA_TO_I2S` reader - 1: I2S input data is from SAR ADC (for DMA), 0: I2S input data is from GPIO matrix"]
pub type DATA_TO_I2S_R = crate::BitReader<bool>;
#[doc = "Field `DATA_TO_I2S` writer - 1: I2S input data is from SAR ADC (for DMA), 0: I2S input data is from GPIO matrix"]
pub type DATA_TO_I2S_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `XPD_SAR_FORCE` reader - Force option to xpd sar blocks."]
pub type XPD_SAR_FORCE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `XPD_SAR_FORCE` writer - Force option to xpd sar blocks."]
pub type XPD_SAR_FORCE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, u8, 2, O>;
#[doc = "Field `WAIT_ARB_CYCLE` reader - Wait arbit signal stable after sar_done."]
pub type WAIT_ARB_CYCLE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WAIT_ARB_CYCLE` writer - Wait arbit signal stable after sar_done."]
pub type WAIT_ARB_CYCLE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bit 0 - 0: select FSM to start SAR ADC. 1: select software to start SAR ADC."]
    #[inline(always)]
    pub fn start_force(&self) -> START_FORCE_R {
        START_FORCE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Start SAR ADC by software."]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 3:4 - 0: single-channel scan mode. 1: double-channel scan mode. 2: alternate-channel scan mode."]
    #[inline(always)]
    pub fn work_mode(&self) -> WORK_MODE_R {
        WORK_MODE_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - 0: select SAR ADC1. 1: select SAR ADC2, only work for single-channel scan mode."]
    #[inline(always)]
    pub fn sar_sel(&self) -> SAR_SEL_R {
        SAR_SEL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SAR clock gate enable bit."]
    #[inline(always)]
    pub fn sar_clk_gated(&self) -> SAR_CLK_GATED_R {
        SAR_CLK_GATED_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:14 - SAR clock divider"]
    #[inline(always)]
    pub fn sar_clk_div(&self) -> SAR_CLK_DIV_R {
        SAR_CLK_DIV_R::new(((self.bits >> 7) & 0xff) as u8)
    }
    #[doc = "Bits 15:18 - 0 ~ 15 means length 1 ~ 16"]
    #[inline(always)]
    pub fn sar1_patt_len(&self) -> SAR1_PATT_LEN_R {
        SAR1_PATT_LEN_R::new(((self.bits >> 15) & 0x0f) as u8)
    }
    #[doc = "Bits 19:22 - 0 ~ 15 means length 1 ~ 16"]
    #[inline(always)]
    pub fn sar2_patt_len(&self) -> SAR2_PATT_LEN_R {
        SAR2_PATT_LEN_R::new(((self.bits >> 19) & 0x0f) as u8)
    }
    #[doc = "Bit 23 - Clear the pointer of pattern table for DIG ADC1 CTRL."]
    #[inline(always)]
    pub fn sar1_patt_p_clear(&self) -> SAR1_PATT_P_CLEAR_R {
        SAR1_PATT_P_CLEAR_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Clear the pointer of pattern table for DIG ADC2 CTRL."]
    #[inline(always)]
    pub fn sar2_patt_p_clear(&self) -> SAR2_PATT_P_CLEAR_R {
        SAR2_PATT_P_CLEAR_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 1: sar_sel will be coded to the MSB of the 16-bit output data, in this case the resolution should not be larger than 11 bits."]
    #[inline(always)]
    pub fn data_sar_sel(&self) -> DATA_SAR_SEL_R {
        DATA_SAR_SEL_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - 1: I2S input data is from SAR ADC (for DMA), 0: I2S input data is from GPIO matrix"]
    #[inline(always)]
    pub fn data_to_i2s(&self) -> DATA_TO_I2S_R {
        DATA_TO_I2S_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 27:28 - Force option to xpd sar blocks."]
    #[inline(always)]
    pub fn xpd_sar_force(&self) -> XPD_SAR_FORCE_R {
        XPD_SAR_FORCE_R::new(((self.bits >> 27) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Wait arbit signal stable after sar_done."]
    #[inline(always)]
    pub fn wait_arb_cycle(&self) -> WAIT_ARB_CYCLE_R {
        WAIT_ARB_CYCLE_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0: select FSM to start SAR ADC. 1: select software to start SAR ADC."]
    #[inline(always)]
    pub fn start_force(&mut self) -> START_FORCE_W<0> {
        START_FORCE_W::new(self)
    }
    #[doc = "Bit 1 - Start SAR ADC by software."]
    #[inline(always)]
    pub fn start(&mut self) -> START_W<1> {
        START_W::new(self)
    }
    #[doc = "Bits 3:4 - 0: single-channel scan mode. 1: double-channel scan mode. 2: alternate-channel scan mode."]
    #[inline(always)]
    pub fn work_mode(&mut self) -> WORK_MODE_W<3> {
        WORK_MODE_W::new(self)
    }
    #[doc = "Bit 5 - 0: select SAR ADC1. 1: select SAR ADC2, only work for single-channel scan mode."]
    #[inline(always)]
    pub fn sar_sel(&mut self) -> SAR_SEL_W<5> {
        SAR_SEL_W::new(self)
    }
    #[doc = "Bit 6 - SAR clock gate enable bit."]
    #[inline(always)]
    pub fn sar_clk_gated(&mut self) -> SAR_CLK_GATED_W<6> {
        SAR_CLK_GATED_W::new(self)
    }
    #[doc = "Bits 7:14 - SAR clock divider"]
    #[inline(always)]
    pub fn sar_clk_div(&mut self) -> SAR_CLK_DIV_W<7> {
        SAR_CLK_DIV_W::new(self)
    }
    #[doc = "Bits 15:18 - 0 ~ 15 means length 1 ~ 16"]
    #[inline(always)]
    pub fn sar1_patt_len(&mut self) -> SAR1_PATT_LEN_W<15> {
        SAR1_PATT_LEN_W::new(self)
    }
    #[doc = "Bits 19:22 - 0 ~ 15 means length 1 ~ 16"]
    #[inline(always)]
    pub fn sar2_patt_len(&mut self) -> SAR2_PATT_LEN_W<19> {
        SAR2_PATT_LEN_W::new(self)
    }
    #[doc = "Bit 23 - Clear the pointer of pattern table for DIG ADC1 CTRL."]
    #[inline(always)]
    pub fn sar1_patt_p_clear(&mut self) -> SAR1_PATT_P_CLEAR_W<23> {
        SAR1_PATT_P_CLEAR_W::new(self)
    }
    #[doc = "Bit 24 - Clear the pointer of pattern table for DIG ADC2 CTRL."]
    #[inline(always)]
    pub fn sar2_patt_p_clear(&mut self) -> SAR2_PATT_P_CLEAR_W<24> {
        SAR2_PATT_P_CLEAR_W::new(self)
    }
    #[doc = "Bit 25 - 1: sar_sel will be coded to the MSB of the 16-bit output data, in this case the resolution should not be larger than 11 bits."]
    #[inline(always)]
    pub fn data_sar_sel(&mut self) -> DATA_SAR_SEL_W<25> {
        DATA_SAR_SEL_W::new(self)
    }
    #[doc = "Bit 26 - 1: I2S input data is from SAR ADC (for DMA), 0: I2S input data is from GPIO matrix"]
    #[inline(always)]
    pub fn data_to_i2s(&mut self) -> DATA_TO_I2S_W<26> {
        DATA_TO_I2S_W::new(self)
    }
    #[doc = "Bits 27:28 - Force option to xpd sar blocks."]
    #[inline(always)]
    pub fn xpd_sar_force(&mut self) -> XPD_SAR_FORCE_W<27> {
        XPD_SAR_FORCE_W::new(self)
    }
    #[doc = "Bits 30:31 - Wait arbit signal stable after sar_done."]
    #[inline(always)]
    pub fn wait_arb_cycle(&mut self) -> WAIT_ARB_CYCLE_W<30> {
        WAIT_ARB_CYCLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DIG ADC common configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl::R](R) reader structure"]
impl crate::Readable for CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl::W](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRL to value 0x407f_8240"]
impl crate::Resettable for CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x407f_8240
    }
}
