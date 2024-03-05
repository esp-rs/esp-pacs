#[doc = "Register `MEM_PD_MASK` reader"]
pub type R = crate::R<MEM_PD_MASK_SPEC>;
#[doc = "Register `MEM_PD_MASK` writer"]
pub type W = crate::W<MEM_PD_MASK_SPEC>;
#[doc = "Field `LSLP_MEM_PD_MASK` reader - reg_lslp_mem_pd_mask"]
pub type LSLP_MEM_PD_MASK_R = crate::BitReader;
#[doc = "Field `LSLP_MEM_PD_MASK` writer - reg_lslp_mem_pd_mask"]
pub type LSLP_MEM_PD_MASK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - reg_lslp_mem_pd_mask"]
    #[inline(always)]
    pub fn lslp_mem_pd_mask(&self) -> LSLP_MEM_PD_MASK_R {
        LSLP_MEM_PD_MASK_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MEM_PD_MASK")
            .field(
                "lslp_mem_pd_mask",
                &format_args!("{}", self.lslp_mem_pd_mask().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<MEM_PD_MASK_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - reg_lslp_mem_pd_mask"]
    #[inline(always)]
    #[must_use]
    pub fn lslp_mem_pd_mask(&mut self) -> LSLP_MEM_PD_MASK_W<MEM_PD_MASK_SPEC> {
        LSLP_MEM_PD_MASK_W::new(self, 0)
    }
}
#[doc = "memory power down mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_pd_mask::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_pd_mask::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MEM_PD_MASK_SPEC;
impl crate::RegisterSpec for MEM_PD_MASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_pd_mask::R`](R) reader structure"]
impl crate::Readable for MEM_PD_MASK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mem_pd_mask::W`](W) writer structure"]
impl crate::Writable for MEM_PD_MASK_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEM_PD_MASK to value 0x01"]
impl crate::Resettable for MEM_PD_MASK_SPEC {
    const RESET_VALUE: u32 = 0x01;
}
