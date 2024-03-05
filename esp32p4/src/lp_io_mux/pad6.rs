#[doc = "Register `PAD6` reader"]
pub type R = crate::R<PAD6_SPEC>;
#[doc = "Register `PAD6` writer"]
pub type W = crate::W<PAD6_SPEC>;
#[doc = "Field `REG_PAD6_DRV` reader - Reserved"]
pub type REG_PAD6_DRV_R = crate::FieldReader;
#[doc = "Field `REG_PAD6_DRV` writer - Reserved"]
pub type REG_PAD6_DRV_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `REG_PAD6_RDE` reader - Reserved"]
pub type REG_PAD6_RDE_R = crate::BitReader;
#[doc = "Field `REG_PAD6_RDE` writer - Reserved"]
pub type REG_PAD6_RDE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_PAD6_RUE` reader - Reserved"]
pub type REG_PAD6_RUE_R = crate::BitReader;
#[doc = "Field `REG_PAD6_RUE` writer - Reserved"]
pub type REG_PAD6_RUE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_PAD6_MUX_SEL` reader - 1:use LP GPIO,0: use digital GPIO"]
pub type REG_PAD6_MUX_SEL_R = crate::BitReader;
#[doc = "Field `REG_PAD6_MUX_SEL` writer - 1:use LP GPIO,0: use digital GPIO"]
pub type REG_PAD6_MUX_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_PAD6_FUN_SEL` reader - function sel"]
pub type REG_PAD6_FUN_SEL_R = crate::FieldReader;
#[doc = "Field `REG_PAD6_FUN_SEL` writer - function sel"]
pub type REG_PAD6_FUN_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `REG_PAD6_SLP_SEL` reader - 1: enable sleep mode during sleep,0: no sleep mode"]
pub type REG_PAD6_SLP_SEL_R = crate::BitReader;
#[doc = "Field `REG_PAD6_SLP_SEL` writer - 1: enable sleep mode during sleep,0: no sleep mode"]
pub type REG_PAD6_SLP_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_PAD6_SLP_IE` reader - input enable in sleep mode"]
pub type REG_PAD6_SLP_IE_R = crate::BitReader;
#[doc = "Field `REG_PAD6_SLP_IE` writer - input enable in sleep mode"]
pub type REG_PAD6_SLP_IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_PAD6_SLP_OE` reader - output enable in sleep mode"]
pub type REG_PAD6_SLP_OE_R = crate::BitReader;
#[doc = "Field `REG_PAD6_SLP_OE` writer - output enable in sleep mode"]
pub type REG_PAD6_SLP_OE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_PAD6_FUN_IE` reader - input enable in work mode"]
pub type REG_PAD6_FUN_IE_R = crate::BitReader;
#[doc = "Field `REG_PAD6_FUN_IE` writer - input enable in work mode"]
pub type REG_PAD6_FUN_IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_PAD6_FILTER_EN` reader - need des"]
pub type REG_PAD6_FILTER_EN_R = crate::BitReader;
#[doc = "Field `REG_PAD6_FILTER_EN` writer - need des"]
pub type REG_PAD6_FILTER_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Reserved"]
    #[inline(always)]
    pub fn reg_pad6_drv(&self) -> REG_PAD6_DRV_R {
        REG_PAD6_DRV_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Reserved"]
    #[inline(always)]
    pub fn reg_pad6_rde(&self) -> REG_PAD6_RDE_R {
        REG_PAD6_RDE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Reserved"]
    #[inline(always)]
    pub fn reg_pad6_rue(&self) -> REG_PAD6_RUE_R {
        REG_PAD6_RUE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 1:use LP GPIO,0: use digital GPIO"]
    #[inline(always)]
    pub fn reg_pad6_mux_sel(&self) -> REG_PAD6_MUX_SEL_R {
        REG_PAD6_MUX_SEL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - function sel"]
    #[inline(always)]
    pub fn reg_pad6_fun_sel(&self) -> REG_PAD6_FUN_SEL_R {
        REG_PAD6_FUN_SEL_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - 1: enable sleep mode during sleep,0: no sleep mode"]
    #[inline(always)]
    pub fn reg_pad6_slp_sel(&self) -> REG_PAD6_SLP_SEL_R {
        REG_PAD6_SLP_SEL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - input enable in sleep mode"]
    #[inline(always)]
    pub fn reg_pad6_slp_ie(&self) -> REG_PAD6_SLP_IE_R {
        REG_PAD6_SLP_IE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - output enable in sleep mode"]
    #[inline(always)]
    pub fn reg_pad6_slp_oe(&self) -> REG_PAD6_SLP_OE_R {
        REG_PAD6_SLP_OE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - input enable in work mode"]
    #[inline(always)]
    pub fn reg_pad6_fun_ie(&self) -> REG_PAD6_FUN_IE_R {
        REG_PAD6_FUN_IE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - need des"]
    #[inline(always)]
    pub fn reg_pad6_filter_en(&self) -> REG_PAD6_FILTER_EN_R {
        REG_PAD6_FILTER_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PAD6")
            .field(
                "reg_pad6_drv",
                &format_args!("{}", self.reg_pad6_drv().bits()),
            )
            .field(
                "reg_pad6_rde",
                &format_args!("{}", self.reg_pad6_rde().bit()),
            )
            .field(
                "reg_pad6_rue",
                &format_args!("{}", self.reg_pad6_rue().bit()),
            )
            .field(
                "reg_pad6_mux_sel",
                &format_args!("{}", self.reg_pad6_mux_sel().bit()),
            )
            .field(
                "reg_pad6_fun_sel",
                &format_args!("{}", self.reg_pad6_fun_sel().bits()),
            )
            .field(
                "reg_pad6_slp_sel",
                &format_args!("{}", self.reg_pad6_slp_sel().bit()),
            )
            .field(
                "reg_pad6_slp_ie",
                &format_args!("{}", self.reg_pad6_slp_ie().bit()),
            )
            .field(
                "reg_pad6_slp_oe",
                &format_args!("{}", self.reg_pad6_slp_oe().bit()),
            )
            .field(
                "reg_pad6_fun_ie",
                &format_args!("{}", self.reg_pad6_fun_ie().bit()),
            )
            .field(
                "reg_pad6_filter_en",
                &format_args!("{}", self.reg_pad6_filter_en().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PAD6_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:1 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reg_pad6_drv(&mut self) -> REG_PAD6_DRV_W<PAD6_SPEC> {
        REG_PAD6_DRV_W::new(self, 0)
    }
    #[doc = "Bit 2 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reg_pad6_rde(&mut self) -> REG_PAD6_RDE_W<PAD6_SPEC> {
        REG_PAD6_RDE_W::new(self, 2)
    }
    #[doc = "Bit 3 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reg_pad6_rue(&mut self) -> REG_PAD6_RUE_W<PAD6_SPEC> {
        REG_PAD6_RUE_W::new(self, 3)
    }
    #[doc = "Bit 4 - 1:use LP GPIO,0: use digital GPIO"]
    #[inline(always)]
    #[must_use]
    pub fn reg_pad6_mux_sel(&mut self) -> REG_PAD6_MUX_SEL_W<PAD6_SPEC> {
        REG_PAD6_MUX_SEL_W::new(self, 4)
    }
    #[doc = "Bits 5:6 - function sel"]
    #[inline(always)]
    #[must_use]
    pub fn reg_pad6_fun_sel(&mut self) -> REG_PAD6_FUN_SEL_W<PAD6_SPEC> {
        REG_PAD6_FUN_SEL_W::new(self, 5)
    }
    #[doc = "Bit 7 - 1: enable sleep mode during sleep,0: no sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn reg_pad6_slp_sel(&mut self) -> REG_PAD6_SLP_SEL_W<PAD6_SPEC> {
        REG_PAD6_SLP_SEL_W::new(self, 7)
    }
    #[doc = "Bit 8 - input enable in sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn reg_pad6_slp_ie(&mut self) -> REG_PAD6_SLP_IE_W<PAD6_SPEC> {
        REG_PAD6_SLP_IE_W::new(self, 8)
    }
    #[doc = "Bit 9 - output enable in sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn reg_pad6_slp_oe(&mut self) -> REG_PAD6_SLP_OE_W<PAD6_SPEC> {
        REG_PAD6_SLP_OE_W::new(self, 9)
    }
    #[doc = "Bit 10 - input enable in work mode"]
    #[inline(always)]
    #[must_use]
    pub fn reg_pad6_fun_ie(&mut self) -> REG_PAD6_FUN_IE_W<PAD6_SPEC> {
        REG_PAD6_FUN_IE_W::new(self, 10)
    }
    #[doc = "Bit 11 - need des"]
    #[inline(always)]
    #[must_use]
    pub fn reg_pad6_filter_en(&mut self) -> REG_PAD6_FILTER_EN_W<PAD6_SPEC> {
        REG_PAD6_FILTER_EN_W::new(self, 11)
    }
}
#[doc = "Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pad6::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pad6::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PAD6_SPEC;
impl crate::RegisterSpec for PAD6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pad6::R`](R) reader structure"]
impl crate::Readable for PAD6_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pad6::W`](W) writer structure"]
impl crate::Writable for PAD6_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PAD6 to value 0x02"]
impl crate::Resettable for PAD6_SPEC {
    const RESET_VALUE: u32 = 0x02;
}
