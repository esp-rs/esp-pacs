#[doc = "Register `EXT_WAKEUP_CONF` reader"]
pub type R = crate::R<EXT_WAKEUP_CONF_SPEC>;
#[doc = "Register `EXT_WAKEUP_CONF` writer"]
pub type W = crate::W<EXT_WAKEUP_CONF_SPEC>;
#[doc = "Field `GPIO_WAKEUP_FILTER` reader - enable filter for gpio wakeup event"]
pub type GPIO_WAKEUP_FILTER_R = crate::BitReader;
#[doc = "Field `GPIO_WAKEUP_FILTER` writer - enable filter for gpio wakeup event"]
pub type GPIO_WAKEUP_FILTER_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 31 - enable filter for gpio wakeup event"]
    #[inline(always)]
    pub fn gpio_wakeup_filter(&self) -> GPIO_WAKEUP_FILTER_R {
        GPIO_WAKEUP_FILTER_R::new(((self.bits >> 31) & 1) != 0)
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
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<EXT_WAKEUP_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 31 - enable filter for gpio wakeup event"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_wakeup_filter(&mut self) -> GPIO_WAKEUP_FILTER_W<EXT_WAKEUP_CONF_SPEC> {
        GPIO_WAKEUP_FILTER_W::new(self, 31)
    }
}
#[doc = "rtc configure register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ext_wakeup_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ext_wakeup_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EXT_WAKEUP_CONF_SPEC;
impl crate::RegisterSpec for EXT_WAKEUP_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ext_wakeup_conf::R`](R) reader structure"]
impl crate::Readable for EXT_WAKEUP_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ext_wakeup_conf::W`](W) writer structure"]
impl crate::Writable for EXT_WAKEUP_CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EXT_WAKEUP_CONF to value 0"]
impl crate::Resettable for EXT_WAKEUP_CONF_SPEC {
    const RESET_VALUE: u32 = 0;
}
