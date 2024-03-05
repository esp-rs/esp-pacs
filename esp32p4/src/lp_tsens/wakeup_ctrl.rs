#[doc = "Register `WAKEUP_CTRL` reader"]
pub type R = crate::R<WAKEUP_CTRL_SPEC>;
#[doc = "Register `WAKEUP_CTRL` writer"]
pub type W = crate::W<WAKEUP_CTRL_SPEC>;
#[doc = "Field `WAKEUP_TH_LOW` reader - Lower threshold."]
pub type WAKEUP_TH_LOW_R = crate::FieldReader;
#[doc = "Field `WAKEUP_TH_LOW` writer - Lower threshold."]
pub type WAKEUP_TH_LOW_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `WAKEUP_TH_HIGH` reader - Upper threshold."]
pub type WAKEUP_TH_HIGH_R = crate::FieldReader;
#[doc = "Field `WAKEUP_TH_HIGH` writer - Upper threshold."]
pub type WAKEUP_TH_HIGH_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `WAKEUP_OVER_UPPER_TH` reader - Indicates that this wakeup event arose from exceeding upper threshold."]
pub type WAKEUP_OVER_UPPER_TH_R = crate::BitReader;
#[doc = "Field `WAKEUP_EN` reader - Tsens wakeup enable."]
pub type WAKEUP_EN_R = crate::BitReader;
#[doc = "Field `WAKEUP_EN` writer - Tsens wakeup enable."]
pub type WAKEUP_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WAKEUP_MODE` reader - 0:absolute value comparison mode. 1: relative value comparison mode."]
pub type WAKEUP_MODE_R = crate::BitReader;
#[doc = "Field `WAKEUP_MODE` writer - 0:absolute value comparison mode. 1: relative value comparison mode."]
pub type WAKEUP_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Lower threshold."]
    #[inline(always)]
    pub fn wakeup_th_low(&self) -> WAKEUP_TH_LOW_R {
        WAKEUP_TH_LOW_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 14:21 - Upper threshold."]
    #[inline(always)]
    pub fn wakeup_th_high(&self) -> WAKEUP_TH_HIGH_R {
        WAKEUP_TH_HIGH_R::new(((self.bits >> 14) & 0xff) as u8)
    }
    #[doc = "Bit 29 - Indicates that this wakeup event arose from exceeding upper threshold."]
    #[inline(always)]
    pub fn wakeup_over_upper_th(&self) -> WAKEUP_OVER_UPPER_TH_R {
        WAKEUP_OVER_UPPER_TH_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Tsens wakeup enable."]
    #[inline(always)]
    pub fn wakeup_en(&self) -> WAKEUP_EN_R {
        WAKEUP_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 0:absolute value comparison mode. 1: relative value comparison mode."]
    #[inline(always)]
    pub fn wakeup_mode(&self) -> WAKEUP_MODE_R {
        WAKEUP_MODE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WAKEUP_CTRL")
            .field(
                "wakeup_th_low",
                &format_args!("{}", self.wakeup_th_low().bits()),
            )
            .field(
                "wakeup_th_high",
                &format_args!("{}", self.wakeup_th_high().bits()),
            )
            .field(
                "wakeup_over_upper_th",
                &format_args!("{}", self.wakeup_over_upper_th().bit()),
            )
            .field("wakeup_en", &format_args!("{}", self.wakeup_en().bit()))
            .field("wakeup_mode", &format_args!("{}", self.wakeup_mode().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<WAKEUP_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:7 - Lower threshold."]
    #[inline(always)]
    #[must_use]
    pub fn wakeup_th_low(&mut self) -> WAKEUP_TH_LOW_W<WAKEUP_CTRL_SPEC> {
        WAKEUP_TH_LOW_W::new(self, 0)
    }
    #[doc = "Bits 14:21 - Upper threshold."]
    #[inline(always)]
    #[must_use]
    pub fn wakeup_th_high(&mut self) -> WAKEUP_TH_HIGH_W<WAKEUP_CTRL_SPEC> {
        WAKEUP_TH_HIGH_W::new(self, 14)
    }
    #[doc = "Bit 30 - Tsens wakeup enable."]
    #[inline(always)]
    #[must_use]
    pub fn wakeup_en(&mut self) -> WAKEUP_EN_W<WAKEUP_CTRL_SPEC> {
        WAKEUP_EN_W::new(self, 30)
    }
    #[doc = "Bit 31 - 0:absolute value comparison mode. 1: relative value comparison mode."]
    #[inline(always)]
    #[must_use]
    pub fn wakeup_mode(&mut self) -> WAKEUP_MODE_W<WAKEUP_CTRL_SPEC> {
        WAKEUP_MODE_W::new(self, 31)
    }
}
#[doc = "Tsens wakeup control registers.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wakeup_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wakeup_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WAKEUP_CTRL_SPEC;
impl crate::RegisterSpec for WAKEUP_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wakeup_ctrl::R`](R) reader structure"]
impl crate::Readable for WAKEUP_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`wakeup_ctrl::W`](W) writer structure"]
impl crate::Writable for WAKEUP_CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WAKEUP_CTRL to value 0x003f_c000"]
impl crate::Resettable for WAKEUP_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x003f_c000;
}
