///Register `RX_TDM_CTRL` reader
pub type R = crate::R<RX_TDM_CTRL_SPEC>;
///Register `RX_TDM_CTRL` writer
pub type W = crate::W<RX_TDM_CTRL_SPEC>;
///Field `RX_TDM_PDM_CHAN0_EN` reader - 1: Enable the valid data input of I2S RX TDM or PDM channel 0. 0: Disable, just input 0 in this channel.
pub type RX_TDM_PDM_CHAN0_EN_R = crate::BitReader;
///Field `RX_TDM_PDM_CHAN0_EN` writer - 1: Enable the valid data input of I2S RX TDM or PDM channel 0. 0: Disable, just input 0 in this channel.
pub type RX_TDM_PDM_CHAN0_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RX_TDM_PDM_CHAN1_EN` reader - 1: Enable the valid data input of I2S RX TDM or PDM channel 1. 0: Disable, just input 0 in this channel.
pub type RX_TDM_PDM_CHAN1_EN_R = crate::BitReader;
///Field `RX_TDM_PDM_CHAN1_EN` writer - 1: Enable the valid data input of I2S RX TDM or PDM channel 1. 0: Disable, just input 0 in this channel.
pub type RX_TDM_PDM_CHAN1_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RX_TDM_TOT_CHAN_NUM` reader - The total channel number of I2S TX TDM mode.
pub type RX_TDM_TOT_CHAN_NUM_R = crate::FieldReader;
///Field `RX_TDM_TOT_CHAN_NUM` writer - The total channel number of I2S TX TDM mode.
pub type RX_TDM_TOT_CHAN_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bit 0 - 1: Enable the valid data input of I2S RX TDM or PDM channel 0. 0: Disable, just input 0 in this channel.
    #[inline(always)]
    pub fn rx_tdm_pdm_chan0_en(&self) -> RX_TDM_PDM_CHAN0_EN_R {
        RX_TDM_PDM_CHAN0_EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - 1: Enable the valid data input of I2S RX TDM or PDM channel 1. 0: Disable, just input 0 in this channel.
    #[inline(always)]
    pub fn rx_tdm_pdm_chan1_en(&self) -> RX_TDM_PDM_CHAN1_EN_R {
        RX_TDM_PDM_CHAN1_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 16:19 - The total channel number of I2S TX TDM mode.
    #[inline(always)]
    pub fn rx_tdm_tot_chan_num(&self) -> RX_TDM_TOT_CHAN_NUM_R {
        RX_TDM_TOT_CHAN_NUM_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RX_TDM_CTRL")
            .field("rx_tdm_pdm_chan0_en", &self.rx_tdm_pdm_chan0_en())
            .field("rx_tdm_pdm_chan1_en", &self.rx_tdm_pdm_chan1_en())
            .field("rx_tdm_tot_chan_num", &self.rx_tdm_tot_chan_num())
            .finish()
    }
}
impl W {
    ///Bit 0 - 1: Enable the valid data input of I2S RX TDM or PDM channel 0. 0: Disable, just input 0 in this channel.
    #[inline(always)]
    #[must_use]
    pub fn rx_tdm_pdm_chan0_en(&mut self) -> RX_TDM_PDM_CHAN0_EN_W<RX_TDM_CTRL_SPEC> {
        RX_TDM_PDM_CHAN0_EN_W::new(self, 0)
    }
    ///Bit 1 - 1: Enable the valid data input of I2S RX TDM or PDM channel 1. 0: Disable, just input 0 in this channel.
    #[inline(always)]
    #[must_use]
    pub fn rx_tdm_pdm_chan1_en(&mut self) -> RX_TDM_PDM_CHAN1_EN_W<RX_TDM_CTRL_SPEC> {
        RX_TDM_PDM_CHAN1_EN_W::new(self, 1)
    }
    ///Bits 16:19 - The total channel number of I2S TX TDM mode.
    #[inline(always)]
    #[must_use]
    pub fn rx_tdm_tot_chan_num(&mut self) -> RX_TDM_TOT_CHAN_NUM_W<RX_TDM_CTRL_SPEC> {
        RX_TDM_TOT_CHAN_NUM_W::new(self, 16)
    }
}
/**I2S TX TDM mode control register

You can [`read`](crate::generic::Reg::read) this register and get [`rx_tdm_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rx_tdm_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct RX_TDM_CTRL_SPEC;
impl crate::RegisterSpec for RX_TDM_CTRL_SPEC {
    type Ux = u32;
}
///`read()` method returns [`rx_tdm_ctrl::R`](R) reader structure
impl crate::Readable for RX_TDM_CTRL_SPEC {}
///`write(|w| ..)` method takes [`rx_tdm_ctrl::W`](W) writer structure
impl crate::Writable for RX_TDM_CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets RX_TDM_CTRL to value 0x03
impl crate::Resettable for RX_TDM_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x03;
}
