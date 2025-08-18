#[doc = "Register `CLKDIV_SYNC` reader"]
pub type R = crate::R<CLKDIV_SYNC_SPEC>;
#[doc = "Register `CLKDIV_SYNC` writer"]
pub type W = crate::W<CLKDIV_SYNC_SPEC>;
#[doc = "Field `CLKDIV` reader - Configures the integral part of the divisor for baud rate generation."]
pub type CLKDIV_R = crate::FieldReader<u16>;
#[doc = "Field `CLKDIV` writer - Configures the integral part of the divisor for baud rate generation."]
pub type CLKDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `CLKDIV_FRAG` reader - Configures the fractional part of the divisor for baud rate generation."]
pub type CLKDIV_FRAG_R = crate::FieldReader;
#[doc = "Field `CLKDIV_FRAG` writer - Configures the fractional part of the divisor for baud rate generation."]
pub type CLKDIV_FRAG_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:11 - Configures the integral part of the divisor for baud rate generation."]
    #[inline(always)]
    pub fn clkdiv(&self) -> CLKDIV_R {
        CLKDIV_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 20:23 - Configures the fractional part of the divisor for baud rate generation."]
    #[inline(always)]
    pub fn clkdiv_frag(&self) -> CLKDIV_FRAG_R {
        CLKDIV_FRAG_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLKDIV_SYNC")
            .field("clkdiv", &self.clkdiv())
            .field("clkdiv_frag", &self.clkdiv_frag())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:11 - Configures the integral part of the divisor for baud rate generation."]
    #[inline(always)]
    pub fn clkdiv(&mut self) -> CLKDIV_W<'_, CLKDIV_SYNC_SPEC> {
        CLKDIV_W::new(self, 0)
    }
    #[doc = "Bits 20:23 - Configures the fractional part of the divisor for baud rate generation."]
    #[inline(always)]
    pub fn clkdiv_frag(&mut self) -> CLKDIV_FRAG_W<'_, CLKDIV_SYNC_SPEC> {
        CLKDIV_FRAG_W::new(self, 20)
    }
}
#[doc = "Clock divider configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`clkdiv_sync::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkdiv_sync::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLKDIV_SYNC_SPEC;
impl crate::RegisterSpec for CLKDIV_SYNC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clkdiv_sync::R`](R) reader structure"]
impl crate::Readable for CLKDIV_SYNC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clkdiv_sync::W`](W) writer structure"]
impl crate::Writable for CLKDIV_SYNC_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CLKDIV_SYNC to value 0x02b6"]
impl crate::Resettable for CLKDIV_SYNC_SPEC {
    const RESET_VALUE: u32 = 0x02b6;
}
