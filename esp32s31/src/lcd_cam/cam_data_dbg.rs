#[doc = "Register `CAM_DATA_DBG` reader"]
pub type R = crate::R<CAM_DATA_DBG_SPEC>;
#[doc = "Register `CAM_DATA_DBG` writer"]
pub type W = crate::W<CAM_DATA_DBG_SPEC>;
#[doc = "Field `CAM_DBG_DATA_RELPACE_ENA` reader - 1:replace the data write to dma.0:not replace the data write to dma"]
pub type CAM_DBG_DATA_RELPACE_ENA_R = crate::BitReader;
#[doc = "Field `CAM_DBG_DATA_RELPACE_ENA` writer - 1:replace the data write to dma.0:not replace the data write to dma"]
pub type CAM_DBG_DATA_RELPACE_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAM_DBG_DATA` reader - the debug data that used to replace data write to dma"]
pub type CAM_DBG_DATA_R = crate::FieldReader<u32>;
#[doc = "Field `CAM_DBG_DATA` writer - the debug data that used to replace data write to dma"]
pub type CAM_DBG_DATA_W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bit 0 - 1:replace the data write to dma.0:not replace the data write to dma"]
    #[inline(always)]
    pub fn cam_dbg_data_relpace_ena(&self) -> CAM_DBG_DATA_RELPACE_ENA_R {
        CAM_DBG_DATA_RELPACE_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:24 - the debug data that used to replace data write to dma"]
    #[inline(always)]
    pub fn cam_dbg_data(&self) -> CAM_DBG_DATA_R {
        CAM_DBG_DATA_R::new((self.bits >> 1) & 0x00ff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CAM_DATA_DBG")
            .field("cam_dbg_data_relpace_ena", &self.cam_dbg_data_relpace_ena())
            .field("cam_dbg_data", &self.cam_dbg_data())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - 1:replace the data write to dma.0:not replace the data write to dma"]
    #[inline(always)]
    pub fn cam_dbg_data_relpace_ena(
        &mut self,
    ) -> CAM_DBG_DATA_RELPACE_ENA_W<'_, CAM_DATA_DBG_SPEC> {
        CAM_DBG_DATA_RELPACE_ENA_W::new(self, 0)
    }
    #[doc = "Bits 1:24 - the debug data that used to replace data write to dma"]
    #[inline(always)]
    pub fn cam_dbg_data(&mut self) -> CAM_DBG_DATA_W<'_, CAM_DATA_DBG_SPEC> {
        CAM_DBG_DATA_W::new(self, 1)
    }
}
#[doc = "LCDCAM CAM debug data register\n\nYou can [`read`](crate::Reg::read) this register and get [`cam_data_dbg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cam_data_dbg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CAM_DATA_DBG_SPEC;
impl crate::RegisterSpec for CAM_DATA_DBG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cam_data_dbg::R`](R) reader structure"]
impl crate::Readable for CAM_DATA_DBG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cam_data_dbg::W`](W) writer structure"]
impl crate::Writable for CAM_DATA_DBG_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CAM_DATA_DBG to value 0"]
impl crate::Resettable for CAM_DATA_DBG_SPEC {}
