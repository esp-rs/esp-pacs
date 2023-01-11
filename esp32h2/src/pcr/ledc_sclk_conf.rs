#[doc = "Register `LEDC_SCLK_CONF` reader"]
pub struct R(crate::R<LEDC_SCLK_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LEDC_SCLK_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LEDC_SCLK_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LEDC_SCLK_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LEDC_SCLK_CONF` writer"]
pub struct W(crate::W<LEDC_SCLK_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LEDC_SCLK_CONF_SPEC>;
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
impl From<crate::W<LEDC_SCLK_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LEDC_SCLK_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LEDC_SCLK_SEL` reader - set this field to select clock-source. 0(default): do not select anyone clock, 1: 80MHz, 2: FOSC, 3: XTAL."]
pub type LEDC_SCLK_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LEDC_SCLK_SEL` writer - set this field to select clock-source. 0(default): do not select anyone clock, 1: 80MHz, 2: FOSC, 3: XTAL."]
pub type LEDC_SCLK_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LEDC_SCLK_CONF_SPEC, u8, u8, 2, O>;
#[doc = "Field `LEDC_SCLK_EN` reader - Set 1 to enable ledc function clock"]
pub type LEDC_SCLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `LEDC_SCLK_EN` writer - Set 1 to enable ledc function clock"]
pub type LEDC_SCLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, LEDC_SCLK_CONF_SPEC, bool, O>;
impl R {
    #[doc = "Bits 20:21 - set this field to select clock-source. 0(default): do not select anyone clock, 1: 80MHz, 2: FOSC, 3: XTAL."]
    #[inline(always)]
    pub fn ledc_sclk_sel(&self) -> LEDC_SCLK_SEL_R {
        LEDC_SCLK_SEL_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 22 - Set 1 to enable ledc function clock"]
    #[inline(always)]
    pub fn ledc_sclk_en(&self) -> LEDC_SCLK_EN_R {
        LEDC_SCLK_EN_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 20:21 - set this field to select clock-source. 0(default): do not select anyone clock, 1: 80MHz, 2: FOSC, 3: XTAL."]
    #[inline(always)]
    #[must_use]
    pub fn ledc_sclk_sel(&mut self) -> LEDC_SCLK_SEL_W<20> {
        LEDC_SCLK_SEL_W::new(self)
    }
    #[doc = "Bit 22 - Set 1 to enable ledc function clock"]
    #[inline(always)]
    #[must_use]
    pub fn ledc_sclk_en(&mut self) -> LEDC_SCLK_EN_W<22> {
        LEDC_SCLK_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LEDC_SCLK configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ledc_sclk_conf](index.html) module"]
pub struct LEDC_SCLK_CONF_SPEC;
impl crate::RegisterSpec for LEDC_SCLK_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ledc_sclk_conf::R](R) reader structure"]
impl crate::Readable for LEDC_SCLK_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ledc_sclk_conf::W](W) writer structure"]
impl crate::Writable for LEDC_SCLK_CONF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LEDC_SCLK_CONF to value 0x0040_0000"]
impl crate::Resettable for LEDC_SCLK_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0x0040_0000;
}
