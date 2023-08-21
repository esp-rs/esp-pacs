#[doc = "Register `UART_ID` reader"]
pub type R = crate::R<UART_ID_SPEC>;
#[doc = "Register `UART_ID` writer"]
pub type W = crate::W<UART_ID_SPEC>;
#[doc = "Field `uart_id` reader - "]
pub type UART_ID_R = crate::FieldReader<u32>;
#[doc = "Field `uart_id` writer - "]
pub type UART_ID_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn uart_id(&self) -> UART_ID_R {
        UART_ID_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UART_ID")
            .field("uart_id", &format_args!("{}", self.uart_id().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<UART_ID_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn uart_id(&mut self) -> UART_ID_W<UART_ID_SPEC, 0> {
        UART_ID_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "UART_ID\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart_id::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uart_id::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UART_ID_SPEC;
impl crate::RegisterSpec for UART_ID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uart_id::R`](R) reader structure"]
impl crate::Readable for UART_ID_SPEC {}
#[doc = "`write(|w| ..)` method takes [`uart_id::W`](W) writer structure"]
impl crate::Writable for UART_ID_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UART_ID to value 0"]
impl crate::Resettable for UART_ID_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
