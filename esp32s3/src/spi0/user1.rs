#[doc = "Register `USER1` reader"]
pub struct R(crate::R<USER1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USER1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USER1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USER1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USER1` writer"]
pub struct W(crate::W<USER1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USER1_SPEC>;
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
impl From<crate::W<USER1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USER1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USR_DUMMY_CYCLELEN` reader - The SPI_CLK cycle length minus 1 of DUMMY phase."]
pub type USR_DUMMY_CYCLELEN_R = crate::FieldReader;
#[doc = "Field `USR_DUMMY_CYCLELEN` writer - The SPI_CLK cycle length minus 1 of DUMMY phase."]
pub type USR_DUMMY_CYCLELEN_W<'a, const O: u8> = crate::FieldWriter<'a, USER1_SPEC, 6, O>;
#[doc = "Field `USR_ADDR_BITLEN` reader - The length in bits of ADDR phase. The register value shall be (bit_num-1)."]
pub type USR_ADDR_BITLEN_R = crate::FieldReader;
#[doc = "Field `USR_ADDR_BITLEN` writer - The length in bits of ADDR phase. The register value shall be (bit_num-1)."]
pub type USR_ADDR_BITLEN_W<'a, const O: u8> = crate::FieldWriter<'a, USER1_SPEC, 6, O>;
impl R {
    #[doc = "Bits 0:5 - The SPI_CLK cycle length minus 1 of DUMMY phase."]
    #[inline(always)]
    pub fn usr_dummy_cyclelen(&self) -> USR_DUMMY_CYCLELEN_R {
        USR_DUMMY_CYCLELEN_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 26:31 - The length in bits of ADDR phase. The register value shall be (bit_num-1)."]
    #[inline(always)]
    pub fn usr_addr_bitlen(&self) -> USR_ADDR_BITLEN_R {
        USR_ADDR_BITLEN_R::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USER1")
            .field(
                "usr_dummy_cyclelen",
                &format_args!("{}", self.usr_dummy_cyclelen().bits()),
            )
            .field(
                "usr_addr_bitlen",
                &format_args!("{}", self.usr_addr_bitlen().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<USER1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:5 - The SPI_CLK cycle length minus 1 of DUMMY phase."]
    #[inline(always)]
    #[must_use]
    pub fn usr_dummy_cyclelen(&mut self) -> USR_DUMMY_CYCLELEN_W<0> {
        USR_DUMMY_CYCLELEN_W::new(self)
    }
    #[doc = "Bits 26:31 - The length in bits of ADDR phase. The register value shall be (bit_num-1)."]
    #[inline(always)]
    #[must_use]
    pub fn usr_addr_bitlen(&mut self) -> USR_ADDR_BITLEN_W<26> {
        USR_ADDR_BITLEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI0 user1 register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [user1](index.html) module"]
pub struct USER1_SPEC;
impl crate::RegisterSpec for USER1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [user1::R](R) reader structure"]
impl crate::Readable for USER1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [user1::W](W) writer structure"]
impl crate::Writable for USER1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USER1 to value 0x5c00_0007"]
impl crate::Resettable for USER1_SPEC {
    const RESET_VALUE: Self::Ux = 0x5c00_0007;
}
