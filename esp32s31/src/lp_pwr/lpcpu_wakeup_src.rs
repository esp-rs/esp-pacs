#[doc = "Register `LPCPU_WAKEUP_SRC` reader"]
pub type R = crate::R<LPCPU_WAKEUP_SRC_SPEC>;
#[doc = "Register `LPCPU_WAKEUP_SRC` writer"]
pub type W = crate::W<LPCPU_WAKEUP_SRC_SPEC>;
#[doc = "Field `LPCPU_WAKEUP_SOURCE_EN` reader - wakeup source enable signal for lpcpu"]
pub type LPCPU_WAKEUP_SOURCE_EN_R = crate::FieldReader<u32>;
#[doc = "Field `LPCPU_WAKEUP_SOURCE_EN` writer - wakeup source enable signal for lpcpu"]
pub type LPCPU_WAKEUP_SOURCE_EN_W<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
impl R {
    #[doc = "Bits 0:30 - wakeup source enable signal for lpcpu"]
    #[inline(always)]
    pub fn lpcpu_wakeup_source_en(&self) -> LPCPU_WAKEUP_SOURCE_EN_R {
        LPCPU_WAKEUP_SOURCE_EN_R::new(self.bits & 0x7fff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPCPU_WAKEUP_SRC")
            .field("lpcpu_wakeup_source_en", &self.lpcpu_wakeup_source_en())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:30 - wakeup source enable signal for lpcpu"]
    #[inline(always)]
    pub fn lpcpu_wakeup_source_en(
        &mut self,
    ) -> LPCPU_WAKEUP_SOURCE_EN_W<'_, LPCPU_WAKEUP_SRC_SPEC> {
        LPCPU_WAKEUP_SOURCE_EN_W::new(self, 0)
    }
}
#[doc = "wakeup source register for lpcpu\n\nYou can [`read`](crate::Reg::read) this register and get [`lpcpu_wakeup_src::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpcpu_wakeup_src::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LPCPU_WAKEUP_SRC_SPEC;
impl crate::RegisterSpec for LPCPU_WAKEUP_SRC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lpcpu_wakeup_src::R`](R) reader structure"]
impl crate::Readable for LPCPU_WAKEUP_SRC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lpcpu_wakeup_src::W`](W) writer structure"]
impl crate::Writable for LPCPU_WAKEUP_SRC_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LPCPU_WAKEUP_SRC to value 0"]
impl crate::Resettable for LPCPU_WAKEUP_SRC_SPEC {}
