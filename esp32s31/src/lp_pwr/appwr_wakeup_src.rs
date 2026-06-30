#[doc = "Register `APPWR_WAKEUP_SRC` reader"]
pub type R = crate::R<APPWR_WAKEUP_SRC_SPEC>;
#[doc = "Register `APPWR_WAKEUP_SRC` writer"]
pub type W = crate::W<APPWR_WAKEUP_SRC_SPEC>;
#[doc = "Field `APPWR_WAKEUP_SOURCE_EN` reader - wakeup enable signal for appwr"]
pub type APPWR_WAKEUP_SOURCE_EN_R = crate::FieldReader<u32>;
#[doc = "Field `APPWR_WAKEUP_SOURCE_EN` writer - wakeup enable signal for appwr"]
pub type APPWR_WAKEUP_SOURCE_EN_W<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
impl R {
    #[doc = "Bits 0:30 - wakeup enable signal for appwr"]
    #[inline(always)]
    pub fn appwr_wakeup_source_en(&self) -> APPWR_WAKEUP_SOURCE_EN_R {
        APPWR_WAKEUP_SOURCE_EN_R::new(self.bits & 0x7fff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APPWR_WAKEUP_SRC")
            .field("appwr_wakeup_source_en", &self.appwr_wakeup_source_en())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:30 - wakeup enable signal for appwr"]
    #[inline(always)]
    pub fn appwr_wakeup_source_en(
        &mut self,
    ) -> APPWR_WAKEUP_SOURCE_EN_W<'_, APPWR_WAKEUP_SRC_SPEC> {
        APPWR_WAKEUP_SOURCE_EN_W::new(self, 0)
    }
}
#[doc = "wakeup source register for appwr\n\nYou can [`read`](crate::Reg::read) this register and get [`appwr_wakeup_src::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`appwr_wakeup_src::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APPWR_WAKEUP_SRC_SPEC;
impl crate::RegisterSpec for APPWR_WAKEUP_SRC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`appwr_wakeup_src::R`](R) reader structure"]
impl crate::Readable for APPWR_WAKEUP_SRC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`appwr_wakeup_src::W`](W) writer structure"]
impl crate::Writable for APPWR_WAKEUP_SRC_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets APPWR_WAKEUP_SRC to value 0"]
impl crate::Resettable for APPWR_WAKEUP_SRC_SPEC {}
