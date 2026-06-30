#[doc = "Register `MULT_FREE` reader"]
pub type R = crate::R<MULT_FREE_SPEC>;
#[doc = "Register `MULT_FREE` writer"]
pub type W = crate::W<MULT_FREE_SPEC>;
#[doc = "Field `MULT_FREE` reader - ECC mult free state register \\\\ 0: non-free \\\\ 1: free \\\\"]
pub type MULT_FREE_R = crate::BitReader;
#[doc = "Field `MULT_FREE` writer - ECC mult free state register \\\\ 0: non-free \\\\ 1: free \\\\"]
pub type MULT_FREE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - ECC mult free state register \\\\ 0: non-free \\\\ 1: free \\\\"]
    #[inline(always)]
    pub fn mult_free(&self) -> MULT_FREE_R {
        MULT_FREE_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MULT_FREE")
            .field("mult_free", &self.mult_free())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - ECC mult free state register \\\\ 0: non-free \\\\ 1: free \\\\"]
    #[inline(always)]
    pub fn mult_free(&mut self) -> MULT_FREE_W<'_, MULT_FREE_SPEC> {
        MULT_FREE_W::new(self, 0)
    }
}
#[doc = "ECC free status register\n\nYou can [`read`](crate::Reg::read) this register and get [`mult_free::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mult_free::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MULT_FREE_SPEC;
impl crate::RegisterSpec for MULT_FREE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mult_free::R`](R) reader structure"]
impl crate::Readable for MULT_FREE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mult_free::W`](W) writer structure"]
impl crate::Writable for MULT_FREE_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MULT_FREE to value 0x01"]
impl crate::Resettable for MULT_FREE_SPEC {
    const RESET_VALUE: u32 = 0x01;
}
