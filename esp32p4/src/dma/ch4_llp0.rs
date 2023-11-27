#[doc = "Register `CH4_LLP0` reader"]
pub type R = crate::R<CH4_LLP0_SPEC>;
#[doc = "Register `CH4_LLP0` writer"]
pub type W = crate::W<CH4_LLP0_SPEC>;
#[doc = "Field `CH4_LMS` reader - NA"]
pub type CH4_LMS_R = crate::BitReader;
#[doc = "Field `CH4_LMS` writer - NA"]
pub type CH4_LMS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH4_LOC0` reader - NA"]
pub type CH4_LOC0_R = crate::FieldReader<u32>;
#[doc = "Field `CH4_LOC0` writer - NA"]
pub type CH4_LOC0_W<'a, REG> = crate::FieldWriter<'a, REG, 26, u32>;
impl R {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn ch4_lms(&self) -> CH4_LMS_R {
        CH4_LMS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 6:31 - NA"]
    #[inline(always)]
    pub fn ch4_loc0(&self) -> CH4_LOC0_R {
        CH4_LOC0_R::new((self.bits >> 6) & 0x03ff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CH4_LLP0")
            .field("ch4_lms", &format_args!("{}", self.ch4_lms().bit()))
            .field("ch4_loc0", &format_args!("{}", self.ch4_loc0().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CH4_LLP0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ch4_lms(&mut self) -> CH4_LMS_W<CH4_LLP0_SPEC> {
        CH4_LMS_W::new(self, 0)
    }
    #[doc = "Bits 6:31 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ch4_loc0(&mut self) -> CH4_LOC0_W<CH4_LLP0_SPEC> {
        CH4_LOC0_W::new(self, 6)
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
#[doc = "NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch4_llp0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch4_llp0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CH4_LLP0_SPEC;
impl crate::RegisterSpec for CH4_LLP0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch4_llp0::R`](R) reader structure"]
impl crate::Readable for CH4_LLP0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ch4_llp0::W`](W) writer structure"]
impl crate::Writable for CH4_LLP0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CH4_LLP0 to value 0"]
impl crate::Resettable for CH4_LLP0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
