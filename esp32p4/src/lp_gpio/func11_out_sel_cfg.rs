#[doc = "Register `FUNC11_OUT_SEL_CFG` reader"]
pub type R = crate::R<FUNC11_OUT_SEL_CFG_SPEC>;
#[doc = "Register `FUNC11_OUT_SEL_CFG` writer"]
pub type W = crate::W<FUNC11_OUT_SEL_CFG_SPEC>;
#[doc = "Field `REG_GPIO_FUNC11_OE_INV_SEL` reader - Reserved"]
pub type REG_GPIO_FUNC11_OE_INV_SEL_R = crate::BitReader;
#[doc = "Field `REG_GPIO_FUNC11_OE_INV_SEL` writer - Reserved"]
pub type REG_GPIO_FUNC11_OE_INV_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_GPIO_FUNC11_OE_SEL` reader - Reserved"]
pub type REG_GPIO_FUNC11_OE_SEL_R = crate::BitReader;
#[doc = "Field `REG_GPIO_FUNC11_OE_SEL` writer - Reserved"]
pub type REG_GPIO_FUNC11_OE_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_GPIO_FUNC11_OUT_INV_SEL` reader - Reserved"]
pub type REG_GPIO_FUNC11_OUT_INV_SEL_R = crate::BitReader;
#[doc = "Field `REG_GPIO_FUNC11_OUT_INV_SEL` writer - Reserved"]
pub type REG_GPIO_FUNC11_OUT_INV_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_GPIO_FUNC11_OUT_SEL` reader - Reserved"]
pub type REG_GPIO_FUNC11_OUT_SEL_R = crate::FieldReader;
#[doc = "Field `REG_GPIO_FUNC11_OUT_SEL` writer - Reserved"]
pub type REG_GPIO_FUNC11_OUT_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bit 0 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_func11_oe_inv_sel(&self) -> REG_GPIO_FUNC11_OE_INV_SEL_R {
        REG_GPIO_FUNC11_OE_INV_SEL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_func11_oe_sel(&self) -> REG_GPIO_FUNC11_OE_SEL_R {
        REG_GPIO_FUNC11_OE_SEL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_func11_out_inv_sel(&self) -> REG_GPIO_FUNC11_OUT_INV_SEL_R {
        REG_GPIO_FUNC11_OUT_INV_SEL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:8 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_func11_out_sel(&self) -> REG_GPIO_FUNC11_OUT_SEL_R {
        REG_GPIO_FUNC11_OUT_SEL_R::new(((self.bits >> 3) & 0x3f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FUNC11_OUT_SEL_CFG")
            .field(
                "reg_gpio_func11_oe_inv_sel",
                &self.reg_gpio_func11_oe_inv_sel(),
            )
            .field("reg_gpio_func11_oe_sel", &self.reg_gpio_func11_oe_sel())
            .field(
                "reg_gpio_func11_out_inv_sel",
                &self.reg_gpio_func11_out_inv_sel(),
            )
            .field("reg_gpio_func11_out_sel", &self.reg_gpio_func11_out_sel())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_func11_oe_inv_sel(
        &mut self,
    ) -> REG_GPIO_FUNC11_OE_INV_SEL_W<FUNC11_OUT_SEL_CFG_SPEC> {
        REG_GPIO_FUNC11_OE_INV_SEL_W::new(self, 0)
    }
    #[doc = "Bit 1 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_func11_oe_sel(&mut self) -> REG_GPIO_FUNC11_OE_SEL_W<FUNC11_OUT_SEL_CFG_SPEC> {
        REG_GPIO_FUNC11_OE_SEL_W::new(self, 1)
    }
    #[doc = "Bit 2 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_func11_out_inv_sel(
        &mut self,
    ) -> REG_GPIO_FUNC11_OUT_INV_SEL_W<FUNC11_OUT_SEL_CFG_SPEC> {
        REG_GPIO_FUNC11_OUT_INV_SEL_W::new(self, 2)
    }
    #[doc = "Bits 3:8 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_func11_out_sel(
        &mut self,
    ) -> REG_GPIO_FUNC11_OUT_SEL_W<FUNC11_OUT_SEL_CFG_SPEC> {
        REG_GPIO_FUNC11_OUT_SEL_W::new(self, 3)
    }
}
#[doc = "Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`func11_out_sel_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`func11_out_sel_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FUNC11_OUT_SEL_CFG_SPEC;
impl crate::RegisterSpec for FUNC11_OUT_SEL_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`func11_out_sel_cfg::R`](R) reader structure"]
impl crate::Readable for FUNC11_OUT_SEL_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`func11_out_sel_cfg::W`](W) writer structure"]
impl crate::Writable for FUNC11_OUT_SEL_CFG_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FUNC11_OUT_SEL_CFG to value 0x0100"]
impl crate::Resettable for FUNC11_OUT_SEL_CFG_SPEC {
    const RESET_VALUE: u32 = 0x0100;
}
