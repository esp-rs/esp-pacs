#[doc = "Register `GPIO_O_HYS_CTRL0` reader"]
pub type R = crate::R<GPIO_O_HYS_CTRL0_SPEC>;
#[doc = "Register `GPIO_O_HYS_CTRL0` writer"]
pub type W = crate::W<GPIO_O_HYS_CTRL0_SPEC>;
#[doc = "Field `REG_GPIO_0_HYS_LOW` reader - hys control for gpio47~16"]
pub type REG_GPIO_0_HYS_LOW_R = crate::FieldReader<u32>;
#[doc = "Field `REG_GPIO_0_HYS_LOW` writer - hys control for gpio47~16"]
pub type REG_GPIO_0_HYS_LOW_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - hys control for gpio47~16"]
    #[inline(always)]
    pub fn reg_gpio_0_hys_low(&self) -> REG_GPIO_0_HYS_LOW_R {
        REG_GPIO_0_HYS_LOW_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIO_O_HYS_CTRL0")
            .field("reg_gpio_0_hys_low", &self.reg_gpio_0_hys_low())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - hys control for gpio47~16"]
    #[inline(always)]
    pub fn reg_gpio_0_hys_low(&mut self) -> REG_GPIO_0_HYS_LOW_W<GPIO_O_HYS_CTRL0_SPEC> {
        REG_GPIO_0_HYS_LOW_W::new(self, 0)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_o_hys_ctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_o_hys_ctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPIO_O_HYS_CTRL0_SPEC;
impl crate::RegisterSpec for GPIO_O_HYS_CTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio_o_hys_ctrl0::R`](R) reader structure"]
impl crate::Readable for GPIO_O_HYS_CTRL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gpio_o_hys_ctrl0::W`](W) writer structure"]
impl crate::Writable for GPIO_O_HYS_CTRL0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GPIO_O_HYS_CTRL0 to value 0"]
impl crate::Resettable for GPIO_O_HYS_CTRL0_SPEC {
    const RESET_VALUE: u32 = 0;
}
