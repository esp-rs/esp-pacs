#[doc = "Register `A_DB_BYPASS` reader"]
pub type R = crate::R<A_DB_BYPASS_SPEC>;
#[doc = "Register `A_DB_BYPASS` writer"]
pub type W = crate::W<A_DB_BYPASS_SPEC>;
#[doc = "Field `A_BYPASS_DB_FILTER` reader - Configures whether or not to bypass video A deblcoking filter. \\\\0: Open the deblock filter\\\\1: Close the deblock filter"]
pub type A_BYPASS_DB_FILTER_R = crate::BitReader;
#[doc = "Field `A_BYPASS_DB_FILTER` writer - Configures whether or not to bypass video A deblcoking filter. \\\\0: Open the deblock filter\\\\1: Close the deblock filter"]
pub type A_BYPASS_DB_FILTER_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Configures whether or not to bypass video A deblcoking filter. \\\\0: Open the deblock filter\\\\1: Close the deblock filter"]
    #[inline(always)]
    pub fn a_bypass_db_filter(&self) -> A_BYPASS_DB_FILTER_R {
        A_BYPASS_DB_FILTER_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("A_DB_BYPASS")
            .field("a_bypass_db_filter", &self.a_bypass_db_filter())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Configures whether or not to bypass video A deblcoking filter. \\\\0: Open the deblock filter\\\\1: Close the deblock filter"]
    #[inline(always)]
    pub fn a_bypass_db_filter(&mut self) -> A_BYPASS_DB_FILTER_W<A_DB_BYPASS_SPEC> {
        A_BYPASS_DB_FILTER_W::new(self, 0)
    }
}
#[doc = "Video A Deblocking bypass register\n\nYou can [`read`](crate::Reg::read) this register and get [`a_db_bypass::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`a_db_bypass::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct A_DB_BYPASS_SPEC;
impl crate::RegisterSpec for A_DB_BYPASS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`a_db_bypass::R`](R) reader structure"]
impl crate::Readable for A_DB_BYPASS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`a_db_bypass::W`](W) writer structure"]
impl crate::Writable for A_DB_BYPASS_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets A_DB_BYPASS to value 0"]
impl crate::Resettable for A_DB_BYPASS_SPEC {
    const RESET_VALUE: u32 = 0;
}
