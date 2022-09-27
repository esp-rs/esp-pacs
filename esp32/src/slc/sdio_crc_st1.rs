#[doc = "Register `SDIO_CRC_ST1` reader"]
pub struct R(crate::R<SDIO_CRC_ST1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SDIO_CRC_ST1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SDIO_CRC_ST1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SDIO_CRC_ST1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SDIO_CRC_ST1` writer"]
pub struct W(crate::W<SDIO_CRC_ST1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SDIO_CRC_ST1_SPEC>;
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
impl From<crate::W<SDIO_CRC_ST1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SDIO_CRC_ST1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMD_CRC_ERR_CNT` reader - "]
pub type CMD_CRC_ERR_CNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ERR_CNT_CLR` reader - "]
pub type ERR_CNT_CLR_R = crate::BitReader<bool>;
#[doc = "Field `ERR_CNT_CLR` writer - "]
pub type ERR_CNT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SDIO_CRC_ST1_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn cmd_crc_err_cnt(&self) -> CMD_CRC_ERR_CNT_R {
        CMD_CRC_ERR_CNT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn err_cnt_clr(&self) -> ERR_CNT_CLR_R {
        ERR_CNT_CLR_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn err_cnt_clr(&mut self) -> ERR_CNT_CLR_W<31> {
        ERR_CNT_CLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdio_crc_st1](index.html) module"]
pub struct SDIO_CRC_ST1_SPEC;
impl crate::RegisterSpec for SDIO_CRC_ST1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sdio_crc_st1::R](R) reader structure"]
impl crate::Readable for SDIO_CRC_ST1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sdio_crc_st1::W](W) writer structure"]
impl crate::Writable for SDIO_CRC_ST1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SDIO_CRC_ST1 to value 0"]
impl crate::Resettable for SDIO_CRC_ST1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
