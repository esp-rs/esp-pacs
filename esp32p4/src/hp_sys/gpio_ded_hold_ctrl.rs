#[doc = "Register `GPIO_DED_HOLD_CTRL` reader"]
pub type R = crate::R<GPIO_DED_HOLD_CTRL_SPEC>;
#[doc = "Register `GPIO_DED_HOLD_CTRL` writer"]
pub type W = crate::W<GPIO_DED_HOLD_CTRL_SPEC>;
#[doc = "Field `REG_GPIO_DED_HOLD` reader - hold control for gpio63~56"]
pub type REG_GPIO_DED_HOLD_R = crate::FieldReader<u32>;
#[doc = "Field `REG_GPIO_DED_HOLD` writer - hold control for gpio63~56"]
pub type REG_GPIO_DED_HOLD_W<'a, REG> = crate::FieldWriter<'a, REG, 26, u32>;
impl R {
    #[doc = "Bits 0:25 - hold control for gpio63~56"]
    #[inline(always)]
    pub fn reg_gpio_ded_hold(&self) -> REG_GPIO_DED_HOLD_R {
        REG_GPIO_DED_HOLD_R::new(self.bits & 0x03ff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIO_DED_HOLD_CTRL")
            .field(
                "reg_gpio_ded_hold",
                &format_args!("{}", self.reg_gpio_ded_hold().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<GPIO_DED_HOLD_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:25 - hold control for gpio63~56"]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_ded_hold(&mut self) -> REG_GPIO_DED_HOLD_W<GPIO_DED_HOLD_CTRL_SPEC> {
        REG_GPIO_DED_HOLD_W::new(self, 0)
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
#[doc = "NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio_ded_hold_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_ded_hold_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPIO_DED_HOLD_CTRL_SPEC;
impl crate::RegisterSpec for GPIO_DED_HOLD_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio_ded_hold_ctrl::R`](R) reader structure"]
impl crate::Readable for GPIO_DED_HOLD_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gpio_ded_hold_ctrl::W`](W) writer structure"]
impl crate::Writable for GPIO_DED_HOLD_CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GPIO_DED_HOLD_CTRL to value 0"]
impl crate::Resettable for GPIO_DED_HOLD_CTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
