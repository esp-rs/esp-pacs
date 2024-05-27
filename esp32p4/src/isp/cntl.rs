///Register `CNTL` reader
pub type R = crate::R<CNTL_SPEC>;
///Register `CNTL` writer
pub type W = crate::W<CNTL_SPEC>;
///Field `MIPI_DATA_EN` reader - this bit configures mipi input data enable. 0: disable, 1: enable
pub type MIPI_DATA_EN_R = crate::BitReader;
///Field `MIPI_DATA_EN` writer - this bit configures mipi input data enable. 0: disable, 1: enable
pub type MIPI_DATA_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ISP_EN` reader - this bit configures isp global enable. 0: disable, 1: enable
pub type ISP_EN_R = crate::BitReader;
///Field `ISP_EN` writer - this bit configures isp global enable. 0: disable, 1: enable
pub type ISP_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BLC_EN` reader - this bit configures blc enable. 0: disable, 1: enable
pub type BLC_EN_R = crate::BitReader;
///Field `BLC_EN` writer - this bit configures blc enable. 0: disable, 1: enable
pub type BLC_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DPC_EN` reader - this bit configures dpc enable. 0: disable, 1: enable
pub type DPC_EN_R = crate::BitReader;
///Field `DPC_EN` writer - this bit configures dpc enable. 0: disable, 1: enable
pub type DPC_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BF_EN` reader - this bit configures bf enable. 0: disable, 1: enable
pub type BF_EN_R = crate::BitReader;
///Field `BF_EN` writer - this bit configures bf enable. 0: disable, 1: enable
pub type BF_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LSC_EN` reader - this bit configures lsc enable. 0: disable, 1: enable
pub type LSC_EN_R = crate::BitReader;
///Field `LSC_EN` writer - this bit configures lsc enable. 0: disable, 1: enable
pub type LSC_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DEMOSAIC_EN` reader - this bit configures demosaic enable. 0: disable, 1: enable
pub type DEMOSAIC_EN_R = crate::BitReader;
///Field `DEMOSAIC_EN` writer - this bit configures demosaic enable. 0: disable, 1: enable
pub type DEMOSAIC_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MEDIAN_EN` reader - this bit configures median enable. 0: disable, 1: enable
pub type MEDIAN_EN_R = crate::BitReader;
///Field `MEDIAN_EN` writer - this bit configures median enable. 0: disable, 1: enable
pub type MEDIAN_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CCM_EN` reader - this bit configures ccm enable. 0: disable, 1: enable
pub type CCM_EN_R = crate::BitReader;
///Field `CCM_EN` writer - this bit configures ccm enable. 0: disable, 1: enable
pub type CCM_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GAMMA_EN` reader - this bit configures gamma enable. 0: disable, 1: enable
pub type GAMMA_EN_R = crate::BitReader;
///Field `GAMMA_EN` writer - this bit configures gamma enable. 0: disable, 1: enable
pub type GAMMA_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RGB2YUV_EN` reader - this bit configures rgb2yuv enable. 0: disable, 1: enable
pub type RGB2YUV_EN_R = crate::BitReader;
///Field `RGB2YUV_EN` writer - this bit configures rgb2yuv enable. 0: disable, 1: enable
pub type RGB2YUV_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SHARP_EN` reader - this bit configures sharp enable. 0: disable, 1: enable
pub type SHARP_EN_R = crate::BitReader;
///Field `SHARP_EN` writer - this bit configures sharp enable. 0: disable, 1: enable
pub type SHARP_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `COLOR_EN` reader - this bit configures color enable. 0: disable, 1: enable
pub type COLOR_EN_R = crate::BitReader;
///Field `COLOR_EN` writer - this bit configures color enable. 0: disable, 1: enable
pub type COLOR_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `YUV2RGB_EN` reader - this bit configures yuv2rgb enable. 0: disable, 1: enable
pub type YUV2RGB_EN_R = crate::BitReader;
///Field `YUV2RGB_EN` writer - this bit configures yuv2rgb enable. 0: disable, 1: enable
pub type YUV2RGB_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AE_EN` reader - this bit configures ae enable. 0: disable, 1: enable
pub type AE_EN_R = crate::BitReader;
///Field `AE_EN` writer - this bit configures ae enable. 0: disable, 1: enable
pub type AE_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AF_EN` reader - this bit configures af enable. 0: disable, 1: enable
pub type AF_EN_R = crate::BitReader;
///Field `AF_EN` writer - this bit configures af enable. 0: disable, 1: enable
pub type AF_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AWB_EN` reader - this bit configures awb enable. 0: disable, 1: enable
pub type AWB_EN_R = crate::BitReader;
///Field `AWB_EN` writer - this bit configures awb enable. 0: disable, 1: enable
pub type AWB_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HIST_EN` reader - this bit configures hist enable. 0: disable, 1: enable
pub type HIST_EN_R = crate::BitReader;
///Field `HIST_EN` writer - this bit configures hist enable. 0: disable, 1: enable
pub type HIST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BYTE_ENDIAN_ORDER` reader - select input idi data byte_endian_order when isp is bypass, 0: csi_data\[31:0\], 1: {\[7:0\], \[15:8\], \[23:16\], \[31:24\]}
pub type BYTE_ENDIAN_ORDER_R = crate::BitReader;
///Field `BYTE_ENDIAN_ORDER` writer - select input idi data byte_endian_order when isp is bypass, 0: csi_data\[31:0\], 1: {\[7:0\], \[15:8\], \[23:16\], \[31:24\]}
pub type BYTE_ENDIAN_ORDER_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ISP_DATA_TYPE` reader - this field configures input data type, 0:RAW8 1:RAW10 2:RAW12
pub type ISP_DATA_TYPE_R = crate::FieldReader;
///Field `ISP_DATA_TYPE` writer - this field configures input data type, 0:RAW8 1:RAW10 2:RAW12
pub type ISP_DATA_TYPE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `ISP_IN_SRC` reader - this field configures input data source, 0:CSI HOST 1:CAM 2:DMA
pub type ISP_IN_SRC_R = crate::FieldReader;
///Field `ISP_IN_SRC` writer - this field configures input data source, 0:CSI HOST 1:CAM 2:DMA
pub type ISP_IN_SRC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `ISP_OUT_TYPE` reader - this field configures pixel output type, 0: RAW8 1: YUV422 2: RGB888 3: YUV420 4: RGB565
pub type ISP_OUT_TYPE_R = crate::FieldReader;
///Field `ISP_OUT_TYPE` writer - this field configures pixel output type, 0: RAW8 1: YUV422 2: RGB888 3: YUV420 4: RGB565
pub type ISP_OUT_TYPE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bit 0 - this bit configures mipi input data enable. 0: disable, 1: enable
    #[inline(always)]
    pub fn mipi_data_en(&self) -> MIPI_DATA_EN_R {
        MIPI_DATA_EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - this bit configures isp global enable. 0: disable, 1: enable
    #[inline(always)]
    pub fn isp_en(&self) -> ISP_EN_R {
        ISP_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - this bit configures blc enable. 0: disable, 1: enable
    #[inline(always)]
    pub fn blc_en(&self) -> BLC_EN_R {
        BLC_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - this bit configures dpc enable. 0: disable, 1: enable
    #[inline(always)]
    pub fn dpc_en(&self) -> DPC_EN_R {
        DPC_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - this bit configures bf enable. 0: disable, 1: enable
    #[inline(always)]
    pub fn bf_en(&self) -> BF_EN_R {
        BF_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - this bit configures lsc enable. 0: disable, 1: enable
    #[inline(always)]
    pub fn lsc_en(&self) -> LSC_EN_R {
        LSC_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - this bit configures demosaic enable. 0: disable, 1: enable
    #[inline(always)]
    pub fn demosaic_en(&self) -> DEMOSAIC_EN_R {
        DEMOSAIC_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - this bit configures median enable. 0: disable, 1: enable
    #[inline(always)]
    pub fn median_en(&self) -> MEDIAN_EN_R {
        MEDIAN_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - this bit configures ccm enable. 0: disable, 1: enable
    #[inline(always)]
    pub fn ccm_en(&self) -> CCM_EN_R {
        CCM_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - this bit configures gamma enable. 0: disable, 1: enable
    #[inline(always)]
    pub fn gamma_en(&self) -> GAMMA_EN_R {
        GAMMA_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - this bit configures rgb2yuv enable. 0: disable, 1: enable
    #[inline(always)]
    pub fn rgb2yuv_en(&self) -> RGB2YUV_EN_R {
        RGB2YUV_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - this bit configures sharp enable. 0: disable, 1: enable
    #[inline(always)]
    pub fn sharp_en(&self) -> SHARP_EN_R {
        SHARP_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - this bit configures color enable. 0: disable, 1: enable
    #[inline(always)]
    pub fn color_en(&self) -> COLOR_EN_R {
        COLOR_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - this bit configures yuv2rgb enable. 0: disable, 1: enable
    #[inline(always)]
    pub fn yuv2rgb_en(&self) -> YUV2RGB_EN_R {
        YUV2RGB_EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - this bit configures ae enable. 0: disable, 1: enable
    #[inline(always)]
    pub fn ae_en(&self) -> AE_EN_R {
        AE_EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - this bit configures af enable. 0: disable, 1: enable
    #[inline(always)]
    pub fn af_en(&self) -> AF_EN_R {
        AF_EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - this bit configures awb enable. 0: disable, 1: enable
    #[inline(always)]
    pub fn awb_en(&self) -> AWB_EN_R {
        AWB_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - this bit configures hist enable. 0: disable, 1: enable
    #[inline(always)]
    pub fn hist_en(&self) -> HIST_EN_R {
        HIST_EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 24 - select input idi data byte_endian_order when isp is bypass, 0: csi_data\[31:0\], 1: {\[7:0\], \[15:8\], \[23:16\], \[31:24\]}
    #[inline(always)]
    pub fn byte_endian_order(&self) -> BYTE_ENDIAN_ORDER_R {
        BYTE_ENDIAN_ORDER_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bits 25:26 - this field configures input data type, 0:RAW8 1:RAW10 2:RAW12
    #[inline(always)]
    pub fn isp_data_type(&self) -> ISP_DATA_TYPE_R {
        ISP_DATA_TYPE_R::new(((self.bits >> 25) & 3) as u8)
    }
    ///Bits 27:28 - this field configures input data source, 0:CSI HOST 1:CAM 2:DMA
    #[inline(always)]
    pub fn isp_in_src(&self) -> ISP_IN_SRC_R {
        ISP_IN_SRC_R::new(((self.bits >> 27) & 3) as u8)
    }
    ///Bits 29:31 - this field configures pixel output type, 0: RAW8 1: YUV422 2: RGB888 3: YUV420 4: RGB565
    #[inline(always)]
    pub fn isp_out_type(&self) -> ISP_OUT_TYPE_R {
        ISP_OUT_TYPE_R::new(((self.bits >> 29) & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CNTL")
            .field("mipi_data_en", &self.mipi_data_en())
            .field("isp_en", &self.isp_en())
            .field("blc_en", &self.blc_en())
            .field("dpc_en", &self.dpc_en())
            .field("bf_en", &self.bf_en())
            .field("lsc_en", &self.lsc_en())
            .field("demosaic_en", &self.demosaic_en())
            .field("median_en", &self.median_en())
            .field("ccm_en", &self.ccm_en())
            .field("gamma_en", &self.gamma_en())
            .field("rgb2yuv_en", &self.rgb2yuv_en())
            .field("sharp_en", &self.sharp_en())
            .field("color_en", &self.color_en())
            .field("yuv2rgb_en", &self.yuv2rgb_en())
            .field("ae_en", &self.ae_en())
            .field("af_en", &self.af_en())
            .field("awb_en", &self.awb_en())
            .field("hist_en", &self.hist_en())
            .field("byte_endian_order", &self.byte_endian_order())
            .field("isp_data_type", &self.isp_data_type())
            .field("isp_in_src", &self.isp_in_src())
            .field("isp_out_type", &self.isp_out_type())
            .finish()
    }
}
impl W {
    ///Bit 0 - this bit configures mipi input data enable. 0: disable, 1: enable
    #[inline(always)]
    #[must_use]
    pub fn mipi_data_en(&mut self) -> MIPI_DATA_EN_W<CNTL_SPEC> {
        MIPI_DATA_EN_W::new(self, 0)
    }
    ///Bit 1 - this bit configures isp global enable. 0: disable, 1: enable
    #[inline(always)]
    #[must_use]
    pub fn isp_en(&mut self) -> ISP_EN_W<CNTL_SPEC> {
        ISP_EN_W::new(self, 1)
    }
    ///Bit 2 - this bit configures blc enable. 0: disable, 1: enable
    #[inline(always)]
    #[must_use]
    pub fn blc_en(&mut self) -> BLC_EN_W<CNTL_SPEC> {
        BLC_EN_W::new(self, 2)
    }
    ///Bit 3 - this bit configures dpc enable. 0: disable, 1: enable
    #[inline(always)]
    #[must_use]
    pub fn dpc_en(&mut self) -> DPC_EN_W<CNTL_SPEC> {
        DPC_EN_W::new(self, 3)
    }
    ///Bit 4 - this bit configures bf enable. 0: disable, 1: enable
    #[inline(always)]
    #[must_use]
    pub fn bf_en(&mut self) -> BF_EN_W<CNTL_SPEC> {
        BF_EN_W::new(self, 4)
    }
    ///Bit 5 - this bit configures lsc enable. 0: disable, 1: enable
    #[inline(always)]
    #[must_use]
    pub fn lsc_en(&mut self) -> LSC_EN_W<CNTL_SPEC> {
        LSC_EN_W::new(self, 5)
    }
    ///Bit 6 - this bit configures demosaic enable. 0: disable, 1: enable
    #[inline(always)]
    #[must_use]
    pub fn demosaic_en(&mut self) -> DEMOSAIC_EN_W<CNTL_SPEC> {
        DEMOSAIC_EN_W::new(self, 6)
    }
    ///Bit 7 - this bit configures median enable. 0: disable, 1: enable
    #[inline(always)]
    #[must_use]
    pub fn median_en(&mut self) -> MEDIAN_EN_W<CNTL_SPEC> {
        MEDIAN_EN_W::new(self, 7)
    }
    ///Bit 8 - this bit configures ccm enable. 0: disable, 1: enable
    #[inline(always)]
    #[must_use]
    pub fn ccm_en(&mut self) -> CCM_EN_W<CNTL_SPEC> {
        CCM_EN_W::new(self, 8)
    }
    ///Bit 9 - this bit configures gamma enable. 0: disable, 1: enable
    #[inline(always)]
    #[must_use]
    pub fn gamma_en(&mut self) -> GAMMA_EN_W<CNTL_SPEC> {
        GAMMA_EN_W::new(self, 9)
    }
    ///Bit 10 - this bit configures rgb2yuv enable. 0: disable, 1: enable
    #[inline(always)]
    #[must_use]
    pub fn rgb2yuv_en(&mut self) -> RGB2YUV_EN_W<CNTL_SPEC> {
        RGB2YUV_EN_W::new(self, 10)
    }
    ///Bit 11 - this bit configures sharp enable. 0: disable, 1: enable
    #[inline(always)]
    #[must_use]
    pub fn sharp_en(&mut self) -> SHARP_EN_W<CNTL_SPEC> {
        SHARP_EN_W::new(self, 11)
    }
    ///Bit 12 - this bit configures color enable. 0: disable, 1: enable
    #[inline(always)]
    #[must_use]
    pub fn color_en(&mut self) -> COLOR_EN_W<CNTL_SPEC> {
        COLOR_EN_W::new(self, 12)
    }
    ///Bit 13 - this bit configures yuv2rgb enable. 0: disable, 1: enable
    #[inline(always)]
    #[must_use]
    pub fn yuv2rgb_en(&mut self) -> YUV2RGB_EN_W<CNTL_SPEC> {
        YUV2RGB_EN_W::new(self, 13)
    }
    ///Bit 14 - this bit configures ae enable. 0: disable, 1: enable
    #[inline(always)]
    #[must_use]
    pub fn ae_en(&mut self) -> AE_EN_W<CNTL_SPEC> {
        AE_EN_W::new(self, 14)
    }
    ///Bit 15 - this bit configures af enable. 0: disable, 1: enable
    #[inline(always)]
    #[must_use]
    pub fn af_en(&mut self) -> AF_EN_W<CNTL_SPEC> {
        AF_EN_W::new(self, 15)
    }
    ///Bit 16 - this bit configures awb enable. 0: disable, 1: enable
    #[inline(always)]
    #[must_use]
    pub fn awb_en(&mut self) -> AWB_EN_W<CNTL_SPEC> {
        AWB_EN_W::new(self, 16)
    }
    ///Bit 17 - this bit configures hist enable. 0: disable, 1: enable
    #[inline(always)]
    #[must_use]
    pub fn hist_en(&mut self) -> HIST_EN_W<CNTL_SPEC> {
        HIST_EN_W::new(self, 17)
    }
    ///Bit 24 - select input idi data byte_endian_order when isp is bypass, 0: csi_data\[31:0\], 1: {\[7:0\], \[15:8\], \[23:16\], \[31:24\]}
    #[inline(always)]
    #[must_use]
    pub fn byte_endian_order(&mut self) -> BYTE_ENDIAN_ORDER_W<CNTL_SPEC> {
        BYTE_ENDIAN_ORDER_W::new(self, 24)
    }
    ///Bits 25:26 - this field configures input data type, 0:RAW8 1:RAW10 2:RAW12
    #[inline(always)]
    #[must_use]
    pub fn isp_data_type(&mut self) -> ISP_DATA_TYPE_W<CNTL_SPEC> {
        ISP_DATA_TYPE_W::new(self, 25)
    }
    ///Bits 27:28 - this field configures input data source, 0:CSI HOST 1:CAM 2:DMA
    #[inline(always)]
    #[must_use]
    pub fn isp_in_src(&mut self) -> ISP_IN_SRC_W<CNTL_SPEC> {
        ISP_IN_SRC_W::new(self, 27)
    }
    ///Bits 29:31 - this field configures pixel output type, 0: RAW8 1: YUV422 2: RGB888 3: YUV420 4: RGB565
    #[inline(always)]
    #[must_use]
    pub fn isp_out_type(&mut self) -> ISP_OUT_TYPE_W<CNTL_SPEC> {
        ISP_OUT_TYPE_W::new(self, 29)
    }
}
/**isp module enable control register

You can [`read`](crate::generic::Reg::read) this register and get [`cntl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cntl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CNTL_SPEC;
impl crate::RegisterSpec for CNTL_SPEC {
    type Ux = u32;
}
///`read()` method returns [`cntl::R`](R) reader structure
impl crate::Readable for CNTL_SPEC {}
///`write(|w| ..)` method takes [`cntl::W`](W) writer structure
impl crate::Writable for CNTL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CNTL to value 0x4000_2442
impl crate::Resettable for CNTL_SPEC {
    const RESET_VALUE: u32 = 0x4000_2442;
}
