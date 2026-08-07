#[doc = "Register `HP_GPIO_SECURITY_1` reader"]
pub type R = crate::R<HP_GPIO_SECURITY_1_SPEC>;
#[doc = "Register `HP_GPIO_SECURITY_1` writer"]
pub type W = crate::W<HP_GPIO_SECURITY_1_SPEC>;
#[doc = "Field `HP_GPIO_LOCK_P1` reader - This field decides whether hp_gpio_config of PIN0~31 can be locked, or not. 0 (default): unlocked. 1: locked."]
pub type HP_GPIO_LOCK_P1_R = crate::FieldReader<u32>;
#[doc = "Field `HP_GPIO_LOCK_P1` writer - This field decides whether hp_gpio_config of PIN0~31 can be locked, or not. 0 (default): unlocked. 1: locked."]
pub type HP_GPIO_LOCK_P1_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - This field decides whether hp_gpio_config of PIN0~31 can be locked, or not. 0 (default): unlocked. 1: locked."]
    #[inline(always)]
    pub fn hp_gpio_lock_p1(&self) -> HP_GPIO_LOCK_P1_R {
        HP_GPIO_LOCK_P1_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HP_GPIO_SECURITY_1")
            .field("hp_gpio_lock_p1", &self.hp_gpio_lock_p1())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - This field decides whether hp_gpio_config of PIN0~31 can be locked, or not. 0 (default): unlocked. 1: locked."]
    #[inline(always)]
    pub fn hp_gpio_lock_p1(&mut self) -> HP_GPIO_LOCK_P1_W<'_, HP_GPIO_SECURITY_1_SPEC> {
        HP_GPIO_LOCK_P1_W::new(self, 0)
    }
}
#[doc = "need des\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_gpio_security_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_gpio_security_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HP_GPIO_SECURITY_1_SPEC;
impl crate::RegisterSpec for HP_GPIO_SECURITY_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hp_gpio_security_1::R`](R) reader structure"]
impl crate::Readable for HP_GPIO_SECURITY_1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hp_gpio_security_1::W`](W) writer structure"]
impl crate::Writable for HP_GPIO_SECURITY_1_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HP_GPIO_SECURITY_1 to value 0"]
impl crate::Resettable for HP_GPIO_SECURITY_1_SPEC {}
