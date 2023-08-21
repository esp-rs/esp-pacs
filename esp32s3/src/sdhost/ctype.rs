#[doc = "Register `CTYPE` reader"]
pub type R = crate::R<CTYPE_SPEC>;
#[doc = "Register `CTYPE` writer"]
pub type W = crate::W<CTYPE_SPEC>;
#[doc = "Field `CARD_WIDTH4` reader - One bit per card indicates if card is 1-bit or 4-bit mode. 0: 1-bit mode; 1: 4-bit mode. Bit\\[1:0\\] correspond to card\\[1:0\\] respectively."]
pub type CARD_WIDTH4_R = crate::FieldReader;
#[doc = "Field `CARD_WIDTH4` writer - One bit per card indicates if card is 1-bit or 4-bit mode. 0: 1-bit mode; 1: 4-bit mode. Bit\\[1:0\\] correspond to card\\[1:0\\] respectively."]
pub type CARD_WIDTH4_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `CARD_WIDTH8` reader - One bit per card indicates if card is in 8-bit mode. 0: Non 8-bit mode; 1: 8-bit mode. Bit\\[17:16\\] correspond to card\\[1:0\\] respectively."]
pub type CARD_WIDTH8_R = crate::FieldReader;
#[doc = "Field `CARD_WIDTH8` writer - One bit per card indicates if card is in 8-bit mode. 0: Non 8-bit mode; 1: 8-bit mode. Bit\\[17:16\\] correspond to card\\[1:0\\] respectively."]
pub type CARD_WIDTH8_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
impl R {
    #[doc = "Bits 0:1 - One bit per card indicates if card is 1-bit or 4-bit mode. 0: 1-bit mode; 1: 4-bit mode. Bit\\[1:0\\] correspond to card\\[1:0\\] respectively."]
    #[inline(always)]
    pub fn card_width4(&self) -> CARD_WIDTH4_R {
        CARD_WIDTH4_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 16:17 - One bit per card indicates if card is in 8-bit mode. 0: Non 8-bit mode; 1: 8-bit mode. Bit\\[17:16\\] correspond to card\\[1:0\\] respectively."]
    #[inline(always)]
    pub fn card_width8(&self) -> CARD_WIDTH8_R {
        CARD_WIDTH8_R::new(((self.bits >> 16) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTYPE")
            .field(
                "card_width4",
                &format_args!("{}", self.card_width4().bits()),
            )
            .field(
                "card_width8",
                &format_args!("{}", self.card_width8().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CTYPE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:1 - One bit per card indicates if card is 1-bit or 4-bit mode. 0: 1-bit mode; 1: 4-bit mode. Bit\\[1:0\\] correspond to card\\[1:0\\] respectively."]
    #[inline(always)]
    #[must_use]
    pub fn card_width4(&mut self) -> CARD_WIDTH4_W<CTYPE_SPEC, 0> {
        CARD_WIDTH4_W::new(self)
    }
    #[doc = "Bits 16:17 - One bit per card indicates if card is in 8-bit mode. 0: Non 8-bit mode; 1: 8-bit mode. Bit\\[17:16\\] correspond to card\\[1:0\\] respectively."]
    #[inline(always)]
    #[must_use]
    pub fn card_width8(&mut self) -> CARD_WIDTH8_W<CTYPE_SPEC, 16> {
        CARD_WIDTH8_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Card bus width configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctype::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctype::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTYPE_SPEC;
impl crate::RegisterSpec for CTYPE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctype::R`](R) reader structure"]
impl crate::Readable for CTYPE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctype::W`](W) writer structure"]
impl crate::Writable for CTYPE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTYPE to value 0"]
impl crate::Resettable for CTYPE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
