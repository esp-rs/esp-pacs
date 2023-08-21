#[doc = "Register `CORE_1_CONTROL_1` reader"]
pub type R = crate::R<CORE_1_CONTROL_1_SPEC>;
#[doc = "Register `CORE_1_CONTROL_1` writer"]
pub type W = crate::W<CORE_1_CONTROL_1_SPEC>;
#[doc = "Field `CONTROL_CORE_1_MESSAGE` reader - it's only a R/W register, no function, software can write any value"]
pub type CONTROL_CORE_1_MESSAGE_R = crate::FieldReader<u32>;
#[doc = "Field `CONTROL_CORE_1_MESSAGE` writer - it's only a R/W register, no function, software can write any value"]
pub type CONTROL_CORE_1_MESSAGE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - it's only a R/W register, no function, software can write any value"]
    #[inline(always)]
    pub fn control_core_1_message(&self) -> CONTROL_CORE_1_MESSAGE_R {
        CONTROL_CORE_1_MESSAGE_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE_1_CONTROL_1")
            .field(
                "control_core_1_message",
                &format_args!("{}", self.control_core_1_message().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CORE_1_CONTROL_1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - it's only a R/W register, no function, software can write any value"]
    #[inline(always)]
    #[must_use]
    pub fn control_core_1_message(&mut self) -> CONTROL_CORE_1_MESSAGE_W<CORE_1_CONTROL_1_SPEC, 0> {
        CONTROL_CORE_1_MESSAGE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Core0 control regiter 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_1_control_1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_1_control_1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CORE_1_CONTROL_1_SPEC;
impl crate::RegisterSpec for CORE_1_CONTROL_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core_1_control_1::R`](R) reader structure"]
impl crate::Readable for CORE_1_CONTROL_1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`core_1_control_1::W`](W) writer structure"]
impl crate::Writable for CORE_1_CONTROL_1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CORE_1_CONTROL_1 to value 0"]
impl crate::Resettable for CORE_1_CONTROL_1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
