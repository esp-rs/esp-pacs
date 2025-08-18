#[doc = "Register `STATUS` reader"]
pub type R = crate::R<STATUS_SPEC>;
#[doc = "Register `STATUS` writer"]
pub type W = crate::W<STATUS_SPEC>;
#[doc = "Field `REG_GPIO_STATUS_DATA` reader - Reserved"]
pub type REG_GPIO_STATUS_DATA_R = crate::FieldReader<u16>;
#[doc = "Field `REG_GPIO_STATUS_DATA` writer - Reserved"]
pub type REG_GPIO_STATUS_DATA_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_status_data(&self) -> REG_GPIO_STATUS_DATA_R {
        REG_GPIO_STATUS_DATA_R::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STATUS")
            .field("reg_gpio_status_data", &self.reg_gpio_status_data())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_status_data(&mut self) -> REG_GPIO_STATUS_DATA_W<'_, STATUS_SPEC> {
        REG_GPIO_STATUS_DATA_W::new(self, 0)
    }
}
#[doc = "Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for STATUS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`status::W`](W) writer structure"]
impl crate::Writable for STATUS_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for STATUS_SPEC {}
