#[doc = "Register `LEDC_SCLK_CONF` reader"]
pub type R = crate::R<LEDC_SCLK_CONF_SPEC>;
#[doc = "Register `LEDC_SCLK_CONF` writer"]
pub type W = crate::W<LEDC_SCLK_CONF_SPEC>;
#[doc = "Field `LEDC_SCLK_SEL` reader - set this field to select clock-source. 0(default): do not select anyone clock, 1: 80MHz, 2: FOSC, 3: XTAL."]
pub type LEDC_SCLK_SEL_R = crate::FieldReader;
#[doc = "Field `LEDC_SCLK_SEL` writer - set this field to select clock-source. 0(default): do not select anyone clock, 1: 80MHz, 2: FOSC, 3: XTAL."]
pub type LEDC_SCLK_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LEDC_SCLK_EN` reader - Set 1 to enable ledc function clock"]
pub type LEDC_SCLK_EN_R = crate::BitReader;
#[doc = "Field `LEDC_SCLK_EN` writer - Set 1 to enable ledc function clock"]
pub type LEDC_SCLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
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
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LEDC_SCLK_CONF")
            .field(
                "ledc_sclk_sel",
                &format_args!("{}", self.ledc_sclk_sel().bits()),
            )
            .field(
                "ledc_sclk_en",
                &format_args!("{}", self.ledc_sclk_en().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<LEDC_SCLK_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 20:21 - set this field to select clock-source. 0(default): do not select anyone clock, 1: 80MHz, 2: FOSC, 3: XTAL."]
    #[inline(always)]
    #[must_use]
    pub fn ledc_sclk_sel(&mut self) -> LEDC_SCLK_SEL_W<LEDC_SCLK_CONF_SPEC> {
        LEDC_SCLK_SEL_W::new(self, 20)
    }
    #[doc = "Bit 22 - Set 1 to enable ledc function clock"]
    #[inline(always)]
    #[must_use]
    pub fn ledc_sclk_en(&mut self) -> LEDC_SCLK_EN_W<LEDC_SCLK_CONF_SPEC> {
        LEDC_SCLK_EN_W::new(self, 22)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "LEDC_SCLK configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ledc_sclk_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ledc_sclk_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LEDC_SCLK_CONF_SPEC;
impl crate::RegisterSpec for LEDC_SCLK_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ledc_sclk_conf::R`](R) reader structure"]
impl crate::Readable for LEDC_SCLK_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ledc_sclk_conf::W`](W) writer structure"]
impl crate::Writable for LEDC_SCLK_CONF_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LEDC_SCLK_CONF to value 0x0040_0000"]
impl crate::Resettable for LEDC_SCLK_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0x0040_0000;
}
