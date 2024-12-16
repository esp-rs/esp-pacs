#[doc = "Register `LACTLOADLO` reader"]
pub type R = crate::R<LACTLOADLO_SPEC>;
#[doc = "Register `LACTLOADLO` writer"]
pub type W = crate::W<LACTLOADLO_SPEC>;
#[doc = "Field `LOAD_LO` reader - Reserved."]
pub type LOAD_LO_R = crate::FieldReader<u32>;
#[doc = "Field `LOAD_LO` writer - Reserved."]
pub type LOAD_LO_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Reserved."]
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
    #[doc = "Bits 0:31 - Reserved."]
    #[inline(always)]
    pub fn load_lo(&mut self) -> LOAD_LO_W<LACTLOADLO_SPEC> {
        LOAD_LO_W::new(self, 0)
    }
}
#[doc = "LACT load low register\n\nYou can [`read`](crate::Reg::read) this register and get [`lactloadlo::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lactloadlo::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LACTLOADLO_SPEC;
impl crate::RegisterSpec for LACTLOADLO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lactloadlo::R`](R) reader structure"]
impl crate::Readable for LACTLOADLO_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lactloadlo::W`](W) writer structure"]
impl crate::Writable for LACTLOADLO_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LACTLOADLO to value 0"]
impl crate::Resettable for LACTLOADLO_SPEC {
    const RESET_VALUE: u32 = 0;
}
