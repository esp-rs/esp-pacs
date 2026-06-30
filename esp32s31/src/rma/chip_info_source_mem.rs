#[doc = "Register `CHIP_INFO_SOURCE_MEM[%s]` reader"]
pub type R = crate::R<CHIP_INFO_SOURCE_MEM_SPEC>;
#[doc = "Register `CHIP_INFO_SOURCE_MEM[%s]` writer"]
pub type W = crate::W<CHIP_INFO_SOURCE_MEM_SPEC>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "RMA chip info memory.\n\nYou can [`read`](crate::Reg::read) this register and get [`chip_info_source_mem::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chip_info_source_mem::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CHIP_INFO_SOURCE_MEM_SPEC;
impl crate::RegisterSpec for CHIP_INFO_SOURCE_MEM_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`chip_info_source_mem::R`](R) reader structure"]
impl crate::Readable for CHIP_INFO_SOURCE_MEM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`chip_info_source_mem::W`](W) writer structure"]
impl crate::Writable for CHIP_INFO_SOURCE_MEM_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CHIP_INFO_SOURCE_MEM[%s] to value 0"]
impl crate::Resettable for CHIP_INFO_SOURCE_MEM_SPEC {}
