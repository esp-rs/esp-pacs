#[doc = "Register `MISO_DLEN` reader"]
pub struct R(crate::R<MISO_DLEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MISO_DLEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MISO_DLEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MISO_DLEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MISO_DLEN` writer"]
pub struct W(crate::W<MISO_DLEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MISO_DLEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<MISO_DLEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MISO_DLEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USR_MISO_DBITLEN` reader - The length in bits of read-data. The register value shall be (bit_num-1). Can be configured in CONF state."]
pub type USR_MISO_DBITLEN_R = crate::FieldReader<u32>;
#[doc = "Field `USR_MISO_DBITLEN` writer - The length in bits of read-data. The register value shall be (bit_num-1). Can be configured in CONF state."]
pub type USR_MISO_DBITLEN_W<'a, const O: u8> = crate::FieldWriter<'a, MISO_DLEN_SPEC, 23, O, u32>;
impl R {
    #[doc = "Bits 0:22 - The length in bits of read-data. The register value shall be (bit_num-1). Can be configured in CONF state."]
    #[inline(always)]
    pub fn usr_miso_dbitlen(&self) -> USR_MISO_DBITLEN_R {
        USR_MISO_DBITLEN_R::new(self.bits & 0x007f_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MISO_DLEN")
            .field(
                "usr_miso_dbitlen",
                &format_args!("{}", self.usr_miso_dbitlen().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<MISO_DLEN_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:22 - The length in bits of read-data. The register value shall be (bit_num-1). Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn usr_miso_dbitlen(&mut self) -> USR_MISO_DBITLEN_W<0> {
        USR_MISO_DBITLEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MISO length\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [miso_dlen](index.html) module"]
pub struct MISO_DLEN_SPEC;
impl crate::RegisterSpec for MISO_DLEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [miso_dlen::R](R) reader structure"]
impl crate::Readable for MISO_DLEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [miso_dlen::W](W) writer structure"]
impl crate::Writable for MISO_DLEN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MISO_DLEN to value 0"]
impl crate::Resettable for MISO_DLEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
