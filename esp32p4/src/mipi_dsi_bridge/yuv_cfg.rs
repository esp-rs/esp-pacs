///Register `YUV_CFG` reader
pub type R = crate::R<YUV_CFG_SPEC>;
///Register `YUV_CFG` writer
pub type W = crate::W<YUV_CFG_SPEC>;
///Field `PROTOCAL` reader - this bit configures yuv protoocl, 0: bt.601, 1: bt.709
pub type PROTOCAL_R = crate::BitReader;
///Field `PROTOCAL` writer - this bit configures yuv protoocl, 0: bt.601, 1: bt.709
pub type PROTOCAL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `YUV_PIX_ENDIAN` reader - this bit configures yuv pixel endian, 0: y0u0y1v1y2u2y3v3, 1: y3u3y2v2y1u1y0v0
pub type YUV_PIX_ENDIAN_R = crate::BitReader;
///Field `YUV_PIX_ENDIAN` writer - this bit configures yuv pixel endian, 0: y0u0y1v1y2u2y3v3, 1: y3u3y2v2y1u1y0v0
pub type YUV_PIX_ENDIAN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `YUV422_FORMAT` reader - this field configures yuv422 store format, 0: yuyv, 1: yvyu, 2: uyvy, 3: vyuy
pub type YUV422_FORMAT_R = crate::FieldReader;
///Field `YUV422_FORMAT` writer - this field configures yuv422 store format, 0: yuyv, 1: yvyu, 2: uyvy, 3: vyuy
pub type YUV422_FORMAT_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bit 0 - this bit configures yuv protoocl, 0: bt.601, 1: bt.709
    #[inline(always)]
    pub fn protocal(&self) -> PROTOCAL_R {
        PROTOCAL_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - this bit configures yuv pixel endian, 0: y0u0y1v1y2u2y3v3, 1: y3u3y2v2y1u1y0v0
    #[inline(always)]
    pub fn yuv_pix_endian(&self) -> YUV_PIX_ENDIAN_R {
        YUV_PIX_ENDIAN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 2:3 - this field configures yuv422 store format, 0: yuyv, 1: yvyu, 2: uyvy, 3: vyuy
    #[inline(always)]
    pub fn yuv422_format(&self) -> YUV422_FORMAT_R {
        YUV422_FORMAT_R::new(((self.bits >> 2) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("YUV_CFG")
            .field("protocal", &self.protocal())
            .field("yuv_pix_endian", &self.yuv_pix_endian())
            .field("yuv422_format", &self.yuv422_format())
            .finish()
    }
}
impl W {
    ///Bit 0 - this bit configures yuv protoocl, 0: bt.601, 1: bt.709
    #[inline(always)]
    #[must_use]
    pub fn protocal(&mut self) -> PROTOCAL_W<YUV_CFG_SPEC> {
        PROTOCAL_W::new(self, 0)
    }
    ///Bit 1 - this bit configures yuv pixel endian, 0: y0u0y1v1y2u2y3v3, 1: y3u3y2v2y1u1y0v0
    #[inline(always)]
    #[must_use]
    pub fn yuv_pix_endian(&mut self) -> YUV_PIX_ENDIAN_W<YUV_CFG_SPEC> {
        YUV_PIX_ENDIAN_W::new(self, 1)
    }
    ///Bits 2:3 - this field configures yuv422 store format, 0: yuyv, 1: yvyu, 2: uyvy, 3: vyuy
    #[inline(always)]
    #[must_use]
    pub fn yuv422_format(&mut self) -> YUV422_FORMAT_W<YUV_CFG_SPEC> {
        YUV422_FORMAT_W::new(self, 2)
    }
}
/**dsi_bridge yuv format config register

You can [`read`](crate::generic::Reg::read) this register and get [`yuv_cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`yuv_cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct YUV_CFG_SPEC;
impl crate::RegisterSpec for YUV_CFG_SPEC {
    type Ux = u32;
}
///`read()` method returns [`yuv_cfg::R`](R) reader structure
impl crate::Readable for YUV_CFG_SPEC {}
///`write(|w| ..)` method takes [`yuv_cfg::W`](W) writer structure
impl crate::Writable for YUV_CFG_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets YUV_CFG to value 0
impl crate::Resettable for YUV_CFG_SPEC {
    const RESET_VALUE: u32 = 0;
}
