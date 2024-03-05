#[doc = "Register `HSCH%s_HPOINT` reader"]
pub type R = crate::R<HSCH_HPOINT_SPEC>;
#[doc = "Register `HSCH%s_HPOINT` writer"]
pub type W = crate::W<HSCH_HPOINT_SPEC>;
#[doc = "Field `HPOINT` reader - The output value changes to high when htimerx(x=\\[0 3\\]) selected by high speed channel0 has reached reg_hpoint_hsch0\\[19:0\\]"]
pub type HPOINT_R = crate::FieldReader<u32>;
#[doc = "Field `HPOINT` writer - The output value changes to high when htimerx(x=\\[0 3\\]) selected by high speed channel0 has reached reg_hpoint_hsch0\\[19:0\\]"]
pub type HPOINT_W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:19 - The output value changes to high when htimerx(x=\\[0 3\\]) selected by high speed channel0 has reached reg_hpoint_hsch0\\[19:0\\]"]
    #[inline(always)]
    pub fn hpoint(&self) -> HPOINT_R {
        HPOINT_R::new(self.bits & 0x000f_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HSCH_HPOINT")
            .field("hpoint", &format_args!("{}", self.hpoint().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HSCH_HPOINT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:19 - The output value changes to high when htimerx(x=\\[0 3\\]) selected by high speed channel0 has reached reg_hpoint_hsch0\\[19:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn hpoint(&mut self) -> HPOINT_W<HSCH_HPOINT_SPEC> {
        HPOINT_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hsch_hpoint::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hsch_hpoint::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HSCH_HPOINT_SPEC;
impl crate::RegisterSpec for HSCH_HPOINT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hsch_hpoint::R`](R) reader structure"]
impl crate::Readable for HSCH_HPOINT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hsch_hpoint::W`](W) writer structure"]
impl crate::Writable for HSCH_HPOINT_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HSCH%s_HPOINT to value 0"]
impl crate::Resettable for HSCH_HPOINT_SPEC {
    const RESET_VALUE: u32 = 0;
}
