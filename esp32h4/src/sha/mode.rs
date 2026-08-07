#[doc = "Register `MODE` reader"]
pub type R = crate::R<MODE_SPEC>;
#[doc = "Register `MODE` writer"]
pub type W = crate::W<MODE_SPEC>;
#[doc = "Field `MODE` reader - Configures the SHA algorithm. \\\\ 0: SHA-1\\\\ 1: SHA2-224\\\\ 2: SHA2-256\\\\ 3: SHA2-384\\\\ 4: SHA2-512\\\\ 5: SHA2-512/224\\\\ 6: SHA2-512/256\\\\ 7: SHA2-512/t\\\\ 8: SHA3-224\\\\ 9: SHA3-256\\\\ 10: SHA3-384\\\\ 11: SHA3-512\\\\ 12: SHAKE128\\\\ 13: SHAKE256\\\\ 14: SM3\\\\"]
pub type MODE_R = crate::FieldReader;
#[doc = "Field `MODE` writer - Configures the SHA algorithm. \\\\ 0: SHA-1\\\\ 1: SHA2-224\\\\ 2: SHA2-256\\\\ 3: SHA2-384\\\\ 4: SHA2-512\\\\ 5: SHA2-512/224\\\\ 6: SHA2-512/256\\\\ 7: SHA2-512/t\\\\ 8: SHA3-224\\\\ 9: SHA3-256\\\\ 10: SHA3-384\\\\ 11: SHA3-512\\\\ 12: SHAKE128\\\\ 13: SHAKE256\\\\ 14: SM3\\\\"]
pub type MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Configures the SHA algorithm. \\\\ 0: SHA-1\\\\ 1: SHA2-224\\\\ 2: SHA2-256\\\\ 3: SHA2-384\\\\ 4: SHA2-512\\\\ 5: SHA2-512/224\\\\ 6: SHA2-512/256\\\\ 7: SHA2-512/t\\\\ 8: SHA3-224\\\\ 9: SHA3-256\\\\ 10: SHA3-384\\\\ 11: SHA3-512\\\\ 12: SHAKE128\\\\ 13: SHAKE256\\\\ 14: SM3\\\\"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new((self.bits & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MODE").field("mode", &self.mode()).finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - Configures the SHA algorithm. \\\\ 0: SHA-1\\\\ 1: SHA2-224\\\\ 2: SHA2-256\\\\ 3: SHA2-384\\\\ 4: SHA2-512\\\\ 5: SHA2-512/224\\\\ 6: SHA2-512/256\\\\ 7: SHA2-512/t\\\\ 8: SHA3-224\\\\ 9: SHA3-256\\\\ 10: SHA3-384\\\\ 11: SHA3-512\\\\ 12: SHAKE128\\\\ 13: SHAKE256\\\\ 14: SM3\\\\"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W<'_, MODE_SPEC> {
        MODE_W::new(self, 0)
    }
}
#[doc = "Configures SHA algorithm\n\nYou can [`read`](crate::Reg::read) this register and get [`mode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MODE_SPEC;
impl crate::RegisterSpec for MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mode::R`](R) reader structure"]
impl crate::Readable for MODE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mode::W`](W) writer structure"]
impl crate::Writable for MODE_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MODE to value 0x02"]
impl crate::Resettable for MODE_SPEC {
    const RESET_VALUE: u32 = 0x02;
}
