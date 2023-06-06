#[doc = "Register `SDIO_SLAVE_ECO_LOW` reader"]
pub struct R(crate::R<SDIO_SLAVE_ECO_LOW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SDIO_SLAVE_ECO_LOW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SDIO_SLAVE_ECO_LOW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SDIO_SLAVE_ECO_LOW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SDIO_SLAVE_ECO_LOW` writer"]
pub struct W(crate::W<SDIO_SLAVE_ECO_LOW_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SDIO_SLAVE_ECO_LOW_SPEC>;
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
impl From<crate::W<SDIO_SLAVE_ECO_LOW_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SDIO_SLAVE_ECO_LOW_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RDN_ECO_LOW` reader - redundant registers for sdio_slave"]
pub type RDN_ECO_LOW_R = crate::FieldReader<u32>;
#[doc = "Field `RDN_ECO_LOW` writer - redundant registers for sdio_slave"]
pub type RDN_ECO_LOW_W<'a, const O: u8> =
    crate::FieldWriter<'a, SDIO_SLAVE_ECO_LOW_SPEC, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - redundant registers for sdio_slave"]
    #[inline(always)]
    pub fn rdn_eco_low(&self) -> RDN_ECO_LOW_R {
        RDN_ECO_LOW_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SDIO_SLAVE_ECO_LOW")
            .field(
                "rdn_eco_low",
                &format_args!("{}", self.rdn_eco_low().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SDIO_SLAVE_ECO_LOW_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - redundant registers for sdio_slave"]
    #[inline(always)]
    #[must_use]
    pub fn rdn_eco_low(&mut self) -> RDN_ECO_LOW_W<0> {
        RDN_ECO_LOW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "sdio_slave redundant control registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdio_slave_eco_low](index.html) module"]
pub struct SDIO_SLAVE_ECO_LOW_SPEC;
impl crate::RegisterSpec for SDIO_SLAVE_ECO_LOW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sdio_slave_eco_low::R](R) reader structure"]
impl crate::Readable for SDIO_SLAVE_ECO_LOW_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sdio_slave_eco_low::W](W) writer structure"]
impl crate::Writable for SDIO_SLAVE_ECO_LOW_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SDIO_SLAVE_ECO_LOW to value 0"]
impl crate::Resettable for SDIO_SLAVE_ECO_LOW_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
