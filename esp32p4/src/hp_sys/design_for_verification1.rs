#[doc = "Register `DESIGN_FOR_VERIFICATION1` reader"]
pub type R = crate::R<DESIGN_FOR_VERIFICATION1_SPEC>;
#[doc = "Register `DESIGN_FOR_VERIFICATION1` writer"]
pub type W = crate::W<DESIGN_FOR_VERIFICATION1_SPEC>;
#[doc = "Field `DFV1` reader - register for DV"]
pub type DFV1_R = crate::FieldReader<u32>;
#[doc = "Field `DFV1` writer - register for DV"]
pub type DFV1_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - register for DV"]
    #[inline(always)]
    pub fn dfv1(&self) -> DFV1_R {
        DFV1_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DESIGN_FOR_VERIFICATION1")
            .field("dfv1", &format_args!("{}", self.dfv1().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DESIGN_FOR_VERIFICATION1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31 - register for DV"]
    #[inline(always)]
    #[must_use]
    pub fn dfv1(&mut self) -> DFV1_W<DESIGN_FOR_VERIFICATION1_SPEC> {
        DFV1_W::new(self, 0)
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
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`design_for_verification1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`design_for_verification1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DESIGN_FOR_VERIFICATION1_SPEC;
impl crate::RegisterSpec for DESIGN_FOR_VERIFICATION1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`design_for_verification1::R`](R) reader structure"]
impl crate::Readable for DESIGN_FOR_VERIFICATION1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`design_for_verification1::W`](W) writer structure"]
impl crate::Writable for DESIGN_FOR_VERIFICATION1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DESIGN_FOR_VERIFICATION1 to value 0"]
impl crate::Resettable for DESIGN_FOR_VERIFICATION1_SPEC {
    const RESET_VALUE: u32 = 0;
}
