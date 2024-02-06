#[doc = "Register `FUNC3_IN_SEL_CFG` reader"]
pub type R = crate::R<FUNC3_IN_SEL_CFG_SPEC>;
#[doc = "Register `FUNC3_IN_SEL_CFG` writer"]
pub type W = crate::W<FUNC3_IN_SEL_CFG_SPEC>;
#[doc = "Field `REG_GPIO_FUNC3_IN_INV_SEL` reader - Reserved"]
pub type REG_GPIO_FUNC3_IN_INV_SEL_R = crate::BitReader;
#[doc = "Field `REG_GPIO_FUNC3_IN_INV_SEL` writer - Reserved"]
pub type REG_GPIO_FUNC3_IN_INV_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_GPIO_SIG3_IN_SEL` reader - Reserved"]
pub type REG_GPIO_SIG3_IN_SEL_R = crate::BitReader;
#[doc = "Field `REG_GPIO_SIG3_IN_SEL` writer - Reserved"]
pub type REG_GPIO_SIG3_IN_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_GPIO_FUNC3_IN_SEL` reader - Reserved"]
pub type REG_GPIO_FUNC3_IN_SEL_R = crate::FieldReader;
#[doc = "Field `REG_GPIO_FUNC3_IN_SEL` writer - Reserved"]
pub type REG_GPIO_FUNC3_IN_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bit 0 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_func3_in_inv_sel(&self) -> REG_GPIO_FUNC3_IN_INV_SEL_R {
        REG_GPIO_FUNC3_IN_INV_SEL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_sig3_in_sel(&self) -> REG_GPIO_SIG3_IN_SEL_R {
        REG_GPIO_SIG3_IN_SEL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:7 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_func3_in_sel(&self) -> REG_GPIO_FUNC3_IN_SEL_R {
        REG_GPIO_FUNC3_IN_SEL_R::new(((self.bits >> 2) & 0x3f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FUNC3_IN_SEL_CFG")
            .field(
                "reg_gpio_func3_in_inv_sel",
                &format_args!("{}", self.reg_gpio_func3_in_inv_sel().bit()),
            )
            .field(
                "reg_gpio_sig3_in_sel",
                &format_args!("{}", self.reg_gpio_sig3_in_sel().bit()),
            )
            .field(
                "reg_gpio_func3_in_sel",
                &format_args!("{}", self.reg_gpio_func3_in_sel().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<FUNC3_IN_SEL_CFG_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_func3_in_inv_sel(
        &mut self,
    ) -> REG_GPIO_FUNC3_IN_INV_SEL_W<FUNC3_IN_SEL_CFG_SPEC> {
        REG_GPIO_FUNC3_IN_INV_SEL_W::new(self, 0)
    }
    #[doc = "Bit 1 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_sig3_in_sel(&mut self) -> REG_GPIO_SIG3_IN_SEL_W<FUNC3_IN_SEL_CFG_SPEC> {
        REG_GPIO_SIG3_IN_SEL_W::new(self, 1)
    }
    #[doc = "Bits 2:7 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_func3_in_sel(&mut self) -> REG_GPIO_FUNC3_IN_SEL_W<FUNC3_IN_SEL_CFG_SPEC> {
        REG_GPIO_FUNC3_IN_SEL_W::new(self, 2)
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
#[doc = "Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func3_in_sel_cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func3_in_sel_cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FUNC3_IN_SEL_CFG_SPEC;
impl crate::RegisterSpec for FUNC3_IN_SEL_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`func3_in_sel_cfg::R`](R) reader structure"]
impl crate::Readable for FUNC3_IN_SEL_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`func3_in_sel_cfg::W`](W) writer structure"]
impl crate::Writable for FUNC3_IN_SEL_CFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FUNC3_IN_SEL_CFG to value 0xc0"]
impl crate::Resettable for FUNC3_IN_SEL_CFG_SPEC {
    const RESET_VALUE: u32 = 0xc0;
}
