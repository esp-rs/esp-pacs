#[doc = "Register `RX_RECOMB_CTRL` reader"]
pub type R = crate::R<RX_RECOMB_CTRL_SPEC>;
#[doc = "Register `RX_RECOMB_CTRL` writer"]
pub type W = crate::W<RX_RECOMB_CTRL_SPEC>;
#[doc = "Field `RX_RECOMB_EN` reader - Set this bit to enable i2s rx data recombination."]
pub type RX_RECOMB_EN_R = crate::BitReader;
#[doc = "Field `RX_RECOMB_EN` writer - Set this bit to enable i2s rx data recombination."]
pub type RX_RECOMB_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_RECOMB_EXT_CH_NUM` reader - The channel number that i2s will extract the data into."]
pub type RX_RECOMB_EXT_CH_NUM_R = crate::FieldReader;
#[doc = "Field `RX_RECOMB_EXT_CH_NUM` writer - The channel number that i2s will extract the data into."]
pub type RX_RECOMB_EXT_CH_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RX_RECOMB_UPDATE` writer - Set this bit to update i2s data recombination configuration, must be performed after changing the config of any recombined-dma-channel."]
pub type RX_RECOMB_UPDATE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Set this bit to enable i2s rx data recombination."]
    #[inline(always)]
    pub fn rx_recomb_en(&self) -> RX_RECOMB_EN_R {
        RX_RECOMB_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - The channel number that i2s will extract the data into."]
    #[inline(always)]
    pub fn rx_recomb_ext_ch_num(&self) -> RX_RECOMB_EXT_CH_NUM_R {
        RX_RECOMB_EXT_CH_NUM_R::new(((self.bits >> 1) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RX_RECOMB_CTRL")
            .field("rx_recomb_en", &self.rx_recomb_en())
            .field("rx_recomb_ext_ch_num", &self.rx_recomb_ext_ch_num())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to enable i2s rx data recombination."]
    #[inline(always)]
    pub fn rx_recomb_en(&mut self) -> RX_RECOMB_EN_W<RX_RECOMB_CTRL_SPEC> {
        RX_RECOMB_EN_W::new(self, 0)
    }
    #[doc = "Bits 1:2 - The channel number that i2s will extract the data into."]
    #[inline(always)]
    pub fn rx_recomb_ext_ch_num(&mut self) -> RX_RECOMB_EXT_CH_NUM_W<RX_RECOMB_CTRL_SPEC> {
        RX_RECOMB_EXT_CH_NUM_W::new(self, 1)
    }
    #[doc = "Bit 31 - Set this bit to update i2s data recombination configuration, must be performed after changing the config of any recombined-dma-channel."]
    #[inline(always)]
    pub fn rx_recomb_update(&mut self) -> RX_RECOMB_UPDATE_W<RX_RECOMB_CTRL_SPEC> {
        RX_RECOMB_UPDATE_W::new(self, 31)
    }
}
#[doc = "I2S RX configure register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_recomb_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_recomb_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RX_RECOMB_CTRL_SPEC;
impl crate::RegisterSpec for RX_RECOMB_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_recomb_ctrl::R`](R) reader structure"]
impl crate::Readable for RX_RECOMB_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rx_recomb_ctrl::W`](W) writer structure"]
impl crate::Writable for RX_RECOMB_CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RX_RECOMB_CTRL to value 0"]
impl crate::Resettable for RX_RECOMB_CTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
