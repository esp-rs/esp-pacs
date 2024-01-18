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
            .field("cpu", &format_args!("{}", self.cpu().bit()))
            .field("dma", &format_args!("{}", self.dma().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PSRAM_FLASH_ADDR_INTERCHANGE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - Set 1 to enable addr interchange between psram and flash in axi matrix when hp cpu access through cache"]
    #[inline(always)]
    #[must_use]
    pub fn cpu(&mut self) -> CPU_W<PSRAM_FLASH_ADDR_INTERCHANGE_SPEC> {
        CPU_W::new(self, 0)
    }
    #[doc = "Bit 1 - Set 1 to enable addr interchange between psram and flash in axi matrix when dma device access, lp core access and hp core access through ahb"]
    #[inline(always)]
    #[must_use]
    pub fn dma(&mut self) -> DMA_W<PSRAM_FLASH_ADDR_INTERCHANGE_SPEC> {
        DMA_W::new(self, 1)
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
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`psram_flash_addr_interchange::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`psram_flash_addr_interchange::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PSRAM_FLASH_ADDR_INTERCHANGE_SPEC;
impl crate::RegisterSpec for PSRAM_FLASH_ADDR_INTERCHANGE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`psram_flash_addr_interchange::R`](R) reader structure"]
impl crate::Readable for PSRAM_FLASH_ADDR_INTERCHANGE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`psram_flash_addr_interchange::W`](W) writer structure"]
impl crate::Writable for PSRAM_FLASH_ADDR_INTERCHANGE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PSRAM_FLASH_ADDR_INTERCHANGE to value 0"]
impl crate::Resettable for PSRAM_FLASH_ADDR_INTERCHANGE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
