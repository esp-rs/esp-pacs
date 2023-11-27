#[doc = "Register `HP_DESIGN_FOR_VERIFICATION1` reader"]
pub type R = crate::R<HP_DESIGN_FOR_VERIFICATION1_SPEC>;
#[doc = "Register `HP_DESIGN_FOR_VERIFICATION1` writer"]
pub type W = crate::W<HP_DESIGN_FOR_VERIFICATION1_SPEC>;
#[doc = "Field `HP_DFV1` reader - register for DV"]
pub type HP_DFV1_R = crate::FieldReader<u32>;
#[doc = "Field `HP_DFV1` writer - register for DV"]
pub type HP_DFV1_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - register for DV"]
    #[inline(always)]
    pub fn hp_dfv1(&self) -> HP_DFV1_R {
        HP_DFV1_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HP_DESIGN_FOR_VERIFICATION1")
            .field("hp_dfv1", &format_args!("{}", self.hp_dfv1().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HP_DESIGN_FOR_VERIFICATION1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31 - register for DV"]
    #[inline(always)]
    #[must_use]
    pub fn hp_dfv1(&mut self) -> HP_DFV1_W<HP_DESIGN_FOR_VERIFICATION1_SPEC> {
        HP_DFV1_W::new(self, 0)
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
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hp_design_for_verification1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_design_for_verification1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HP_DESIGN_FOR_VERIFICATION1_SPEC;
impl crate::RegisterSpec for HP_DESIGN_FOR_VERIFICATION1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hp_design_for_verification1::R`](R) reader structure"]
impl crate::Readable for HP_DESIGN_FOR_VERIFICATION1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hp_design_for_verification1::W`](W) writer structure"]
impl crate::Writable for HP_DESIGN_FOR_VERIFICATION1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HP_DESIGN_FOR_VERIFICATION1 to value 0"]
impl crate::Resettable for HP_DESIGN_FOR_VERIFICATION1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
