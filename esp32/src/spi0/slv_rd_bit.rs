#[doc = "Register `SLV_RD_BIT` reader"]
pub struct R(crate::R<SLV_RD_BIT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SLV_RD_BIT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SLV_RD_BIT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SLV_RD_BIT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SLV_RD_BIT` writer"]
pub struct W(crate::W<SLV_RD_BIT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SLV_RD_BIT_SPEC>;
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
impl From<crate::W<SLV_RD_BIT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SLV_RD_BIT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLV_RDATA_BIT` reader - In the slave mode it is the bit length of read data. The value is the length - 1."]
pub type SLV_RDATA_BIT_R = crate::FieldReader<u32>;
#[doc = "Field `SLV_RDATA_BIT` writer - In the slave mode it is the bit length of read data. The value is the length - 1."]
pub type SLV_RDATA_BIT_W<'a, const O: u8> = crate::FieldWriter<'a, SLV_RD_BIT_SPEC, 24, O, u32>;
impl R {
    #[doc = "Bits 0:23 - In the slave mode it is the bit length of read data. The value is the length - 1."]
    #[inline(always)]
    pub fn slv_rdata_bit(&self) -> SLV_RDATA_BIT_R {
        SLV_RDATA_BIT_R::new(self.bits & 0x00ff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SLV_RD_BIT")
            .field(
                "slv_rdata_bit",
                &format_args!("{}", self.slv_rdata_bit().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SLV_RD_BIT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:23 - In the slave mode it is the bit length of read data. The value is the length - 1."]
    #[inline(always)]
    #[must_use]
    pub fn slv_rdata_bit(&mut self) -> SLV_RDATA_BIT_W<0> {
        SLV_RDATA_BIT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [slv_rd_bit](index.html) module"]
pub struct SLV_RD_BIT_SPEC;
impl crate::RegisterSpec for SLV_RD_BIT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [slv_rd_bit::R](R) reader structure"]
impl crate::Readable for SLV_RD_BIT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [slv_rd_bit::W](W) writer structure"]
impl crate::Writable for SLV_RD_BIT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SLV_RD_BIT to value 0"]
impl crate::Resettable for SLV_RD_BIT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
