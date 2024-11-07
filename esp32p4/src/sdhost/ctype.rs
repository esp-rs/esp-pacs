#[doc = "Register `CTYPE` reader"]
pub type R = crate::R<CTYPE_SPEC>;
#[doc = "Register `CTYPE` writer"]
pub type W = crate::W<CTYPE_SPEC>;
#[doc = "Field `CARD_WIDTH4` reader - One bit per card indicates if card is 1-bit or 4-bit mode. 0: 1-bit mode; 1: 4-bit mode. Bit\\[1:0\\] correspond to card\\[1:0\\] respectively."]
pub type CARD_WIDTH4_R = crate::FieldReader;
#[doc = "Field `CARD_WIDTH4` writer - One bit per card indicates if card is 1-bit or 4-bit mode. 0: 1-bit mode; 1: 4-bit mode. Bit\\[1:0\\] correspond to card\\[1:0\\] respectively."]
pub type CARD_WIDTH4_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CARD_WIDTH8` reader - One bit per card indicates if card is in 8-bit mode. 0: Non 8-bit mode; 1: 8-bit mode. Bit\\[17:16\\] correspond to card\\[1:0\\] respectively."]
pub type CARD_WIDTH8_R = crate::FieldReader;
#[doc = "Field `CARD_WIDTH8` writer - One bit per card indicates if card is in 8-bit mode. 0: Non 8-bit mode; 1: 8-bit mode. Bit\\[17:16\\] correspond to card\\[1:0\\] respectively."]
pub type CARD_WIDTH8_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
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
            .field("card_width4", &self.card_width4())
            .field("card_width8", &self.card_width8())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - One bit per card indicates if card is 1-bit or 4-bit mode. 0: 1-bit mode; 1: 4-bit mode. Bit\\[1:0\\] correspond to card\\[1:0\\] respectively."]
    #[inline(always)]
    pub fn card_width4(&mut self) -> CARD_WIDTH4_W<CTYPE_SPEC> {
        CARD_WIDTH4_W::new(self, 0)
    }
    #[doc = "Bits 16:17 - One bit per card indicates if card is in 8-bit mode. 0: Non 8-bit mode; 1: 8-bit mode. Bit\\[17:16\\] correspond to card\\[1:0\\] respectively."]
    #[inline(always)]
    pub fn card_width8(&mut self) -> CARD_WIDTH8_W<CTYPE_SPEC> {
        CARD_WIDTH8_W::new(self, 16)
    }
}
#[doc = "Card bus width configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctype::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctype::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTYPE_SPEC;
impl crate::RegisterSpec for CTYPE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctype::R`](R) reader structure"]
impl crate::Readable for CTYPE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctype::W`](W) writer structure"]
impl crate::Writable for CTYPE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTYPE to value 0"]
impl crate::Resettable for CTYPE_SPEC {
    const RESET_VALUE: u32 = 0;
}
