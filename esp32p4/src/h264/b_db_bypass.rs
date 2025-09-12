#[doc = "Register `B_DB_BYPASS` reader"]
pub type R = crate::R<B_DB_BYPASS_SPEC>;
#[doc = "Register `B_DB_BYPASS` writer"]
pub type W = crate::W<B_DB_BYPASS_SPEC>;
#[doc = "Field `B_BYPASS_DB_FILTER` reader - Configures whether or not to bypass video B deblcoking filter. \\\\0: Open the deblock filter\\\\1: Close the deblock filter"]
pub type B_BYPASS_DB_FILTER_R = crate::BitReader;
#[doc = "Field `B_BYPASS_DB_FILTER` writer - Configures whether or not to bypass video B deblcoking filter. \\\\0: Open the deblock filter\\\\1: Close the deblock filter"]
pub type B_BYPASS_DB_FILTER_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Configures whether or not to bypass video B deblcoking filter. \\\\0: Open the deblock filter\\\\1: Close the deblock filter"]
    #[inline(always)]
    pub fn b_bypass_db_filter(&self) -> B_BYPASS_DB_FILTER_R {
        B_BYPASS_DB_FILTER_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("B_DB_BYPASS")
            .field("b_bypass_db_filter", &self.b_bypass_db_filter())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Configures whether or not to bypass video B deblcoking filter. \\\\0: Open the deblock filter\\\\1: Close the deblock filter"]
    #[inline(always)]
    pub fn b_bypass_db_filter(&mut self) -> B_BYPASS_DB_FILTER_W<'_, B_DB_BYPASS_SPEC> {
        B_BYPASS_DB_FILTER_W::new(self, 0)
    }
}
#[doc = "Video B Deblocking bypass register\n\nYou can [`read`](crate::Reg::read) this register and get [`b_db_bypass::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`b_db_bypass::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct B_DB_BYPASS_SPEC;
impl crate::RegisterSpec for B_DB_BYPASS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`b_db_bypass::R`](R) reader structure"]
impl crate::Readable for B_DB_BYPASS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`b_db_bypass::W`](W) writer structure"]
impl crate::Writable for B_DB_BYPASS_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets B_DB_BYPASS to value 0"]
impl crate::Resettable for B_DB_BYPASS_SPEC {}
