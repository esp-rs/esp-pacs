#[doc = "Register `SMEM_PMS%s_SIZE` reader"]
pub type R = crate::R<SMEM_PMS_SIZE_SPEC>;
#[doc = "Register `SMEM_PMS%s_SIZE` writer"]
pub type W = crate::W<SMEM_PMS_SIZE_SPEC>;
#[doc = "Field `SMEM_PMS_SIZE` reader - SPI1 external RAM PMS section %s address region is (SPI_SMEM_PMS%s_ADDR_S, SPI_SMEM_PMS%s_ADDR_S + SPI_SMEM_PMS%s_SIZE)"]
pub type SMEM_PMS_SIZE_R = crate::FieldReader<u32>;
#[doc = "Field `SMEM_PMS_SIZE` writer - SPI1 external RAM PMS section %s address region is (SPI_SMEM_PMS%s_ADDR_S, SPI_SMEM_PMS%s_ADDR_S + SPI_SMEM_PMS%s_SIZE)"]
pub type SMEM_PMS_SIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 17, u32>;
impl R {
    #[doc = "Bits 0:16 - SPI1 external RAM PMS section %s address region is (SPI_SMEM_PMS%s_ADDR_S, SPI_SMEM_PMS%s_ADDR_S + SPI_SMEM_PMS%s_SIZE)"]
    #[inline(always)]
    pub fn smem_pms_size(&self) -> SMEM_PMS_SIZE_R {
        SMEM_PMS_SIZE_R::new(self.bits & 0x0001_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SMEM_PMS_SIZE")
            .field("smem_pms_size", &self.smem_pms_size())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:16 - SPI1 external RAM PMS section %s address region is (SPI_SMEM_PMS%s_ADDR_S, SPI_SMEM_PMS%s_ADDR_S + SPI_SMEM_PMS%s_SIZE)"]
    #[inline(always)]
    pub fn smem_pms_size(&mut self) -> SMEM_PMS_SIZE_W<SMEM_PMS_SIZE_SPEC> {
        SMEM_PMS_SIZE_W::new(self, 0)
    }
}
#[doc = "SPI1 external RAM PMS section %s start address register\n\nYou can [`read`](crate::Reg::read) this register and get [`smem_pms_size::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smem_pms_size::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SMEM_PMS_SIZE_SPEC;
impl crate::RegisterSpec for SMEM_PMS_SIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`smem_pms_size::R`](R) reader structure"]
impl crate::Readable for SMEM_PMS_SIZE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`smem_pms_size::W`](W) writer structure"]
impl crate::Writable for SMEM_PMS_SIZE_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SMEM_PMS%s_SIZE to value 0x1000"]
impl crate::Resettable for SMEM_PMS_SIZE_SPEC {
    const RESET_VALUE: u32 = 0x1000;
}
