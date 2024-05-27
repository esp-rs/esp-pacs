#[doc = "Register `SLP_REJECT_CONF` reader"]
pub type R = crate::R<SLP_REJECT_CONF_SPEC>;
#[doc = "Register `SLP_REJECT_CONF` writer"]
pub type W = crate::W<SLP_REJECT_CONF_SPEC>;
#[doc = "Field `GPIO_REJECT_EN` reader - enable GPIO reject"]
pub type GPIO_REJECT_EN_R = crate::BitReader;
#[doc = "Field `GPIO_REJECT_EN` writer - enable GPIO reject"]
pub type GPIO_REJECT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDIO_REJECT_EN` reader - enable SDIO reject"]
pub type SDIO_REJECT_EN_R = crate::BitReader;
#[doc = "Field `SDIO_REJECT_EN` writer - enable SDIO reject"]
pub type SDIO_REJECT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LIGHT_SLP_REJECT_EN` reader - enable reject for light sleep"]
pub type LIGHT_SLP_REJECT_EN_R = crate::BitReader;
#[doc = "Field `LIGHT_SLP_REJECT_EN` writer - enable reject for light sleep"]
pub type LIGHT_SLP_REJECT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEEP_SLP_REJECT_EN` reader - enable reject for deep sleep"]
pub type DEEP_SLP_REJECT_EN_R = crate::BitReader;
#[doc = "Field `DEEP_SLP_REJECT_EN` writer - enable reject for deep sleep"]
pub type DEEP_SLP_REJECT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REJECT_CAUSE` reader - sleep reject cause"]
pub type REJECT_CAUSE_R = crate::FieldReader;
impl R {
    #[doc = "Bit 24 - enable GPIO reject"]
    #[inline(always)]
    pub fn gpio_reject_en(&self) -> GPIO_REJECT_EN_R {
        GPIO_REJECT_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - enable SDIO reject"]
    #[inline(always)]
    pub fn sdio_reject_en(&self) -> SDIO_REJECT_EN_R {
        SDIO_REJECT_EN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - enable reject for light sleep"]
    #[inline(always)]
    pub fn light_slp_reject_en(&self) -> LIGHT_SLP_REJECT_EN_R {
        LIGHT_SLP_REJECT_EN_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - enable reject for deep sleep"]
    #[inline(always)]
    pub fn deep_slp_reject_en(&self) -> DEEP_SLP_REJECT_EN_R {
        DEEP_SLP_REJECT_EN_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:31 - sleep reject cause"]
    #[inline(always)]
    pub fn reject_cause(&self) -> REJECT_CAUSE_R {
        REJECT_CAUSE_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SLP_REJECT_CONF")
            .field("gpio_reject_en", &self.gpio_reject_en())
            .field("sdio_reject_en", &self.sdio_reject_en())
            .field("light_slp_reject_en", &self.light_slp_reject_en())
            .field("deep_slp_reject_en", &self.deep_slp_reject_en())
            .field("reject_cause", &self.reject_cause())
            .finish()
    }
}
impl W {
    #[doc = "Bit 24 - enable GPIO reject"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_reject_en(&mut self) -> GPIO_REJECT_EN_W<SLP_REJECT_CONF_SPEC> {
        GPIO_REJECT_EN_W::new(self, 24)
    }
    #[doc = "Bit 25 - enable SDIO reject"]
    #[inline(always)]
    #[must_use]
    pub fn sdio_reject_en(&mut self) -> SDIO_REJECT_EN_W<SLP_REJECT_CONF_SPEC> {
        SDIO_REJECT_EN_W::new(self, 25)
    }
    #[doc = "Bit 26 - enable reject for light sleep"]
    #[inline(always)]
    #[must_use]
    pub fn light_slp_reject_en(&mut self) -> LIGHT_SLP_REJECT_EN_W<SLP_REJECT_CONF_SPEC> {
        LIGHT_SLP_REJECT_EN_W::new(self, 26)
    }
    #[doc = "Bit 27 - enable reject for deep sleep"]
    #[inline(always)]
    #[must_use]
    pub fn deep_slp_reject_en(&mut self) -> DEEP_SLP_REJECT_EN_W<SLP_REJECT_CONF_SPEC> {
        DEEP_SLP_REJECT_EN_W::new(self, 27)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slp_reject_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slp_reject_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SLP_REJECT_CONF_SPEC;
impl crate::RegisterSpec for SLP_REJECT_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`slp_reject_conf::R`](R) reader structure"]
impl crate::Readable for SLP_REJECT_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`slp_reject_conf::W`](W) writer structure"]
impl crate::Writable for SLP_REJECT_CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SLP_REJECT_CONF to value 0"]
impl crate::Resettable for SLP_REJECT_CONF_SPEC {
    const RESET_VALUE: u32 = 0;
}
