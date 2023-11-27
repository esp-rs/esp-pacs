#[doc = "Register `LP_TCM_RAM_RDN_ECO_CS` reader"]
pub type R = crate::R<LP_TCM_RAM_RDN_ECO_CS_SPEC>;
#[doc = "Register `LP_TCM_RAM_RDN_ECO_CS` writer"]
pub type W = crate::W<LP_TCM_RAM_RDN_ECO_CS_SPEC>;
#[doc = "Field `LP_TCM_RAM_RDN_ECO_EN` reader - need_des"]
pub type LP_TCM_RAM_RDN_ECO_EN_R = crate::BitReader;
#[doc = "Field `LP_TCM_RAM_RDN_ECO_EN` writer - need_des"]
pub type LP_TCM_RAM_RDN_ECO_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_TCM_RAM_RDN_ECO_RESULT` reader - need_des"]
pub type LP_TCM_RAM_RDN_ECO_RESULT_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn lp_tcm_ram_rdn_eco_en(&self) -> LP_TCM_RAM_RDN_ECO_EN_R {
        LP_TCM_RAM_RDN_ECO_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - need_des"]
    #[inline(always)]
    pub fn lp_tcm_ram_rdn_eco_result(&self) -> LP_TCM_RAM_RDN_ECO_RESULT_R {
        LP_TCM_RAM_RDN_ECO_RESULT_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LP_TCM_RAM_RDN_ECO_CS")
            .field(
                "lp_tcm_ram_rdn_eco_en",
                &format_args!("{}", self.lp_tcm_ram_rdn_eco_en().bit()),
            )
            .field(
                "lp_tcm_ram_rdn_eco_result",
                &format_args!("{}", self.lp_tcm_ram_rdn_eco_result().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<LP_TCM_RAM_RDN_ECO_CS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_tcm_ram_rdn_eco_en(&mut self) -> LP_TCM_RAM_RDN_ECO_EN_W<LP_TCM_RAM_RDN_ECO_CS_SPEC> {
        LP_TCM_RAM_RDN_ECO_EN_W::new(self, 0)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_tcm_ram_rdn_eco_cs::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_tcm_ram_rdn_eco_cs::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LP_TCM_RAM_RDN_ECO_CS_SPEC;
impl crate::RegisterSpec for LP_TCM_RAM_RDN_ECO_CS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lp_tcm_ram_rdn_eco_cs::R`](R) reader structure"]
impl crate::Readable for LP_TCM_RAM_RDN_ECO_CS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lp_tcm_ram_rdn_eco_cs::W`](W) writer structure"]
impl crate::Writable for LP_TCM_RAM_RDN_ECO_CS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LP_TCM_RAM_RDN_ECO_CS to value 0"]
impl crate::Resettable for LP_TCM_RAM_RDN_ECO_CS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
