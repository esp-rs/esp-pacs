#[doc = "Register `LP_GPIO_SECURITY` reader"]
pub type R = crate::R<LP_GPIO_SECURITY_SPEC>;
#[doc = "Register `LP_GPIO_SECURITY` writer"]
pub type W = crate::W<LP_GPIO_SECURITY_SPEC>;
#[doc = "Field `LP_GPIO_LOCK` reader - This field decides whether lp_gpio_config can be locked, or not. 0 (default): unlocked. 1: locked."]
pub type LP_GPIO_LOCK_R = crate::FieldReader;
#[doc = "Field `LP_GPIO_LOCK` writer - This field decides whether lp_gpio_config can be locked, or not. 0 (default): unlocked. 1: locked."]
pub type LP_GPIO_LOCK_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - This field decides whether lp_gpio_config can be locked, or not. 0 (default): unlocked. 1: locked."]
    #[inline(always)]
    pub fn lp_gpio_lock(&self) -> LP_GPIO_LOCK_R {
        LP_GPIO_LOCK_R::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LP_GPIO_SECURITY")
            .field("lp_gpio_lock", &self.lp_gpio_lock())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - This field decides whether lp_gpio_config can be locked, or not. 0 (default): unlocked. 1: locked."]
    #[inline(always)]
    pub fn lp_gpio_lock(&mut self) -> LP_GPIO_LOCK_W<'_, LP_GPIO_SECURITY_SPEC> {
        LP_GPIO_LOCK_W::new(self, 0)
    }
}
#[doc = "need des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_gpio_security::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_gpio_security::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LP_GPIO_SECURITY_SPEC;
impl crate::RegisterSpec for LP_GPIO_SECURITY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lp_gpio_security::R`](R) reader structure"]
impl crate::Readable for LP_GPIO_SECURITY_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lp_gpio_security::W`](W) writer structure"]
impl crate::Writable for LP_GPIO_SECURITY_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LP_GPIO_SECURITY to value 0"]
impl crate::Resettable for LP_GPIO_SECURITY_SPEC {}
