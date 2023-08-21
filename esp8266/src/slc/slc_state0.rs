#[doc = "Register `SLC_STATE0` reader"]
pub type R = crate::R<SLC_STATE0_SPEC>;
#[doc = "Register `SLC_STATE0` writer"]
pub type W = crate::W<SLC_STATE0_SPEC>;
#[doc = "Field `Register` reader - "]
pub type REGISTER_R = crate::FieldReader<u32>;
#[doc = "Field `Register` writer - "]
pub type REGISTER_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn register(&self) -> REGISTER_R {
        REGISTER_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SLC_STATE0")
            .field("register", &format_args!("{}", self.register().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SLC_STATE0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn register(&mut self) -> REGISTER_W<SLC_STATE0_SPEC, 0> {
        REGISTER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "SLC_STATE0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slc_state0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slc_state0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SLC_STATE0_SPEC;
impl crate::RegisterSpec for SLC_STATE0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`slc_state0::R`](R) reader structure"]
impl crate::Readable for SLC_STATE0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`slc_state0::W`](W) writer structure"]
impl crate::Writable for SLC_STATE0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SLC_STATE0 to value 0"]
impl crate::Resettable for SLC_STATE0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
