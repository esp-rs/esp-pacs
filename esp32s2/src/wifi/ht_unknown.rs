#[doc = "Register `HT_UNKNOWN%s` reader"]
pub type R = crate::R<HT_UNKNOWN_SPEC>;
#[doc = "Register `HT_UNKNOWN%s` writer"]
pub type W = crate::W<HT_UNKNOWN_SPEC>;
#[doc = "Field `LENGTH` reader - The length of the PPDU"]
pub type LENGTH_R = crate::FieldReader<u32>;
#[doc = "Field `LENGTH` writer - The length of the PPDU"]
pub type LENGTH_W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:19 - The length of the PPDU"]
    #[inline(always)]
    pub fn length(&self) -> LENGTH_R {
        LENGTH_R::new(self.bits & 0x000f_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HT_UNKNOWN")
            .field("length", &self.length())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:19 - The length of the PPDU"]
    #[inline(always)]
    pub fn length(&mut self) -> LENGTH_W<HT_UNKNOWN_SPEC> {
        LENGTH_W::new(self, 0)
    }
}
#[doc = "exact meaning and name unknown, related to HT\n\nYou can [`read`](crate::Reg::read) this register and get [`ht_unknown::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ht_unknown::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HT_UNKNOWN_SPEC;
impl crate::RegisterSpec for HT_UNKNOWN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ht_unknown::R`](R) reader structure"]
impl crate::Readable for HT_UNKNOWN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ht_unknown::W`](W) writer structure"]
impl crate::Writable for HT_UNKNOWN_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HT_UNKNOWN%s to value 0"]
impl crate::Resettable for HT_UNKNOWN_SPEC {
    const RESET_VALUE: u32 = 0;
}
