///Register `RDN_ECO_HIGH` reader
pub type R = crate::R<RDN_ECO_HIGH_SPEC>;
///Register `RDN_ECO_HIGH` writer
pub type W = crate::W<RDN_ECO_HIGH_SPEC>;
///Field `RDN_ECO_HIGH` reader - rdn_eco_high
pub type RDN_ECO_HIGH_R = crate::FieldReader<u32>;
///Field `RDN_ECO_HIGH` writer - rdn_eco_high
pub type RDN_ECO_HIGH_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - rdn_eco_high
    #[inline(always)]
    pub fn rdn_eco_high(&self) -> RDN_ECO_HIGH_R {
        RDN_ECO_HIGH_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RDN_ECO_HIGH")
            .field("rdn_eco_high", &self.rdn_eco_high())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - rdn_eco_high
    #[inline(always)]
    #[must_use]
    pub fn rdn_eco_high(&mut self) -> RDN_ECO_HIGH_W<RDN_ECO_HIGH_SPEC> {
        RDN_ECO_HIGH_W::new(self, 0)
    }
}
/**rdn eco all high register

You can [`read`](crate::generic::Reg::read) this register and get [`rdn_eco_high::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rdn_eco_high::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct RDN_ECO_HIGH_SPEC;
impl crate::RegisterSpec for RDN_ECO_HIGH_SPEC {
    type Ux = u32;
}
///`read()` method returns [`rdn_eco_high::R`](R) reader structure
impl crate::Readable for RDN_ECO_HIGH_SPEC {}
///`write(|w| ..)` method takes [`rdn_eco_high::W`](W) writer structure
impl crate::Writable for RDN_ECO_HIGH_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets RDN_ECO_HIGH to value 0xffff_ffff
impl crate::Resettable for RDN_ECO_HIGH_SPEC {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
