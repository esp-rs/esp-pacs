#[doc = "Register `TOUCH_PAD5` reader"]
pub type R = crate::R<TOUCH_PAD5_SPEC>;
#[doc = "Register `TOUCH_PAD5` writer"]
pub type W = crate::W<TOUCH_PAD5_SPEC>;
#[doc = "Field `FUN_IE` reader - input enable in work mode"]
pub type FUN_IE_R = crate::BitReader;
#[doc = "Field `FUN_IE` writer - input enable in work mode"]
pub type FUN_IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLP_OE` reader - output enable in sleep mode"]
pub type SLP_OE_R = crate::BitReader;
#[doc = "Field `SLP_OE` writer - output enable in sleep mode"]
pub type SLP_OE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLP_IE` reader - input enable in sleep mode"]
pub type SLP_IE_R = crate::BitReader;
#[doc = "Field `SLP_IE` writer - input enable in sleep mode"]
pub type SLP_IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLP_SEL` reader - 1: enable sleep mode during sleep,0: no sleep mode"]
pub type SLP_SEL_R = crate::BitReader;
#[doc = "Field `SLP_SEL` writer - 1: enable sleep mode during sleep,0: no sleep mode"]
pub type SLP_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FUN_SEL` reader - function sel"]
pub type FUN_SEL_R = crate::FieldReader;
#[doc = "Field `FUN_SEL` writer - function sel"]
pub type FUN_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MUX_SEL` reader - 1: use RTC GPIO,0: use digital GPIO"]
pub type MUX_SEL_R = crate::BitReader;
#[doc = "Field `MUX_SEL` writer - 1: use RTC GPIO,0: use digital GPIO"]
pub type MUX_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XPD` reader - TOUCH_XPD"]
pub type XPD_R = crate::BitReader;
#[doc = "Field `XPD` writer - TOUCH_XPD"]
pub type XPD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIE_OPT` reader - TOUCH_TIE_OPT"]
pub type TIE_OPT_R = crate::BitReader;
#[doc = "Field `TIE_OPT` writer - TOUCH_TIE_OPT"]
pub type TIE_OPT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `START` reader - TOUCH_START"]
pub type START_R = crate::BitReader;
#[doc = "Field `START` writer - TOUCH_START"]
pub type START_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RUE` reader - RUE"]
pub type RUE_R = crate::BitReader;
#[doc = "Field `RUE` writer - RUE"]
pub type RUE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RDE` reader - RDE"]
pub type RDE_R = crate::BitReader;
#[doc = "Field `RDE` writer - RDE"]
pub type RDE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DRV` reader - DRV"]
pub type DRV_R = crate::FieldReader;
#[doc = "Field `DRV` writer - DRV"]
pub type DRV_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 13 - input enable in work mode"]
    #[inline(always)]
    pub fn fun_ie(&self) -> FUN_IE_R {
        FUN_IE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - output enable in sleep mode"]
    #[inline(always)]
    pub fn slp_oe(&self) -> SLP_OE_R {
        SLP_OE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - input enable in sleep mode"]
    #[inline(always)]
    pub fn slp_ie(&self) -> SLP_IE_R {
        SLP_IE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - 1: enable sleep mode during sleep,0: no sleep mode"]
    #[inline(always)]
    pub fn slp_sel(&self) -> SLP_SEL_R {
        SLP_SEL_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:18 - function sel"]
    #[inline(always)]
    pub fn fun_sel(&self) -> FUN_SEL_R {
        FUN_SEL_R::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bit 19 - 1: use RTC GPIO,0: use digital GPIO"]
    #[inline(always)]
    pub fn mux_sel(&self) -> MUX_SEL_R {
        MUX_SEL_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - TOUCH_XPD"]
    #[inline(always)]
    pub fn xpd(&self) -> XPD_R {
        XPD_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - TOUCH_TIE_OPT"]
    #[inline(always)]
    pub fn tie_opt(&self) -> TIE_OPT_R {
        TIE_OPT_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - TOUCH_START"]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 27 - RUE"]
    #[inline(always)]
    pub fn rue(&self) -> RUE_R {
        RUE_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - RDE"]
    #[inline(always)]
    pub fn rde(&self) -> RDE_R {
        RDE_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bits 29:30 - DRV"]
    #[inline(always)]
    pub fn drv(&self) -> DRV_R {
        DRV_R::new(((self.bits >> 29) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TOUCH_PAD5")
            .field("fun_ie", &self.fun_ie().bit())
            .field("slp_oe", &self.slp_oe().bit())
            .field("slp_ie", &self.slp_ie().bit())
            .field("slp_sel", &self.slp_sel().bit())
            .field("fun_sel", &self.fun_sel().bits())
            .field("mux_sel", &self.mux_sel().bit())
            .field("xpd", &self.xpd().bit())
            .field("tie_opt", &self.tie_opt().bit())
            .field("start", &self.start().bit())
            .field("rue", &self.rue().bit())
            .field("rde", &self.rde().bit())
            .field("drv", &self.drv().bits())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TOUCH_PAD5_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 13 - input enable in work mode"]
    #[inline(always)]
    #[must_use]
    pub fn fun_ie(&mut self) -> FUN_IE_W<TOUCH_PAD5_SPEC> {
        FUN_IE_W::new(self, 13)
    }
    #[doc = "Bit 14 - output enable in sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn slp_oe(&mut self) -> SLP_OE_W<TOUCH_PAD5_SPEC> {
        SLP_OE_W::new(self, 14)
    }
    #[doc = "Bit 15 - input enable in sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn slp_ie(&mut self) -> SLP_IE_W<TOUCH_PAD5_SPEC> {
        SLP_IE_W::new(self, 15)
    }
    #[doc = "Bit 16 - 1: enable sleep mode during sleep,0: no sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn slp_sel(&mut self) -> SLP_SEL_W<TOUCH_PAD5_SPEC> {
        SLP_SEL_W::new(self, 16)
    }
    #[doc = "Bits 17:18 - function sel"]
    #[inline(always)]
    #[must_use]
    pub fn fun_sel(&mut self) -> FUN_SEL_W<TOUCH_PAD5_SPEC> {
        FUN_SEL_W::new(self, 17)
    }
    #[doc = "Bit 19 - 1: use RTC GPIO,0: use digital GPIO"]
    #[inline(always)]
    #[must_use]
    pub fn mux_sel(&mut self) -> MUX_SEL_W<TOUCH_PAD5_SPEC> {
        MUX_SEL_W::new(self, 19)
    }
    #[doc = "Bit 20 - TOUCH_XPD"]
    #[inline(always)]
    #[must_use]
    pub fn xpd(&mut self) -> XPD_W<TOUCH_PAD5_SPEC> {
        XPD_W::new(self, 20)
    }
    #[doc = "Bit 21 - TOUCH_TIE_OPT"]
    #[inline(always)]
    #[must_use]
    pub fn tie_opt(&mut self) -> TIE_OPT_W<TOUCH_PAD5_SPEC> {
        TIE_OPT_W::new(self, 21)
    }
    #[doc = "Bit 22 - TOUCH_START"]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> START_W<TOUCH_PAD5_SPEC> {
        START_W::new(self, 22)
    }
    #[doc = "Bit 27 - RUE"]
    #[inline(always)]
    #[must_use]
    pub fn rue(&mut self) -> RUE_W<TOUCH_PAD5_SPEC> {
        RUE_W::new(self, 27)
    }
    #[doc = "Bit 28 - RDE"]
    #[inline(always)]
    #[must_use]
    pub fn rde(&mut self) -> RDE_W<TOUCH_PAD5_SPEC> {
        RDE_W::new(self, 28)
    }
    #[doc = "Bits 29:30 - DRV"]
    #[inline(always)]
    #[must_use]
    pub fn drv(&mut self) -> DRV_W<TOUCH_PAD5_SPEC> {
        DRV_W::new(self, 29)
    }
}
#[doc = "configure RTC PAD5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`touch_pad5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`touch_pad5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TOUCH_PAD5_SPEC;
impl crate::RegisterSpec for TOUCH_PAD5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`touch_pad5::R`](R) reader structure"]
impl crate::Readable for TOUCH_PAD5_SPEC {}
#[doc = "`write(|w| ..)` method takes [`touch_pad5::W`](W) writer structure"]
impl crate::Writable for TOUCH_PAD5_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TOUCH_PAD5 to value 0x5000_0000"]
impl crate::Resettable for TOUCH_PAD5_SPEC {
    const RESET_VALUE: u32 = 0x5000_0000;
}
