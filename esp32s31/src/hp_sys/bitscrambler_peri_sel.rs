#[doc = "Register `BITSCRAMBLER_PERI_SEL` reader"]
pub type R = crate::R<BITSCRAMBLER_PERI_SEL_SPEC>;
#[doc = "Register `BITSCRAMBLER_PERI_SEL` writer"]
pub type W = crate::W<BITSCRAMBLER_PERI_SEL_SPEC>;
#[doc = "Field `BITSCRAMBLER_PERI_RX_SEL` reader - Set this field to sel peri with DMA RX interface to connec with bitscrambler: lcd_cam: 5'h1, gpspi2: 5'h2: gpspi3: 5'h3, parl_io:5'h4, aes: 5'h5, sha: 5'h6, adc: 5'h7, i2s0_ch0: 5'h8, i2s0_ch1: 5'h9, i2s0_ch2: 5'ha, i2s0_ch3: 5'hb, i2s1_ch0: 5'hc, i2s1_ch1: 5'hd, i2s1_ch2: 5'he, i2s1_ch3: 5'hf, uhci0: 5'h10, RMT: 5'h11, none: else."]
pub type BITSCRAMBLER_PERI_RX_SEL_R = crate::FieldReader;
#[doc = "Field `BITSCRAMBLER_PERI_RX_SEL` writer - Set this field to sel peri with DMA RX interface to connec with bitscrambler: lcd_cam: 5'h1, gpspi2: 5'h2: gpspi3: 5'h3, parl_io:5'h4, aes: 5'h5, sha: 5'h6, adc: 5'h7, i2s0_ch0: 5'h8, i2s0_ch1: 5'h9, i2s0_ch2: 5'ha, i2s0_ch3: 5'hb, i2s1_ch0: 5'hc, i2s1_ch1: 5'hd, i2s1_ch2: 5'he, i2s1_ch3: 5'hf, uhci0: 5'h10, RMT: 5'h11, none: else."]
pub type BITSCRAMBLER_PERI_RX_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `BITSCRAMBLER_PERI_TX_SEL` reader - Set this field to sel peri with DMA TX interface to connec with bitscrambler: lcd_cam: 5'h1, gpspi2: 5'h2: gpspi3: 5'h3, parl_io:5'h4, aes: 5'h5, sha: 5'h6, adc: 5'h7, i2s0_ch0: 5'h8, i2s0_ch1: 5'h9, i2s0_ch2: 5'ha, i2s0_ch3: 5'hb, i2s1_ch0: 5'hc, i2s1_ch1: 5'hd, i2s1_ch2: 5'he, i2s1_ch3: 5'hf, uhci0: 5'h10, RMT: 5'h11, none: else."]
pub type BITSCRAMBLER_PERI_TX_SEL_R = crate::FieldReader;
#[doc = "Field `BITSCRAMBLER_PERI_TX_SEL` writer - Set this field to sel peri with DMA TX interface to connec with bitscrambler: lcd_cam: 5'h1, gpspi2: 5'h2: gpspi3: 5'h3, parl_io:5'h4, aes: 5'h5, sha: 5'h6, adc: 5'h7, i2s0_ch0: 5'h8, i2s0_ch1: 5'h9, i2s0_ch2: 5'ha, i2s0_ch3: 5'hb, i2s1_ch0: 5'hc, i2s1_ch1: 5'hd, i2s1_ch2: 5'he, i2s1_ch3: 5'hf, uhci0: 5'h10, RMT: 5'h11, none: else."]
pub type BITSCRAMBLER_PERI_TX_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Set this field to sel peri with DMA RX interface to connec with bitscrambler: lcd_cam: 5'h1, gpspi2: 5'h2: gpspi3: 5'h3, parl_io:5'h4, aes: 5'h5, sha: 5'h6, adc: 5'h7, i2s0_ch0: 5'h8, i2s0_ch1: 5'h9, i2s0_ch2: 5'ha, i2s0_ch3: 5'hb, i2s1_ch0: 5'hc, i2s1_ch1: 5'hd, i2s1_ch2: 5'he, i2s1_ch3: 5'hf, uhci0: 5'h10, RMT: 5'h11, none: else."]
    #[inline(always)]
    pub fn bitscrambler_peri_rx_sel(&self) -> BITSCRAMBLER_PERI_RX_SEL_R {
        BITSCRAMBLER_PERI_RX_SEL_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - Set this field to sel peri with DMA TX interface to connec with bitscrambler: lcd_cam: 5'h1, gpspi2: 5'h2: gpspi3: 5'h3, parl_io:5'h4, aes: 5'h5, sha: 5'h6, adc: 5'h7, i2s0_ch0: 5'h8, i2s0_ch1: 5'h9, i2s0_ch2: 5'ha, i2s0_ch3: 5'hb, i2s1_ch0: 5'hc, i2s1_ch1: 5'hd, i2s1_ch2: 5'he, i2s1_ch3: 5'hf, uhci0: 5'h10, RMT: 5'h11, none: else."]
    #[inline(always)]
    pub fn bitscrambler_peri_tx_sel(&self) -> BITSCRAMBLER_PERI_TX_SEL_R {
        BITSCRAMBLER_PERI_TX_SEL_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BITSCRAMBLER_PERI_SEL")
            .field("bitscrambler_peri_rx_sel", &self.bitscrambler_peri_rx_sel())
            .field("bitscrambler_peri_tx_sel", &self.bitscrambler_peri_tx_sel())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:4 - Set this field to sel peri with DMA RX interface to connec with bitscrambler: lcd_cam: 5'h1, gpspi2: 5'h2: gpspi3: 5'h3, parl_io:5'h4, aes: 5'h5, sha: 5'h6, adc: 5'h7, i2s0_ch0: 5'h8, i2s0_ch1: 5'h9, i2s0_ch2: 5'ha, i2s0_ch3: 5'hb, i2s1_ch0: 5'hc, i2s1_ch1: 5'hd, i2s1_ch2: 5'he, i2s1_ch3: 5'hf, uhci0: 5'h10, RMT: 5'h11, none: else."]
    #[inline(always)]
    pub fn bitscrambler_peri_rx_sel(
        &mut self,
    ) -> BITSCRAMBLER_PERI_RX_SEL_W<'_, BITSCRAMBLER_PERI_SEL_SPEC> {
        BITSCRAMBLER_PERI_RX_SEL_W::new(self, 0)
    }
    #[doc = "Bits 5:9 - Set this field to sel peri with DMA TX interface to connec with bitscrambler: lcd_cam: 5'h1, gpspi2: 5'h2: gpspi3: 5'h3, parl_io:5'h4, aes: 5'h5, sha: 5'h6, adc: 5'h7, i2s0_ch0: 5'h8, i2s0_ch1: 5'h9, i2s0_ch2: 5'ha, i2s0_ch3: 5'hb, i2s1_ch0: 5'hc, i2s1_ch1: 5'hd, i2s1_ch2: 5'he, i2s1_ch3: 5'hf, uhci0: 5'h10, RMT: 5'h11, none: else."]
    #[inline(always)]
    pub fn bitscrambler_peri_tx_sel(
        &mut self,
    ) -> BITSCRAMBLER_PERI_TX_SEL_W<'_, BITSCRAMBLER_PERI_SEL_SPEC> {
        BITSCRAMBLER_PERI_TX_SEL_W::new(self, 5)
    }
}
#[doc = "Bitscrambler Peri Sel\n\nYou can [`read`](crate::Reg::read) this register and get [`bitscrambler_peri_sel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bitscrambler_peri_sel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BITSCRAMBLER_PERI_SEL_SPEC;
impl crate::RegisterSpec for BITSCRAMBLER_PERI_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bitscrambler_peri_sel::R`](R) reader structure"]
impl crate::Readable for BITSCRAMBLER_PERI_SEL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bitscrambler_peri_sel::W`](W) writer structure"]
impl crate::Writable for BITSCRAMBLER_PERI_SEL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BITSCRAMBLER_PERI_SEL to value 0"]
impl crate::Resettable for BITSCRAMBLER_PERI_SEL_SPEC {}
