#[doc = "Register `AE_WIN_RECIPROCAL` reader"]
pub type R = crate::R<AE_WIN_RECIPROCAL_SPEC>;
#[doc = "Register `AE_WIN_RECIPROCAL` writer"]
pub type W = crate::W<AE_WIN_RECIPROCAL_SPEC>;
#[doc = "Field `AE_SUBWIN_RECIP` reader - this field configures the reciprocal of each subwin_pixnum, 20bit fraction"]
pub type AE_SUBWIN_RECIP_R = crate::FieldReader<u32>;
#[doc = "Field `AE_SUBWIN_RECIP` writer - this field configures the reciprocal of each subwin_pixnum, 20bit fraction"]
pub type AE_SUBWIN_RECIP_W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:19 - this field configures the reciprocal of each subwin_pixnum, 20bit fraction"]
    #[inline(always)]
    pub fn ae_subwin_recip(&self) -> AE_SUBWIN_RECIP_R {
        AE_SUBWIN_RECIP_R::new(self.bits & 0x000f_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AE_WIN_RECIPROCAL")
            .field(
                "ae_subwin_recip",
                &format_args!("{}", self.ae_subwin_recip().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<AE_WIN_RECIPROCAL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:19 - this field configures the reciprocal of each subwin_pixnum, 20bit fraction"]
    #[inline(always)]
    #[must_use]
    pub fn ae_subwin_recip(&mut self) -> AE_SUBWIN_RECIP_W<AE_WIN_RECIPROCAL_SPEC> {
        AE_SUBWIN_RECIP_W::new(self, 0)
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
#[doc = "reciprocal of ae sub-window pixel number\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ae_win_reciprocal::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ae_win_reciprocal::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AE_WIN_RECIPROCAL_SPEC;
impl crate::RegisterSpec for AE_WIN_RECIPROCAL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ae_win_reciprocal::R`](R) reader structure"]
impl crate::Readable for AE_WIN_RECIPROCAL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ae_win_reciprocal::W`](W) writer structure"]
impl crate::Writable for AE_WIN_RECIPROCAL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AE_WIN_RECIPROCAL to value 0"]
impl crate::Resettable for AE_WIN_RECIPROCAL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
