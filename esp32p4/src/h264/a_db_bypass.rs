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
            .field(
                "a_bypass_db_filter",
                &format_args!("{}", self.a_bypass_db_filter().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<A_DB_BYPASS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - Configures whether or not to bypass video A deblcoking filter. \\\\0: Open the deblock filter\\\\1: Close the deblock filter"]
    #[inline(always)]
    #[must_use]
    pub fn a_bypass_db_filter(&mut self) -> A_BYPASS_DB_FILTER_W<A_DB_BYPASS_SPEC> {
        A_BYPASS_DB_FILTER_W::new(self, 0)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Video A Deblocking bypass register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`a_db_bypass::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`a_db_bypass::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct A_DB_BYPASS_SPEC;
impl crate::RegisterSpec for A_DB_BYPASS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`a_db_bypass::R`](R) reader structure"]
impl crate::Readable for A_DB_BYPASS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`a_db_bypass::W`](W) writer structure"]
impl crate::Writable for A_DB_BYPASS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets A_DB_BYPASS to value 0"]
impl crate::Resettable for A_DB_BYPASS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
