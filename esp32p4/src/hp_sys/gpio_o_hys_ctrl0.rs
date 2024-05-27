///Register `GPIO_O_HYS_CTRL0` reader
pub type R = crate::R<GPIO_O_HYS_CTRL0_SPEC>;
///Register `GPIO_O_HYS_CTRL0` writer
pub type W = crate::W<GPIO_O_HYS_CTRL0_SPEC>;
///Field `REG_GPIO_0_HYS_LOW` reader - hys control for gpio47~16
pub type REG_GPIO_0_HYS_LOW_R = crate::FieldReader<u32>;
///Field `REG_GPIO_0_HYS_LOW` writer - hys control for gpio47~16
pub type REG_GPIO_0_HYS_LOW_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - hys control for gpio47~16
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
    ///Bits 0:31 - hys control for gpio47~16
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_0_hys_low(&mut self) -> REG_GPIO_0_HYS_LOW_W<GPIO_O_HYS_CTRL0_SPEC> {
        REG_GPIO_0_HYS_LOW_W::new(self, 0)
    }
}
/**NA

You can [`read`](crate::generic::Reg::read) this register and get [`gpio_o_hys_ctrl0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_o_hys_ctrl0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct GPIO_O_HYS_CTRL0_SPEC;
impl crate::RegisterSpec for GPIO_O_HYS_CTRL0_SPEC {
    type Ux = u32;
}
///`read()` method returns [`gpio_o_hys_ctrl0::R`](R) reader structure
impl crate::Readable for GPIO_O_HYS_CTRL0_SPEC {}
///`write(|w| ..)` method takes [`gpio_o_hys_ctrl0::W`](W) writer structure
impl crate::Writable for GPIO_O_HYS_CTRL0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets GPIO_O_HYS_CTRL0 to value 0
impl crate::Resettable for GPIO_O_HYS_CTRL0_SPEC {
    const RESET_VALUE: u32 = 0;
}
