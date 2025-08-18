#[doc = "Register `PSRAM_FLASH_ADDR_INTERCHANGE` reader"]
pub type R = crate::R<PSRAM_FLASH_ADDR_INTERCHANGE_SPEC>;
#[doc = "Register `PSRAM_FLASH_ADDR_INTERCHANGE` writer"]
pub type W = crate::W<PSRAM_FLASH_ADDR_INTERCHANGE_SPEC>;
#[doc = "Field `CPU` reader - Set 1 to enable addr interchange between psram and flash in axi matrix when hp cpu access through cache"]
pub type CPU_R = crate::BitReader;
#[doc = "Field `CPU` writer - Set 1 to enable addr interchange between psram and flash in axi matrix when hp cpu access through cache"]
pub type CPU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA` reader - Set 1 to enable addr interchange between psram and flash in axi matrix when dma device access, lp core access and hp core access through ahb"]
pub type DMA_R = crate::BitReader;
#[doc = "Field `DMA` writer - Set 1 to enable addr interchange between psram and flash in axi matrix when dma device access, lp core access and hp core access through ahb"]
pub type DMA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Set 1 to enable addr interchange between psram and flash in axi matrix when hp cpu access through cache"]
    #[inline(always)]
    pub fn cpu(&self) -> CPU_R {
        CPU_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set 1 to enable addr interchange between psram and flash in axi matrix when dma device access, lp core access and hp core access through ahb"]
    #[inline(always)]
    pub fn dma(&self) -> DMA_R {
        DMA_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PSRAM_FLASH_ADDR_INTERCHANGE")
            .field("cpu", &self.cpu())
            .field("dma", &self.dma())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Set 1 to enable addr interchange between psram and flash in axi matrix when hp cpu access through cache"]
    #[inline(always)]
    pub fn cpu(&mut self) -> CPU_W<'_, PSRAM_FLASH_ADDR_INTERCHANGE_SPEC> {
        CPU_W::new(self, 0)
    }
    #[doc = "Bit 1 - Set 1 to enable addr interchange between psram and flash in axi matrix when dma device access, lp core access and hp core access through ahb"]
    #[inline(always)]
    pub fn dma(&mut self) -> DMA_W<'_, PSRAM_FLASH_ADDR_INTERCHANGE_SPEC> {
        DMA_W::new(self, 1)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`psram_flash_addr_interchange::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psram_flash_addr_interchange::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PSRAM_FLASH_ADDR_INTERCHANGE_SPEC;
impl crate::RegisterSpec for PSRAM_FLASH_ADDR_INTERCHANGE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`psram_flash_addr_interchange::R`](R) reader structure"]
impl crate::Readable for PSRAM_FLASH_ADDR_INTERCHANGE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`psram_flash_addr_interchange::W`](W) writer structure"]
impl crate::Writable for PSRAM_FLASH_ADDR_INTERCHANGE_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PSRAM_FLASH_ADDR_INTERCHANGE to value 0"]
impl crate::Resettable for PSRAM_FLASH_ADDR_INTERCHANGE_SPEC {}
