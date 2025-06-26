#[doc = "Register `CLKDIV` reader"]
pub type R = crate::R<CLKDIV_SPEC>;
#[doc = "Register `CLKDIV` writer"]
pub type W = crate::W<CLKDIV_SPEC>;
#[doc = "Field `CLKDIV` reader - The integral part of the frequency divisor."]
pub type CLKDIV_R = crate::FieldReader<u32>;
#[doc = "Field `CLKDIV` writer - The integral part of the frequency divisor."]
pub type CLKDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
#[doc = "Field `FRAG` reader - The fractional part of the frequency divisor."]
pub type FRAG_R = crate::FieldReader;
#[doc = "Field `FRAG` writer - The fractional part of the frequency divisor."]
pub type FRAG_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:19 - The integral part of the frequency divisor."]
    #[inline(always)]
    pub fn clkdiv(&self) -> CLKDIV_R {
        CLKDIV_R::new(self.bits & 0x000f_ffff)
    }
    #[doc = "Bits 20:23 - The fractional part of the frequency divisor."]
    #[inline(always)]
    pub fn frag(&self) -> FRAG_R {
        FRAG_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLKDIV")
            .field("clkdiv", &self.clkdiv())
            .field("frag", &self.frag())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:19 - The integral part of the frequency divisor."]
    #[inline(always)]
    pub fn clkdiv(&mut self) -> CLKDIV_W<CLKDIV_SPEC> {
        CLKDIV_W::new(self, 0)
    }
    #[doc = "Bits 20:23 - The fractional part of the frequency divisor."]
    #[inline(always)]
    pub fn frag(&mut self) -> FRAG_W<CLKDIV_SPEC> {
        FRAG_W::new(self, 20)
    }
}
#[doc = "Clock divider configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`clkdiv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkdiv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLKDIV_SPEC;
impl crate::RegisterSpec for CLKDIV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clkdiv::R`](R) reader structure"]
impl crate::Readable for CLKDIV_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clkdiv::W`](W) writer structure"]
impl crate::Writable for CLKDIV_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CLKDIV to value 0x02b6"]
impl crate::Resettable for CLKDIV_SPEC {
    const RESET_VALUE: u32 = 0x02b6;
}
