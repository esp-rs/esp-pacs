#[doc = "Register `HP_DESIGN_FOR_VERIFICATION0` reader"]
pub type R = crate::R<HP_DESIGN_FOR_VERIFICATION0_SPEC>;
#[doc = "Register `HP_DESIGN_FOR_VERIFICATION0` writer"]
pub type W = crate::W<HP_DESIGN_FOR_VERIFICATION0_SPEC>;
#[doc = "Field `HP_DFV0` reader - register for DV"]
pub type HP_DFV0_R = crate::FieldReader<u32>;
#[doc = "Field `HP_DFV0` writer - register for DV"]
pub type HP_DFV0_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - register for DV"]
    #[inline(always)]
    pub fn hp_dfv0(&self) -> HP_DFV0_R {
        HP_DFV0_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HP_DESIGN_FOR_VERIFICATION0")
            .field("hp_dfv0", &format_args!("{}", self.hp_dfv0().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HP_DESIGN_FOR_VERIFICATION0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31 - register for DV"]
    #[inline(always)]
    #[must_use]
    pub fn hp_dfv0(&mut self) -> HP_DFV0_W<HP_DESIGN_FOR_VERIFICATION0_SPEC> {
        HP_DFV0_W::new(self, 0)
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
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hp_design_for_verification0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_design_for_verification0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HP_DESIGN_FOR_VERIFICATION0_SPEC;
impl crate::RegisterSpec for HP_DESIGN_FOR_VERIFICATION0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hp_design_for_verification0::R`](R) reader structure"]
impl crate::Readable for HP_DESIGN_FOR_VERIFICATION0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hp_design_for_verification0::W`](W) writer structure"]
impl crate::Writable for HP_DESIGN_FOR_VERIFICATION0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HP_DESIGN_FOR_VERIFICATION0 to value 0"]
impl crate::Resettable for HP_DESIGN_FOR_VERIFICATION0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
