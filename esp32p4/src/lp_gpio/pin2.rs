///Register `PIN2` reader
pub type R = crate::R<PIN2_SPEC>;
///Register `PIN2` writer
pub type W = crate::W<PIN2_SPEC>;
///Field `REG_GPIO_PIN2_WAKEUP_ENABLE` reader - Reserved
pub type REG_GPIO_PIN2_WAKEUP_ENABLE_R = crate::BitReader;
///Field `REG_GPIO_PIN2_WAKEUP_ENABLE` writer - Reserved
pub type REG_GPIO_PIN2_WAKEUP_ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `REG_GPIO_PIN2_INT_TYPE` reader - Reserved
pub type REG_GPIO_PIN2_INT_TYPE_R = crate::FieldReader;
///Field `REG_GPIO_PIN2_INT_TYPE` writer - Reserved
pub type REG_GPIO_PIN2_INT_TYPE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `REG_GPIO_PIN2_PAD_DRIVER` reader - Reserved
pub type REG_GPIO_PIN2_PAD_DRIVER_R = crate::BitReader;
///Field `REG_GPIO_PIN2_PAD_DRIVER` writer - Reserved
pub type REG_GPIO_PIN2_PAD_DRIVER_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `REG_GPI2_PIN0_EDGE_WAKEUP_CLR` writer - need des
pub type REG_GPI2_PIN0_EDGE_WAKEUP_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Reserved
    #[inline(always)]
    pub fn reg_gpio_pin2_wakeup_enable(&self) -> REG_GPIO_PIN2_WAKEUP_ENABLE_R {
        REG_GPIO_PIN2_WAKEUP_ENABLE_R::new((self.bits & 1) != 0)
    }
    ///Bits 1:3 - Reserved
    #[inline(always)]
    pub fn reg_gpio_pin2_int_type(&self) -> REG_GPIO_PIN2_INT_TYPE_R {
        REG_GPIO_PIN2_INT_TYPE_R::new(((self.bits >> 1) & 7) as u8)
    }
    ///Bit 4 - Reserved
    #[inline(always)]
    pub fn reg_gpio_pin2_pad_driver(&self) -> REG_GPIO_PIN2_PAD_DRIVER_R {
        REG_GPIO_PIN2_PAD_DRIVER_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PIN2")
            .field(
                "reg_gpio_pin2_wakeup_enable",
                &self.reg_gpio_pin2_wakeup_enable(),
            )
            .field("reg_gpio_pin2_int_type", &self.reg_gpio_pin2_int_type())
            .field("reg_gpio_pin2_pad_driver", &self.reg_gpio_pin2_pad_driver())
            .finish()
    }
}
impl W {
    ///Bit 0 - Reserved
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_pin2_wakeup_enable(&mut self) -> REG_GPIO_PIN2_WAKEUP_ENABLE_W<PIN2_SPEC> {
        REG_GPIO_PIN2_WAKEUP_ENABLE_W::new(self, 0)
    }
    ///Bits 1:3 - Reserved
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_pin2_int_type(&mut self) -> REG_GPIO_PIN2_INT_TYPE_W<PIN2_SPEC> {
        REG_GPIO_PIN2_INT_TYPE_W::new(self, 1)
    }
    ///Bit 4 - Reserved
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_pin2_pad_driver(&mut self) -> REG_GPIO_PIN2_PAD_DRIVER_W<PIN2_SPEC> {
        REG_GPIO_PIN2_PAD_DRIVER_W::new(self, 4)
    }
    ///Bit 5 - need des
    #[inline(always)]
    #[must_use]
    pub fn reg_gpi2_pin0_edge_wakeup_clr(&mut self) -> REG_GPI2_PIN0_EDGE_WAKEUP_CLR_W<PIN2_SPEC> {
        REG_GPI2_PIN0_EDGE_WAKEUP_CLR_W::new(self, 5)
    }
}
/**Reserved

You can [`read`](crate::generic::Reg::read) this register and get [`pin2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pin2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PIN2_SPEC;
impl crate::RegisterSpec for PIN2_SPEC {
    type Ux = u32;
}
///`read()` method returns [`pin2::R`](R) reader structure
impl crate::Readable for PIN2_SPEC {}
///`write(|w| ..)` method takes [`pin2::W`](W) writer structure
impl crate::Writable for PIN2_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets PIN2 to value 0
impl crate::Resettable for PIN2_SPEC {
    const RESET_VALUE: u32 = 0;
}
