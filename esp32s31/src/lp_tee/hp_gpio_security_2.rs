#[doc = "Register `HP_GPIO_SECURITY_2` reader"]
pub type R = crate::R<HP_GPIO_SECURITY_2_SPEC>;
#[doc = "Register `HP_GPIO_SECURITY_2` writer"]
pub type W = crate::W<HP_GPIO_SECURITY_2_SPEC>;
#[doc = "Field `HP_GPIO_LOCK_P2` reader - This field decides whether hp_gpio_config of PIN32~62 can be locked, or not. 0 (default): unlocked. 1: locked."]
pub type HP_GPIO_LOCK_P2_R = crate::FieldReader<u32>;
#[doc = "Field `HP_GPIO_LOCK_P2` writer - This field decides whether hp_gpio_config of PIN32~62 can be locked, or not. 0 (default): unlocked. 1: locked."]
pub type HP_GPIO_LOCK_P2_W<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
impl R {
    #[doc = "Bits 0:30 - This field decides whether hp_gpio_config of PIN32~62 can be locked, or not. 0 (default): unlocked. 1: locked."]
    #[inline(always)]
    pub fn hp_gpio_lock_p2(&self) -> HP_GPIO_LOCK_P2_R {
        HP_GPIO_LOCK_P2_R::new(self.bits & 0x7fff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HP_GPIO_SECURITY_2")
            .field("hp_gpio_lock_p2", &self.hp_gpio_lock_p2())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:30 - This field decides whether hp_gpio_config of PIN32~62 can be locked, or not. 0 (default): unlocked. 1: locked."]
    #[inline(always)]
    pub fn hp_gpio_lock_p2(&mut self) -> HP_GPIO_LOCK_P2_W<'_, HP_GPIO_SECURITY_2_SPEC> {
        HP_GPIO_LOCK_P2_W::new(self, 0)
    }
}
#[doc = "need des\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_gpio_security_2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_gpio_security_2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HP_GPIO_SECURITY_2_SPEC;
impl crate::RegisterSpec for HP_GPIO_SECURITY_2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hp_gpio_security_2::R`](R) reader structure"]
impl crate::Readable for HP_GPIO_SECURITY_2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hp_gpio_security_2::W`](W) writer structure"]
impl crate::Writable for HP_GPIO_SECURITY_2_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HP_GPIO_SECURITY_2 to value 0"]
impl crate::Resettable for HP_GPIO_SECURITY_2_SPEC {}
