#[doc = "Register `PLCP2%s` reader"]
pub type R = crate::R<PLCP2_SPEC>;
#[doc = "Register `PLCP2%s` writer"]
pub type W = crate::W<PLCP2_SPEC>;
#[doc = "Field `UNKNOWN` reader - meaning unknown, set to one for TX"]
pub type UNKNOWN_R = crate::BitReader;
#[doc = "Field `UNKNOWN` writer - meaning unknown, set to one for TX"]
pub type UNKNOWN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 5 - meaning unknown, set to one for TX"]
    #[inline(always)]
    pub fn unknown(&self) -> UNKNOWN_R {
        UNKNOWN_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PLCP2")
            .field("unknown", &self.unknown())
            .finish()
    }
}
impl W {
    #[doc = "Bit 5 - meaning unknown, set to one for TX"]
    #[inline(always)]
    pub fn unknown(&mut self) -> UNKNOWN_W<PLCP2_SPEC> {
        UNKNOWN_W::new(self, 5)
    }
}
#[doc = "PLCP2\n\nYou can [`read`](crate::Reg::read) this register and get [`plcp2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`plcp2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PLCP2_SPEC;
impl crate::RegisterSpec for PLCP2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`plcp2::R`](R) reader structure"]
impl crate::Readable for PLCP2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`plcp2::W`](W) writer structure"]
impl crate::Writable for PLCP2_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PLCP2%s to value 0"]
impl crate::Resettable for PLCP2_SPEC {
    const RESET_VALUE: u32 = 0;
}
