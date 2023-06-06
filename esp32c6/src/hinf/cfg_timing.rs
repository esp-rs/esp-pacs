#[doc = "Register `CFG_TIMING` reader"]
pub struct R(crate::R<CFG_TIMING_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFG_TIMING_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFG_TIMING_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFG_TIMING_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFG_TIMING` writer"]
pub struct W(crate::W<CFG_TIMING_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFG_TIMING_SPEC>;
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
impl From<crate::W<CFG_TIMING_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFG_TIMING_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NCRC` reader - configure Ncrc parameter in sdr50/104 mode, no more than 6."]
pub type NCRC_R = crate::FieldReader;
#[doc = "Field `NCRC` writer - configure Ncrc parameter in sdr50/104 mode, no more than 6."]
pub type NCRC_W<'a, const O: u8> = crate::FieldWriter<'a, CFG_TIMING_SPEC, 3, O>;
#[doc = "Field `PST_END_CMD_LOW_VALUE` reader - configure cycles to lower cmd after voltage is changed to 1.8V."]
pub type PST_END_CMD_LOW_VALUE_R = crate::FieldReader;
#[doc = "Field `PST_END_CMD_LOW_VALUE` writer - configure cycles to lower cmd after voltage is changed to 1.8V."]
pub type PST_END_CMD_LOW_VALUE_W<'a, const O: u8> = crate::FieldWriter<'a, CFG_TIMING_SPEC, 7, O>;
#[doc = "Field `PST_END_DATA_LOW_VALUE` reader - configure cycles to lower data after voltage is changed to 1.8V."]
pub type PST_END_DATA_LOW_VALUE_R = crate::FieldReader;
#[doc = "Field `PST_END_DATA_LOW_VALUE` writer - configure cycles to lower data after voltage is changed to 1.8V."]
pub type PST_END_DATA_LOW_VALUE_W<'a, const O: u8> = crate::FieldWriter<'a, CFG_TIMING_SPEC, 6, O>;
#[doc = "Field `SDCLK_STOP_THRES` reader - Configure the number of cycles of module clk to judge sdclk has stopped"]
pub type SDCLK_STOP_THRES_R = crate::FieldReader<u16>;
#[doc = "Field `SDCLK_STOP_THRES` writer - Configure the number of cycles of module clk to judge sdclk has stopped"]
pub type SDCLK_STOP_THRES_W<'a, const O: u8> = crate::FieldWriter<'a, CFG_TIMING_SPEC, 11, O, u16>;
#[doc = "Field `SAMPLE_CLK_DIVIDER` reader - module clk divider to sample sdclk"]
pub type SAMPLE_CLK_DIVIDER_R = crate::FieldReader;
#[doc = "Field `SAMPLE_CLK_DIVIDER` writer - module clk divider to sample sdclk"]
pub type SAMPLE_CLK_DIVIDER_W<'a, const O: u8> = crate::FieldWriter<'a, CFG_TIMING_SPEC, 4, O>;
impl R {
    #[doc = "Bits 0:2 - configure Ncrc parameter in sdr50/104 mode, no more than 6."]
    #[inline(always)]
    pub fn ncrc(&self) -> NCRC_R {
        NCRC_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:9 - configure cycles to lower cmd after voltage is changed to 1.8V."]
    #[inline(always)]
    pub fn pst_end_cmd_low_value(&self) -> PST_END_CMD_LOW_VALUE_R {
        PST_END_CMD_LOW_VALUE_R::new(((self.bits >> 3) & 0x7f) as u8)
    }
    #[doc = "Bits 10:15 - configure cycles to lower data after voltage is changed to 1.8V."]
    #[inline(always)]
    pub fn pst_end_data_low_value(&self) -> PST_END_DATA_LOW_VALUE_R {
        PST_END_DATA_LOW_VALUE_R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bits 16:26 - Configure the number of cycles of module clk to judge sdclk has stopped"]
    #[inline(always)]
    pub fn sdclk_stop_thres(&self) -> SDCLK_STOP_THRES_R {
        SDCLK_STOP_THRES_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
    #[doc = "Bits 28:31 - module clk divider to sample sdclk"]
    #[inline(always)]
    pub fn sample_clk_divider(&self) -> SAMPLE_CLK_DIVIDER_R {
        SAMPLE_CLK_DIVIDER_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFG_TIMING")
            .field("ncrc", &format_args!("{}", self.ncrc().bits()))
            .field(
                "pst_end_cmd_low_value",
                &format_args!("{}", self.pst_end_cmd_low_value().bits()),
            )
            .field(
                "pst_end_data_low_value",
                &format_args!("{}", self.pst_end_data_low_value().bits()),
            )
            .field(
                "sdclk_stop_thres",
                &format_args!("{}", self.sdclk_stop_thres().bits()),
            )
            .field(
                "sample_clk_divider",
                &format_args!("{}", self.sample_clk_divider().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CFG_TIMING_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:2 - configure Ncrc parameter in sdr50/104 mode, no more than 6."]
    #[inline(always)]
    #[must_use]
    pub fn ncrc(&mut self) -> NCRC_W<0> {
        NCRC_W::new(self)
    }
    #[doc = "Bits 3:9 - configure cycles to lower cmd after voltage is changed to 1.8V."]
    #[inline(always)]
    #[must_use]
    pub fn pst_end_cmd_low_value(&mut self) -> PST_END_CMD_LOW_VALUE_W<3> {
        PST_END_CMD_LOW_VALUE_W::new(self)
    }
    #[doc = "Bits 10:15 - configure cycles to lower data after voltage is changed to 1.8V."]
    #[inline(always)]
    #[must_use]
    pub fn pst_end_data_low_value(&mut self) -> PST_END_DATA_LOW_VALUE_W<10> {
        PST_END_DATA_LOW_VALUE_W::new(self)
    }
    #[doc = "Bits 16:26 - Configure the number of cycles of module clk to judge sdclk has stopped"]
    #[inline(always)]
    #[must_use]
    pub fn sdclk_stop_thres(&mut self) -> SDCLK_STOP_THRES_W<16> {
        SDCLK_STOP_THRES_W::new(self)
    }
    #[doc = "Bits 28:31 - module clk divider to sample sdclk"]
    #[inline(always)]
    #[must_use]
    pub fn sample_clk_divider(&mut self) -> SAMPLE_CLK_DIVIDER_W<28> {
        SAMPLE_CLK_DIVIDER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timing configuration registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfg_timing](index.html) module"]
pub struct CFG_TIMING_SPEC;
impl crate::RegisterSpec for CFG_TIMING_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfg_timing::R](R) reader structure"]
impl crate::Readable for CFG_TIMING_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfg_timing::W](W) writer structure"]
impl crate::Writable for CFG_TIMING_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFG_TIMING to value 0x1578_0812"]
impl crate::Resettable for CFG_TIMING_SPEC {
    const RESET_VALUE: Self::Ux = 0x1578_0812;
}
