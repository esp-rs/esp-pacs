#[doc = "Register `SDIO_SLAVE_ECO_HIGH` reader"]
pub struct R(crate::R<SDIO_SLAVE_ECO_HIGH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SDIO_SLAVE_ECO_HIGH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SDIO_SLAVE_ECO_HIGH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SDIO_SLAVE_ECO_HIGH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SDIO_SLAVE_ECO_HIGH` writer"]
pub struct W(crate::W<SDIO_SLAVE_ECO_HIGH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SDIO_SLAVE_ECO_HIGH_SPEC>;
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
impl From<crate::W<SDIO_SLAVE_ECO_HIGH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SDIO_SLAVE_ECO_HIGH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RDN_ECO_HIGH` reader - redundant registers for sdio_slave"]
pub type RDN_ECO_HIGH_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RDN_ECO_HIGH` writer - redundant registers for sdio_slave"]
pub type RDN_ECO_HIGH_W<'a, const O: u8> =
    crate::FieldWriter<'a, SDIO_SLAVE_ECO_HIGH_SPEC, 32, O, u32, u32>;
impl R {
    #[doc = "Bits 0:31 - redundant registers for sdio_slave"]
    #[inline(always)]
    pub fn rdn_eco_high(&self) -> RDN_ECO_HIGH_R {
        RDN_ECO_HIGH_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SDIO_SLAVE_ECO_HIGH")
            .field(
                "rdn_eco_high",
                &format_args!("{}", self.rdn_eco_high().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SDIO_SLAVE_ECO_HIGH_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - redundant registers for sdio_slave"]
    #[inline(always)]
    #[must_use]
    pub fn rdn_eco_high(&mut self) -> RDN_ECO_HIGH_W<0> {
        RDN_ECO_HIGH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "sdio_slave redundant control registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdio_slave_eco_high](index.html) module"]
pub struct SDIO_SLAVE_ECO_HIGH_SPEC;
impl crate::RegisterSpec for SDIO_SLAVE_ECO_HIGH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sdio_slave_eco_high::R](R) reader structure"]
impl crate::Readable for SDIO_SLAVE_ECO_HIGH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sdio_slave_eco_high::W](W) writer structure"]
impl crate::Writable for SDIO_SLAVE_ECO_HIGH_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SDIO_SLAVE_ECO_HIGH to value 0xffff_ffff"]
impl crate::Resettable for SDIO_SLAVE_ECO_HIGH_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
