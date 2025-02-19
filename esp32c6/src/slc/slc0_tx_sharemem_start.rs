#[doc = "Register `SLC0_TX_SHAREMEM_START` reader"]
pub type R = crate::R<SLC0_TX_SHAREMEM_START_SPEC>;
#[doc = "Register `SLC0_TX_SHAREMEM_START` writer"]
pub type W = crate::W<SLC0_TX_SHAREMEM_START_SPEC>;
#[doc = "Field `SDIO_SDIO_SLC0_TX_SHAREMEM_START_ADDR` reader - Configures SLC0 host to slave channel AHB start address boundary."]
pub type SDIO_SDIO_SLC0_TX_SHAREMEM_START_ADDR_R = crate::FieldReader<u32>;
#[doc = "Field `SDIO_SDIO_SLC0_TX_SHAREMEM_START_ADDR` writer - Configures SLC0 host to slave channel AHB start address boundary."]
pub type SDIO_SDIO_SLC0_TX_SHAREMEM_START_ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Configures SLC0 host to slave channel AHB start address boundary."]
    #[inline(always)]
    pub fn sdio_sdio_slc0_tx_sharemem_start_addr(&self) -> SDIO_SDIO_SLC0_TX_SHAREMEM_START_ADDR_R {
        SDIO_SDIO_SLC0_TX_SHAREMEM_START_ADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SLC0_TX_SHAREMEM_START")
            .field(
                "sdio_sdio_slc0_tx_sharemem_start_addr",
                &self.sdio_sdio_slc0_tx_sharemem_start_addr(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Configures SLC0 host to slave channel AHB start address boundary."]
    #[inline(always)]
    pub fn sdio_sdio_slc0_tx_sharemem_start_addr(
        &mut self,
    ) -> SDIO_SDIO_SLC0_TX_SHAREMEM_START_ADDR_W<SLC0_TX_SHAREMEM_START_SPEC> {
        SDIO_SDIO_SLC0_TX_SHAREMEM_START_ADDR_W::new(self, 0)
    }
}
#[doc = "SLC0 AHB TX start address range\n\nYou can [`read`](crate::Reg::read) this register and get [`slc0_tx_sharemem_start::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`slc0_tx_sharemem_start::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SLC0_TX_SHAREMEM_START_SPEC;
impl crate::RegisterSpec for SLC0_TX_SHAREMEM_START_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`slc0_tx_sharemem_start::R`](R) reader structure"]
impl crate::Readable for SLC0_TX_SHAREMEM_START_SPEC {}
#[doc = "`write(|w| ..)` method takes [`slc0_tx_sharemem_start::W`](W) writer structure"]
impl crate::Writable for SLC0_TX_SHAREMEM_START_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SLC0_TX_SHAREMEM_START to value 0"]
impl crate::Resettable for SLC0_TX_SHAREMEM_START_SPEC {
    const RESET_VALUE: u32 = 0;
}
