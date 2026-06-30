#[doc = "Register `LPCPU_REJECT_SRC` reader"]
pub type R = crate::R<LPCPU_REJECT_SRC_SPEC>;
#[doc = "Register `LPCPU_REJECT_SRC` writer"]
pub type W = crate::W<LPCPU_REJECT_SRC_SPEC>;
#[doc = "Field `LPCPU_REJECT_SOURCE_EN` reader - reject source enable signal for lpcpu"]
pub type LPCPU_REJECT_SOURCE_EN_R = crate::FieldReader<u32>;
#[doc = "Field `LPCPU_REJECT_SOURCE_EN` writer - reject source enable signal for lpcpu"]
pub type LPCPU_REJECT_SOURCE_EN_W<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
#[doc = "Field `LPCPU_SLEEP_REJECT_EN` reader - 1: sleep request can be rejected 0: sleep request cannot be rejected"]
pub type LPCPU_SLEEP_REJECT_EN_R = crate::BitReader;
#[doc = "Field `LPCPU_SLEEP_REJECT_EN` writer - 1: sleep request can be rejected 0: sleep request cannot be rejected"]
pub type LPCPU_SLEEP_REJECT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:30 - reject source enable signal for lpcpu"]
    #[inline(always)]
    pub fn lpcpu_reject_source_en(&self) -> LPCPU_REJECT_SOURCE_EN_R {
        LPCPU_REJECT_SOURCE_EN_R::new(self.bits & 0x7fff_ffff)
    }
    #[doc = "Bit 31 - 1: sleep request can be rejected 0: sleep request cannot be rejected"]
    #[inline(always)]
    pub fn lpcpu_sleep_reject_en(&self) -> LPCPU_SLEEP_REJECT_EN_R {
        LPCPU_SLEEP_REJECT_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPCPU_REJECT_SRC")
            .field("lpcpu_reject_source_en", &self.lpcpu_reject_source_en())
            .field("lpcpu_sleep_reject_en", &self.lpcpu_sleep_reject_en())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:30 - reject source enable signal for lpcpu"]
    #[inline(always)]
    pub fn lpcpu_reject_source_en(
        &mut self,
    ) -> LPCPU_REJECT_SOURCE_EN_W<'_, LPCPU_REJECT_SRC_SPEC> {
        LPCPU_REJECT_SOURCE_EN_W::new(self, 0)
    }
    #[doc = "Bit 31 - 1: sleep request can be rejected 0: sleep request cannot be rejected"]
    #[inline(always)]
    pub fn lpcpu_sleep_reject_en(&mut self) -> LPCPU_SLEEP_REJECT_EN_W<'_, LPCPU_REJECT_SRC_SPEC> {
        LPCPU_SLEEP_REJECT_EN_W::new(self, 31)
    }
}
#[doc = "reject source register for lpcpu\n\nYou can [`read`](crate::Reg::read) this register and get [`lpcpu_reject_src::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpcpu_reject_src::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LPCPU_REJECT_SRC_SPEC;
impl crate::RegisterSpec for LPCPU_REJECT_SRC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lpcpu_reject_src::R`](R) reader structure"]
impl crate::Readable for LPCPU_REJECT_SRC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lpcpu_reject_src::W`](W) writer structure"]
impl crate::Writable for LPCPU_REJECT_SRC_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LPCPU_REJECT_SRC to value 0"]
impl crate::Resettable for LPCPU_REJECT_SRC_SPEC {}
