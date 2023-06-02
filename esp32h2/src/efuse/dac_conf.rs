#[doc = "Register `DAC_CONF` reader"]
pub struct R(crate::R<DAC_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DAC_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DAC_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DAC_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DAC_CONF` writer"]
pub struct W(crate::W<DAC_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DAC_CONF_SPEC>;
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
impl From<crate::W<DAC_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DAC_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DAC_CLK_DIV` reader - Controls the division factor of the rising clock of the programming voltage."]
pub type DAC_CLK_DIV_R = crate::FieldReader;
#[doc = "Field `DAC_CLK_DIV` writer - Controls the division factor of the rising clock of the programming voltage."]
pub type DAC_CLK_DIV_W<'a, const O: u8> = crate::FieldWriter<'a, DAC_CONF_SPEC, 8, O>;
#[doc = "Field `DAC_CLK_PAD_SEL` reader - Don't care."]
pub type DAC_CLK_PAD_SEL_R = crate::BitReader;
#[doc = "Field `DAC_CLK_PAD_SEL` writer - Don't care."]
pub type DAC_CLK_PAD_SEL_W<'a, const O: u8> = crate::BitWriter<'a, DAC_CONF_SPEC, O>;
#[doc = "Field `DAC_NUM` reader - Controls the rising period of the programming voltage."]
pub type DAC_NUM_R = crate::FieldReader;
#[doc = "Field `DAC_NUM` writer - Controls the rising period of the programming voltage."]
pub type DAC_NUM_W<'a, const O: u8> = crate::FieldWriter<'a, DAC_CONF_SPEC, 8, O>;
#[doc = "Field `OE_CLR` reader - Reduces the power supply of the programming voltage."]
pub type OE_CLR_R = crate::BitReader;
#[doc = "Field `OE_CLR` writer - Reduces the power supply of the programming voltage."]
pub type OE_CLR_W<'a, const O: u8> = crate::BitWriter<'a, DAC_CONF_SPEC, O>;
impl R {
    #[doc = "Bits 0:7 - Controls the division factor of the rising clock of the programming voltage."]
    #[inline(always)]
    pub fn dac_clk_div(&self) -> DAC_CLK_DIV_R {
        DAC_CLK_DIV_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - Don't care."]
    #[inline(always)]
    pub fn dac_clk_pad_sel(&self) -> DAC_CLK_PAD_SEL_R {
        DAC_CLK_PAD_SEL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:16 - Controls the rising period of the programming voltage."]
    #[inline(always)]
    pub fn dac_num(&self) -> DAC_NUM_R {
        DAC_NUM_R::new(((self.bits >> 9) & 0xff) as u8)
    }
    #[doc = "Bit 17 - Reduces the power supply of the programming voltage."]
    #[inline(always)]
    pub fn oe_clr(&self) -> OE_CLR_R {
        OE_CLR_R::new(((self.bits >> 17) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DAC_CONF")
            .field(
                "dac_clk_div",
                &format_args!("{}", self.dac_clk_div().bits()),
            )
            .field(
                "dac_clk_pad_sel",
                &format_args!("{}", self.dac_clk_pad_sel().bit()),
            )
            .field("dac_num", &format_args!("{}", self.dac_num().bits()))
            .field("oe_clr", &format_args!("{}", self.oe_clr().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DAC_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7 - Controls the division factor of the rising clock of the programming voltage."]
    #[inline(always)]
    #[must_use]
    pub fn dac_clk_div(&mut self) -> DAC_CLK_DIV_W<0> {
        DAC_CLK_DIV_W::new(self)
    }
    #[doc = "Bit 8 - Don't care."]
    #[inline(always)]
    #[must_use]
    pub fn dac_clk_pad_sel(&mut self) -> DAC_CLK_PAD_SEL_W<8> {
        DAC_CLK_PAD_SEL_W::new(self)
    }
    #[doc = "Bits 9:16 - Controls the rising period of the programming voltage."]
    #[inline(always)]
    #[must_use]
    pub fn dac_num(&mut self) -> DAC_NUM_W<9> {
        DAC_NUM_W::new(self)
    }
    #[doc = "Bit 17 - Reduces the power supply of the programming voltage."]
    #[inline(always)]
    #[must_use]
    pub fn oe_clr(&mut self) -> OE_CLR_W<17> {
        OE_CLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Controls the eFuse programming voltage.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dac_conf](index.html) module"]
pub struct DAC_CONF_SPEC;
impl crate::RegisterSpec for DAC_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dac_conf::R](R) reader structure"]
impl crate::Readable for DAC_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dac_conf::W](W) writer structure"]
impl crate::Writable for DAC_CONF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DAC_CONF to value 0x0001_fe17"]
impl crate::Resettable for DAC_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0x0001_fe17;
}
