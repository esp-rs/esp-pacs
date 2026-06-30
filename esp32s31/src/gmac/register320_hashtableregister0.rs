#[doc = "Register `REGISTER320_HASHTABLEREGISTER0` reader"]
pub type R = crate::R<REGISTER320_HASHTABLEREGISTER0_SPEC>;
#[doc = "Register `REGISTER320_HASHTABLEREGISTER0` writer"]
pub type W = crate::W<REGISTER320_HASHTABLEREGISTER0_SPEC>;
#[doc = "Field `HT31T0` reader - First 32 bits of Hash Table This field contains the first 32 Bits _31:0_ of the Hash table Note Registers 321 through 327 are similar to Register 320 _Hash Table Register 0_ Registers 324 through 327 are present only when you select the 256bit Hash table during core configuration"]
pub type HT31T0_R = crate::FieldReader<u32>;
#[doc = "Field `HT31T0` writer - First 32 bits of Hash Table This field contains the first 32 Bits _31:0_ of the Hash table Note Registers 321 through 327 are similar to Register 320 _Hash Table Register 0_ Registers 324 through 327 are present only when you select the 256bit Hash table during core configuration"]
pub type HT31T0_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - First 32 bits of Hash Table This field contains the first 32 Bits _31:0_ of the Hash table Note Registers 321 through 327 are similar to Register 320 _Hash Table Register 0_ Registers 324 through 327 are present only when you select the 256bit Hash table during core configuration"]
    #[inline(always)]
    pub fn ht31t0(&self) -> HT31T0_R {
        HT31T0_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REGISTER320_HASHTABLEREGISTER0")
            .field("ht31t0", &self.ht31t0())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - First 32 bits of Hash Table This field contains the first 32 Bits _31:0_ of the Hash table Note Registers 321 through 327 are similar to Register 320 _Hash Table Register 0_ Registers 324 through 327 are present only when you select the 256bit Hash table during core configuration"]
    #[inline(always)]
    pub fn ht31t0(&mut self) -> HT31T0_W<'_, REGISTER320_HASHTABLEREGISTER0_SPEC> {
        HT31T0_W::new(self, 0)
    }
}
#[doc = "This register contains the first 32 bits of the hash table when the width of the Hash table is 128 bits or 256 bits\n\nYou can [`read`](crate::Reg::read) this register and get [`register320_hashtableregister0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register320_hashtableregister0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REGISTER320_HASHTABLEREGISTER0_SPEC;
impl crate::RegisterSpec for REGISTER320_HASHTABLEREGISTER0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`register320_hashtableregister0::R`](R) reader structure"]
impl crate::Readable for REGISTER320_HASHTABLEREGISTER0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`register320_hashtableregister0::W`](W) writer structure"]
impl crate::Writable for REGISTER320_HASHTABLEREGISTER0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REGISTER320_HASHTABLEREGISTER0 to value 0"]
impl crate::Resettable for REGISTER320_HASHTABLEREGISTER0_SPEC {}
