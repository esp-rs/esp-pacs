#[doc = "Register `PIN6` reader"]
pub type R = crate::R<PIN6_SPEC>;
#[doc = "Register `PIN6` writer"]
pub type W = crate::W<PIN6_SPEC>;
#[doc = "Field `REG_GPIO_PIN6_WAKEUP_ENABLE` reader - Reserved"]
pub type REG_GPIO_PIN6_WAKEUP_ENABLE_R = crate::BitReader;
#[doc = "Field `REG_GPIO_PIN6_WAKEUP_ENABLE` writer - Reserved"]
pub type REG_GPIO_PIN6_WAKEUP_ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_GPIO_PIN6_INT_TYPE` reader - Reserved"]
pub type REG_GPIO_PIN6_INT_TYPE_R = crate::FieldReader;
#[doc = "Field `REG_GPIO_PIN6_INT_TYPE` writer - Reserved"]
pub type REG_GPIO_PIN6_INT_TYPE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `REG_GPIO_PIN6_PAD_DRIVER` reader - Reserved"]
pub type REG_GPIO_PIN6_PAD_DRIVER_R = crate::BitReader;
#[doc = "Field `REG_GPIO_PIN6_PAD_DRIVER` writer - Reserved"]
pub type REG_GPIO_PIN6_PAD_DRIVER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_GPI6_PIN0_EDGE_WAKEUP_CLR` writer - need des"]
pub type REG_GPI6_PIN0_EDGE_WAKEUP_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_pin6_wakeup_enable(&self) -> REG_GPIO_PIN6_WAKEUP_ENABLE_R {
        REG_GPIO_PIN6_WAKEUP_ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_pin6_int_type(&self) -> REG_GPIO_PIN6_INT_TYPE_R {
        REG_GPIO_PIN6_INT_TYPE_R::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bit 4 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_pin6_pad_driver(&self) -> REG_GPIO_PIN6_PAD_DRIVER_R {
        REG_GPIO_PIN6_PAD_DRIVER_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PIN6")
            .field(
                "reg_gpio_pin6_wakeup_enable",
                &format_args!("{}", self.reg_gpio_pin6_wakeup_enable().bit()),
            )
            .field(
                "reg_gpio_pin6_int_type",
                &format_args!("{}", self.reg_gpio_pin6_int_type().bits()),
            )
            .field(
                "reg_gpio_pin6_pad_driver",
                &format_args!("{}", self.reg_gpio_pin6_pad_driver().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PIN6_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_pin6_wakeup_enable(&mut self) -> REG_GPIO_PIN6_WAKEUP_ENABLE_W<PIN6_SPEC> {
        REG_GPIO_PIN6_WAKEUP_ENABLE_W::new(self, 0)
    }
    #[doc = "Bits 1:3 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_pin6_int_type(&mut self) -> REG_GPIO_PIN6_INT_TYPE_W<PIN6_SPEC> {
        REG_GPIO_PIN6_INT_TYPE_W::new(self, 1)
    }
    #[doc = "Bit 4 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_pin6_pad_driver(&mut self) -> REG_GPIO_PIN6_PAD_DRIVER_W<PIN6_SPEC> {
        REG_GPIO_PIN6_PAD_DRIVER_W::new(self, 4)
    }
    #[doc = "Bit 5 - need des"]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpi6_pin0_edge_wakeup_clr(&mut self) -> REG_GPI6_PIN0_EDGE_WAKEUP_CLR_W<PIN6_SPEC> {
        REG_GPI6_PIN0_EDGE_WAKEUP_CLR_W::new(self, 5)
    }
}
#[doc = "Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pin6::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pin6::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PIN6_SPEC;
impl crate::RegisterSpec for PIN6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pin6::R`](R) reader structure"]
impl crate::Readable for PIN6_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pin6::W`](W) writer structure"]
impl crate::Writable for PIN6_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PIN6 to value 0"]
impl crate::Resettable for PIN6_SPEC {
    const RESET_VALUE: u32 = 0;
}
