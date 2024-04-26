#[doc = "Register `CTRL2` reader"]
pub type R = crate::R<CTRL2_SPEC>;
#[doc = "Register `CTRL2` writer"]
pub type W = crate::W<CTRL2_SPEC>;
#[doc = "Field `MEAS_NUM_LIMIT` reader - Need add description"]
pub type MEAS_NUM_LIMIT_R = crate::BitReader;
#[doc = "Field `MEAS_NUM_LIMIT` writer - Need add description"]
pub type MEAS_NUM_LIMIT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MAX_MEAS_NUM` reader - max conversion number"]
pub type MAX_MEAS_NUM_R = crate::FieldReader;
#[doc = "Field `MAX_MEAS_NUM` writer - max conversion number"]
pub type MAX_MEAS_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SAR1_INV` reader - 1: data to DIG ADC1 CTRL is inverted, otherwise not"]
pub type SAR1_INV_R = crate::BitReader;
#[doc = "Field `SAR1_INV` writer - 1: data to DIG ADC1 CTRL is inverted, otherwise not"]
pub type SAR1_INV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAR2_INV` reader - 1: data to DIG ADC2 CTRL is inverted, otherwise not"]
pub type SAR2_INV_R = crate::BitReader;
#[doc = "Field `SAR2_INV` writer - 1: data to DIG ADC2 CTRL is inverted, otherwise not"]
pub type SAR2_INV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER_TARGET` reader - to set saradc timer target"]
pub type TIMER_TARGET_R = crate::FieldReader<u16>;
#[doc = "Field `TIMER_TARGET` writer - to set saradc timer target"]
pub type TIMER_TARGET_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `TIMER_EN` reader - to enable saradc timer trigger"]
pub type TIMER_EN_R = crate::BitReader;
#[doc = "Field `TIMER_EN` writer - to enable saradc timer trigger"]
pub type TIMER_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Need add description"]
    #[inline(always)]
    pub fn meas_num_limit(&self) -> MEAS_NUM_LIMIT_R {
        MEAS_NUM_LIMIT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:8 - max conversion number"]
    #[inline(always)]
    pub fn max_meas_num(&self) -> MAX_MEAS_NUM_R {
        MAX_MEAS_NUM_R::new(((self.bits >> 1) & 0xff) as u8)
    }
    #[doc = "Bit 9 - 1: data to DIG ADC1 CTRL is inverted, otherwise not"]
    #[inline(always)]
    pub fn sar1_inv(&self) -> SAR1_INV_R {
        SAR1_INV_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 1: data to DIG ADC2 CTRL is inverted, otherwise not"]
    #[inline(always)]
    pub fn sar2_inv(&self) -> SAR2_INV_R {
        SAR2_INV_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 12:23 - to set saradc timer target"]
    #[inline(always)]
    pub fn timer_target(&self) -> TIMER_TARGET_R {
        TIMER_TARGET_R::new(((self.bits >> 12) & 0x0fff) as u16)
    }
    #[doc = "Bit 24 - to enable saradc timer trigger"]
    #[inline(always)]
    pub fn timer_en(&self) -> TIMER_EN_R {
        TIMER_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL2")
            .field(
                "meas_num_limit",
                &format_args!("{}", self.meas_num_limit().bit()),
            )
            .field(
                "max_meas_num",
                &format_args!("{}", self.max_meas_num().bits()),
            )
            .field("sar1_inv", &format_args!("{}", self.sar1_inv().bit()))
            .field("sar2_inv", &format_args!("{}", self.sar2_inv().bit()))
            .field(
                "timer_target",
                &format_args!("{}", self.timer_target().bits()),
            )
            .field("timer_en", &format_args!("{}", self.timer_en().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CTRL2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - Need add description"]
    #[inline(always)]
    #[must_use]
    pub fn meas_num_limit(&mut self) -> MEAS_NUM_LIMIT_W<CTRL2_SPEC> {
        MEAS_NUM_LIMIT_W::new(self, 0)
    }
    #[doc = "Bits 1:8 - max conversion number"]
    #[inline(always)]
    #[must_use]
    pub fn max_meas_num(&mut self) -> MAX_MEAS_NUM_W<CTRL2_SPEC> {
        MAX_MEAS_NUM_W::new(self, 1)
    }
    #[doc = "Bit 9 - 1: data to DIG ADC1 CTRL is inverted, otherwise not"]
    #[inline(always)]
    #[must_use]
    pub fn sar1_inv(&mut self) -> SAR1_INV_W<CTRL2_SPEC> {
        SAR1_INV_W::new(self, 9)
    }
    #[doc = "Bit 10 - 1: data to DIG ADC2 CTRL is inverted, otherwise not"]
    #[inline(always)]
    #[must_use]
    pub fn sar2_inv(&mut self) -> SAR2_INV_W<CTRL2_SPEC> {
        SAR2_INV_W::new(self, 10)
    }
    #[doc = "Bits 12:23 - to set saradc timer target"]
    #[inline(always)]
    #[must_use]
    pub fn timer_target(&mut self) -> TIMER_TARGET_W<CTRL2_SPEC> {
        TIMER_TARGET_W::new(self, 12)
    }
    #[doc = "Bit 24 - to enable saradc timer trigger"]
    #[inline(always)]
    #[must_use]
    pub fn timer_en(&mut self) -> TIMER_EN_W<CTRL2_SPEC> {
        TIMER_EN_W::new(self, 24)
    }
}
#[doc = "register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRL2_SPEC;
impl crate::RegisterSpec for CTRL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl2::R`](R) reader structure"]
impl crate::Readable for CTRL2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrl2::W`](W) writer structure"]
impl crate::Writable for CTRL2_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL2 to value 0xa1fe"]
impl crate::Resettable for CTRL2_SPEC {
    const RESET_VALUE: u32 = 0xa1fe;
}
