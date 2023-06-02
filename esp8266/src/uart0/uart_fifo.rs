#[doc = "Register `UART_FIFO` reader"]
pub struct R(crate::R<UART_FIFO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UART_FIFO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UART_FIFO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UART_FIFO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UART_FIFO` writer"]
pub struct W(crate::W<UART_FIFO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UART_FIFO_SPEC>;
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
impl From<crate::W<UART_FIFO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UART_FIFO_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `rxfifo_rd_byte` reader - R/W share the same address"]
pub type RXFIFO_RD_BYTE_R = crate::FieldReader;
#[doc = "Field `rxfifo_write_byte` reader - R/W share the same address"]
pub type RXFIFO_WRITE_BYTE_R = crate::FieldReader;
#[doc = "Field `rxfifo_write_byte` writer - R/W share the same address"]
pub type RXFIFO_WRITE_BYTE_W<'a, const O: u8> = crate::FieldWriter<'a, UART_FIFO_SPEC, 8, O>;
impl R {
    #[doc = "Bits 0:7 - R/W share the same address"]
    #[inline(always)]
    pub fn rxfifo_rd_byte(&self) -> RXFIFO_RD_BYTE_R {
        RXFIFO_RD_BYTE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - R/W share the same address"]
    #[inline(always)]
    pub fn rxfifo_write_byte(&self) -> RXFIFO_WRITE_BYTE_R {
        RXFIFO_WRITE_BYTE_R::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UART_FIFO")
            .field(
                "rxfifo_rd_byte",
                &format_args!("{}", self.rxfifo_rd_byte().bits()),
            )
            .field(
                "rxfifo_write_byte",
                &format_args!("{}", self.rxfifo_write_byte().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<UART_FIFO_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7 - R/W share the same address"]
    #[inline(always)]
    #[must_use]
    pub fn rxfifo_write_byte(&mut self) -> RXFIFO_WRITE_BYTE_W<0> {
        RXFIFO_WRITE_BYTE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART FIFO,length 128\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_fifo](index.html) module"]
pub struct UART_FIFO_SPEC;
impl crate::RegisterSpec for UART_FIFO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uart_fifo::R](R) reader structure"]
impl crate::Readable for UART_FIFO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uart_fifo::W](W) writer structure"]
impl crate::Writable for UART_FIFO_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UART_FIFO to value 0"]
impl crate::Resettable for UART_FIFO_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
