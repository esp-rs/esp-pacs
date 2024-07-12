#[doc = "Register `LACTALARMHI` reader"]
pub type R = crate::R<LACTALARMHI_SPEC>;
#[doc = "Register `LACTALARMHI` writer"]
pub type W = crate::W<LACTALARMHI_SPEC>;
#[doc = "Field `ALARM_HI` reader - "]
pub type ALARM_HI_R = crate::FieldReader<u32>;
#[doc = "Field `ALARM_HI` writer - "]
pub type ALARM_HI_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn alarm_hi(&self) -> ALARM_HI_R {
        ALARM_HI_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LACTALARMHI")
            .field("alarm_hi", &self.alarm_hi())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn alarm_hi(&mut self) -> ALARM_HI_W<LACTALARMHI_SPEC> {
        ALARM_HI_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`lactalarmhi::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lactalarmhi::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LACTALARMHI_SPEC;
impl crate::RegisterSpec for LACTALARMHI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lactalarmhi::R`](R) reader structure"]
impl crate::Readable for LACTALARMHI_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lactalarmhi::W`](W) writer structure"]
impl crate::Writable for LACTALARMHI_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LACTALARMHI to value 0"]
impl crate::Resettable for LACTALARMHI_SPEC {
    const RESET_VALUE: u32 = 0;
}
