#[doc = "Register `XTL_EXT_CTR` reader"]
pub type R = crate::R<XTL_EXT_CTR_SPEC>;
#[doc = "Register `XTL_EXT_CTR` writer"]
pub type W = crate::W<XTL_EXT_CTR_SPEC>;
#[doc = "Field `SEL` reader - Select the external crystal power down enable source to get into sleep mode. 0: select GPIO0. 1: select GPIO1, etc. The input value on this pin XOR RTC_CNTL_EXT_XTL_CONF_REG\\[30\\] is the crystal power down enable signal."]
pub type SEL_R = crate::FieldReader;
#[doc = "Field `SEL` writer - Select the external crystal power down enable source to get into sleep mode. 0: select GPIO0. 1: select GPIO1, etc. The input value on this pin XOR RTC_CNTL_EXT_XTL_CONF_REG\\[30\\] is the crystal power down enable signal."]
pub type SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 27:31 - Select the external crystal power down enable source to get into sleep mode. 0: select GPIO0. 1: select GPIO1, etc. The input value on this pin XOR RTC_CNTL_EXT_XTL_CONF_REG\\[30\\] is the crystal power down enable signal."]
    #[inline(always)]
    pub fn sel(&self) -> SEL_R {
        SEL_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("XTL_EXT_CTR")
            .field("sel", &format_args!("{}", self.sel().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<XTL_EXT_CTR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 27:31 - Select the external crystal power down enable source to get into sleep mode. 0: select GPIO0. 1: select GPIO1, etc. The input value on this pin XOR RTC_CNTL_EXT_XTL_CONF_REG\\[30\\] is the crystal power down enable signal."]
    #[inline(always)]
    #[must_use]
    pub fn sel(&mut self) -> SEL_W<XTL_EXT_CTR_SPEC> {
        SEL_W::new(self, 27)
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
#[doc = "Crystal power down enable GPIO source\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`xtl_ext_ctr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`xtl_ext_ctr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct XTL_EXT_CTR_SPEC;
impl crate::RegisterSpec for XTL_EXT_CTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`xtl_ext_ctr::R`](R) reader structure"]
impl crate::Readable for XTL_EXT_CTR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`xtl_ext_ctr::W`](W) writer structure"]
impl crate::Writable for XTL_EXT_CTR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets XTL_EXT_CTR to value 0"]
impl crate::Resettable for XTL_EXT_CTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
