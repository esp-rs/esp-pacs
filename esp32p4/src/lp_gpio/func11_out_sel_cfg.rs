///Register `FUNC11_OUT_SEL_CFG` reader
pub type R = crate::R<FUNC11_OUT_SEL_CFG_SPEC>;
///Register `FUNC11_OUT_SEL_CFG` writer
pub type W = crate::W<FUNC11_OUT_SEL_CFG_SPEC>;
///Field `REG_GPIO_FUNC11_OE_INV_SEL` reader - Reserved
pub type REG_GPIO_FUNC11_OE_INV_SEL_R = crate::BitReader;
///Field `REG_GPIO_FUNC11_OE_INV_SEL` writer - Reserved
pub type REG_GPIO_FUNC11_OE_INV_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `REG_GPIO_FUNC11_OE_SEL` reader - Reserved
pub type REG_GPIO_FUNC11_OE_SEL_R = crate::BitReader;
///Field `REG_GPIO_FUNC11_OE_SEL` writer - Reserved
pub type REG_GPIO_FUNC11_OE_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `REG_GPIO_FUNC11_OUT_INV_SEL` reader - Reserved
pub type REG_GPIO_FUNC11_OUT_INV_SEL_R = crate::BitReader;
///Field `REG_GPIO_FUNC11_OUT_INV_SEL` writer - Reserved
pub type REG_GPIO_FUNC11_OUT_INV_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `REG_GPIO_FUNC11_OUT_SEL` reader - Reserved
pub type REG_GPIO_FUNC11_OUT_SEL_R = crate::FieldReader;
///Field `REG_GPIO_FUNC11_OUT_SEL` writer - Reserved
pub type REG_GPIO_FUNC11_OUT_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    ///Bit 0 - Reserved
    #[inline(always)]
    pub fn reg_gpio_func11_oe_inv_sel(&self) -> REG_GPIO_FUNC11_OE_INV_SEL_R {
        REG_GPIO_FUNC11_OE_INV_SEL_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Reserved
    #[inline(always)]
    pub fn reg_gpio_func11_oe_sel(&self) -> REG_GPIO_FUNC11_OE_SEL_R {
        REG_GPIO_FUNC11_OE_SEL_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Reserved
    #[inline(always)]
    pub fn reg_gpio_func11_out_inv_sel(&self) -> REG_GPIO_FUNC11_OUT_INV_SEL_R {
        REG_GPIO_FUNC11_OUT_INV_SEL_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 3:8 - Reserved
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
    ///Bit 0 - Reserved
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_func11_oe_inv_sel(
        &mut self,
    ) -> REG_GPIO_FUNC11_OE_INV_SEL_W<FUNC11_OUT_SEL_CFG_SPEC> {
        REG_GPIO_FUNC11_OE_INV_SEL_W::new(self, 0)
    }
    ///Bit 1 - Reserved
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_func11_oe_sel(&mut self) -> REG_GPIO_FUNC11_OE_SEL_W<FUNC11_OUT_SEL_CFG_SPEC> {
        REG_GPIO_FUNC11_OE_SEL_W::new(self, 1)
    }
    ///Bit 2 - Reserved
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_func11_out_inv_sel(
        &mut self,
    ) -> REG_GPIO_FUNC11_OUT_INV_SEL_W<FUNC11_OUT_SEL_CFG_SPEC> {
        REG_GPIO_FUNC11_OUT_INV_SEL_W::new(self, 2)
    }
    ///Bits 3:8 - Reserved
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_func11_out_sel(
        &mut self,
    ) -> REG_GPIO_FUNC11_OUT_SEL_W<FUNC11_OUT_SEL_CFG_SPEC> {
        REG_GPIO_FUNC11_OUT_SEL_W::new(self, 3)
    }
}
/**Reserved

You can [`read`](crate::generic::Reg::read) this register and get [`func11_out_sel_cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func11_out_sel_cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct FUNC11_OUT_SEL_CFG_SPEC;
impl crate::RegisterSpec for FUNC11_OUT_SEL_CFG_SPEC {
    type Ux = u32;
}
///`read()` method returns [`func11_out_sel_cfg::R`](R) reader structure
impl crate::Readable for FUNC11_OUT_SEL_CFG_SPEC {}
///`write(|w| ..)` method takes [`func11_out_sel_cfg::W`](W) writer structure
impl crate::Writable for FUNC11_OUT_SEL_CFG_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets FUNC11_OUT_SEL_CFG to value 0x0100
impl crate::Resettable for FUNC11_OUT_SEL_CFG_SPEC {
    const RESET_VALUE: u32 = 0x0100;
}
