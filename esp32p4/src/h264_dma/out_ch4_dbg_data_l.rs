#[doc = "Register `OUT_CH4_DBG_DATA_L` reader"]
pub type R = crate::R<OUT_CH4_DBG_DATA_L_SPEC>;
#[doc = "Register `OUT_CH4_DBG_DATA_L` writer"]
pub type W = crate::W<OUT_CH4_DBG_DATA_L_SPEC>;
#[doc = "Field `H264_OUT_CH4_DBG_DATA_L` reader - configures out channel 4 debug data bit 31-0"]
pub type H264_OUT_CH4_DBG_DATA_L_R = crate::FieldReader<u32>;
#[doc = "Field `H264_OUT_CH4_DBG_DATA_L` writer - configures out channel 4 debug data bit 31-0"]
pub type H264_OUT_CH4_DBG_DATA_L_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - configures out channel 4 debug data bit 31-0"]
    #[inline(always)]
    pub fn h264_out_ch4_dbg_data_l(&self) -> H264_OUT_CH4_DBG_DATA_L_R {
        H264_OUT_CH4_DBG_DATA_L_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OUT_CH4_DBG_DATA_L")
            .field("h264_out_ch4_dbg_data_l", &self.h264_out_ch4_dbg_data_l())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - configures out channel 4 debug data bit 31-0"]
    #[inline(always)]
    pub fn h264_out_ch4_dbg_data_l(
        &mut self,
    ) -> H264_OUT_CH4_DBG_DATA_L_W<'_, OUT_CH4_DBG_DATA_L_SPEC> {
        H264_OUT_CH4_DBG_DATA_L_W::new(self, 0)
    }
}
#[doc = "out channel 4 debug data register\n\nYou can [`read`](crate::Reg::read) this register and get [`out_ch4_dbg_data_l::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out_ch4_dbg_data_l::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OUT_CH4_DBG_DATA_L_SPEC;
impl crate::RegisterSpec for OUT_CH4_DBG_DATA_L_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`out_ch4_dbg_data_l::R`](R) reader structure"]
impl crate::Readable for OUT_CH4_DBG_DATA_L_SPEC {}
#[doc = "`write(|w| ..)` method takes [`out_ch4_dbg_data_l::W`](W) writer structure"]
impl crate::Writable for OUT_CH4_DBG_DATA_L_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OUT_CH4_DBG_DATA_L to value 0"]
impl crate::Resettable for OUT_CH4_DBG_DATA_L_SPEC {}
