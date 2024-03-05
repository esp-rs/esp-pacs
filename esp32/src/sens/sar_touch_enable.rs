#[doc = "Register `SAR_TOUCH_ENABLE` reader"]
pub type R = crate::R<SAR_TOUCH_ENABLE_SPEC>;
#[doc = "Register `SAR_TOUCH_ENABLE` writer"]
pub type W = crate::W<SAR_TOUCH_ENABLE_SPEC>;
#[doc = "Field `TOUCH_PAD_WORKEN` reader - Bitmap defining the working set during the measurement."]
pub type TOUCH_PAD_WORKEN_R = crate::FieldReader<u16>;
#[doc = "Field `TOUCH_PAD_WORKEN` writer - Bitmap defining the working set during the measurement."]
pub type TOUCH_PAD_WORKEN_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `TOUCH_PAD_OUTEN2` reader - Bitmap defining SET2 for generating wakeup interrupt. SET2 is \"touched\" only if at least one of touch pad in SET2 is \"touched\"."]
pub type TOUCH_PAD_OUTEN2_R = crate::FieldReader<u16>;
#[doc = "Field `TOUCH_PAD_OUTEN2` writer - Bitmap defining SET2 for generating wakeup interrupt. SET2 is \"touched\" only if at least one of touch pad in SET2 is \"touched\"."]
pub type TOUCH_PAD_OUTEN2_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `TOUCH_PAD_OUTEN1` reader - Bitmap defining SET1 for generating wakeup interrupt. SET1 is \"touched\" only if at least one of touch pad in SET1 is \"touched\"."]
pub type TOUCH_PAD_OUTEN1_R = crate::FieldReader<u16>;
#[doc = "Field `TOUCH_PAD_OUTEN1` writer - Bitmap defining SET1 for generating wakeup interrupt. SET1 is \"touched\" only if at least one of touch pad in SET1 is \"touched\"."]
pub type TOUCH_PAD_OUTEN1_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - Bitmap defining the working set during the measurement."]
    #[inline(always)]
    pub fn touch_pad_worken(&self) -> TOUCH_PAD_WORKEN_R {
        TOUCH_PAD_WORKEN_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:19 - Bitmap defining SET2 for generating wakeup interrupt. SET2 is \"touched\" only if at least one of touch pad in SET2 is \"touched\"."]
    #[inline(always)]
    pub fn touch_pad_outen2(&self) -> TOUCH_PAD_OUTEN2_R {
        TOUCH_PAD_OUTEN2_R::new(((self.bits >> 10) & 0x03ff) as u16)
    }
    #[doc = "Bits 20:29 - Bitmap defining SET1 for generating wakeup interrupt. SET1 is \"touched\" only if at least one of touch pad in SET1 is \"touched\"."]
    #[inline(always)]
    pub fn touch_pad_outen1(&self) -> TOUCH_PAD_OUTEN1_R {
        TOUCH_PAD_OUTEN1_R::new(((self.bits >> 20) & 0x03ff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR_TOUCH_ENABLE")
            .field(
                "touch_pad_worken",
                &format_args!("{}", self.touch_pad_worken().bits()),
            )
            .field(
                "touch_pad_outen2",
                &format_args!("{}", self.touch_pad_outen2().bits()),
            )
            .field(
                "touch_pad_outen1",
                &format_args!("{}", self.touch_pad_outen1().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SAR_TOUCH_ENABLE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:9 - Bitmap defining the working set during the measurement."]
    #[inline(always)]
    #[must_use]
    pub fn touch_pad_worken(&mut self) -> TOUCH_PAD_WORKEN_W<SAR_TOUCH_ENABLE_SPEC> {
        TOUCH_PAD_WORKEN_W::new(self, 0)
    }
    #[doc = "Bits 10:19 - Bitmap defining SET2 for generating wakeup interrupt. SET2 is \"touched\" only if at least one of touch pad in SET2 is \"touched\"."]
    #[inline(always)]
    #[must_use]
    pub fn touch_pad_outen2(&mut self) -> TOUCH_PAD_OUTEN2_W<SAR_TOUCH_ENABLE_SPEC> {
        TOUCH_PAD_OUTEN2_W::new(self, 10)
    }
    #[doc = "Bits 20:29 - Bitmap defining SET1 for generating wakeup interrupt. SET1 is \"touched\" only if at least one of touch pad in SET1 is \"touched\"."]
    #[inline(always)]
    #[must_use]
    pub fn touch_pad_outen1(&mut self) -> TOUCH_PAD_OUTEN1_W<SAR_TOUCH_ENABLE_SPEC> {
        TOUCH_PAD_OUTEN1_W::new(self, 20)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sar_touch_enable::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sar_touch_enable::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SAR_TOUCH_ENABLE_SPEC;
impl crate::RegisterSpec for SAR_TOUCH_ENABLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sar_touch_enable::R`](R) reader structure"]
impl crate::Readable for SAR_TOUCH_ENABLE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sar_touch_enable::W`](W) writer structure"]
impl crate::Writable for SAR_TOUCH_ENABLE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SAR_TOUCH_ENABLE to value 0x3fff_ffff"]
impl crate::Resettable for SAR_TOUCH_ENABLE_SPEC {
    const RESET_VALUE: u32 = 0x3fff_ffff;
}
