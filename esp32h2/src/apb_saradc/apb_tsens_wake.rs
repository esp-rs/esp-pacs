#[doc = "Register `APB_TSENS_WAKE` reader"]
pub type R = crate::R<APB_TSENS_WAKE_SPEC>;
#[doc = "Register `APB_TSENS_WAKE` writer"]
pub type W = crate::W<APB_TSENS_WAKE_SPEC>;
#[doc = "Field `WAKEUP_TH_LOW` reader - reg_wakeup_th_low"]
pub type WAKEUP_TH_LOW_R = crate::FieldReader;
#[doc = "Field `WAKEUP_TH_LOW` writer - reg_wakeup_th_low"]
pub type WAKEUP_TH_LOW_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `WAKEUP_TH_HIGH` reader - reg_wakeup_th_high"]
pub type WAKEUP_TH_HIGH_R = crate::FieldReader;
#[doc = "Field `WAKEUP_TH_HIGH` writer - reg_wakeup_th_high"]
pub type WAKEUP_TH_HIGH_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `WAKEUP_OVER_UPPER_TH` reader - reg_wakeup_over_upper_th"]
pub type WAKEUP_OVER_UPPER_TH_R = crate::BitReader;
#[doc = "Field `WAKEUP_MODE` reader - reg_wakeup_mode"]
pub type WAKEUP_MODE_R = crate::BitReader;
#[doc = "Field `WAKEUP_MODE` writer - reg_wakeup_mode"]
pub type WAKEUP_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WAKEUP_EN` reader - reg_wakeup_en"]
pub type WAKEUP_EN_R = crate::BitReader;
#[doc = "Field `WAKEUP_EN` writer - reg_wakeup_en"]
pub type WAKEUP_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - reg_wakeup_th_low"]
    #[inline(always)]
    pub fn wakeup_th_low(&self) -> WAKEUP_TH_LOW_R {
        WAKEUP_TH_LOW_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - reg_wakeup_th_high"]
    #[inline(always)]
    pub fn wakeup_th_high(&self) -> WAKEUP_TH_HIGH_R {
        WAKEUP_TH_HIGH_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16 - reg_wakeup_over_upper_th"]
    #[inline(always)]
    pub fn wakeup_over_upper_th(&self) -> WAKEUP_OVER_UPPER_TH_R {
        WAKEUP_OVER_UPPER_TH_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - reg_wakeup_mode"]
    #[inline(always)]
    pub fn wakeup_mode(&self) -> WAKEUP_MODE_R {
        WAKEUP_MODE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - reg_wakeup_en"]
    #[inline(always)]
    pub fn wakeup_en(&self) -> WAKEUP_EN_R {
        WAKEUP_EN_R::new(((self.bits >> 18) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB_TSENS_WAKE")
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
            .field("wakeup_mode", &format_args!("{}", self.wakeup_mode().bit()))
            .field("wakeup_en", &format_args!("{}", self.wakeup_en().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<APB_TSENS_WAKE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:7 - reg_wakeup_th_low"]
    #[inline(always)]
    #[must_use]
    pub fn wakeup_th_low(&mut self) -> WAKEUP_TH_LOW_W<APB_TSENS_WAKE_SPEC> {
        WAKEUP_TH_LOW_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - reg_wakeup_th_high"]
    #[inline(always)]
    #[must_use]
    pub fn wakeup_th_high(&mut self) -> WAKEUP_TH_HIGH_W<APB_TSENS_WAKE_SPEC> {
        WAKEUP_TH_HIGH_W::new(self, 8)
    }
    #[doc = "Bit 17 - reg_wakeup_mode"]
    #[inline(always)]
    #[must_use]
    pub fn wakeup_mode(&mut self) -> WAKEUP_MODE_W<APB_TSENS_WAKE_SPEC> {
        WAKEUP_MODE_W::new(self, 17)
    }
    #[doc = "Bit 18 - reg_wakeup_en"]
    #[inline(always)]
    #[must_use]
    pub fn wakeup_en(&mut self) -> WAKEUP_EN_W<APB_TSENS_WAKE_SPEC> {
        WAKEUP_EN_W::new(self, 18)
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
#[doc = "digital tsens configure register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb_tsens_wake::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb_tsens_wake::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APB_TSENS_WAKE_SPEC;
impl crate::RegisterSpec for APB_TSENS_WAKE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb_tsens_wake::R`](R) reader structure"]
impl crate::Readable for APB_TSENS_WAKE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`apb_tsens_wake::W`](W) writer structure"]
impl crate::Writable for APB_TSENS_WAKE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets APB_TSENS_WAKE to value 0xff00"]
impl crate::Resettable for APB_TSENS_WAKE_SPEC {
    const RESET_VALUE: Self::Ux = 0xff00;
}
