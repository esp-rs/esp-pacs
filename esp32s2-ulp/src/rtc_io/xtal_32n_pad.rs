#[doc = "Register `XTAL_32N_PAD` reader"]
pub type R = crate::R<XTAL_32N_PAD_SPEC>;
#[doc = "Register `XTAL_32N_PAD` writer"]
pub type W = crate::W<XTAL_32N_PAD_SPEC>;
#[doc = "Field `X32N_FUN_IE` reader - Input enable in normal execution."]
pub type X32N_FUN_IE_R = crate::BitReader;
#[doc = "Field `X32N_FUN_IE` writer - Input enable in normal execution."]
pub type X32N_FUN_IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `X32N_SLP_OE` reader - Output enable in sleep mode."]
pub type X32N_SLP_OE_R = crate::BitReader;
#[doc = "Field `X32N_SLP_OE` writer - Output enable in sleep mode."]
pub type X32N_SLP_OE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `X32N_SLP_IE` reader - Input enable in sleep mode."]
pub type X32N_SLP_IE_R = crate::BitReader;
#[doc = "Field `X32N_SLP_IE` writer - Input enable in sleep mode."]
pub type X32N_SLP_IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `X32N_SLP_SEL` reader - 1: enable sleep mode. 0: no sleep mode."]
pub type X32N_SLP_SEL_R = crate::BitReader;
#[doc = "Field `X32N_SLP_SEL` writer - 1: enable sleep mode. 0: no sleep mode."]
pub type X32N_SLP_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `X32N_FUN_SEL` reader - Function selection."]
pub type X32N_FUN_SEL_R = crate::FieldReader;
#[doc = "Field `X32N_FUN_SEL` writer - Function selection."]
pub type X32N_FUN_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `X32N_MUX_SEL` reader - 1: use RTC GPIO. 0: use digital GPIO."]
pub type X32N_MUX_SEL_R = crate::BitReader;
#[doc = "Field `X32N_MUX_SEL` writer - 1: use RTC GPIO. 0: use digital GPIO."]
pub type X32N_MUX_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `X32N_RUE` reader - Pull-down enable of the pad. 1: internal pull-down enabled. 0: internal pull-down disabled."]
pub type X32N_RUE_R = crate::BitReader;
#[doc = "Field `X32N_RUE` writer - Pull-down enable of the pad. 1: internal pull-down enabled. 0: internal pull-down disabled."]
pub type X32N_RUE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `X32N_RDE` reader - Pull-up enable of the pad. 1: internal pull-up enabled. 0: internal pull-up disabled."]
pub type X32N_RDE_R = crate::BitReader;
#[doc = "Field `X32N_RDE` writer - Pull-up enable of the pad. 1: internal pull-up enabled. 0: internal pull-up disabled."]
pub type X32N_RDE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `X32N_DRV` reader - Select the drive strength of the pad. 0: ~5 mA. 1: ~10 mA. 2: ~20 mA. 3: ~40 mA."]
pub type X32N_DRV_R = crate::FieldReader;
#[doc = "Field `X32N_DRV` writer - Select the drive strength of the pad. 0: ~5 mA. 1: ~10 mA. 2: ~20 mA. 3: ~40 mA."]
pub type X32N_DRV_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 13 - Input enable in normal execution."]
    #[inline(always)]
    pub fn x32n_fun_ie(&self) -> X32N_FUN_IE_R {
        X32N_FUN_IE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Output enable in sleep mode."]
    #[inline(always)]
    pub fn x32n_slp_oe(&self) -> X32N_SLP_OE_R {
        X32N_SLP_OE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Input enable in sleep mode."]
    #[inline(always)]
    pub fn x32n_slp_ie(&self) -> X32N_SLP_IE_R {
        X32N_SLP_IE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - 1: enable sleep mode. 0: no sleep mode."]
    #[inline(always)]
    pub fn x32n_slp_sel(&self) -> X32N_SLP_SEL_R {
        X32N_SLP_SEL_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:18 - Function selection."]
    #[inline(always)]
    pub fn x32n_fun_sel(&self) -> X32N_FUN_SEL_R {
        X32N_FUN_SEL_R::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bit 19 - 1: use RTC GPIO. 0: use digital GPIO."]
    #[inline(always)]
    pub fn x32n_mux_sel(&self) -> X32N_MUX_SEL_R {
        X32N_MUX_SEL_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 27 - Pull-down enable of the pad. 1: internal pull-down enabled. 0: internal pull-down disabled."]
    #[inline(always)]
    pub fn x32n_rue(&self) -> X32N_RUE_R {
        X32N_RUE_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Pull-up enable of the pad. 1: internal pull-up enabled. 0: internal pull-up disabled."]
    #[inline(always)]
    pub fn x32n_rde(&self) -> X32N_RDE_R {
        X32N_RDE_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bits 29:30 - Select the drive strength of the pad. 0: ~5 mA. 1: ~10 mA. 2: ~20 mA. 3: ~40 mA."]
    #[inline(always)]
    pub fn x32n_drv(&self) -> X32N_DRV_R {
        X32N_DRV_R::new(((self.bits >> 29) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("XTAL_32N_PAD")
            .field("x32n_fun_ie", &self.x32n_fun_ie().bit())
            .field("x32n_slp_oe", &self.x32n_slp_oe().bit())
            .field("x32n_slp_ie", &self.x32n_slp_ie().bit())
            .field("x32n_slp_sel", &self.x32n_slp_sel().bit())
            .field("x32n_fun_sel", &self.x32n_fun_sel().bits())
            .field("x32n_mux_sel", &self.x32n_mux_sel().bit())
            .field("x32n_rue", &self.x32n_rue().bit())
            .field("x32n_rde", &self.x32n_rde().bit())
            .field("x32n_drv", &self.x32n_drv().bits())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<XTAL_32N_PAD_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 13 - Input enable in normal execution."]
    #[inline(always)]
    #[must_use]
    pub fn x32n_fun_ie(&mut self) -> X32N_FUN_IE_W<XTAL_32N_PAD_SPEC> {
        X32N_FUN_IE_W::new(self, 13)
    }
    #[doc = "Bit 14 - Output enable in sleep mode."]
    #[inline(always)]
    #[must_use]
    pub fn x32n_slp_oe(&mut self) -> X32N_SLP_OE_W<XTAL_32N_PAD_SPEC> {
        X32N_SLP_OE_W::new(self, 14)
    }
    #[doc = "Bit 15 - Input enable in sleep mode."]
    #[inline(always)]
    #[must_use]
    pub fn x32n_slp_ie(&mut self) -> X32N_SLP_IE_W<XTAL_32N_PAD_SPEC> {
        X32N_SLP_IE_W::new(self, 15)
    }
    #[doc = "Bit 16 - 1: enable sleep mode. 0: no sleep mode."]
    #[inline(always)]
    #[must_use]
    pub fn x32n_slp_sel(&mut self) -> X32N_SLP_SEL_W<XTAL_32N_PAD_SPEC> {
        X32N_SLP_SEL_W::new(self, 16)
    }
    #[doc = "Bits 17:18 - Function selection."]
    #[inline(always)]
    #[must_use]
    pub fn x32n_fun_sel(&mut self) -> X32N_FUN_SEL_W<XTAL_32N_PAD_SPEC> {
        X32N_FUN_SEL_W::new(self, 17)
    }
    #[doc = "Bit 19 - 1: use RTC GPIO. 0: use digital GPIO."]
    #[inline(always)]
    #[must_use]
    pub fn x32n_mux_sel(&mut self) -> X32N_MUX_SEL_W<XTAL_32N_PAD_SPEC> {
        X32N_MUX_SEL_W::new(self, 19)
    }
    #[doc = "Bit 27 - Pull-down enable of the pad. 1: internal pull-down enabled. 0: internal pull-down disabled."]
    #[inline(always)]
    #[must_use]
    pub fn x32n_rue(&mut self) -> X32N_RUE_W<XTAL_32N_PAD_SPEC> {
        X32N_RUE_W::new(self, 27)
    }
    #[doc = "Bit 28 - Pull-up enable of the pad. 1: internal pull-up enabled. 0: internal pull-up disabled."]
    #[inline(always)]
    #[must_use]
    pub fn x32n_rde(&mut self) -> X32N_RDE_W<XTAL_32N_PAD_SPEC> {
        X32N_RDE_W::new(self, 28)
    }
    #[doc = "Bits 29:30 - Select the drive strength of the pad. 0: ~5 mA. 1: ~10 mA. 2: ~20 mA. 3: ~40 mA."]
    #[inline(always)]
    #[must_use]
    pub fn x32n_drv(&mut self) -> X32N_DRV_W<XTAL_32N_PAD_SPEC> {
        X32N_DRV_W::new(self, 29)
    }
}
#[doc = "32KHz crystal N-pad configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`xtal_32n_pad::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`xtal_32n_pad::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct XTAL_32N_PAD_SPEC;
impl crate::RegisterSpec for XTAL_32N_PAD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`xtal_32n_pad::R`](R) reader structure"]
impl crate::Readable for XTAL_32N_PAD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`xtal_32n_pad::W`](W) writer structure"]
impl crate::Writable for XTAL_32N_PAD_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets XTAL_32N_PAD to value 0x4000_0000"]
impl crate::Resettable for XTAL_32N_PAD_SPEC {
    const RESET_VALUE: u32 = 0x4000_0000;
}
