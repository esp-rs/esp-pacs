#[doc = "Register `GPIO_EXT_VERSION` reader"]
pub type R = crate::R<GPIO_EXT_VERSION_SPEC>;
#[doc = "Register `GPIO_EXT_VERSION` writer"]
pub type W = crate::W<GPIO_EXT_VERSION_SPEC>;
#[doc = "Field `GPIO_EXT_DATE` reader - Version control register."]
pub type GPIO_EXT_DATE_R = crate::FieldReader<u32>;
#[doc = "Field `GPIO_EXT_DATE` writer - Version control register."]
pub type GPIO_EXT_DATE_W<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bits 0:27 - Version control register."]
    #[inline(always)]
    pub fn gpio_ext_date(&self) -> GPIO_EXT_DATE_R {
        GPIO_EXT_DATE_R::new(self.bits & 0x0fff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIO_EXT_VERSION")
            .field("gpio_ext_date", &self.gpio_ext_date())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:27 - Version control register."]
    #[inline(always)]
    pub fn gpio_ext_date(&mut self) -> GPIO_EXT_DATE_W<'_, GPIO_EXT_VERSION_SPEC> {
        GPIO_EXT_DATE_W::new(self, 0)
    }
}
#[doc = "Version Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_ext_version::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_ext_version::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPIO_EXT_VERSION_SPEC;
impl crate::RegisterSpec for GPIO_EXT_VERSION_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio_ext_version::R`](R) reader structure"]
impl crate::Readable for GPIO_EXT_VERSION_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gpio_ext_version::W`](W) writer structure"]
impl crate::Writable for GPIO_EXT_VERSION_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO_EXT_VERSION to value 0x0240_8150"]
impl crate::Resettable for GPIO_EXT_VERSION_SPEC {
    const RESET_VALUE: u32 = 0x0240_8150;
}
