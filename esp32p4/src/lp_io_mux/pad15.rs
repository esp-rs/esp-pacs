#[doc = "Register `PAD15` reader"]
pub type R = crate::R<PAD15_SPEC>;
#[doc = "Register `PAD15` writer"]
pub type W = crate::W<PAD15_SPEC>;
#[doc = "Field `REG_PAD15_DRV` reader - Reserved"]
pub type REG_PAD15_DRV_R = crate::FieldReader;
#[doc = "Field `REG_PAD15_DRV` writer - Reserved"]
pub type REG_PAD15_DRV_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `REG_PAD15_RDE` reader - Reserved"]
pub type REG_PAD15_RDE_R = crate::BitReader;
#[doc = "Field `REG_PAD15_RDE` writer - Reserved"]
pub type REG_PAD15_RDE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_PAD15_RUE` reader - Reserved"]
pub type REG_PAD15_RUE_R = crate::BitReader;
#[doc = "Field `REG_PAD15_RUE` writer - Reserved"]
pub type REG_PAD15_RUE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_PAD15_MUX_SEL` reader - 1:use LP GPIO,0: use digital GPIO"]
pub type REG_PAD15_MUX_SEL_R = crate::BitReader;
#[doc = "Field `REG_PAD15_MUX_SEL` writer - 1:use LP GPIO,0: use digital GPIO"]
pub type REG_PAD15_MUX_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_PAD15_FUN_SEL` reader - function sel"]
pub type REG_PAD15_FUN_SEL_R = crate::FieldReader;
#[doc = "Field `REG_PAD15_FUN_SEL` writer - function sel"]
pub type REG_PAD15_FUN_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `REG_PAD15_SLP_SEL` reader - 1: enable sleep mode during sleep,0: no sleep mode"]
pub type REG_PAD15_SLP_SEL_R = crate::BitReader;
#[doc = "Field `REG_PAD15_SLP_SEL` writer - 1: enable sleep mode during sleep,0: no sleep mode"]
pub type REG_PAD15_SLP_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_PAD15_SLP_IE` reader - input enable in sleep mode"]
pub type REG_PAD15_SLP_IE_R = crate::BitReader;
#[doc = "Field `REG_PAD15_SLP_IE` writer - input enable in sleep mode"]
pub type REG_PAD15_SLP_IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_PAD15_SLP_OE` reader - output enable in sleep mode"]
pub type REG_PAD15_SLP_OE_R = crate::BitReader;
#[doc = "Field `REG_PAD15_SLP_OE` writer - output enable in sleep mode"]
pub type REG_PAD15_SLP_OE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_PAD15_FUN_IE` reader - input enable in work mode"]
pub type REG_PAD15_FUN_IE_R = crate::BitReader;
#[doc = "Field `REG_PAD15_FUN_IE` writer - input enable in work mode"]
pub type REG_PAD15_FUN_IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_PAD15_FILTER_EN` reader - need des"]
pub type REG_PAD15_FILTER_EN_R = crate::BitReader;
#[doc = "Field `REG_PAD15_FILTER_EN` writer - need des"]
pub type REG_PAD15_FILTER_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Reserved"]
    #[inline(always)]
    pub fn reg_pad15_drv(&self) -> REG_PAD15_DRV_R {
        REG_PAD15_DRV_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Reserved"]
    #[inline(always)]
    pub fn reg_pad15_rde(&self) -> REG_PAD15_RDE_R {
        REG_PAD15_RDE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Reserved"]
    #[inline(always)]
    pub fn reg_pad15_rue(&self) -> REG_PAD15_RUE_R {
        REG_PAD15_RUE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 1:use LP GPIO,0: use digital GPIO"]
    #[inline(always)]
    pub fn reg_pad15_mux_sel(&self) -> REG_PAD15_MUX_SEL_R {
        REG_PAD15_MUX_SEL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - function sel"]
    #[inline(always)]
    pub fn reg_pad15_fun_sel(&self) -> REG_PAD15_FUN_SEL_R {
        REG_PAD15_FUN_SEL_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - 1: enable sleep mode during sleep,0: no sleep mode"]
    #[inline(always)]
    pub fn reg_pad15_slp_sel(&self) -> REG_PAD15_SLP_SEL_R {
        REG_PAD15_SLP_SEL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - input enable in sleep mode"]
    #[inline(always)]
    pub fn reg_pad15_slp_ie(&self) -> REG_PAD15_SLP_IE_R {
        REG_PAD15_SLP_IE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - output enable in sleep mode"]
    #[inline(always)]
    pub fn reg_pad15_slp_oe(&self) -> REG_PAD15_SLP_OE_R {
        REG_PAD15_SLP_OE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - input enable in work mode"]
    #[inline(always)]
    pub fn reg_pad15_fun_ie(&self) -> REG_PAD15_FUN_IE_R {
        REG_PAD15_FUN_IE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - need des"]
    #[inline(always)]
    pub fn reg_pad15_filter_en(&self) -> REG_PAD15_FILTER_EN_R {
        REG_PAD15_FILTER_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PAD15")
            .field(
                "reg_pad15_drv",
                &format_args!("{}", self.reg_pad15_drv().bits()),
            )
            .field(
                "reg_pad15_rde",
                &format_args!("{}", self.reg_pad15_rde().bit()),
            )
            .field(
                "reg_pad15_rue",
                &format_args!("{}", self.reg_pad15_rue().bit()),
            )
            .field(
                "reg_pad15_mux_sel",
                &format_args!("{}", self.reg_pad15_mux_sel().bit()),
            )
            .field(
                "reg_pad15_fun_sel",
                &format_args!("{}", self.reg_pad15_fun_sel().bits()),
            )
            .field(
                "reg_pad15_slp_sel",
                &format_args!("{}", self.reg_pad15_slp_sel().bit()),
            )
            .field(
                "reg_pad15_slp_ie",
                &format_args!("{}", self.reg_pad15_slp_ie().bit()),
            )
            .field(
                "reg_pad15_slp_oe",
                &format_args!("{}", self.reg_pad15_slp_oe().bit()),
            )
            .field(
                "reg_pad15_fun_ie",
                &format_args!("{}", self.reg_pad15_fun_ie().bit()),
            )
            .field(
                "reg_pad15_filter_en",
                &format_args!("{}", self.reg_pad15_filter_en().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PAD15_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:1 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reg_pad15_drv(&mut self) -> REG_PAD15_DRV_W<PAD15_SPEC> {
        REG_PAD15_DRV_W::new(self, 0)
    }
    #[doc = "Bit 2 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reg_pad15_rde(&mut self) -> REG_PAD15_RDE_W<PAD15_SPEC> {
        REG_PAD15_RDE_W::new(self, 2)
    }
    #[doc = "Bit 3 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reg_pad15_rue(&mut self) -> REG_PAD15_RUE_W<PAD15_SPEC> {
        REG_PAD15_RUE_W::new(self, 3)
    }
    #[doc = "Bit 4 - 1:use LP GPIO,0: use digital GPIO"]
    #[inline(always)]
    #[must_use]
    pub fn reg_pad15_mux_sel(&mut self) -> REG_PAD15_MUX_SEL_W<PAD15_SPEC> {
        REG_PAD15_MUX_SEL_W::new(self, 4)
    }
    #[doc = "Bits 5:6 - function sel"]
    #[inline(always)]
    #[must_use]
    pub fn reg_pad15_fun_sel(&mut self) -> REG_PAD15_FUN_SEL_W<PAD15_SPEC> {
        REG_PAD15_FUN_SEL_W::new(self, 5)
    }
    #[doc = "Bit 7 - 1: enable sleep mode during sleep,0: no sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn reg_pad15_slp_sel(&mut self) -> REG_PAD15_SLP_SEL_W<PAD15_SPEC> {
        REG_PAD15_SLP_SEL_W::new(self, 7)
    }
    #[doc = "Bit 8 - input enable in sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn reg_pad15_slp_ie(&mut self) -> REG_PAD15_SLP_IE_W<PAD15_SPEC> {
        REG_PAD15_SLP_IE_W::new(self, 8)
    }
    #[doc = "Bit 9 - output enable in sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn reg_pad15_slp_oe(&mut self) -> REG_PAD15_SLP_OE_W<PAD15_SPEC> {
        REG_PAD15_SLP_OE_W::new(self, 9)
    }
    #[doc = "Bit 10 - input enable in work mode"]
    #[inline(always)]
    #[must_use]
    pub fn reg_pad15_fun_ie(&mut self) -> REG_PAD15_FUN_IE_W<PAD15_SPEC> {
        REG_PAD15_FUN_IE_W::new(self, 10)
    }
    #[doc = "Bit 11 - need des"]
    #[inline(always)]
    #[must_use]
    pub fn reg_pad15_filter_en(&mut self) -> REG_PAD15_FILTER_EN_W<PAD15_SPEC> {
        REG_PAD15_FILTER_EN_W::new(self, 11)
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
#[doc = "Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pad15::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pad15::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PAD15_SPEC;
impl crate::RegisterSpec for PAD15_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pad15::R`](R) reader structure"]
impl crate::Readable for PAD15_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pad15::W`](W) writer structure"]
impl crate::Writable for PAD15_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PAD15 to value 0x02"]
impl crate::Resettable for PAD15_SPEC {
    const RESET_VALUE: Self::Ux = 0x02;
}
