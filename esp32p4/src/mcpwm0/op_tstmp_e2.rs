#[doc = "Register `OP%s_TSTMP_E2` reader"]
pub type R = crate::R<OP_TSTMP_E2_SPEC>;
#[doc = "Register `OP%s_TSTMP_E2` writer"]
pub type W = crate::W<OP_TSTMP_E2_SPEC>;
#[doc = "Field `OP_TSTMP_E2` reader - Configures generator%s timer stamp E2 value register"]
pub type OP_TSTMP_E2_R = crate::FieldReader<u16>;
#[doc = "Field `OP_TSTMP_E2` writer - Configures generator%s timer stamp E2 value register"]
pub type OP_TSTMP_E2_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Configures generator%s timer stamp E2 value register"]
    #[inline(always)]
    pub fn op_tstmp_e2(&self) -> OP_TSTMP_E2_R {
        OP_TSTMP_E2_R::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OP_TSTMP_E2")
            .field(
                "op_tstmp_e2",
                &format_args!("{}", self.op_tstmp_e2().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<OP_TSTMP_E2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:15 - Configures generator%s timer stamp E2 value register"]
    #[inline(always)]
    #[must_use]
    pub fn op_tstmp_e2(&mut self) -> OP_TSTMP_E2_W<OP_TSTMP_E2_SPEC> {
        OP_TSTMP_E2_W::new(self, 0)
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
#[doc = "Generator%s timer stamp E2 value register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`op_tstmp_e2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`op_tstmp_e2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OP_TSTMP_E2_SPEC;
impl crate::RegisterSpec for OP_TSTMP_E2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`op_tstmp_e2::R`](R) reader structure"]
impl crate::Readable for OP_TSTMP_E2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`op_tstmp_e2::W`](W) writer structure"]
impl crate::Writable for OP_TSTMP_E2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OP%s_TSTMP_E2 to value 0"]
impl crate::Resettable for OP_TSTMP_E2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
