#[doc = "Register `HOST_CM_CTRL` reader"]
pub type R = crate::R<HOST_CM_CTRL_SPEC>;
#[doc = "Register `HOST_CM_CTRL` writer"]
pub type W = crate::W<HOST_CM_CTRL_SPEC>;
#[doc = "Field `CSI_HOST_CM_EN` reader - Configures whether to enable cm output"]
pub type CSI_HOST_CM_EN_R = crate::BitReader;
#[doc = "Field `CSI_HOST_CM_EN` writer - Configures whether to enable cm output"]
pub type CSI_HOST_CM_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSI_HOST_CM_BYPASS` reader - Configures whether to bypass cm"]
pub type CSI_HOST_CM_BYPASS_R = crate::BitReader;
#[doc = "Field `CSI_HOST_CM_BYPASS` writer - Configures whether to bypass cm"]
pub type CSI_HOST_CM_BYPASS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSI_HOST_CM_RX` reader - Configures whether to bypass cm"]
pub type CSI_HOST_CM_RX_R = crate::FieldReader;
#[doc = "Field `CSI_HOST_CM_RX` writer - Configures whether to bypass cm"]
pub type CSI_HOST_CM_RX_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CSI_HOST_CM_RX_RGB_FORMAT` reader - Configures whether to bypass cm"]
pub type CSI_HOST_CM_RX_RGB_FORMAT_R = crate::FieldReader;
#[doc = "Field `CSI_HOST_CM_RX_RGB_FORMAT` writer - Configures whether to bypass cm"]
pub type CSI_HOST_CM_RX_RGB_FORMAT_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CSI_HOST_CM_RX_YUV422_FORMAT` reader - Configures whether to bypass cm"]
pub type CSI_HOST_CM_RX_YUV422_FORMAT_R = crate::FieldReader;
#[doc = "Field `CSI_HOST_CM_RX_YUV422_FORMAT` writer - Configures whether to bypass cm"]
pub type CSI_HOST_CM_RX_YUV422_FORMAT_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CSI_HOST_CM_TX` reader - Configures whether to bypass cm"]
pub type CSI_HOST_CM_TX_R = crate::FieldReader;
#[doc = "Field `CSI_HOST_CM_TX` writer - Configures whether to bypass cm"]
pub type CSI_HOST_CM_TX_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CSI_HOST_CM_LANE_NUM` reader - Configures lane number that csi used, valid only rgb888 to rgb888. 0: 1-lane, 1: 2-lane"]
pub type CSI_HOST_CM_LANE_NUM_R = crate::BitReader;
#[doc = "Field `CSI_HOST_CM_LANE_NUM` writer - Configures lane number that csi used, valid only rgb888 to rgb888. 0: 1-lane, 1: 2-lane"]
pub type CSI_HOST_CM_LANE_NUM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSI_HOST_CM_16BIT_SWAP` reader - Configures whether to swap idi32 high and low 16-bit"]
pub type CSI_HOST_CM_16BIT_SWAP_R = crate::BitReader;
#[doc = "Field `CSI_HOST_CM_16BIT_SWAP` writer - Configures whether to swap idi32 high and low 16-bit"]
pub type CSI_HOST_CM_16BIT_SWAP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSI_HOST_CM_8BIT_SWAP` reader - Configures whether to swap idi32 high and low 8-bit"]
pub type CSI_HOST_CM_8BIT_SWAP_R = crate::BitReader;
#[doc = "Field `CSI_HOST_CM_8BIT_SWAP` writer - Configures whether to swap idi32 high and low 8-bit"]
pub type CSI_HOST_CM_8BIT_SWAP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Configures whether to enable cm output"]
    #[inline(always)]
    pub fn csi_host_cm_en(&self) -> CSI_HOST_CM_EN_R {
        CSI_HOST_CM_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Configures whether to bypass cm"]
    #[inline(always)]
    pub fn csi_host_cm_bypass(&self) -> CSI_HOST_CM_BYPASS_R {
        CSI_HOST_CM_BYPASS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Configures whether to bypass cm"]
    #[inline(always)]
    pub fn csi_host_cm_rx(&self) -> CSI_HOST_CM_RX_R {
        CSI_HOST_CM_RX_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:6 - Configures whether to bypass cm"]
    #[inline(always)]
    pub fn csi_host_cm_rx_rgb_format(&self) -> CSI_HOST_CM_RX_RGB_FORMAT_R {
        CSI_HOST_CM_RX_RGB_FORMAT_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 7:8 - Configures whether to bypass cm"]
    #[inline(always)]
    pub fn csi_host_cm_rx_yuv422_format(&self) -> CSI_HOST_CM_RX_YUV422_FORMAT_R {
        CSI_HOST_CM_RX_YUV422_FORMAT_R::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bits 9:10 - Configures whether to bypass cm"]
    #[inline(always)]
    pub fn csi_host_cm_tx(&self) -> CSI_HOST_CM_TX_R {
        CSI_HOST_CM_TX_R::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bit 11 - Configures lane number that csi used, valid only rgb888 to rgb888. 0: 1-lane, 1: 2-lane"]
    #[inline(always)]
    pub fn csi_host_cm_lane_num(&self) -> CSI_HOST_CM_LANE_NUM_R {
        CSI_HOST_CM_LANE_NUM_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Configures whether to swap idi32 high and low 16-bit"]
    #[inline(always)]
    pub fn csi_host_cm_16bit_swap(&self) -> CSI_HOST_CM_16BIT_SWAP_R {
        CSI_HOST_CM_16BIT_SWAP_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Configures whether to swap idi32 high and low 8-bit"]
    #[inline(always)]
    pub fn csi_host_cm_8bit_swap(&self) -> CSI_HOST_CM_8BIT_SWAP_R {
        CSI_HOST_CM_8BIT_SWAP_R::new(((self.bits >> 13) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HOST_CM_CTRL")
            .field("csi_host_cm_en", &self.csi_host_cm_en())
            .field("csi_host_cm_bypass", &self.csi_host_cm_bypass())
            .field("csi_host_cm_rx", &self.csi_host_cm_rx())
            .field(
                "csi_host_cm_rx_rgb_format",
                &self.csi_host_cm_rx_rgb_format(),
            )
            .field(
                "csi_host_cm_rx_yuv422_format",
                &self.csi_host_cm_rx_yuv422_format(),
            )
            .field("csi_host_cm_tx", &self.csi_host_cm_tx())
            .field("csi_host_cm_lane_num", &self.csi_host_cm_lane_num())
            .field("csi_host_cm_16bit_swap", &self.csi_host_cm_16bit_swap())
            .field("csi_host_cm_8bit_swap", &self.csi_host_cm_8bit_swap())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Configures whether to enable cm output"]
    #[inline(always)]
    pub fn csi_host_cm_en(&mut self) -> CSI_HOST_CM_EN_W<'_, HOST_CM_CTRL_SPEC> {
        CSI_HOST_CM_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Configures whether to bypass cm"]
    #[inline(always)]
    pub fn csi_host_cm_bypass(&mut self) -> CSI_HOST_CM_BYPASS_W<'_, HOST_CM_CTRL_SPEC> {
        CSI_HOST_CM_BYPASS_W::new(self, 1)
    }
    #[doc = "Bits 2:3 - Configures whether to bypass cm"]
    #[inline(always)]
    pub fn csi_host_cm_rx(&mut self) -> CSI_HOST_CM_RX_W<'_, HOST_CM_CTRL_SPEC> {
        CSI_HOST_CM_RX_W::new(self, 2)
    }
    #[doc = "Bits 4:6 - Configures whether to bypass cm"]
    #[inline(always)]
    pub fn csi_host_cm_rx_rgb_format(
        &mut self,
    ) -> CSI_HOST_CM_RX_RGB_FORMAT_W<'_, HOST_CM_CTRL_SPEC> {
        CSI_HOST_CM_RX_RGB_FORMAT_W::new(self, 4)
    }
    #[doc = "Bits 7:8 - Configures whether to bypass cm"]
    #[inline(always)]
    pub fn csi_host_cm_rx_yuv422_format(
        &mut self,
    ) -> CSI_HOST_CM_RX_YUV422_FORMAT_W<'_, HOST_CM_CTRL_SPEC> {
        CSI_HOST_CM_RX_YUV422_FORMAT_W::new(self, 7)
    }
    #[doc = "Bits 9:10 - Configures whether to bypass cm"]
    #[inline(always)]
    pub fn csi_host_cm_tx(&mut self) -> CSI_HOST_CM_TX_W<'_, HOST_CM_CTRL_SPEC> {
        CSI_HOST_CM_TX_W::new(self, 9)
    }
    #[doc = "Bit 11 - Configures lane number that csi used, valid only rgb888 to rgb888. 0: 1-lane, 1: 2-lane"]
    #[inline(always)]
    pub fn csi_host_cm_lane_num(&mut self) -> CSI_HOST_CM_LANE_NUM_W<'_, HOST_CM_CTRL_SPEC> {
        CSI_HOST_CM_LANE_NUM_W::new(self, 11)
    }
    #[doc = "Bit 12 - Configures whether to swap idi32 high and low 16-bit"]
    #[inline(always)]
    pub fn csi_host_cm_16bit_swap(&mut self) -> CSI_HOST_CM_16BIT_SWAP_W<'_, HOST_CM_CTRL_SPEC> {
        CSI_HOST_CM_16BIT_SWAP_W::new(self, 12)
    }
    #[doc = "Bit 13 - Configures whether to swap idi32 high and low 8-bit"]
    #[inline(always)]
    pub fn csi_host_cm_8bit_swap(&mut self) -> CSI_HOST_CM_8BIT_SWAP_W<'_, HOST_CM_CTRL_SPEC> {
        CSI_HOST_CM_8BIT_SWAP_W::new(self, 13)
    }
}
#[doc = "CSI HOST color mode convert configuration.\n\nYou can [`read`](crate::Reg::read) this register and get [`host_cm_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`host_cm_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HOST_CM_CTRL_SPEC;
impl crate::RegisterSpec for HOST_CM_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`host_cm_ctrl::R`](R) reader structure"]
impl crate::Readable for HOST_CM_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`host_cm_ctrl::W`](W) writer structure"]
impl crate::Writable for HOST_CM_CTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HOST_CM_CTRL to value 0x0803"]
impl crate::Resettable for HOST_CM_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x0803;
}
