#[doc = "Register `RTC_DEBUG_SEL` reader"]
pub type R = crate::R<RTC_DEBUG_SEL_SPEC>;
#[doc = "Register `RTC_DEBUG_SEL` writer"]
pub type W = crate::W<RTC_DEBUG_SEL_SPEC>;
#[doc = "Field `RTC_DEBUG_SEL0` reader - "]
pub type RTC_DEBUG_SEL0_R = crate::FieldReader;
#[doc = "Field `RTC_DEBUG_SEL0` writer - "]
pub type RTC_DEBUG_SEL0_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `RTC_DEBUG_SEL1` reader - "]
pub type RTC_DEBUG_SEL1_R = crate::FieldReader;
#[doc = "Field `RTC_DEBUG_SEL1` writer - "]
pub type RTC_DEBUG_SEL1_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `RTC_DEBUG_SEL2` reader - "]
pub type RTC_DEBUG_SEL2_R = crate::FieldReader;
#[doc = "Field `RTC_DEBUG_SEL2` writer - "]
pub type RTC_DEBUG_SEL2_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `RTC_DEBUG_SEL3` reader - "]
pub type RTC_DEBUG_SEL3_R = crate::FieldReader;
#[doc = "Field `RTC_DEBUG_SEL3` writer - "]
pub type RTC_DEBUG_SEL3_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `RTC_DEBUG_SEL4` reader - "]
pub type RTC_DEBUG_SEL4_R = crate::FieldReader;
#[doc = "Field `RTC_DEBUG_SEL4` writer - "]
pub type RTC_DEBUG_SEL4_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `RTC_DEBUG_12M_NO_GATING` reader - "]
pub type RTC_DEBUG_12M_NO_GATING_R = crate::BitReader;
#[doc = "Field `RTC_DEBUG_12M_NO_GATING` writer - "]
pub type RTC_DEBUG_12M_NO_GATING_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn rtc_debug_sel0(&self) -> RTC_DEBUG_SEL0_R {
        RTC_DEBUG_SEL0_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9"]
    #[inline(always)]
    pub fn rtc_debug_sel1(&self) -> RTC_DEBUG_SEL1_R {
        RTC_DEBUG_SEL1_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14"]
    #[inline(always)]
    pub fn rtc_debug_sel2(&self) -> RTC_DEBUG_SEL2_R {
        RTC_DEBUG_SEL2_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 15:19"]
    #[inline(always)]
    pub fn rtc_debug_sel3(&self) -> RTC_DEBUG_SEL3_R {
        RTC_DEBUG_SEL3_R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    #[doc = "Bits 20:24"]
    #[inline(always)]
    pub fn rtc_debug_sel4(&self) -> RTC_DEBUG_SEL4_R {
        RTC_DEBUG_SEL4_R::new(((self.bits >> 20) & 0x1f) as u8)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn rtc_debug_12m_no_gating(&self) -> RTC_DEBUG_12M_NO_GATING_R {
        RTC_DEBUG_12M_NO_GATING_R::new(((self.bits >> 25) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RTC_DEBUG_SEL")
            .field(
                "rtc_debug_sel0",
                &format_args!("{}", self.rtc_debug_sel0().bits()),
            )
            .field(
                "rtc_debug_sel1",
                &format_args!("{}", self.rtc_debug_sel1().bits()),
            )
            .field(
                "rtc_debug_sel2",
                &format_args!("{}", self.rtc_debug_sel2().bits()),
            )
            .field(
                "rtc_debug_sel3",
                &format_args!("{}", self.rtc_debug_sel3().bits()),
            )
            .field(
                "rtc_debug_sel4",
                &format_args!("{}", self.rtc_debug_sel4().bits()),
            )
            .field(
                "rtc_debug_12m_no_gating",
                &format_args!("{}", self.rtc_debug_12m_no_gating().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RTC_DEBUG_SEL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_debug_sel0(&mut self) -> RTC_DEBUG_SEL0_W<RTC_DEBUG_SEL_SPEC> {
        RTC_DEBUG_SEL0_W::new(self, 0)
    }
    #[doc = "Bits 5:9"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_debug_sel1(&mut self) -> RTC_DEBUG_SEL1_W<RTC_DEBUG_SEL_SPEC> {
        RTC_DEBUG_SEL1_W::new(self, 5)
    }
    #[doc = "Bits 10:14"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_debug_sel2(&mut self) -> RTC_DEBUG_SEL2_W<RTC_DEBUG_SEL_SPEC> {
        RTC_DEBUG_SEL2_W::new(self, 10)
    }
    #[doc = "Bits 15:19"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_debug_sel3(&mut self) -> RTC_DEBUG_SEL3_W<RTC_DEBUG_SEL_SPEC> {
        RTC_DEBUG_SEL3_W::new(self, 15)
    }
    #[doc = "Bits 20:24"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_debug_sel4(&mut self) -> RTC_DEBUG_SEL4_W<RTC_DEBUG_SEL_SPEC> {
        RTC_DEBUG_SEL4_W::new(self, 20)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_debug_12m_no_gating(&mut self) -> RTC_DEBUG_12M_NO_GATING_W<RTC_DEBUG_SEL_SPEC> {
        RTC_DEBUG_12M_NO_GATING_W::new(self, 25)
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
#[doc = "RTC debug select register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_debug_sel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_debug_sel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RTC_DEBUG_SEL_SPEC;
impl crate::RegisterSpec for RTC_DEBUG_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtc_debug_sel::R`](R) reader structure"]
impl crate::Readable for RTC_DEBUG_SEL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rtc_debug_sel::W`](W) writer structure"]
impl crate::Writable for RTC_DEBUG_SEL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RTC_DEBUG_SEL to value 0"]
impl crate::Resettable for RTC_DEBUG_SEL_SPEC {
    const RESET_VALUE: u32 = 0;
}
