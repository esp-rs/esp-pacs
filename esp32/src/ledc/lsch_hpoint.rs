#[doc = "Register `LSCH%s_HPOINT` reader"]
pub type R = crate::R<LSCH_HPOINT_SPEC>;
#[doc = "Register `LSCH%s_HPOINT` writer"]
pub type W = crate::W<LSCH_HPOINT_SPEC>;
#[doc = "Field `HPOINT` reader - The output value changes to high when lstimerx(x=\\[0 3\\]) selected by low speed channel0 has reached reg_hpoint_lsch0\\[19:0\\]"]
pub type HPOINT_R = crate::FieldReader<u32>;
#[doc = "Field `HPOINT` writer - The output value changes to high when lstimerx(x=\\[0 3\\]) selected by low speed channel0 has reached reg_hpoint_lsch0\\[19:0\\]"]
pub type HPOINT_W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:19 - The output value changes to high when lstimerx(x=\\[0 3\\]) selected by low speed channel0 has reached reg_hpoint_lsch0\\[19:0\\]"]
    #[inline(always)]
    pub fn hpoint(&self) -> HPOINT_R {
        HPOINT_R::new(self.bits & 0x000f_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LSCH_HPOINT")
            .field("hpoint", &format_args!("{}", self.hpoint().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<LSCH_HPOINT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:19 - The output value changes to high when lstimerx(x=\\[0 3\\]) selected by low speed channel0 has reached reg_hpoint_lsch0\\[19:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn hpoint(&mut self) -> HPOINT_W<LSCH_HPOINT_SPEC> {
        HPOINT_W::new(self, 0)
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
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lsch_hpoint::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lsch_hpoint::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LSCH_HPOINT_SPEC;
impl crate::RegisterSpec for LSCH_HPOINT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lsch_hpoint::R`](R) reader structure"]
impl crate::Readable for LSCH_HPOINT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lsch_hpoint::W`](W) writer structure"]
impl crate::Writable for LSCH_HPOINT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LSCH%s_HPOINT to value 0"]
impl crate::Resettable for LSCH_HPOINT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
