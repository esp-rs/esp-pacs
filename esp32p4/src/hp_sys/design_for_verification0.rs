#[doc = "Register `DESIGN_FOR_VERIFICATION0` reader"]
pub type R = crate::R<DESIGN_FOR_VERIFICATION0_SPEC>;
#[doc = "Register `DESIGN_FOR_VERIFICATION0` writer"]
pub type W = crate::W<DESIGN_FOR_VERIFICATION0_SPEC>;
#[doc = "Field `DFV0` reader - register for DV"]
pub type DFV0_R = crate::FieldReader<u32>;
#[doc = "Field `DFV0` writer - register for DV"]
pub type DFV0_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - register for DV"]
    #[inline(always)]
    pub fn dfv0(&self) -> DFV0_R {
        DFV0_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DESIGN_FOR_VERIFICATION0")
            .field("dfv0", &self.dfv0())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - register for DV"]
    #[inline(always)]
    pub fn dfv0(&mut self) -> DFV0_W<DESIGN_FOR_VERIFICATION0_SPEC> {
        DFV0_W::new(self, 0)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`design_for_verification0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`design_for_verification0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DESIGN_FOR_VERIFICATION0_SPEC;
impl crate::RegisterSpec for DESIGN_FOR_VERIFICATION0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`design_for_verification0::R`](R) reader structure"]
impl crate::Readable for DESIGN_FOR_VERIFICATION0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`design_for_verification0::W`](W) writer structure"]
impl crate::Writable for DESIGN_FOR_VERIFICATION0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DESIGN_FOR_VERIFICATION0 to value 0"]
impl crate::Resettable for DESIGN_FOR_VERIFICATION0_SPEC {}
