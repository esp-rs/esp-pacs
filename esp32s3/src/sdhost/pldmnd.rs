#[doc = "Register `PLDMND` writer"]
pub type W = crate::W<PLDMND_SPEC>;
#[doc = "Field `PD` writer - Poll Demand. If the OWNER bit of a descriptor is not set, the FSM goes to the Suspend state. The host needs to write any value into this register for the IDMAC FSM to resume normal descriptor fetch operation. This is a write only ."]
pub type PD_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PLDMND_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:31 - Poll Demand. If the OWNER bit of a descriptor is not set, the FSM goes to the Suspend state. The host needs to write any value into this register for the IDMAC FSM to resume normal descriptor fetch operation. This is a write only ."]
    #[inline(always)]
    #[must_use]
    pub fn pd(&mut self) -> PD_W<PLDMND_SPEC> {
        PD_W::new(self, 0)
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
#[doc = "Poll demand configuration register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pldmnd::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PLDMND_SPEC;
impl crate::RegisterSpec for PLDMND_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`pldmnd::W`](W) writer structure"]
impl crate::Writable for PLDMND_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PLDMND to value 0"]
impl crate::Resettable for PLDMND_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
