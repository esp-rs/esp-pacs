#[doc = "Register `BITSCRAMBLER_PERI_SEL` reader"]
pub type R = crate::R<BITSCRAMBLER_PERI_SEL_SPEC>;
#[doc = "Register `BITSCRAMBLER_PERI_SEL` writer"]
pub type W = crate::W<BITSCRAMBLER_PERI_SEL_SPEC>;
#[doc = "Field `BITSCRAMBLER_PERI_RX_SEL` reader - Set this field to sel peri with DMA RX interface to connec with bitscrambler: 4'h0 : lcd_cam, 4'h1: gpspi2, 4'h2: gpspi3, 4'h3: parl_io, 4'h4: aes, 4'h5: sha, 4'h6: adc, 4'h7: i2s0, 4'h8: i2s1, 4'h9: i2s2, 4'ha: i3c_mst, 4'hb: uhci0, 4'hc: RMT, else : none"]
pub type BITSCRAMBLER_PERI_RX_SEL_R = crate::FieldReader;
#[doc = "Field `BITSCRAMBLER_PERI_RX_SEL` writer - Set this field to sel peri with DMA RX interface to connec with bitscrambler: 4'h0 : lcd_cam, 4'h1: gpspi2, 4'h2: gpspi3, 4'h3: parl_io, 4'h4: aes, 4'h5: sha, 4'h6: adc, 4'h7: i2s0, 4'h8: i2s1, 4'h9: i2s2, 4'ha: i3c_mst, 4'hb: uhci0, 4'hc: RMT, else : none"]
pub type BITSCRAMBLER_PERI_RX_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `BITSCRAMBLER_PERI_TX_SEL` reader - Set this field to sel peri with DMA TX interface to connec with bitscrambler: 4'h0 : lcd_cam, 4'h1: gpspi2, 4'h2: gpspi3, 4'h3: parl_io, 4'h4: aes, 4'h5: sha, 4'h6: adc, 4'h7: i2s0, 4'h8: i2s1, 4'h9: i2s2, 4'ha: i3c_mst, 4'hb: uhci0, 4'hc: RMT, else : none"]
pub type BITSCRAMBLER_PERI_TX_SEL_R = crate::FieldReader;
#[doc = "Field `BITSCRAMBLER_PERI_TX_SEL` writer - Set this field to sel peri with DMA TX interface to connec with bitscrambler: 4'h0 : lcd_cam, 4'h1: gpspi2, 4'h2: gpspi3, 4'h3: parl_io, 4'h4: aes, 4'h5: sha, 4'h6: adc, 4'h7: i2s0, 4'h8: i2s1, 4'h9: i2s2, 4'ha: i3c_mst, 4'hb: uhci0, 4'hc: RMT, else : none"]
pub type BITSCRAMBLER_PERI_TX_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Set this field to sel peri with DMA RX interface to connec with bitscrambler: 4'h0 : lcd_cam, 4'h1: gpspi2, 4'h2: gpspi3, 4'h3: parl_io, 4'h4: aes, 4'h5: sha, 4'h6: adc, 4'h7: i2s0, 4'h8: i2s1, 4'h9: i2s2, 4'ha: i3c_mst, 4'hb: uhci0, 4'hc: RMT, else : none"]
    #[inline(always)]
    pub fn bitscrambler_peri_rx_sel(&self) -> BITSCRAMBLER_PERI_RX_SEL_R {
        BITSCRAMBLER_PERI_RX_SEL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Set this field to sel peri with DMA TX interface to connec with bitscrambler: 4'h0 : lcd_cam, 4'h1: gpspi2, 4'h2: gpspi3, 4'h3: parl_io, 4'h4: aes, 4'h5: sha, 4'h6: adc, 4'h7: i2s0, 4'h8: i2s1, 4'h9: i2s2, 4'ha: i3c_mst, 4'hb: uhci0, 4'hc: RMT, else : none"]
    #[inline(always)]
    pub fn bitscrambler_peri_tx_sel(&self) -> BITSCRAMBLER_PERI_TX_SEL_R {
        BITSCRAMBLER_PERI_TX_SEL_R::new(((self.bits >> 4) & 0x0f) as u8)
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
    #[doc = "Bits 0:3 - Set this field to sel peri with DMA RX interface to connec with bitscrambler: 4'h0 : lcd_cam, 4'h1: gpspi2, 4'h2: gpspi3, 4'h3: parl_io, 4'h4: aes, 4'h5: sha, 4'h6: adc, 4'h7: i2s0, 4'h8: i2s1, 4'h9: i2s2, 4'ha: i3c_mst, 4'hb: uhci0, 4'hc: RMT, else : none"]
    #[inline(always)]
    #[must_use]
    pub fn bitscrambler_peri_rx_sel(
        &mut self,
    ) -> BITSCRAMBLER_PERI_RX_SEL_W<BITSCRAMBLER_PERI_SEL_SPEC> {
        BITSCRAMBLER_PERI_RX_SEL_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - Set this field to sel peri with DMA TX interface to connec with bitscrambler: 4'h0 : lcd_cam, 4'h1: gpspi2, 4'h2: gpspi3, 4'h3: parl_io, 4'h4: aes, 4'h5: sha, 4'h6: adc, 4'h7: i2s0, 4'h8: i2s1, 4'h9: i2s2, 4'ha: i3c_mst, 4'hb: uhci0, 4'hc: RMT, else : none"]
    #[inline(always)]
    #[must_use]
    pub fn bitscrambler_peri_tx_sel(
        &mut self,
    ) -> BITSCRAMBLER_PERI_TX_SEL_W<BITSCRAMBLER_PERI_SEL_SPEC> {
        BITSCRAMBLER_PERI_TX_SEL_W::new(self, 4)
    }
}
#[doc = "Bitscrambler Peri Sel\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bitscrambler_peri_sel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bitscrambler_peri_sel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BITSCRAMBLER_PERI_SEL_SPEC;
impl crate::RegisterSpec for BITSCRAMBLER_PERI_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bitscrambler_peri_sel::R`](R) reader structure"]
impl crate::Readable for BITSCRAMBLER_PERI_SEL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bitscrambler_peri_sel::W`](W) writer structure"]
impl crate::Writable for BITSCRAMBLER_PERI_SEL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BITSCRAMBLER_PERI_SEL to value 0xff"]
impl crate::Resettable for BITSCRAMBLER_PERI_SEL_SPEC {
    const RESET_VALUE: u32 = 0xff;
}
