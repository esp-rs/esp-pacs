#[doc = "Register `CACHE_MMU_POWER_CTRL` reader"]
pub struct R(crate::R<CACHE_MMU_POWER_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CACHE_MMU_POWER_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CACHE_MMU_POWER_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CACHE_MMU_POWER_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CACHE_MMU_POWER_CTRL` writer"]
pub struct W(crate::W<CACHE_MMU_POWER_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CACHE_MMU_POWER_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CACHE_MMU_POWER_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CACHE_MMU_POWER_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CACHE_MMU_MEM_FORCE_ON` reader - The bit is used to enable clock gating to save power when access mmu memory, 0: enable, 1: disable"]
pub type CACHE_MMU_MEM_FORCE_ON_R = crate::BitReader;
#[doc = "Field `CACHE_MMU_MEM_FORCE_ON` writer - The bit is used to enable clock gating to save power when access mmu memory, 0: enable, 1: disable"]
pub type CACHE_MMU_MEM_FORCE_ON_W<'a, const O: u8> =
    crate::BitWriter<'a, CACHE_MMU_POWER_CTRL_SPEC, O>;
#[doc = "Field `CACHE_MMU_MEM_FORCE_PD` reader - The bit is used to power mmu memory down, 0: follow_rtc_lslp_pd, 1: power down"]
pub type CACHE_MMU_MEM_FORCE_PD_R = crate::BitReader;
#[doc = "Field `CACHE_MMU_MEM_FORCE_PD` writer - The bit is used to power mmu memory down, 0: follow_rtc_lslp_pd, 1: power down"]
pub type CACHE_MMU_MEM_FORCE_PD_W<'a, const O: u8> =
    crate::BitWriter<'a, CACHE_MMU_POWER_CTRL_SPEC, O>;
#[doc = "Field `CACHE_MMU_MEM_FORCE_PU` reader - The bit is used to power mmu memory down, 0: follow_rtc_lslp_pd, 1: power up"]
pub type CACHE_MMU_MEM_FORCE_PU_R = crate::BitReader;
#[doc = "Field `CACHE_MMU_MEM_FORCE_PU` writer - The bit is used to power mmu memory down, 0: follow_rtc_lslp_pd, 1: power up"]
pub type CACHE_MMU_MEM_FORCE_PU_W<'a, const O: u8> =
    crate::BitWriter<'a, CACHE_MMU_POWER_CTRL_SPEC, O>;
impl R {
    #[doc = "Bit 0 - The bit is used to enable clock gating to save power when access mmu memory, 0: enable, 1: disable"]
    #[inline(always)]
    pub fn cache_mmu_mem_force_on(&self) -> CACHE_MMU_MEM_FORCE_ON_R {
        CACHE_MMU_MEM_FORCE_ON_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The bit is used to power mmu memory down, 0: follow_rtc_lslp_pd, 1: power down"]
    #[inline(always)]
    pub fn cache_mmu_mem_force_pd(&self) -> CACHE_MMU_MEM_FORCE_PD_R {
        CACHE_MMU_MEM_FORCE_PD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The bit is used to power mmu memory down, 0: follow_rtc_lslp_pd, 1: power up"]
    #[inline(always)]
    pub fn cache_mmu_mem_force_pu(&self) -> CACHE_MMU_MEM_FORCE_PU_R {
        CACHE_MMU_MEM_FORCE_PU_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CACHE_MMU_POWER_CTRL")
            .field(
                "cache_mmu_mem_force_on",
                &format_args!("{}", self.cache_mmu_mem_force_on().bit()),
            )
            .field(
                "cache_mmu_mem_force_pd",
                &format_args!("{}", self.cache_mmu_mem_force_pd().bit()),
            )
            .field(
                "cache_mmu_mem_force_pu",
                &format_args!("{}", self.cache_mmu_mem_force_pu().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CACHE_MMU_POWER_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - The bit is used to enable clock gating to save power when access mmu memory, 0: enable, 1: disable"]
    #[inline(always)]
    #[must_use]
    pub fn cache_mmu_mem_force_on(&mut self) -> CACHE_MMU_MEM_FORCE_ON_W<0> {
        CACHE_MMU_MEM_FORCE_ON_W::new(self)
    }
    #[doc = "Bit 1 - The bit is used to power mmu memory down, 0: follow_rtc_lslp_pd, 1: power down"]
    #[inline(always)]
    #[must_use]
    pub fn cache_mmu_mem_force_pd(&mut self) -> CACHE_MMU_MEM_FORCE_PD_W<1> {
        CACHE_MMU_MEM_FORCE_PD_W::new(self)
    }
    #[doc = "Bit 2 - The bit is used to power mmu memory down, 0: follow_rtc_lslp_pd, 1: power up"]
    #[inline(always)]
    #[must_use]
    pub fn cache_mmu_mem_force_pu(&mut self) -> CACHE_MMU_MEM_FORCE_PU_W<2> {
        CACHE_MMU_MEM_FORCE_PU_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This description will be updated in the near future.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cache_mmu_power_ctrl](index.html) module"]
pub struct CACHE_MMU_POWER_CTRL_SPEC;
impl crate::RegisterSpec for CACHE_MMU_POWER_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cache_mmu_power_ctrl::R](R) reader structure"]
impl crate::Readable for CACHE_MMU_POWER_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cache_mmu_power_ctrl::W](W) writer structure"]
impl crate::Writable for CACHE_MMU_POWER_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CACHE_MMU_POWER_CTRL to value 0x05"]
impl crate::Resettable for CACHE_MMU_POWER_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x05;
}
