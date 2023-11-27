#[doc = "Register `FILTER_MATCH_CONTROL` reader"]
pub type R = crate::R<FILTER_MATCH_CONTROL_SPEC>;
#[doc = "Register `FILTER_MATCH_CONTROL` writer"]
pub type W = crate::W<FILTER_MATCH_CONTROL_SPEC>;
#[doc = "Field `MATCH_CHOICE_PRIVILEGE` reader - Select match which privilege level when \\hyperref\\[fielddesc:TRACEMATCHPRIVILEGE\\]{TRACE_MATCH_PRIVILEGE} is set. \\\\1: machine mode. \\\\0: user mode"]
pub type MATCH_CHOICE_PRIVILEGE_R = crate::BitReader;
#[doc = "Field `MATCH_CHOICE_PRIVILEGE` writer - Select match which privilege level when \\hyperref\\[fielddesc:TRACEMATCHPRIVILEGE\\]{TRACE_MATCH_PRIVILEGE} is set. \\\\1: machine mode. \\\\0: user mode"]
pub type MATCH_CHOICE_PRIVILEGE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MATCH_VALUE_INTERRUPT` reader - Select which match which itype when \\hyperref\\[fielddesc:TRACEMATCHINTERRUPT\\]{TRACE_MATCH_INTERRUP} is set. \\\\1: match itype of 2. \\\\0: match itype or 1."]
pub type MATCH_VALUE_INTERRUPT_R = crate::BitReader;
#[doc = "Field `MATCH_VALUE_INTERRUPT` writer - Select which match which itype when \\hyperref\\[fielddesc:TRACEMATCHINTERRUPT\\]{TRACE_MATCH_INTERRUP} is set. \\\\1: match itype of 2. \\\\0: match itype or 1."]
pub type MATCH_VALUE_INTERRUPT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MATCH_CHOICE_ECAUSE` reader - specified which ecause matched."]
pub type MATCH_CHOICE_ECAUSE_R = crate::FieldReader;
#[doc = "Field `MATCH_CHOICE_ECAUSE` writer - specified which ecause matched."]
pub type MATCH_CHOICE_ECAUSE_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bit 0 - Select match which privilege level when \\hyperref\\[fielddesc:TRACEMATCHPRIVILEGE\\]{TRACE_MATCH_PRIVILEGE} is set. \\\\1: machine mode. \\\\0: user mode"]
    #[inline(always)]
    pub fn match_choice_privilege(&self) -> MATCH_CHOICE_PRIVILEGE_R {
        MATCH_CHOICE_PRIVILEGE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Select which match which itype when \\hyperref\\[fielddesc:TRACEMATCHINTERRUPT\\]{TRACE_MATCH_INTERRUP} is set. \\\\1: match itype of 2. \\\\0: match itype or 1."]
    #[inline(always)]
    pub fn match_value_interrupt(&self) -> MATCH_VALUE_INTERRUPT_R {
        MATCH_VALUE_INTERRUPT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:7 - specified which ecause matched."]
    #[inline(always)]
    pub fn match_choice_ecause(&self) -> MATCH_CHOICE_ECAUSE_R {
        MATCH_CHOICE_ECAUSE_R::new(((self.bits >> 2) & 0x3f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FILTER_MATCH_CONTROL")
            .field(
                "match_choice_privilege",
                &format_args!("{}", self.match_choice_privilege().bit()),
            )
            .field(
                "match_value_interrupt",
                &format_args!("{}", self.match_value_interrupt().bit()),
            )
            .field(
                "match_choice_ecause",
                &format_args!("{}", self.match_choice_ecause().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<FILTER_MATCH_CONTROL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - Select match which privilege level when \\hyperref\\[fielddesc:TRACEMATCHPRIVILEGE\\]{TRACE_MATCH_PRIVILEGE} is set. \\\\1: machine mode. \\\\0: user mode"]
    #[inline(always)]
    #[must_use]
    pub fn match_choice_privilege(
        &mut self,
    ) -> MATCH_CHOICE_PRIVILEGE_W<FILTER_MATCH_CONTROL_SPEC> {
        MATCH_CHOICE_PRIVILEGE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Select which match which itype when \\hyperref\\[fielddesc:TRACEMATCHINTERRUPT\\]{TRACE_MATCH_INTERRUP} is set. \\\\1: match itype of 2. \\\\0: match itype or 1."]
    #[inline(always)]
    #[must_use]
    pub fn match_value_interrupt(&mut self) -> MATCH_VALUE_INTERRUPT_W<FILTER_MATCH_CONTROL_SPEC> {
        MATCH_VALUE_INTERRUPT_W::new(self, 1)
    }
    #[doc = "Bits 2:7 - specified which ecause matched."]
    #[inline(always)]
    #[must_use]
    pub fn match_choice_ecause(&mut self) -> MATCH_CHOICE_ECAUSE_W<FILTER_MATCH_CONTROL_SPEC> {
        MATCH_CHOICE_ECAUSE_W::new(self, 2)
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
#[doc = "filter match control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`filter_match_control::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`filter_match_control::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FILTER_MATCH_CONTROL_SPEC;
impl crate::RegisterSpec for FILTER_MATCH_CONTROL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`filter_match_control::R`](R) reader structure"]
impl crate::Readable for FILTER_MATCH_CONTROL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`filter_match_control::W`](W) writer structure"]
impl crate::Writable for FILTER_MATCH_CONTROL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FILTER_MATCH_CONTROL to value 0"]
impl crate::Resettable for FILTER_MATCH_CONTROL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
