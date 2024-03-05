#[doc = "Register `LLP0` reader"]
pub type R = crate::R<LLP0_SPEC>;
#[doc = "Register `LLP0` writer"]
pub type W = crate::W<LLP0_SPEC>;
#[doc = "Field `CH1_LMS` reader - NA"]
pub type CH1_LMS_R = crate::BitReader;
#[doc = "Field `CH1_LMS` writer - NA"]
pub type CH1_LMS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1_LOC0` reader - NA"]
pub type CH1_LOC0_R = crate::FieldReader<u32>;
#[doc = "Field `CH1_LOC0` writer - NA"]
pub type CH1_LOC0_W<'a, REG> = crate::FieldWriter<'a, REG, 26, u32>;
impl R {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn ch1_lms(&self) -> CH1_LMS_R {
        CH1_LMS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 6:31 - NA"]
    #[inline(always)]
    pub fn ch1_loc0(&self) -> CH1_LOC0_R {
        CH1_LOC0_R::new((self.bits >> 6) & 0x03ff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LLP0")
            .field("ch1_lms", &format_args!("{}", self.ch1_lms().bit()))
            .field("ch1_loc0", &format_args!("{}", self.ch1_loc0().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<LLP0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ch1_lms(&mut self) -> CH1_LMS_W<LLP0_SPEC> {
        CH1_LMS_W::new(self, 0)
    }
    #[doc = "Bits 6:31 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ch1_loc0(&mut self) -> CH1_LOC0_W<LLP0_SPEC> {
        CH1_LOC0_W::new(self, 6)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`llp0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`llp0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LLP0_SPEC;
impl crate::RegisterSpec for LLP0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`llp0::R`](R) reader structure"]
impl crate::Readable for LLP0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`llp0::W`](W) writer structure"]
impl crate::Writable for LLP0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LLP0 to value 0"]
impl crate::Resettable for LLP0_SPEC {
    const RESET_VALUE: u32 = 0;
}
