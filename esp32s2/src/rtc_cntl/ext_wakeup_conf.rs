#[doc = "Register `EXT_WAKEUP_CONF` reader"]
pub type R = crate::R<EXT_WAKEUP_CONF_SPEC>;
#[doc = "Register `EXT_WAKEUP_CONF` writer"]
pub type W = crate::W<EXT_WAKEUP_CONF_SPEC>;
#[doc = "Field `GPIO_WAKEUP_FILTER` reader - Set this bit to enable the GPIO wakeup event filter."]
pub type GPIO_WAKEUP_FILTER_R = crate::BitReader;
#[doc = "Field `GPIO_WAKEUP_FILTER` writer - Set this bit to enable the GPIO wakeup event filter."]
pub type GPIO_WAKEUP_FILTER_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EXT_WAKEUP0_LV` reader - 0: external wakeup 0 at low level 1: external wakeup 0 at high level"]
pub type EXT_WAKEUP0_LV_R = crate::BitReader;
#[doc = "Field `EXT_WAKEUP0_LV` writer - 0: external wakeup 0 at low level 1: external wakeup 0 at high level"]
pub type EXT_WAKEUP0_LV_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EXT_WAKEUP1_LV` reader - 0: external wakeup 1 at low level 1: external wakeup 1 at high level"]
pub type EXT_WAKEUP1_LV_R = crate::BitReader;
#[doc = "Field `EXT_WAKEUP1_LV` writer - 0: external wakeup 1 at low level 1: external wakeup 1 at high level"]
pub type EXT_WAKEUP1_LV_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 29 - Set this bit to enable the GPIO wakeup event filter."]
    #[inline(always)]
    pub fn gpio_wakeup_filter(&self) -> GPIO_WAKEUP_FILTER_R {
        GPIO_WAKEUP_FILTER_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - 0: external wakeup 0 at low level 1: external wakeup 0 at high level"]
    #[inline(always)]
    pub fn ext_wakeup0_lv(&self) -> EXT_WAKEUP0_LV_R {
        EXT_WAKEUP0_LV_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 0: external wakeup 1 at low level 1: external wakeup 1 at high level"]
    #[inline(always)]
    pub fn ext_wakeup1_lv(&self) -> EXT_WAKEUP1_LV_R {
        EXT_WAKEUP1_LV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EXT_WAKEUP_CONF")
            .field(
                "gpio_wakeup_filter",
                &format_args!("{}", self.gpio_wakeup_filter().bit()),
            )
            .field(
                "ext_wakeup0_lv",
                &format_args!("{}", self.ext_wakeup0_lv().bit()),
            )
            .field(
                "ext_wakeup1_lv",
                &format_args!("{}", self.ext_wakeup1_lv().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<EXT_WAKEUP_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 29 - Set this bit to enable the GPIO wakeup event filter."]
    #[inline(always)]
    #[must_use]
    pub fn gpio_wakeup_filter(&mut self) -> GPIO_WAKEUP_FILTER_W<EXT_WAKEUP_CONF_SPEC, 29> {
        GPIO_WAKEUP_FILTER_W::new(self)
    }
    #[doc = "Bit 30 - 0: external wakeup 0 at low level 1: external wakeup 0 at high level"]
    #[inline(always)]
    #[must_use]
    pub fn ext_wakeup0_lv(&mut self) -> EXT_WAKEUP0_LV_W<EXT_WAKEUP_CONF_SPEC, 30> {
        EXT_WAKEUP0_LV_W::new(self)
    }
    #[doc = "Bit 31 - 0: external wakeup 1 at low level 1: external wakeup 1 at high level"]
    #[inline(always)]
    #[must_use]
    pub fn ext_wakeup1_lv(&mut self) -> EXT_WAKEUP1_LV_W<EXT_WAKEUP_CONF_SPEC, 31> {
        EXT_WAKEUP1_LV_W::new(self)
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
#[doc = "GPIO wakeup configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ext_wakeup_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ext_wakeup_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EXT_WAKEUP_CONF_SPEC;
impl crate::RegisterSpec for EXT_WAKEUP_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ext_wakeup_conf::R`](R) reader structure"]
impl crate::Readable for EXT_WAKEUP_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ext_wakeup_conf::W`](W) writer structure"]
impl crate::Writable for EXT_WAKEUP_CONF_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EXT_WAKEUP_CONF to value 0"]
impl crate::Resettable for EXT_WAKEUP_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
