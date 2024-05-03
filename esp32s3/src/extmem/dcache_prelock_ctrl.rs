#[doc = "Register `DCACHE_PRELOCK_CTRL` reader"]
pub type R = crate::R<DCACHE_PRELOCK_CTRL_SPEC>;
#[doc = "Register `DCACHE_PRELOCK_CTRL` writer"]
pub type W = crate::W<DCACHE_PRELOCK_CTRL_SPEC>;
#[doc = "Field `DCACHE_PRELOCK_SCT0_EN` reader - The bit is used to enable the first section of prelock function."]
pub type DCACHE_PRELOCK_SCT0_EN_R = crate::BitReader;
#[doc = "Field `DCACHE_PRELOCK_SCT0_EN` writer - The bit is used to enable the first section of prelock function."]
pub type DCACHE_PRELOCK_SCT0_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCACHE_PRELOCK_SCT1_EN` reader - The bit is used to enable the second section of prelock function."]
pub type DCACHE_PRELOCK_SCT1_EN_R = crate::BitReader;
#[doc = "Field `DCACHE_PRELOCK_SCT1_EN` writer - The bit is used to enable the second section of prelock function."]
pub type DCACHE_PRELOCK_SCT1_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - The bit is used to enable the first section of prelock function."]
    #[inline(always)]
    pub fn dcache_prelock_sct0_en(&self) -> DCACHE_PRELOCK_SCT0_EN_R {
        DCACHE_PRELOCK_SCT0_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The bit is used to enable the second section of prelock function."]
    #[inline(always)]
    pub fn dcache_prelock_sct1_en(&self) -> DCACHE_PRELOCK_SCT1_EN_R {
        DCACHE_PRELOCK_SCT1_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DCACHE_PRELOCK_CTRL")
            .field(
                "dcache_prelock_sct0_en",
                &self.dcache_prelock_sct0_en().bit(),
            )
            .field(
                "dcache_prelock_sct1_en",
                &self.dcache_prelock_sct1_en().bit(),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DCACHE_PRELOCK_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - The bit is used to enable the first section of prelock function."]
    #[inline(always)]
    #[must_use]
    pub fn dcache_prelock_sct0_en(&mut self) -> DCACHE_PRELOCK_SCT0_EN_W<DCACHE_PRELOCK_CTRL_SPEC> {
        DCACHE_PRELOCK_SCT0_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - The bit is used to enable the second section of prelock function."]
    #[inline(always)]
    #[must_use]
    pub fn dcache_prelock_sct1_en(&mut self) -> DCACHE_PRELOCK_SCT1_EN_W<DCACHE_PRELOCK_CTRL_SPEC> {
        DCACHE_PRELOCK_SCT1_EN_W::new(self, 1)
    }
}
#[doc = "******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcache_prelock_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcache_prelock_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DCACHE_PRELOCK_CTRL_SPEC;
impl crate::RegisterSpec for DCACHE_PRELOCK_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcache_prelock_ctrl::R`](R) reader structure"]
impl crate::Readable for DCACHE_PRELOCK_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dcache_prelock_ctrl::W`](W) writer structure"]
impl crate::Writable for DCACHE_PRELOCK_CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DCACHE_PRELOCK_CTRL to value 0"]
impl crate::Resettable for DCACHE_PRELOCK_CTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
