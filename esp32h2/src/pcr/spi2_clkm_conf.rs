#[doc = "Register `SPI2_CLKM_CONF` reader"]
pub struct R(crate::R<SPI2_CLKM_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI2_CLKM_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI2_CLKM_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI2_CLKM_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPI2_CLKM_CONF` writer"]
pub struct W(crate::W<SPI2_CLKM_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI2_CLKM_CONF_SPEC>;
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
impl From<crate::W<SPI2_CLKM_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI2_CLKM_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPI2_CLKM_SEL` reader - set this field to select clock-source. 0(default): XTAL, 1: 80MHz, 2: FOSC, 3: reserved."]
pub type SPI2_CLKM_SEL_R = crate::FieldReader;
#[doc = "Field `SPI2_CLKM_SEL` writer - set this field to select clock-source. 0(default): XTAL, 1: 80MHz, 2: FOSC, 3: reserved."]
pub type SPI2_CLKM_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, SPI2_CLKM_CONF_SPEC, 2, O>;
#[doc = "Field `SPI2_CLKM_EN` reader - Set 1 to enable spi2 function clock"]
pub type SPI2_CLKM_EN_R = crate::BitReader;
#[doc = "Field `SPI2_CLKM_EN` writer - Set 1 to enable spi2 function clock"]
pub type SPI2_CLKM_EN_W<'a, const O: u8> = crate::BitWriter<'a, SPI2_CLKM_CONF_SPEC, O>;
impl R {
    #[doc = "Bits 20:21 - set this field to select clock-source. 0(default): XTAL, 1: 80MHz, 2: FOSC, 3: reserved."]
    #[inline(always)]
    pub fn spi2_clkm_sel(&self) -> SPI2_CLKM_SEL_R {
        SPI2_CLKM_SEL_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 22 - Set 1 to enable spi2 function clock"]
    #[inline(always)]
    pub fn spi2_clkm_en(&self) -> SPI2_CLKM_EN_R {
        SPI2_CLKM_EN_R::new(((self.bits >> 22) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI2_CLKM_CONF")
            .field(
                "spi2_clkm_sel",
                &format_args!("{}", self.spi2_clkm_sel().bits()),
            )
            .field(
                "spi2_clkm_en",
                &format_args!("{}", self.spi2_clkm_en().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SPI2_CLKM_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 20:21 - set this field to select clock-source. 0(default): XTAL, 1: 80MHz, 2: FOSC, 3: reserved."]
    #[inline(always)]
    #[must_use]
    pub fn spi2_clkm_sel(&mut self) -> SPI2_CLKM_SEL_W<20> {
        SPI2_CLKM_SEL_W::new(self)
    }
    #[doc = "Bit 22 - Set 1 to enable spi2 function clock"]
    #[inline(always)]
    #[must_use]
    pub fn spi2_clkm_en(&mut self) -> SPI2_CLKM_EN_W<22> {
        SPI2_CLKM_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI2_CLKM configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi2_clkm_conf](index.html) module"]
pub struct SPI2_CLKM_CONF_SPEC;
impl crate::RegisterSpec for SPI2_CLKM_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi2_clkm_conf::R](R) reader structure"]
impl crate::Readable for SPI2_CLKM_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi2_clkm_conf::W](W) writer structure"]
impl crate::Writable for SPI2_CLKM_CONF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPI2_CLKM_CONF to value 0x0040_0000"]
impl crate::Resettable for SPI2_CLKM_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0x0040_0000;
}
