#[doc = "Register `SYSCLK_CONF` reader"]
pub struct R(crate::R<SYSCLK_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYSCLK_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYSCLK_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYSCLK_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYSCLK_CONF` writer"]
pub struct W(crate::W<SYSCLK_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYSCLK_CONF_SPEC>;
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
impl From<crate::W<SYSCLK_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYSCLK_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRE_DIV_CNT` reader - This field is used to set the count of prescaler of XTAL_CLK."]
pub type PRE_DIV_CNT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PRE_DIV_CNT` writer - This field is used to set the count of prescaler of XTAL_CLK."]
pub type PRE_DIV_CNT_W<'a> = crate::FieldWriter<'a, u32, SYSCLK_CONF_SPEC, u16, u16, 10, 0>;
#[doc = "Field `SOC_CLK_SEL` reader - This field is used to select soc clock."]
pub type SOC_CLK_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SOC_CLK_SEL` writer - This field is used to select soc clock."]
pub type SOC_CLK_SEL_W<'a> = crate::FieldWriter<'a, u32, SYSCLK_CONF_SPEC, u8, u8, 2, 10>;
#[doc = "Field `CLK_XTAL_FREQ` reader - This field is used to read xtal frequency in MHz."]
pub type CLK_XTAL_FREQ_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CLK_DIV_EN` reader - reg_clk_div_en"]
pub type CLK_DIV_EN_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:9 - This field is used to set the count of prescaler of XTAL_CLK."]
    #[inline(always)]
    pub fn pre_div_cnt(&self) -> PRE_DIV_CNT_R {
        PRE_DIV_CNT_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:11 - This field is used to select soc clock."]
    #[inline(always)]
    pub fn soc_clk_sel(&self) -> SOC_CLK_SEL_R {
        SOC_CLK_SEL_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:18 - This field is used to read xtal frequency in MHz."]
    #[inline(always)]
    pub fn clk_xtal_freq(&self) -> CLK_XTAL_FREQ_R {
        CLK_XTAL_FREQ_R::new(((self.bits >> 12) & 0x7f) as u8)
    }
    #[doc = "Bit 19 - reg_clk_div_en"]
    #[inline(always)]
    pub fn clk_div_en(&self) -> CLK_DIV_EN_R {
        CLK_DIV_EN_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - This field is used to set the count of prescaler of XTAL_CLK."]
    #[inline(always)]
    pub fn pre_div_cnt(&mut self) -> PRE_DIV_CNT_W {
        PRE_DIV_CNT_W::new(self)
    }
    #[doc = "Bits 10:11 - This field is used to select soc clock."]
    #[inline(always)]
    pub fn soc_clk_sel(&mut self) -> SOC_CLK_SEL_W {
        SOC_CLK_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "system clock config register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sysclk_conf](index.html) module"]
pub struct SYSCLK_CONF_SPEC;
impl crate::RegisterSpec for SYSCLK_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sysclk_conf::R](R) reader structure"]
impl crate::Readable for SYSCLK_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sysclk_conf::W](W) writer structure"]
impl crate::Writable for SYSCLK_CONF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SYSCLK_CONF to value 0x01"]
impl crate::Resettable for SYSCLK_CONF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
