#[doc = "Register `AXI_BLOCK_EN` reader"]
pub type R = crate::R<AXI_BLOCK_EN_SPEC>;
#[doc = "Register `AXI_BLOCK_EN` writer"]
pub type W = crate::W<AXI_BLOCK_EN_SPEC>;
#[doc = "Field `AXI_BLOCK_EN` reader - Configures to if block transmission when get exceltion.\\\\ 0: transparent transmission to slave \\\\ 1: don't transparent transmission to slave \\\\"]
pub type AXI_BLOCK_EN_R = crate::BitReader;
#[doc = "Field `AXI_BLOCK_EN` writer - Configures to if block transmission when get exceltion.\\\\ 0: transparent transmission to slave \\\\ 1: don't transparent transmission to slave \\\\"]
pub type AXI_BLOCK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Configures to if block transmission when get exceltion.\\\\ 0: transparent transmission to slave \\\\ 1: don't transparent transmission to slave \\\\"]
    #[inline(always)]
    pub fn axi_block_en(&self) -> AXI_BLOCK_EN_R {
        AXI_BLOCK_EN_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AXI_BLOCK_EN")
            .field("axi_block_en", &self.axi_block_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Configures to if block transmission when get exceltion.\\\\ 0: transparent transmission to slave \\\\ 1: don't transparent transmission to slave \\\\"]
    #[inline(always)]
    pub fn axi_block_en(&mut self) -> AXI_BLOCK_EN_W<'_, AXI_BLOCK_EN_SPEC> {
        AXI_BLOCK_EN_W::new(self, 0)
    }
}
#[doc = "M6 status clear register\n\nYou can [`read`](crate::Reg::read) this register and get [`axi_block_en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`axi_block_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AXI_BLOCK_EN_SPEC;
impl crate::RegisterSpec for AXI_BLOCK_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`axi_block_en::R`](R) reader structure"]
impl crate::Readable for AXI_BLOCK_EN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`axi_block_en::W`](W) writer structure"]
impl crate::Writable for AXI_BLOCK_EN_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AXI_BLOCK_EN to value 0x01"]
impl crate::Resettable for AXI_BLOCK_EN_SPEC {
    const RESET_VALUE: u32 = 0x01;
}
