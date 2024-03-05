#[doc = "Register `INT_RAW` reader"]
pub type R = crate::R<INT_RAW_SPEC>;
#[doc = "Register `INT_RAW` writer"]
pub type W = crate::W<INT_RAW_SPEC>;
#[doc = "Field `THRES1_LOW_INT_RAW` reader - need_des"]
pub type THRES1_LOW_INT_RAW_R = crate::BitReader;
#[doc = "Field `THRES1_LOW_INT_RAW` writer - need_des"]
pub type THRES1_LOW_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `THRES0_LOW_INT_RAW` reader - need_des"]
pub type THRES0_LOW_INT_RAW_R = crate::BitReader;
#[doc = "Field `THRES0_LOW_INT_RAW` writer - need_des"]
pub type THRES0_LOW_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `THRES1_HIGH_INT_RAW` reader - need_des"]
pub type THRES1_HIGH_INT_RAW_R = crate::BitReader;
#[doc = "Field `THRES1_HIGH_INT_RAW` writer - need_des"]
pub type THRES1_HIGH_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `THRES0_HIGH_INT_RAW` reader - need_des"]
pub type THRES0_HIGH_INT_RAW_R = crate::BitReader;
#[doc = "Field `THRES0_HIGH_INT_RAW` writer - need_des"]
pub type THRES0_HIGH_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAR2_DONE_INT_RAW` reader - need_des"]
pub type SAR2_DONE_INT_RAW_R = crate::BitReader;
#[doc = "Field `SAR2_DONE_INT_RAW` writer - need_des"]
pub type SAR2_DONE_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAR1_DONE_INT_RAW` reader - need_des"]
pub type SAR1_DONE_INT_RAW_R = crate::BitReader;
#[doc = "Field `SAR1_DONE_INT_RAW` writer - need_des"]
pub type SAR1_DONE_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 26 - need_des"]
    #[inline(always)]
    pub fn thres1_low_int_raw(&self) -> THRES1_LOW_INT_RAW_R {
        THRES1_LOW_INT_RAW_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - need_des"]
    #[inline(always)]
    pub fn thres0_low_int_raw(&self) -> THRES0_LOW_INT_RAW_R {
        THRES0_LOW_INT_RAW_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - need_des"]
    #[inline(always)]
    pub fn thres1_high_int_raw(&self) -> THRES1_HIGH_INT_RAW_R {
        THRES1_HIGH_INT_RAW_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - need_des"]
    #[inline(always)]
    pub fn thres0_high_int_raw(&self) -> THRES0_HIGH_INT_RAW_R {
        THRES0_HIGH_INT_RAW_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn sar2_done_int_raw(&self) -> SAR2_DONE_INT_RAW_R {
        SAR2_DONE_INT_RAW_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn sar1_done_int_raw(&self) -> SAR1_DONE_INT_RAW_R {
        SAR1_DONE_INT_RAW_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_RAW")
            .field(
                "thres1_low_int_raw",
                &format_args!("{}", self.thres1_low_int_raw().bit()),
            )
            .field(
                "thres0_low_int_raw",
                &format_args!("{}", self.thres0_low_int_raw().bit()),
            )
            .field(
                "thres1_high_int_raw",
                &format_args!("{}", self.thres1_high_int_raw().bit()),
            )
            .field(
                "thres0_high_int_raw",
                &format_args!("{}", self.thres0_high_int_raw().bit()),
            )
            .field(
                "sar2_done_int_raw",
                &format_args!("{}", self.sar2_done_int_raw().bit()),
            )
            .field(
                "sar1_done_int_raw",
                &format_args!("{}", self.sar1_done_int_raw().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_RAW_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 26 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn thres1_low_int_raw(&mut self) -> THRES1_LOW_INT_RAW_W<INT_RAW_SPEC> {
        THRES1_LOW_INT_RAW_W::new(self, 26)
    }
    #[doc = "Bit 27 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn thres0_low_int_raw(&mut self) -> THRES0_LOW_INT_RAW_W<INT_RAW_SPEC> {
        THRES0_LOW_INT_RAW_W::new(self, 27)
    }
    #[doc = "Bit 28 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn thres1_high_int_raw(&mut self) -> THRES1_HIGH_INT_RAW_W<INT_RAW_SPEC> {
        THRES1_HIGH_INT_RAW_W::new(self, 28)
    }
    #[doc = "Bit 29 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn thres0_high_int_raw(&mut self) -> THRES0_HIGH_INT_RAW_W<INT_RAW_SPEC> {
        THRES0_HIGH_INT_RAW_W::new(self, 29)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn sar2_done_int_raw(&mut self) -> SAR2_DONE_INT_RAW_W<INT_RAW_SPEC> {
        SAR2_DONE_INT_RAW_W::new(self, 30)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn sar1_done_int_raw(&mut self) -> SAR1_DONE_INT_RAW_W<INT_RAW_SPEC> {
        SAR1_DONE_INT_RAW_W::new(self, 31)
    }
}
#[doc = "Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_raw::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_raw::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_RAW_SPEC;
impl crate::RegisterSpec for INT_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_raw::R`](R) reader structure"]
impl crate::Readable for INT_RAW_SPEC {}
#[doc = "`write(|w| ..)` method takes [`int_raw::W`](W) writer structure"]
impl crate::Writable for INT_RAW_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT_RAW to value 0"]
impl crate::Resettable for INT_RAW_SPEC {
    const RESET_VALUE: u32 = 0;
}
