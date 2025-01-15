#[doc = "Register `HW_STAT_BLOCK_ERR` reader"]
pub type R = crate::R<HW_STAT_BLOCK_ERR_SPEC>;
#[doc = "Register `HW_STAT_BLOCK_ERR` writer"]
pub type W = crate::W<HW_STAT_BLOCK_ERR_SPEC>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`hw_stat_block_err::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hw_stat_block_err::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HW_STAT_BLOCK_ERR_SPEC;
impl crate::RegisterSpec for HW_STAT_BLOCK_ERR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hw_stat_block_err::R`](R) reader structure"]
impl crate::Readable for HW_STAT_BLOCK_ERR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hw_stat_block_err::W`](W) writer structure"]
impl crate::Writable for HW_STAT_BLOCK_ERR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HW_STAT_BLOCK_ERR to value 0"]
impl crate::Resettable for HW_STAT_BLOCK_ERR_SPEC {
    const RESET_VALUE: u32 = 0;
}