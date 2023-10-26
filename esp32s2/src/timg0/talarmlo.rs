#[doc = "Register `T%sALARMLO` reader"]
pub type R = crate::R<TALARMLO_SPEC>;
#[doc = "Register `T%sALARMLO` writer"]
pub type W = crate::W<TALARMLO_SPEC>;
#[doc = "Field `ALARM_LO` reader - Timer %s alarm trigger time-base counter value, low 32 bits."]
pub type ALARM_LO_R = crate::FieldReader<u32>;
#[doc = "Field `ALARM_LO` writer - Timer %s alarm trigger time-base counter value, low 32 bits."]
pub type ALARM_LO_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Timer %s alarm trigger time-base counter value, low 32 bits."]
    #[inline(always)]
    pub fn alarm_lo(&self) -> ALARM_LO_R {
        ALARM_LO_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TALARMLO")
            .field("alarm_lo", &format_args!("{}", self.alarm_lo().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TALARMLO_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - Timer %s alarm trigger time-base counter value, low 32 bits."]
    #[inline(always)]
    #[must_use]
    pub fn alarm_lo(&mut self) -> ALARM_LO_W<TALARMLO_SPEC, 0> {
        ALARM_LO_W::new(self)
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
#[doc = "Timer %s alarm value, low 32 bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`talarmlo::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`talarmlo::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TALARMLO_SPEC;
impl crate::RegisterSpec for TALARMLO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`talarmlo::R`](R) reader structure"]
impl crate::Readable for TALARMLO_SPEC {}
#[doc = "`write(|w| ..)` method takes [`talarmlo::W`](W) writer structure"]
impl crate::Writable for TALARMLO_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets T%sALARMLO to value 0"]
impl crate::Resettable for TALARMLO_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
