#[doc = "Register `SAR_READER2_CTRL` reader"]
pub struct R(crate::R<SAR_READER2_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAR_READER2_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAR_READER2_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAR_READER2_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SAR_READER2_CTRL` writer"]
pub struct W(crate::W<SAR_READER2_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SAR_READER2_CTRL_SPEC>;
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
impl From<crate::W<SAR_READER2_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SAR_READER2_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SAR_SAR2_CLK_DIV` reader - clock divider"]
pub type SAR_SAR2_CLK_DIV_R = crate::FieldReader;
#[doc = "Field `SAR_SAR2_CLK_DIV` writer - clock divider"]
pub type SAR_SAR2_CLK_DIV_W<'a, const O: u8> = crate::FieldWriter<'a, SAR_READER2_CTRL_SPEC, 8, O>;
#[doc = "Field `SAR_SAR2_WAIT_ARB_CYCLE` reader - wait arbit stable after sar_done"]
pub type SAR_SAR2_WAIT_ARB_CYCLE_R = crate::FieldReader;
#[doc = "Field `SAR_SAR2_WAIT_ARB_CYCLE` writer - wait arbit stable after sar_done"]
pub type SAR_SAR2_WAIT_ARB_CYCLE_W<'a, const O: u8> =
    crate::FieldWriter<'a, SAR_READER2_CTRL_SPEC, 2, O>;
#[doc = "Field `SAR_SAR2_CLK_GATED` reader - ******* Description ***********"]
pub type SAR_SAR2_CLK_GATED_R = crate::BitReader;
#[doc = "Field `SAR_SAR2_CLK_GATED` writer - ******* Description ***********"]
pub type SAR_SAR2_CLK_GATED_W<'a, const O: u8> = crate::BitWriter<'a, SAR_READER2_CTRL_SPEC, O>;
#[doc = "Field `SAR_SAR2_SAMPLE_NUM` reader - ******* Description ***********"]
pub type SAR_SAR2_SAMPLE_NUM_R = crate::FieldReader;
#[doc = "Field `SAR_SAR2_SAMPLE_NUM` writer - ******* Description ***********"]
pub type SAR_SAR2_SAMPLE_NUM_W<'a, const O: u8> =
    crate::FieldWriter<'a, SAR_READER2_CTRL_SPEC, 8, O>;
#[doc = "Field `SAR_SAR2_DATA_INV` reader - Invert SAR ADC2 data"]
pub type SAR_SAR2_DATA_INV_R = crate::BitReader;
#[doc = "Field `SAR_SAR2_DATA_INV` writer - Invert SAR ADC2 data"]
pub type SAR_SAR2_DATA_INV_W<'a, const O: u8> = crate::BitWriter<'a, SAR_READER2_CTRL_SPEC, O>;
#[doc = "Field `SAR_SAR2_INT_EN` reader - enable saradc2 to send out interrupt"]
pub type SAR_SAR2_INT_EN_R = crate::BitReader;
#[doc = "Field `SAR_SAR2_INT_EN` writer - enable saradc2 to send out interrupt"]
pub type SAR_SAR2_INT_EN_W<'a, const O: u8> = crate::BitWriter<'a, SAR_READER2_CTRL_SPEC, O>;
impl R {
    #[doc = "Bits 0:7 - clock divider"]
    #[inline(always)]
    pub fn sar_sar2_clk_div(&self) -> SAR_SAR2_CLK_DIV_R {
        SAR_SAR2_CLK_DIV_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:17 - wait arbit stable after sar_done"]
    #[inline(always)]
    pub fn sar_sar2_wait_arb_cycle(&self) -> SAR_SAR2_WAIT_ARB_CYCLE_R {
        SAR_SAR2_WAIT_ARB_CYCLE_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18 - ******* Description ***********"]
    #[inline(always)]
    pub fn sar_sar2_clk_gated(&self) -> SAR_SAR2_CLK_GATED_R {
        SAR_SAR2_CLK_GATED_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:26 - ******* Description ***********"]
    #[inline(always)]
    pub fn sar_sar2_sample_num(&self) -> SAR_SAR2_SAMPLE_NUM_R {
        SAR_SAR2_SAMPLE_NUM_R::new(((self.bits >> 19) & 0xff) as u8)
    }
    #[doc = "Bit 29 - Invert SAR ADC2 data"]
    #[inline(always)]
    pub fn sar_sar2_data_inv(&self) -> SAR_SAR2_DATA_INV_R {
        SAR_SAR2_DATA_INV_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - enable saradc2 to send out interrupt"]
    #[inline(always)]
    pub fn sar_sar2_int_en(&self) -> SAR_SAR2_INT_EN_R {
        SAR_SAR2_INT_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR_READER2_CTRL")
            .field(
                "sar_sar2_clk_div",
                &format_args!("{}", self.sar_sar2_clk_div().bits()),
            )
            .field(
                "sar_sar2_wait_arb_cycle",
                &format_args!("{}", self.sar_sar2_wait_arb_cycle().bits()),
            )
            .field(
                "sar_sar2_clk_gated",
                &format_args!("{}", self.sar_sar2_clk_gated().bit()),
            )
            .field(
                "sar_sar2_sample_num",
                &format_args!("{}", self.sar_sar2_sample_num().bits()),
            )
            .field(
                "sar_sar2_data_inv",
                &format_args!("{}", self.sar_sar2_data_inv().bit()),
            )
            .field(
                "sar_sar2_int_en",
                &format_args!("{}", self.sar_sar2_int_en().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SAR_READER2_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7 - clock divider"]
    #[inline(always)]
    #[must_use]
    pub fn sar_sar2_clk_div(&mut self) -> SAR_SAR2_CLK_DIV_W<0> {
        SAR_SAR2_CLK_DIV_W::new(self)
    }
    #[doc = "Bits 16:17 - wait arbit stable after sar_done"]
    #[inline(always)]
    #[must_use]
    pub fn sar_sar2_wait_arb_cycle(&mut self) -> SAR_SAR2_WAIT_ARB_CYCLE_W<16> {
        SAR_SAR2_WAIT_ARB_CYCLE_W::new(self)
    }
    #[doc = "Bit 18 - ******* Description ***********"]
    #[inline(always)]
    #[must_use]
    pub fn sar_sar2_clk_gated(&mut self) -> SAR_SAR2_CLK_GATED_W<18> {
        SAR_SAR2_CLK_GATED_W::new(self)
    }
    #[doc = "Bits 19:26 - ******* Description ***********"]
    #[inline(always)]
    #[must_use]
    pub fn sar_sar2_sample_num(&mut self) -> SAR_SAR2_SAMPLE_NUM_W<19> {
        SAR_SAR2_SAMPLE_NUM_W::new(self)
    }
    #[doc = "Bit 29 - Invert SAR ADC2 data"]
    #[inline(always)]
    #[must_use]
    pub fn sar_sar2_data_inv(&mut self) -> SAR_SAR2_DATA_INV_W<29> {
        SAR_SAR2_DATA_INV_W::new(self)
    }
    #[doc = "Bit 30 - enable saradc2 to send out interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn sar_sar2_int_en(&mut self) -> SAR_SAR2_INT_EN_W<30> {
        SAR_SAR2_INT_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "configure saradc2 reader\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sar_reader2_ctrl](index.html) module"]
pub struct SAR_READER2_CTRL_SPEC;
impl crate::RegisterSpec for SAR_READER2_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sar_reader2_ctrl::R](R) reader structure"]
impl crate::Readable for SAR_READER2_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sar_reader2_ctrl::W](W) writer structure"]
impl crate::Writable for SAR_READER2_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SAR_READER2_CTRL to value 0x4005_0002"]
impl crate::Resettable for SAR_READER2_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x4005_0002;
}
