///Register `RDN_ECO_CS` reader
pub type R = crate::R<RDN_ECO_CS_SPEC>;
///Register `RDN_ECO_CS` writer
pub type W = crate::W<RDN_ECO_CS_SPEC>;
///Field `RDN_ECO_EN` reader - rdn_eco_en
pub type RDN_ECO_EN_R = crate::BitReader;
///Field `RDN_ECO_EN` writer - rdn_eco_en
pub type RDN_ECO_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RDN_ECO_RESULT` reader - rdn_eco_result
pub type RDN_ECO_RESULT_R = crate::BitReader;
impl R {
    ///Bit 0 - rdn_eco_en
    #[inline(always)]
    pub fn rdn_eco_en(&self) -> RDN_ECO_EN_R {
        RDN_ECO_EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - rdn_eco_result
    #[inline(always)]
    pub fn rdn_eco_result(&self) -> RDN_ECO_RESULT_R {
        RDN_ECO_RESULT_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RDN_ECO_CS")
            .field("rdn_eco_en", &self.rdn_eco_en())
            .field("rdn_eco_result", &self.rdn_eco_result())
            .finish()
    }
}
impl W {
    ///Bit 0 - rdn_eco_en
    #[inline(always)]
    #[must_use]
    pub fn rdn_eco_en(&mut self) -> RDN_ECO_EN_W<RDN_ECO_CS_SPEC> {
        RDN_ECO_EN_W::new(self, 0)
    }
}
/**rdn eco cs register

You can [`read`](crate::generic::Reg::read) this register and get [`rdn_eco_cs::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rdn_eco_cs::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct RDN_ECO_CS_SPEC;
impl crate::RegisterSpec for RDN_ECO_CS_SPEC {
    type Ux = u32;
}
///`read()` method returns [`rdn_eco_cs::R`](R) reader structure
impl crate::Readable for RDN_ECO_CS_SPEC {}
///`write(|w| ..)` method takes [`rdn_eco_cs::W`](W) writer structure
impl crate::Writable for RDN_ECO_CS_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets RDN_ECO_CS to value 0
impl crate::Resettable for RDN_ECO_CS_SPEC {
    const RESET_VALUE: u32 = 0;
}
