#[doc = "Register `CHANNEL` reader"]
pub type R = crate::R<CHANNEL_SPEC>;
#[doc = "Register `CHANNEL` writer"]
pub type W = crate::W<CHANNEL_SPEC>;
#[doc = "Field `HOP` reader - "]
pub type HOP_R = crate::FieldReader;
#[doc = "Field `HOP` writer - "]
pub type HOP_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SEC_HOP` reader - "]
pub type SEC_HOP_R = crate::FieldReader;
#[doc = "Field `SEC_HOP` writer - "]
pub type SEC_HOP_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RX_MULTI_CH_EN` reader - "]
pub type RX_MULTI_CH_EN_R = crate::BitReader;
#[doc = "Field `RX_MULTI_CH_EN` writer - "]
pub type RX_MULTI_CH_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_WAIT_PREMABLE_TIME` reader - "]
pub type RX_WAIT_PREMABLE_TIME_R = crate::FieldReader;
#[doc = "Field `RX_WAIT_PREMABLE_TIME` writer - "]
pub type RX_WAIT_PREMABLE_TIME_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `RX_PREAMBLE_PATTERN_SEL` reader - "]
pub type RX_PREAMBLE_PATTERN_SEL_R = crate::FieldReader;
#[doc = "Field `RX_PREAMBLE_PATTERN_SEL` writer - "]
pub type RX_PREAMBLE_PATTERN_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PRI_CH_I2C_SEL` reader - "]
pub type PRI_CH_I2C_SEL_R = crate::BitReader;
#[doc = "Field `PRI_CH_I2C_SEL` writer - "]
pub type PRI_CH_I2C_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEC_CH_I2C_SEL` reader - "]
pub type SEC_CH_I2C_SEL_R = crate::BitReader;
#[doc = "Field `SEC_CH_I2C_SEL` writer - "]
pub type SEC_CH_I2C_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FAST_FREQHOP_ENA` reader - "]
pub type FAST_FREQHOP_ENA_R = crate::BitReader;
#[doc = "Field `FAST_FREQHOP_ENA` writer - "]
pub type FAST_FREQHOP_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FREQ_BASE_SEL` reader - "]
pub type FREQ_BASE_SEL_R = crate::BitReader;
#[doc = "Field `FREQ_BASE_SEL` writer - "]
pub type FREQ_BASE_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn hop(&self) -> HOP_R {
        HOP_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn sec_hop(&self) -> SEC_HOP_R {
        SEC_HOP_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn rx_multi_ch_en(&self) -> RX_MULTI_CH_EN_R {
        RX_MULTI_CH_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:23"]
    #[inline(always)]
    pub fn rx_wait_premable_time(&self) -> RX_WAIT_PREMABLE_TIME_R {
        RX_WAIT_PREMABLE_TIME_R::new(((self.bits >> 17) & 0x7f) as u8)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn rx_preamble_pattern_sel(&self) -> RX_PREAMBLE_PATTERN_SEL_R {
        RX_PREAMBLE_PATTERN_SEL_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn pri_ch_i2c_sel(&self) -> PRI_CH_I2C_SEL_R {
        PRI_CH_I2C_SEL_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn sec_ch_i2c_sel(&self) -> SEC_CH_I2C_SEL_R {
        SEC_CH_I2C_SEL_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn fast_freqhop_ena(&self) -> FAST_FREQHOP_ENA_R {
        FAST_FREQHOP_ENA_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn freq_base_sel(&self) -> FREQ_BASE_SEL_R {
        FREQ_BASE_SEL_R::new(((self.bits >> 29) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CHANNEL")
            .field("hop", &self.hop())
            .field("sec_hop", &self.sec_hop())
            .field("rx_multi_ch_en", &self.rx_multi_ch_en())
            .field("rx_wait_premable_time", &self.rx_wait_premable_time())
            .field("rx_preamble_pattern_sel", &self.rx_preamble_pattern_sel())
            .field("pri_ch_i2c_sel", &self.pri_ch_i2c_sel())
            .field("sec_ch_i2c_sel", &self.sec_ch_i2c_sel())
            .field("fast_freqhop_ena", &self.fast_freqhop_ena())
            .field("freq_base_sel", &self.freq_base_sel())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn hop(&mut self) -> HOP_W<'_, CHANNEL_SPEC> {
        HOP_W::new(self, 0)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn sec_hop(&mut self) -> SEC_HOP_W<'_, CHANNEL_SPEC> {
        SEC_HOP_W::new(self, 8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn rx_multi_ch_en(&mut self) -> RX_MULTI_CH_EN_W<'_, CHANNEL_SPEC> {
        RX_MULTI_CH_EN_W::new(self, 16)
    }
    #[doc = "Bits 17:23"]
    #[inline(always)]
    pub fn rx_wait_premable_time(&mut self) -> RX_WAIT_PREMABLE_TIME_W<'_, CHANNEL_SPEC> {
        RX_WAIT_PREMABLE_TIME_W::new(self, 17)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn rx_preamble_pattern_sel(&mut self) -> RX_PREAMBLE_PATTERN_SEL_W<'_, CHANNEL_SPEC> {
        RX_PREAMBLE_PATTERN_SEL_W::new(self, 24)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn pri_ch_i2c_sel(&mut self) -> PRI_CH_I2C_SEL_W<'_, CHANNEL_SPEC> {
        PRI_CH_I2C_SEL_W::new(self, 26)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn sec_ch_i2c_sel(&mut self) -> SEC_CH_I2C_SEL_W<'_, CHANNEL_SPEC> {
        SEC_CH_I2C_SEL_W::new(self, 27)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn fast_freqhop_ena(&mut self) -> FAST_FREQHOP_ENA_W<'_, CHANNEL_SPEC> {
        FAST_FREQHOP_ENA_W::new(self, 28)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn freq_base_sel(&mut self) -> FREQ_BASE_SEL_W<'_, CHANNEL_SPEC> {
        FREQ_BASE_SEL_W::new(self, 29)
    }
}
#[doc = "CHANNEL\n\nYou can [`read`](crate::Reg::read) this register and get [`channel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`channel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CHANNEL_SPEC;
impl crate::RegisterSpec for CHANNEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`channel::R`](R) reader structure"]
impl crate::Readable for CHANNEL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`channel::W`](W) writer structure"]
impl crate::Writable for CHANNEL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CHANNEL to value 0"]
impl crate::Resettable for CHANNEL_SPEC {}
