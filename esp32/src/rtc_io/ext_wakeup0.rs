#[doc = "Register `EXT_WAKEUP0` reader"]
pub type R = crate::R<EXT_WAKEUP0_SPEC>;
#[doc = "Register `EXT_WAKEUP0` writer"]
pub type W = crate::W<EXT_WAKEUP0_SPEC>;
#[doc = "Field `SEL` reader - select the wakeup source Ó0Ó select GPIO0 Ó1Ó select GPIO2 ...Ò17Ó select GPIO17"]
pub type SEL_R = crate::FieldReader;
#[doc = "Field `SEL` writer - select the wakeup source Ó0Ó select GPIO0 Ó1Ó select GPIO2 ...Ò17Ó select GPIO17"]
pub type SEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
impl R {
    #[doc = "Bits 27:31 - select the wakeup source Ó0Ó select GPIO0 Ó1Ó select GPIO2 ...Ò17Ó select GPIO17"]
    #[inline(always)]
    pub fn sel(&self) -> SEL_R {
        SEL_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EXT_WAKEUP0")
            .field("sel", &format_args!("{}", self.sel().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<EXT_WAKEUP0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 27:31 - select the wakeup source Ó0Ó select GPIO0 Ó1Ó select GPIO2 ...Ò17Ó select GPIO17"]
    #[inline(always)]
    #[must_use]
    pub fn sel(&mut self) -> SEL_W<EXT_WAKEUP0_SPEC, 27> {
        SEL_W::new(self)
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
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ext_wakeup0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ext_wakeup0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EXT_WAKEUP0_SPEC;
impl crate::RegisterSpec for EXT_WAKEUP0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ext_wakeup0::R`](R) reader structure"]
impl crate::Readable for EXT_WAKEUP0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ext_wakeup0::W`](W) writer structure"]
impl crate::Writable for EXT_WAKEUP0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EXT_WAKEUP0 to value 0"]
impl crate::Resettable for EXT_WAKEUP0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
