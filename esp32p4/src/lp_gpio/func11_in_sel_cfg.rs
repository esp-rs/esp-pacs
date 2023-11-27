#[doc = "Register `FUNC11_IN_SEL_CFG` reader"]
pub type R = crate::R<FUNC11_IN_SEL_CFG_SPEC>;
#[doc = "Register `FUNC11_IN_SEL_CFG` writer"]
pub type W = crate::W<FUNC11_IN_SEL_CFG_SPEC>;
#[doc = "Field `REG_GPIO_FUNC11_IN_INV_SEL` reader - Reserved"]
pub type REG_GPIO_FUNC11_IN_INV_SEL_R = crate::BitReader;
#[doc = "Field `REG_GPIO_FUNC11_IN_INV_SEL` writer - Reserved"]
pub type REG_GPIO_FUNC11_IN_INV_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_GPIO_SIG11_IN_SEL` reader - Reserved"]
pub type REG_GPIO_SIG11_IN_SEL_R = crate::BitReader;
#[doc = "Field `REG_GPIO_SIG11_IN_SEL` writer - Reserved"]
pub type REG_GPIO_SIG11_IN_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_GPIO_FUNC11_IN_SEL` reader - Reserved"]
pub type REG_GPIO_FUNC11_IN_SEL_R = crate::FieldReader;
#[doc = "Field `REG_GPIO_FUNC11_IN_SEL` writer - Reserved"]
pub type REG_GPIO_FUNC11_IN_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bit 0 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_func11_in_inv_sel(&self) -> REG_GPIO_FUNC11_IN_INV_SEL_R {
        REG_GPIO_FUNC11_IN_INV_SEL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_sig11_in_sel(&self) -> REG_GPIO_SIG11_IN_SEL_R {
        REG_GPIO_SIG11_IN_SEL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:7 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_func11_in_sel(&self) -> REG_GPIO_FUNC11_IN_SEL_R {
        REG_GPIO_FUNC11_IN_SEL_R::new(((self.bits >> 2) & 0x3f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FUNC11_IN_SEL_CFG")
            .field(
                "reg_gpio_func11_in_inv_sel",
                &format_args!("{}", self.reg_gpio_func11_in_inv_sel().bit()),
            )
            .field(
                "reg_gpio_sig11_in_sel",
                &format_args!("{}", self.reg_gpio_sig11_in_sel().bit()),
            )
            .field(
                "reg_gpio_func11_in_sel",
                &format_args!("{}", self.reg_gpio_func11_in_sel().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<FUNC11_IN_SEL_CFG_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_func11_in_inv_sel(
        &mut self,
    ) -> REG_GPIO_FUNC11_IN_INV_SEL_W<FUNC11_IN_SEL_CFG_SPEC> {
        REG_GPIO_FUNC11_IN_INV_SEL_W::new(self, 0)
    }
    #[doc = "Bit 1 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_sig11_in_sel(&mut self) -> REG_GPIO_SIG11_IN_SEL_W<FUNC11_IN_SEL_CFG_SPEC> {
        REG_GPIO_SIG11_IN_SEL_W::new(self, 1)
    }
    #[doc = "Bits 2:7 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_func11_in_sel(&mut self) -> REG_GPIO_FUNC11_IN_SEL_W<FUNC11_IN_SEL_CFG_SPEC> {
        REG_GPIO_FUNC11_IN_SEL_W::new(self, 2)
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
#[doc = "Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func11_in_sel_cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func11_in_sel_cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FUNC11_IN_SEL_CFG_SPEC;
impl crate::RegisterSpec for FUNC11_IN_SEL_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`func11_in_sel_cfg::R`](R) reader structure"]
impl crate::Readable for FUNC11_IN_SEL_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`func11_in_sel_cfg::W`](W) writer structure"]
impl crate::Writable for FUNC11_IN_SEL_CFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FUNC11_IN_SEL_CFG to value 0x80"]
impl crate::Resettable for FUNC11_IN_SEL_CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0x80;
}
