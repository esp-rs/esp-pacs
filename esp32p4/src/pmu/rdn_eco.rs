///Register `RDN_ECO` reader
pub type R = crate::R<RDN_ECO_SPEC>;
///Register `RDN_ECO` writer
pub type W = crate::W<RDN_ECO_SPEC>;
///Field `PMU_RDN_ECO_RESULT` reader - need_des
pub type PMU_RDN_ECO_RESULT_R = crate::BitReader;
///Field `PMU_RDN_ECO_EN` reader - need_des
pub type PMU_RDN_ECO_EN_R = crate::BitReader;
///Field `PMU_RDN_ECO_EN` writer - need_des
pub type PMU_RDN_ECO_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - need_des
    #[inline(always)]
    pub fn pmu_rdn_eco_result(&self) -> PMU_RDN_ECO_RESULT_R {
        PMU_RDN_ECO_RESULT_R::new((self.bits & 1) != 0)
    }
    ///Bit 31 - need_des
    #[inline(always)]
    pub fn pmu_rdn_eco_en(&self) -> PMU_RDN_ECO_EN_R {
        PMU_RDN_ECO_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RDN_ECO")
            .field("pmu_rdn_eco_result", &self.pmu_rdn_eco_result())
            .field("pmu_rdn_eco_en", &self.pmu_rdn_eco_en())
            .finish()
    }
}
impl W {
    ///Bit 31 - need_des
    #[inline(always)]
    #[must_use]
    pub fn pmu_rdn_eco_en(&mut self) -> PMU_RDN_ECO_EN_W<RDN_ECO_SPEC> {
        PMU_RDN_ECO_EN_W::new(self, 31)
    }
}
/**need_des

You can [`read`](crate::generic::Reg::read) this register and get [`rdn_eco::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rdn_eco::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct RDN_ECO_SPEC;
impl crate::RegisterSpec for RDN_ECO_SPEC {
    type Ux = u32;
}
///`read()` method returns [`rdn_eco::R`](R) reader structure
impl crate::Readable for RDN_ECO_SPEC {}
///`write(|w| ..)` method takes [`rdn_eco::W`](W) writer structure
impl crate::Writable for RDN_ECO_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets RDN_ECO to value 0
impl crate::Resettable for RDN_ECO_SPEC {
    const RESET_VALUE: u32 = 0;
}
