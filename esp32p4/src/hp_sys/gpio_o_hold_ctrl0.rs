#[doc = "Register `GPIO_O_HOLD_CTRL0` reader"]
pub type R = crate::R<GPIO_O_HOLD_CTRL0_SPEC>;
#[doc = "Register `GPIO_O_HOLD_CTRL0` writer"]
pub type W = crate::W<GPIO_O_HOLD_CTRL0_SPEC>;
#[doc = "Field `REG_GPIO_0_HOLD_LOW` reader - hold control for gpio47~16"]
pub type REG_GPIO_0_HOLD_LOW_R = crate::FieldReader<u32>;
#[doc = "Field `REG_GPIO_0_HOLD_LOW` writer - hold control for gpio47~16"]
pub type REG_GPIO_0_HOLD_LOW_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - hold control for gpio47~16"]
    #[inline(always)]
    pub fn reg_gpio_0_hold_low(&self) -> REG_GPIO_0_HOLD_LOW_R {
        REG_GPIO_0_HOLD_LOW_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIO_O_HOLD_CTRL0")
            .field(
                "reg_gpio_0_hold_low",
                &format_args!("{}", self.reg_gpio_0_hold_low().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<GPIO_O_HOLD_CTRL0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31 - hold control for gpio47~16"]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_0_hold_low(&mut self) -> REG_GPIO_0_HOLD_LOW_W<GPIO_O_HOLD_CTRL0_SPEC> {
        REG_GPIO_0_HOLD_LOW_W::new(self, 0)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio_o_hold_ctrl0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_o_hold_ctrl0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPIO_O_HOLD_CTRL0_SPEC;
impl crate::RegisterSpec for GPIO_O_HOLD_CTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio_o_hold_ctrl0::R`](R) reader structure"]
impl crate::Readable for GPIO_O_HOLD_CTRL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gpio_o_hold_ctrl0::W`](W) writer structure"]
impl crate::Writable for GPIO_O_HOLD_CTRL0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GPIO_O_HOLD_CTRL0 to value 0"]
impl crate::Resettable for GPIO_O_HOLD_CTRL0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
