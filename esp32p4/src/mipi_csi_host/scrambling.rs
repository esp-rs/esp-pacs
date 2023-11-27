#[doc = "Register `SCRAMBLING` reader"]
pub type R = crate::R<SCRAMBLING_SPEC>;
#[doc = "Register `SCRAMBLING` writer"]
pub type W = crate::W<SCRAMBLING_SPEC>;
#[doc = "Field `SCRAMBLE_ENABLE` reader - NA"]
pub type SCRAMBLE_ENABLE_R = crate::BitReader;
#[doc = "Field `SCRAMBLE_ENABLE` writer - NA"]
pub type SCRAMBLE_ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn scramble_enable(&self) -> SCRAMBLE_ENABLE_R {
        SCRAMBLE_ENABLE_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SCRAMBLING")
            .field(
                "scramble_enable",
                &format_args!("{}", self.scramble_enable().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SCRAMBLING_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn scramble_enable(&mut self) -> SCRAMBLE_ENABLE_W<SCRAMBLING_SPEC> {
        SCRAMBLE_ENABLE_W::new(self, 0)
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
#[doc = "NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scrambling::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scrambling::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SCRAMBLING_SPEC;
impl crate::RegisterSpec for SCRAMBLING_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scrambling::R`](R) reader structure"]
impl crate::Readable for SCRAMBLING_SPEC {}
#[doc = "`write(|w| ..)` method takes [`scrambling::W`](W) writer structure"]
impl crate::Writable for SCRAMBLING_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCRAMBLING to value 0"]
impl crate::Resettable for SCRAMBLING_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
