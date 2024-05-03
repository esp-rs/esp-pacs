#[doc = "Register `PRO_CACHE_2` reader"]
pub type R = crate::R<PRO_CACHE_2_SPEC>;
#[doc = "Register `PRO_CACHE_2` writer"]
pub type W = crate::W<PRO_CACHE_2_SPEC>;
#[doc = "Field `PRO_CACHE_ILG_CLR` reader - The clear signal for cache access interrupt."]
pub type PRO_CACHE_ILG_CLR_R = crate::BitReader;
#[doc = "Field `PRO_CACHE_ILG_CLR` writer - The clear signal for cache access interrupt."]
pub type PRO_CACHE_ILG_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRO_CACHE_ILG_EN` reader - The enable signal for cache access interrupt."]
pub type PRO_CACHE_ILG_EN_R = crate::BitReader;
#[doc = "Field `PRO_CACHE_ILG_EN` writer - The enable signal for cache access interrupt."]
pub type PRO_CACHE_ILG_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRO_CACHE_ILG_INTR` reader - Cache access interrupt signal."]
pub type PRO_CACHE_ILG_INTR_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - The clear signal for cache access interrupt."]
    #[inline(always)]
    pub fn pro_cache_ilg_clr(&self) -> PRO_CACHE_ILG_CLR_R {
        PRO_CACHE_ILG_CLR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The enable signal for cache access interrupt."]
    #[inline(always)]
    pub fn pro_cache_ilg_en(&self) -> PRO_CACHE_ILG_EN_R {
        PRO_CACHE_ILG_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Cache access interrupt signal."]
    #[inline(always)]
    pub fn pro_cache_ilg_intr(&self) -> PRO_CACHE_ILG_INTR_R {
        PRO_CACHE_ILG_INTR_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRO_CACHE_2")
            .field("pro_cache_ilg_clr", &self.pro_cache_ilg_clr().bit())
            .field("pro_cache_ilg_en", &self.pro_cache_ilg_en().bit())
            .field("pro_cache_ilg_intr", &self.pro_cache_ilg_intr().bit())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PRO_CACHE_2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - The clear signal for cache access interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn pro_cache_ilg_clr(&mut self) -> PRO_CACHE_ILG_CLR_W<PRO_CACHE_2_SPEC> {
        PRO_CACHE_ILG_CLR_W::new(self, 0)
    }
    #[doc = "Bit 1 - The enable signal for cache access interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn pro_cache_ilg_en(&mut self) -> PRO_CACHE_ILG_EN_W<PRO_CACHE_2_SPEC> {
        PRO_CACHE_ILG_EN_W::new(self, 1)
    }
}
#[doc = "Cache permission control register 2.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_cache_2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_cache_2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRO_CACHE_2_SPEC;
impl crate::RegisterSpec for PRO_CACHE_2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pro_cache_2::R`](R) reader structure"]
impl crate::Readable for PRO_CACHE_2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pro_cache_2::W`](W) writer structure"]
impl crate::Writable for PRO_CACHE_2_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PRO_CACHE_2 to value 0"]
impl crate::Resettable for PRO_CACHE_2_SPEC {
    const RESET_VALUE: u32 = 0;
}
