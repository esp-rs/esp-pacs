#[doc = "Register `TX_CRC` reader"]
pub struct R(crate::R<TX_CRC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TX_CRC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TX_CRC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TX_CRC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TX_CRC` writer"]
pub struct W(crate::W<TX_CRC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TX_CRC_SPEC>;
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
impl From<crate::W<TX_CRC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TX_CRC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA` reader - For SPI1 the value of crc32 for 256 bits data."]
pub type DATA_R = crate::FieldReader<u32>;
#[doc = "Field `DATA` writer - For SPI1 the value of crc32 for 256 bits data."]
pub type DATA_W<'a, const O: u8> = crate::FieldWriter<'a, TX_CRC_SPEC, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - For SPI1 the value of crc32 for 256 bits data."]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TX_CRC")
            .field("data", &format_args!("{}", self.data().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TX_CRC_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - For SPI1 the value of crc32 for 256 bits data."]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DATA_W<0> {
        DATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx_crc](index.html) module"]
pub struct TX_CRC_SPEC;
impl crate::RegisterSpec for TX_CRC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tx_crc::R](R) reader structure"]
impl crate::Readable for TX_CRC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tx_crc::W](W) writer structure"]
impl crate::Writable for TX_CRC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TX_CRC to value 0"]
impl crate::Resettable for TX_CRC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
