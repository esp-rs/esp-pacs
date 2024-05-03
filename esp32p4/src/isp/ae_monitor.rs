#[doc = "Register `AE_MONITOR` reader"]
pub type R = crate::R<AE_MONITOR_SPEC>;
#[doc = "Register `AE_MONITOR` writer"]
pub type W = crate::W<AE_MONITOR_SPEC>;
#[doc = "Field `TL` reader - this field configures the lower lum threshold of ae monitor"]
pub type TL_R = crate::FieldReader;
#[doc = "Field `TL` writer - this field configures the lower lum threshold of ae monitor"]
pub type TL_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TH` reader - this field configures the higher lum threshold of ae monitor"]
pub type TH_R = crate::FieldReader;
#[doc = "Field `TH` writer - this field configures the higher lum threshold of ae monitor"]
pub type TH_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PERIOD` reader - this field cnfigures ae monitor frame period"]
pub type PERIOD_R = crate::FieldReader;
#[doc = "Field `PERIOD` writer - this field cnfigures ae monitor frame period"]
pub type PERIOD_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:7 - this field configures the lower lum threshold of ae monitor"]
    #[inline(always)]
    pub fn tl(&self) -> TL_R {
        TL_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - this field configures the higher lum threshold of ae monitor"]
    #[inline(always)]
    pub fn th(&self) -> TH_R {
        TH_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:21 - this field cnfigures ae monitor frame period"]
    #[inline(always)]
    pub fn period(&self) -> PERIOD_R {
        PERIOD_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AE_MONITOR")
            .field("tl", &self.tl().bits())
            .field("th", &self.th().bits())
            .field("period", &self.period().bits())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<AE_MONITOR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:7 - this field configures the lower lum threshold of ae monitor"]
    #[inline(always)]
    #[must_use]
    pub fn tl(&mut self) -> TL_W<AE_MONITOR_SPEC> {
        TL_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - this field configures the higher lum threshold of ae monitor"]
    #[inline(always)]
    #[must_use]
    pub fn th(&mut self) -> TH_W<AE_MONITOR_SPEC> {
        TH_W::new(self, 8)
    }
    #[doc = "Bits 16:21 - this field cnfigures ae monitor frame period"]
    #[inline(always)]
    #[must_use]
    pub fn period(&mut self) -> PERIOD_W<AE_MONITOR_SPEC> {
        PERIOD_W::new(self, 16)
    }
}
#[doc = "ae monitor control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ae_monitor::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ae_monitor::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AE_MONITOR_SPEC;
impl crate::RegisterSpec for AE_MONITOR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ae_monitor::R`](R) reader structure"]
impl crate::Readable for AE_MONITOR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ae_monitor::W`](W) writer structure"]
impl crate::Writable for AE_MONITOR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AE_MONITOR to value 0"]
impl crate::Resettable for AE_MONITOR_SPEC {
    const RESET_VALUE: u32 = 0;
}
