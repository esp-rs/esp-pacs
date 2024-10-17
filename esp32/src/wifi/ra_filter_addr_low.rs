#[doc = "Register `RA_FILTER_ADDR_LOW` reader"]
pub type R = crate::R<RA_FILTER_ADDR_LOW_SPEC>;
#[doc = "Register `RA_FILTER_ADDR_LOW` writer"]
pub type W = crate::W<RA_FILTER_ADDR_LOW_SPEC>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "first 4 bytes of RA MAC address filter\n\nYou can [`read`](crate::Reg::read) this register and get [`ra_filter_addr_low::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ra_filter_addr_low::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RA_FILTER_ADDR_LOW_SPEC;
impl crate::RegisterSpec for RA_FILTER_ADDR_LOW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ra_filter_addr_low::R`](R) reader structure"]
impl crate::Readable for RA_FILTER_ADDR_LOW_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ra_filter_addr_low::W`](W) writer structure"]
impl crate::Writable for RA_FILTER_ADDR_LOW_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RA_FILTER_ADDR_LOW to value 0"]
impl crate::Resettable for RA_FILTER_ADDR_LOW_SPEC {
    const RESET_VALUE: u32 = 0;
}
