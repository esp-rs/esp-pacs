#[doc = "Register `LP_TCM_RAM_RDN_ECO_LOW` reader"]
pub type R = crate::R<LP_TCM_RAM_RDN_ECO_LOW_SPEC>;
#[doc = "Register `LP_TCM_RAM_RDN_ECO_LOW` writer"]
pub type W = crate::W<LP_TCM_RAM_RDN_ECO_LOW_SPEC>;
#[doc = "Field `LP_TCM_RAM_RDN_ECO_LOW` reader - need_des"]
pub type LP_TCM_RAM_RDN_ECO_LOW_R = crate::FieldReader<u32>;
#[doc = "Field `LP_TCM_RAM_RDN_ECO_LOW` writer - need_des"]
pub type LP_TCM_RAM_RDN_ECO_LOW_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - need_des"]
    #[inline(always)]
    pub fn lp_tcm_ram_rdn_eco_low(&self) -> LP_TCM_RAM_RDN_ECO_LOW_R {
        LP_TCM_RAM_RDN_ECO_LOW_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LP_TCM_RAM_RDN_ECO_LOW")
            .field(
                "lp_tcm_ram_rdn_eco_low",
                &format_args!("{}", self.lp_tcm_ram_rdn_eco_low().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<LP_TCM_RAM_RDN_ECO_LOW_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_tcm_ram_rdn_eco_low(
        &mut self,
    ) -> LP_TCM_RAM_RDN_ECO_LOW_W<LP_TCM_RAM_RDN_ECO_LOW_SPEC> {
        LP_TCM_RAM_RDN_ECO_LOW_W::new(self, 0)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_tcm_ram_rdn_eco_low::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_tcm_ram_rdn_eco_low::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LP_TCM_RAM_RDN_ECO_LOW_SPEC;
impl crate::RegisterSpec for LP_TCM_RAM_RDN_ECO_LOW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lp_tcm_ram_rdn_eco_low::R`](R) reader structure"]
impl crate::Readable for LP_TCM_RAM_RDN_ECO_LOW_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lp_tcm_ram_rdn_eco_low::W`](W) writer structure"]
impl crate::Writable for LP_TCM_RAM_RDN_ECO_LOW_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LP_TCM_RAM_RDN_ECO_LOW to value 0"]
impl crate::Resettable for LP_TCM_RAM_RDN_ECO_LOW_SPEC {
    const RESET_VALUE: u32 = 0;
}
