///Register `LACTLOADLO` reader
pub type R = crate::R<LACTLOADLO_SPEC>;
///Register `LACTLOADLO` writer
pub type W = crate::W<LACTLOADLO_SPEC>;
///Field `LOAD_LO` reader - Reserved.
pub type LOAD_LO_R = crate::FieldReader<u32>;
///Field `LOAD_LO` writer - Reserved.
pub type LOAD_LO_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Reserved.
    #[inline(always)]
    pub fn load_lo(&self) -> LOAD_LO_R {
        LOAD_LO_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LACTLOADLO")
            .field("load_lo", &self.load_lo())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Reserved.
    #[inline(always)]
    #[must_use]
    pub fn load_lo(&mut self) -> LOAD_LO_W<LACTLOADLO_SPEC> {
        LOAD_LO_W::new(self, 0)
    }
}
/**LACT load low register

You can [`read`](crate::generic::Reg::read) this register and get [`lactloadlo::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lactloadlo::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct LACTLOADLO_SPEC;
impl crate::RegisterSpec for LACTLOADLO_SPEC {
    type Ux = u32;
}
///`read()` method returns [`lactloadlo::R`](R) reader structure
impl crate::Readable for LACTLOADLO_SPEC {}
///`write(|w| ..)` method takes [`lactloadlo::W`](W) writer structure
impl crate::Writable for LACTLOADLO_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets LACTLOADLO to value 0
impl crate::Resettable for LACTLOADLO_SPEC {
    const RESET_VALUE: u32 = 0;
}
