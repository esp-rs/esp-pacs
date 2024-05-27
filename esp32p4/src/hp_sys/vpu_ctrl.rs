#[doc = "Register `VPU_CTRL` reader"]
pub type R = crate::R<VPU_CTRL_SPEC>;
#[doc = "Register `VPU_CTRL` writer"]
pub type W = crate::W<VPU_CTRL_SPEC>;
#[doc = "Field `PPA_LSLP_MEM_PD` reader - N/A"]
pub type PPA_LSLP_MEM_PD_R = crate::BitReader;
#[doc = "Field `PPA_LSLP_MEM_PD` writer - N/A"]
pub type PPA_LSLP_MEM_PD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JPEG_SDSLP_MEM_PD` reader - N/A"]
pub type JPEG_SDSLP_MEM_PD_R = crate::BitReader;
#[doc = "Field `JPEG_SDSLP_MEM_PD` writer - N/A"]
pub type JPEG_SDSLP_MEM_PD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JPEG_LSLP_MEM_PD` reader - N/A"]
pub type JPEG_LSLP_MEM_PD_R = crate::BitReader;
#[doc = "Field `JPEG_LSLP_MEM_PD` writer - N/A"]
pub type JPEG_LSLP_MEM_PD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JPEG_DSLP_MEM_PD` reader - N/A"]
pub type JPEG_DSLP_MEM_PD_R = crate::BitReader;
#[doc = "Field `JPEG_DSLP_MEM_PD` writer - N/A"]
pub type JPEG_DSLP_MEM_PD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA2D_LSLP_MEM_PD` reader - N/A"]
pub type DMA2D_LSLP_MEM_PD_R = crate::BitReader;
#[doc = "Field `DMA2D_LSLP_MEM_PD` writer - N/A"]
pub type DMA2D_LSLP_MEM_PD_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - N/A"]
    #[inline(always)]
    pub fn ppa_lslp_mem_pd(&self) -> PPA_LSLP_MEM_PD_R {
        PPA_LSLP_MEM_PD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - N/A"]
    #[inline(always)]
    pub fn jpeg_sdslp_mem_pd(&self) -> JPEG_SDSLP_MEM_PD_R {
        JPEG_SDSLP_MEM_PD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - N/A"]
    #[inline(always)]
    pub fn jpeg_lslp_mem_pd(&self) -> JPEG_LSLP_MEM_PD_R {
        JPEG_LSLP_MEM_PD_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - N/A"]
    #[inline(always)]
    pub fn jpeg_dslp_mem_pd(&self) -> JPEG_DSLP_MEM_PD_R {
        JPEG_DSLP_MEM_PD_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - N/A"]
    #[inline(always)]
    pub fn dma2d_lslp_mem_pd(&self) -> DMA2D_LSLP_MEM_PD_R {
        DMA2D_LSLP_MEM_PD_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VPU_CTRL")
            .field("ppa_lslp_mem_pd", &self.ppa_lslp_mem_pd())
            .field("jpeg_sdslp_mem_pd", &self.jpeg_sdslp_mem_pd())
            .field("jpeg_lslp_mem_pd", &self.jpeg_lslp_mem_pd())
            .field("jpeg_dslp_mem_pd", &self.jpeg_dslp_mem_pd())
            .field("dma2d_lslp_mem_pd", &self.dma2d_lslp_mem_pd())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn ppa_lslp_mem_pd(&mut self) -> PPA_LSLP_MEM_PD_W<VPU_CTRL_SPEC> {
        PPA_LSLP_MEM_PD_W::new(self, 0)
    }
    #[doc = "Bit 1 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn jpeg_sdslp_mem_pd(&mut self) -> JPEG_SDSLP_MEM_PD_W<VPU_CTRL_SPEC> {
        JPEG_SDSLP_MEM_PD_W::new(self, 1)
    }
    #[doc = "Bit 2 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn jpeg_lslp_mem_pd(&mut self) -> JPEG_LSLP_MEM_PD_W<VPU_CTRL_SPEC> {
        JPEG_LSLP_MEM_PD_W::new(self, 2)
    }
    #[doc = "Bit 3 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn jpeg_dslp_mem_pd(&mut self) -> JPEG_DSLP_MEM_PD_W<VPU_CTRL_SPEC> {
        JPEG_DSLP_MEM_PD_W::new(self, 3)
    }
    #[doc = "Bit 4 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn dma2d_lslp_mem_pd(&mut self) -> DMA2D_LSLP_MEM_PD_W<VPU_CTRL_SPEC> {
        DMA2D_LSLP_MEM_PD_W::new(self, 4)
    }
}
#[doc = "N/A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vpu_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vpu_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VPU_CTRL_SPEC;
impl crate::RegisterSpec for VPU_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vpu_ctrl::R`](R) reader structure"]
impl crate::Readable for VPU_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`vpu_ctrl::W`](W) writer structure"]
impl crate::Writable for VPU_CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VPU_CTRL to value 0"]
impl crate::Resettable for VPU_CTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
