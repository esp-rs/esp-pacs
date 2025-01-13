#[doc = "Register `HT_SIG%s` reader"]
pub type R = crate::R<HT_SIG_SPEC>;
#[doc = "Register `HT_SIG%s` writer"]
pub type W = crate::W<HT_SIG_SPEC>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "HT-SIG field in HT preamble\n\nYou can [`read`](crate::Reg::read) this register and get [`ht_sig::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ht_sig::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HT_SIG_SPEC;
impl crate::RegisterSpec for HT_SIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ht_sig::R`](R) reader structure"]
impl crate::Readable for HT_SIG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ht_sig::W`](W) writer structure"]
impl crate::Writable for HT_SIG_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HT_SIG%s to value 0"]
impl crate::Resettable for HT_SIG_SPEC {
    const RESET_VALUE: u32 = 0;
}
