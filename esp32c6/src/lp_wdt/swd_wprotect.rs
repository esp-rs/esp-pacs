#[doc = "Register `SWD_WPROTECT` reader"]
pub type R = crate::R<SWD_WPROTECT_SPEC>;
#[doc = "Register `SWD_WPROTECT` writer"]
pub type W = crate::W<SWD_WPROTECT_SPEC>;
#[doc = "Field `SWD_WKEY` reader - need_des"]
pub type SWD_WKEY_R = crate::FieldReader<u32>;
#[doc = "Field `SWD_WKEY` writer - need_des"]
pub type SWD_WKEY_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - need_des"]
    #[inline(always)]
    pub fn swd_wkey(&self) -> SWD_WKEY_R {
        SWD_WKEY_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SWD_WPROTECT")
            .field("swd_wkey", &format_args!("{}", self.swd_wkey().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SWD_WPROTECT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn swd_wkey(&mut self) -> SWD_WKEY_W<SWD_WPROTECT_SPEC, 0> {
        SWD_WKEY_W::new(self)
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
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swd_wprotect::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swd_wprotect::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SWD_WPROTECT_SPEC;
impl crate::RegisterSpec for SWD_WPROTECT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swd_wprotect::R`](R) reader structure"]
impl crate::Readable for SWD_WPROTECT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`swd_wprotect::W`](W) writer structure"]
impl crate::Writable for SWD_WPROTECT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SWD_WPROTECT to value 0"]
impl crate::Resettable for SWD_WPROTECT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
