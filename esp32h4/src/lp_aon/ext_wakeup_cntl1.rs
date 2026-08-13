#[doc = "Register `EXT_WAKEUP_CNTL1` reader"]
pub type R = crate::R<EXT_WAKEUP_CNTL1_SPEC>;
#[doc = "Register `EXT_WAKEUP_CNTL1` writer"]
pub type W = crate::W<EXT_WAKEUP_CNTL1_SPEC>;
#[doc = "Field `EXT_WAKEUP_STATUS` reader - get external wakeup status bitmap"]
pub type EXT_WAKEUP_STATUS_R = crate::FieldReader<u16>;
#[doc = "Field `EXT_WAKEUP_STATUS_CLR` writer - clear external wakeup status 1: clear 0: no operation"]
pub type EXT_WAKEUP_STATUS_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXT_WAKEUP_FILTER` reader - enable external filter or not 1: enable 0: disable"]
pub type EXT_WAKEUP_FILTER_R = crate::BitReader;
#[doc = "Field `EXT_WAKEUP_FILTER` writer - enable external filter or not 1: enable 0: disable"]
pub type EXT_WAKEUP_FILTER_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - get external wakeup status bitmap"]
    #[inline(always)]
    pub fn ext_wakeup_status(&self) -> EXT_WAKEUP_STATUS_R {
        EXT_WAKEUP_STATUS_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 31 - enable external filter or not 1: enable 0: disable"]
    #[inline(always)]
    pub fn ext_wakeup_filter(&self) -> EXT_WAKEUP_FILTER_R {
        EXT_WAKEUP_FILTER_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EXT_WAKEUP_CNTL1")
            .field("ext_wakeup_status", &self.ext_wakeup_status())
            .field("ext_wakeup_filter", &self.ext_wakeup_filter())
            .finish()
    }
}
impl W {
    #[doc = "Bit 30 - clear external wakeup status 1: clear 0: no operation"]
    #[inline(always)]
    pub fn ext_wakeup_status_clr(&mut self) -> EXT_WAKEUP_STATUS_CLR_W<'_, EXT_WAKEUP_CNTL1_SPEC> {
        EXT_WAKEUP_STATUS_CLR_W::new(self, 30)
    }
    #[doc = "Bit 31 - enable external filter or not 1: enable 0: disable"]
    #[inline(always)]
    pub fn ext_wakeup_filter(&mut self) -> EXT_WAKEUP_FILTER_W<'_, EXT_WAKEUP_CNTL1_SPEC> {
        EXT_WAKEUP_FILTER_W::new(self, 31)
    }
}
#[doc = "configure alwayson external io wakeup\n\nYou can [`read`](crate::Reg::read) this register and get [`ext_wakeup_cntl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ext_wakeup_cntl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EXT_WAKEUP_CNTL1_SPEC;
impl crate::RegisterSpec for EXT_WAKEUP_CNTL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ext_wakeup_cntl1::R`](R) reader structure"]
impl crate::Readable for EXT_WAKEUP_CNTL1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ext_wakeup_cntl1::W`](W) writer structure"]
impl crate::Writable for EXT_WAKEUP_CNTL1_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EXT_WAKEUP_CNTL1 to value 0"]
impl crate::Resettable for EXT_WAKEUP_CNTL1_SPEC {}
