#[doc = "Register `PAD10` reader"]
pub type R = crate::R<PAD10_SPEC>;
#[doc = "Register `PAD10` writer"]
pub type W = crate::W<PAD10_SPEC>;
#[doc = "Field `REG_PAD10_DRV` reader - Reserved"]
pub type REG_PAD10_DRV_R = crate::FieldReader;
#[doc = "Field `REG_PAD10_DRV` writer - Reserved"]
pub type REG_PAD10_DRV_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `REG_PAD10_RDE` reader - Reserved"]
pub type REG_PAD10_RDE_R = crate::BitReader;
#[doc = "Field `REG_PAD10_RDE` writer - Reserved"]
pub type REG_PAD10_RDE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_PAD10_RUE` reader - Reserved"]
pub type REG_PAD10_RUE_R = crate::BitReader;
#[doc = "Field `REG_PAD10_RUE` writer - Reserved"]
pub type REG_PAD10_RUE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_PAD10_MUX_SEL` reader - 1:use LP GPIO,0: use digital GPIO"]
pub type REG_PAD10_MUX_SEL_R = crate::BitReader;
#[doc = "Field `REG_PAD10_MUX_SEL` writer - 1:use LP GPIO,0: use digital GPIO"]
pub type REG_PAD10_MUX_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_PAD10_FUN_SEL` reader - function sel"]
pub type REG_PAD10_FUN_SEL_R = crate::FieldReader;
#[doc = "Field `REG_PAD10_FUN_SEL` writer - function sel"]
pub type REG_PAD10_FUN_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `REG_PAD10_SLP_SEL` reader - 1: enable sleep mode during sleep,0: no sleep mode"]
pub type REG_PAD10_SLP_SEL_R = crate::BitReader;
#[doc = "Field `REG_PAD10_SLP_SEL` writer - 1: enable sleep mode during sleep,0: no sleep mode"]
pub type REG_PAD10_SLP_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_PAD10_SLP_IE` reader - input enable in sleep mode"]
pub type REG_PAD10_SLP_IE_R = crate::BitReader;
#[doc = "Field `REG_PAD10_SLP_IE` writer - input enable in sleep mode"]
pub type REG_PAD10_SLP_IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_PAD10_SLP_OE` reader - output enable in sleep mode"]
pub type REG_PAD10_SLP_OE_R = crate::BitReader;
#[doc = "Field `REG_PAD10_SLP_OE` writer - output enable in sleep mode"]
pub type REG_PAD10_SLP_OE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_PAD10_FUN_IE` reader - input enable in work mode"]
pub type REG_PAD10_FUN_IE_R = crate::BitReader;
#[doc = "Field `REG_PAD10_FUN_IE` writer - input enable in work mode"]
pub type REG_PAD10_FUN_IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_PAD10_FILTER_EN` reader - need des"]
pub type REG_PAD10_FILTER_EN_R = crate::BitReader;
#[doc = "Field `REG_PAD10_FILTER_EN` writer - need des"]
pub type REG_PAD10_FILTER_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Reserved"]
    #[inline(always)]
    pub fn reg_pad10_drv(&self) -> REG_PAD10_DRV_R {
        REG_PAD10_DRV_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Reserved"]
    #[inline(always)]
    pub fn reg_pad10_rde(&self) -> REG_PAD10_RDE_R {
        REG_PAD10_RDE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Reserved"]
    #[inline(always)]
    pub fn reg_pad10_rue(&self) -> REG_PAD10_RUE_R {
        REG_PAD10_RUE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 1:use LP GPIO,0: use digital GPIO"]
    #[inline(always)]
    pub fn reg_pad10_mux_sel(&self) -> REG_PAD10_MUX_SEL_R {
        REG_PAD10_MUX_SEL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - function sel"]
    #[inline(always)]
    pub fn reg_pad10_fun_sel(&self) -> REG_PAD10_FUN_SEL_R {
        REG_PAD10_FUN_SEL_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - 1: enable sleep mode during sleep,0: no sleep mode"]
    #[inline(always)]
    pub fn reg_pad10_slp_sel(&self) -> REG_PAD10_SLP_SEL_R {
        REG_PAD10_SLP_SEL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - input enable in sleep mode"]
    #[inline(always)]
    pub fn reg_pad10_slp_ie(&self) -> REG_PAD10_SLP_IE_R {
        REG_PAD10_SLP_IE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - output enable in sleep mode"]
    #[inline(always)]
    pub fn reg_pad10_slp_oe(&self) -> REG_PAD10_SLP_OE_R {
        REG_PAD10_SLP_OE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - input enable in work mode"]
    #[inline(always)]
    pub fn reg_pad10_fun_ie(&self) -> REG_PAD10_FUN_IE_R {
        REG_PAD10_FUN_IE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - need des"]
    #[inline(always)]
    pub fn reg_pad10_filter_en(&self) -> REG_PAD10_FILTER_EN_R {
        REG_PAD10_FILTER_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PAD10")
            .field("reg_pad10_drv", &self.reg_pad10_drv())
            .field("reg_pad10_rde", &self.reg_pad10_rde())
            .field("reg_pad10_rue", &self.reg_pad10_rue())
            .field("reg_pad10_mux_sel", &self.reg_pad10_mux_sel())
            .field("reg_pad10_fun_sel", &self.reg_pad10_fun_sel())
            .field("reg_pad10_slp_sel", &self.reg_pad10_slp_sel())
            .field("reg_pad10_slp_ie", &self.reg_pad10_slp_ie())
            .field("reg_pad10_slp_oe", &self.reg_pad10_slp_oe())
            .field("reg_pad10_fun_ie", &self.reg_pad10_fun_ie())
            .field("reg_pad10_filter_en", &self.reg_pad10_filter_en())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - Reserved"]
    #[inline(always)]
    pub fn reg_pad10_drv(&mut self) -> REG_PAD10_DRV_W<'_, PAD10_SPEC> {
        REG_PAD10_DRV_W::new(self, 0)
    }
    #[doc = "Bit 2 - Reserved"]
    #[inline(always)]
    pub fn reg_pad10_rde(&mut self) -> REG_PAD10_RDE_W<'_, PAD10_SPEC> {
        REG_PAD10_RDE_W::new(self, 2)
    }
    #[doc = "Bit 3 - Reserved"]
    #[inline(always)]
    pub fn reg_pad10_rue(&mut self) -> REG_PAD10_RUE_W<'_, PAD10_SPEC> {
        REG_PAD10_RUE_W::new(self, 3)
    }
    #[doc = "Bit 4 - 1:use LP GPIO,0: use digital GPIO"]
    #[inline(always)]
    pub fn reg_pad10_mux_sel(&mut self) -> REG_PAD10_MUX_SEL_W<'_, PAD10_SPEC> {
        REG_PAD10_MUX_SEL_W::new(self, 4)
    }
    #[doc = "Bits 5:6 - function sel"]
    #[inline(always)]
    pub fn reg_pad10_fun_sel(&mut self) -> REG_PAD10_FUN_SEL_W<'_, PAD10_SPEC> {
        REG_PAD10_FUN_SEL_W::new(self, 5)
    }
    #[doc = "Bit 7 - 1: enable sleep mode during sleep,0: no sleep mode"]
    #[inline(always)]
    pub fn reg_pad10_slp_sel(&mut self) -> REG_PAD10_SLP_SEL_W<'_, PAD10_SPEC> {
        REG_PAD10_SLP_SEL_W::new(self, 7)
    }
    #[doc = "Bit 8 - input enable in sleep mode"]
    #[inline(always)]
    pub fn reg_pad10_slp_ie(&mut self) -> REG_PAD10_SLP_IE_W<'_, PAD10_SPEC> {
        REG_PAD10_SLP_IE_W::new(self, 8)
    }
    #[doc = "Bit 9 - output enable in sleep mode"]
    #[inline(always)]
    pub fn reg_pad10_slp_oe(&mut self) -> REG_PAD10_SLP_OE_W<'_, PAD10_SPEC> {
        REG_PAD10_SLP_OE_W::new(self, 9)
    }
    #[doc = "Bit 10 - input enable in work mode"]
    #[inline(always)]
    pub fn reg_pad10_fun_ie(&mut self) -> REG_PAD10_FUN_IE_W<'_, PAD10_SPEC> {
        REG_PAD10_FUN_IE_W::new(self, 10)
    }
    #[doc = "Bit 11 - need des"]
    #[inline(always)]
    pub fn reg_pad10_filter_en(&mut self) -> REG_PAD10_FILTER_EN_W<'_, PAD10_SPEC> {
        REG_PAD10_FILTER_EN_W::new(self, 11)
    }
}
#[doc = "Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`pad10::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad10::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PAD10_SPEC;
impl crate::RegisterSpec for PAD10_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pad10::R`](R) reader structure"]
impl crate::Readable for PAD10_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pad10::W`](W) writer structure"]
impl crate::Writable for PAD10_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PAD10 to value 0x02"]
impl crate::Resettable for PAD10_SPEC {
    const RESET_VALUE: u32 = 0x02;
}
