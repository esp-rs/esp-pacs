#[doc = "Register `LP_AONCLKRST_MUX_HPSYS_RESET_BYPASS` reader"]
pub type R = crate::R<LP_AONCLKRST_MUX_HPSYS_RESET_BYPASS_SPEC>;
#[doc = "Register `LP_AONCLKRST_MUX_HPSYS_RESET_BYPASS` writer"]
pub type W = crate::W<LP_AONCLKRST_MUX_HPSYS_RESET_BYPASS_SPEC>;
#[doc = "Field `LP_AONCLKRST_MUX_HPSYS_RESET_BYPASS` reader - reserved"]
pub type LP_AONCLKRST_MUX_HPSYS_RESET_BYPASS_R = crate::FieldReader<u32>;
#[doc = "Field `LP_AONCLKRST_MUX_HPSYS_RESET_BYPASS` writer - reserved"]
pub type LP_AONCLKRST_MUX_HPSYS_RESET_BYPASS_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - reserved"]
    #[inline(always)]
    pub fn lp_aonclkrst_mux_hpsys_reset_bypass(&self) -> LP_AONCLKRST_MUX_HPSYS_RESET_BYPASS_R {
        LP_AONCLKRST_MUX_HPSYS_RESET_BYPASS_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LP_AONCLKRST_MUX_HPSYS_RESET_BYPASS")
            .field(
                "lp_aonclkrst_mux_hpsys_reset_bypass",
                &self.lp_aonclkrst_mux_hpsys_reset_bypass(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - reserved"]
    #[inline(always)]
    #[must_use]
    pub fn lp_aonclkrst_mux_hpsys_reset_bypass(
        &mut self,
    ) -> LP_AONCLKRST_MUX_HPSYS_RESET_BYPASS_W<LP_AONCLKRST_MUX_HPSYS_RESET_BYPASS_SPEC> {
        LP_AONCLKRST_MUX_HPSYS_RESET_BYPASS_W::new(self, 0)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_aonclkrst_mux_hpsys_reset_bypass::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_aonclkrst_mux_hpsys_reset_bypass::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LP_AONCLKRST_MUX_HPSYS_RESET_BYPASS_SPEC;
impl crate::RegisterSpec for LP_AONCLKRST_MUX_HPSYS_RESET_BYPASS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lp_aonclkrst_mux_hpsys_reset_bypass::R`](R) reader structure"]
impl crate::Readable for LP_AONCLKRST_MUX_HPSYS_RESET_BYPASS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lp_aonclkrst_mux_hpsys_reset_bypass::W`](W) writer structure"]
impl crate::Writable for LP_AONCLKRST_MUX_HPSYS_RESET_BYPASS_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LP_AONCLKRST_MUX_HPSYS_RESET_BYPASS to value 0xffff_ffff"]
impl crate::Resettable for LP_AONCLKRST_MUX_HPSYS_RESET_BYPASS_SPEC {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
