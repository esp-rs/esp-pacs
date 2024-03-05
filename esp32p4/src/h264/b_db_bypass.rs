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
            .field(
                "b_bypass_db_filter",
                &format_args!("{}", self.b_bypass_db_filter().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<B_DB_BYPASS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - Configures whether or not to bypass video B deblcoking filter. \\\\0: Open the deblock filter\\\\1: Close the deblock filter"]
    #[inline(always)]
    #[must_use]
    pub fn b_bypass_db_filter(&mut self) -> B_BYPASS_DB_FILTER_W<B_DB_BYPASS_SPEC> {
        B_BYPASS_DB_FILTER_W::new(self, 0)
    }
}
#[doc = "Video B Deblocking bypass register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`b_db_bypass::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`b_db_bypass::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct B_DB_BYPASS_SPEC;
impl crate::RegisterSpec for B_DB_BYPASS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`b_db_bypass::R`](R) reader structure"]
impl crate::Readable for B_DB_BYPASS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`b_db_bypass::W`](W) writer structure"]
impl crate::Writable for B_DB_BYPASS_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets B_DB_BYPASS to value 0"]
impl crate::Resettable for B_DB_BYPASS_SPEC {
    const RESET_VALUE: u32 = 0;
}
