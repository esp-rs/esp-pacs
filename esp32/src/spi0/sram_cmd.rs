#[doc = "Register `SRAM_CMD` reader"]
pub struct R(crate::R<SRAM_CMD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SRAM_CMD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SRAM_CMD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SRAM_CMD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SRAM_CMD` writer"]
pub struct W(crate::W<SRAM_CMD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SRAM_CMD_SPEC>;
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
impl From<crate::W<SRAM_CMD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SRAM_CMD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SRAM_DIO` reader - For SPI0 SRAM DIO mode enable . SRAM DIO enable command will be send when the bit is set. The bit will be cleared once the operation done."]
pub type SRAM_DIO_R = crate::BitReader;
#[doc = "Field `SRAM_DIO` writer - For SPI0 SRAM DIO mode enable . SRAM DIO enable command will be send when the bit is set. The bit will be cleared once the operation done."]
pub type SRAM_DIO_W<'a, const O: u8> = crate::BitWriter<'a, SRAM_CMD_SPEC, O>;
#[doc = "Field `SRAM_QIO` reader - For SPI0 SRAM QIO mode enable . SRAM QIO enable command will be send when the bit is set. The bit will be cleared once the operation done."]
pub type SRAM_QIO_R = crate::BitReader;
#[doc = "Field `SRAM_QIO` writer - For SPI0 SRAM QIO mode enable . SRAM QIO enable command will be send when the bit is set. The bit will be cleared once the operation done."]
pub type SRAM_QIO_W<'a, const O: u8> = crate::BitWriter<'a, SRAM_CMD_SPEC, O>;
#[doc = "Field `SRAM_RSTIO` reader - For SPI0 SRAM IO mode reset enable. SRAM IO mode reset operation will be triggered when the bit is set. The bit will be cleared once the operation done"]
pub type SRAM_RSTIO_R = crate::BitReader;
#[doc = "Field `SRAM_RSTIO` writer - For SPI0 SRAM IO mode reset enable. SRAM IO mode reset operation will be triggered when the bit is set. The bit will be cleared once the operation done"]
pub type SRAM_RSTIO_W<'a, const O: u8> = crate::BitWriter<'a, SRAM_CMD_SPEC, O>;
impl R {
    #[doc = "Bit 0 - For SPI0 SRAM DIO mode enable . SRAM DIO enable command will be send when the bit is set. The bit will be cleared once the operation done."]
    #[inline(always)]
    pub fn sram_dio(&self) -> SRAM_DIO_R {
        SRAM_DIO_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - For SPI0 SRAM QIO mode enable . SRAM QIO enable command will be send when the bit is set. The bit will be cleared once the operation done."]
    #[inline(always)]
    pub fn sram_qio(&self) -> SRAM_QIO_R {
        SRAM_QIO_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - For SPI0 SRAM IO mode reset enable. SRAM IO mode reset operation will be triggered when the bit is set. The bit will be cleared once the operation done"]
    #[inline(always)]
    pub fn sram_rstio(&self) -> SRAM_RSTIO_R {
        SRAM_RSTIO_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SRAM_CMD")
            .field("sram_dio", &format_args!("{}", self.sram_dio().bit()))
            .field("sram_qio", &format_args!("{}", self.sram_qio().bit()))
            .field("sram_rstio", &format_args!("{}", self.sram_rstio().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SRAM_CMD_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - For SPI0 SRAM DIO mode enable . SRAM DIO enable command will be send when the bit is set. The bit will be cleared once the operation done."]
    #[inline(always)]
    #[must_use]
    pub fn sram_dio(&mut self) -> SRAM_DIO_W<0> {
        SRAM_DIO_W::new(self)
    }
    #[doc = "Bit 1 - For SPI0 SRAM QIO mode enable . SRAM QIO enable command will be send when the bit is set. The bit will be cleared once the operation done."]
    #[inline(always)]
    #[must_use]
    pub fn sram_qio(&mut self) -> SRAM_QIO_W<1> {
        SRAM_QIO_W::new(self)
    }
    #[doc = "Bit 4 - For SPI0 SRAM IO mode reset enable. SRAM IO mode reset operation will be triggered when the bit is set. The bit will be cleared once the operation done"]
    #[inline(always)]
    #[must_use]
    pub fn sram_rstio(&mut self) -> SRAM_RSTIO_W<4> {
        SRAM_RSTIO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sram_cmd](index.html) module"]
pub struct SRAM_CMD_SPEC;
impl crate::RegisterSpec for SRAM_CMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sram_cmd::R](R) reader structure"]
impl crate::Readable for SRAM_CMD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sram_cmd::W](W) writer structure"]
impl crate::Writable for SRAM_CMD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SRAM_CMD to value 0"]
impl crate::Resettable for SRAM_CMD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
