#[doc = "Register `PAD%s` reader"]
pub type R = crate::R<PAD_SPEC>;
#[doc = "Register `PAD%s` writer"]
pub type W = crate::W<PAD_SPEC>;
#[doc = "Field `DRV` reader - Reserved"]
pub type DRV_R = crate::FieldReader;
#[doc = "Field `DRV` writer - Reserved"]
pub type DRV_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RDE` reader - Reserved"]
pub type RDE_R = crate::BitReader;
#[doc = "Field `RDE` writer - Reserved"]
pub type RDE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RUE` reader - Reserved"]
pub type RUE_R = crate::BitReader;
#[doc = "Field `RUE` writer - Reserved"]
pub type RUE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MUX_SEL` reader - 1:use LP GPIO,0: use digital GPIO"]
pub type MUX_SEL_R = crate::BitReader;
#[doc = "Field `MUX_SEL` writer - 1:use LP GPIO,0: use digital GPIO"]
pub type MUX_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FUN_SEL` reader - function sel"]
pub type FUN_SEL_R = crate::FieldReader;
#[doc = "Field `FUN_SEL` writer - function sel"]
pub type FUN_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SLP_SEL` reader - 1: enable sleep mode during sleep,0: no sleep mode"]
pub type SLP_SEL_R = crate::BitReader;
#[doc = "Field `SLP_SEL` writer - 1: enable sleep mode during sleep,0: no sleep mode"]
pub type SLP_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLP_IE` reader - input enable in sleep mode"]
pub type SLP_IE_R = crate::BitReader;
#[doc = "Field `SLP_IE` writer - input enable in sleep mode"]
pub type SLP_IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLP_OE` reader - output enable in sleep mode"]
pub type SLP_OE_R = crate::BitReader;
#[doc = "Field `SLP_OE` writer - output enable in sleep mode"]
pub type SLP_OE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FUN_IE` reader - input enable in work mode"]
pub type FUN_IE_R = crate::BitReader;
#[doc = "Field `FUN_IE` writer - input enable in work mode"]
pub type FUN_IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FILTER_EN` reader - need des"]
pub type FILTER_EN_R = crate::BitReader;
#[doc = "Field `FILTER_EN` writer - need des"]
pub type FILTER_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Reserved"]
    #[inline(always)]
    pub fn drv(&self) -> DRV_R {
        DRV_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Reserved"]
    #[inline(always)]
    pub fn rde(&self) -> RDE_R {
        RDE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Reserved"]
    #[inline(always)]
    pub fn rue(&self) -> RUE_R {
        RUE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 1:use LP GPIO,0: use digital GPIO"]
    #[inline(always)]
    pub fn mux_sel(&self) -> MUX_SEL_R {
        MUX_SEL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - function sel"]
    #[inline(always)]
    pub fn fun_sel(&self) -> FUN_SEL_R {
        FUN_SEL_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - 1: enable sleep mode during sleep,0: no sleep mode"]
    #[inline(always)]
    pub fn slp_sel(&self) -> SLP_SEL_R {
        SLP_SEL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - input enable in sleep mode"]
    #[inline(always)]
    pub fn slp_ie(&self) -> SLP_IE_R {
        SLP_IE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - output enable in sleep mode"]
    #[inline(always)]
    pub fn slp_oe(&self) -> SLP_OE_R {
        SLP_OE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - input enable in work mode"]
    #[inline(always)]
    pub fn fun_ie(&self) -> FUN_IE_R {
        FUN_IE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - need des"]
    #[inline(always)]
    pub fn filter_en(&self) -> FILTER_EN_R {
        FILTER_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PAD")
            .field("drv", &self.drv())
            .field("rde", &self.rde())
            .field("rue", &self.rue())
            .field("mux_sel", &self.mux_sel())
            .field("fun_sel", &self.fun_sel())
            .field("slp_sel", &self.slp_sel())
            .field("slp_ie", &self.slp_ie())
            .field("slp_oe", &self.slp_oe())
            .field("fun_ie", &self.fun_ie())
            .field("filter_en", &self.filter_en())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - Reserved"]
    #[inline(always)]
    pub fn drv(&mut self) -> DRV_W<'_, PAD_SPEC> {
        DRV_W::new(self, 0)
    }
    #[doc = "Bit 2 - Reserved"]
    #[inline(always)]
    pub fn rde(&mut self) -> RDE_W<'_, PAD_SPEC> {
        RDE_W::new(self, 2)
    }
    #[doc = "Bit 3 - Reserved"]
    #[inline(always)]
    pub fn rue(&mut self) -> RUE_W<'_, PAD_SPEC> {
        RUE_W::new(self, 3)
    }
    #[doc = "Bit 4 - 1:use LP GPIO,0: use digital GPIO"]
    #[inline(always)]
    pub fn mux_sel(&mut self) -> MUX_SEL_W<'_, PAD_SPEC> {
        MUX_SEL_W::new(self, 4)
    }
    #[doc = "Bits 5:6 - function sel"]
    #[inline(always)]
    pub fn fun_sel(&mut self) -> FUN_SEL_W<'_, PAD_SPEC> {
        FUN_SEL_W::new(self, 5)
    }
    #[doc = "Bit 7 - 1: enable sleep mode during sleep,0: no sleep mode"]
    #[inline(always)]
    pub fn slp_sel(&mut self) -> SLP_SEL_W<'_, PAD_SPEC> {
        SLP_SEL_W::new(self, 7)
    }
    #[doc = "Bit 8 - input enable in sleep mode"]
    #[inline(always)]
    pub fn slp_ie(&mut self) -> SLP_IE_W<'_, PAD_SPEC> {
        SLP_IE_W::new(self, 8)
    }
    #[doc = "Bit 9 - output enable in sleep mode"]
    #[inline(always)]
    pub fn slp_oe(&mut self) -> SLP_OE_W<'_, PAD_SPEC> {
        SLP_OE_W::new(self, 9)
    }
    #[doc = "Bit 10 - input enable in work mode"]
    #[inline(always)]
    pub fn fun_ie(&mut self) -> FUN_IE_W<'_, PAD_SPEC> {
        FUN_IE_W::new(self, 10)
    }
    #[doc = "Bit 11 - need des"]
    #[inline(always)]
    pub fn filter_en(&mut self) -> FILTER_EN_W<'_, PAD_SPEC> {
        FILTER_EN_W::new(self, 11)
    }
}
#[doc = "LP IO MUX configuration register for pad %s\n\nYou can [`read`](crate::Reg::read) this register and get [`pad::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PAD_SPEC;
impl crate::RegisterSpec for PAD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pad::R`](R) reader structure"]
impl crate::Readable for PAD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pad::W`](W) writer structure"]
impl crate::Writable for PAD_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PAD%s to value 0"]
impl crate::Resettable for PAD_SPEC {}
