#[doc = "Register `SLC_INTVEC_TOHOST` reader"]
pub type R = crate::R<SLC_INTVEC_TOHOST_SPEC>;
#[doc = "Register `SLC_INTVEC_TOHOST` writer"]
pub type W = crate::W<SLC_INTVEC_TOHOST_SPEC>;
#[doc = "Field `SLC_TOHOST_INTVEC` reader - "]
pub type SLC_TOHOST_INTVEC_R = crate::FieldReader;
#[doc = "Field `SLC_TOHOST_INTVEC` writer - "]
pub type SLC_TOHOST_INTVEC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn slc_tohost_intvec(&self) -> SLC_TOHOST_INTVEC_R {
        SLC_TOHOST_INTVEC_R::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SLC_INTVEC_TOHOST")
            .field(
                "slc_tohost_intvec",
                &format_args!("{}", self.slc_tohost_intvec().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SLC_INTVEC_TOHOST_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn slc_tohost_intvec(&mut self) -> SLC_TOHOST_INTVEC_W<SLC_INTVEC_TOHOST_SPEC, 0> {
        SLC_TOHOST_INTVEC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "SLC_INTVEC_TOHOST\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slc_intvec_tohost::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slc_intvec_tohost::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SLC_INTVEC_TOHOST_SPEC;
impl crate::RegisterSpec for SLC_INTVEC_TOHOST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`slc_intvec_tohost::R`](R) reader structure"]
impl crate::Readable for SLC_INTVEC_TOHOST_SPEC {}
#[doc = "`write(|w| ..)` method takes [`slc_intvec_tohost::W`](W) writer structure"]
impl crate::Writable for SLC_INTVEC_TOHOST_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SLC_INTVEC_TOHOST to value 0"]
impl crate::Resettable for SLC_INTVEC_TOHOST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
