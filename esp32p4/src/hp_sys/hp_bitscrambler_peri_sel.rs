#[doc = "Register `HP_BITSCRAMBLER_PERI_SEL` reader"]
pub type R = crate::R<HP_BITSCRAMBLER_PERI_SEL_SPEC>;
#[doc = "Register `HP_BITSCRAMBLER_PERI_SEL` writer"]
pub type W = crate::W<HP_BITSCRAMBLER_PERI_SEL_SPEC>;
#[doc = "Field `HP_BITSCRAMBLER_PERI_RX_SEL` reader - Set this field to sel peri with DMA RX interface to connec with bitscrambler: 4'h0 : lcd_cam, 4'h1: gpspi2, 4'h2: gpspi3, 4'h3: parl_io, 4'h4: aes, 4'h5: sha, 4'h6: adc, 4'h7: i2s0, 4'h8: i2s1, 4'h9: i2s2, 4'ha: i3c_mst, 4'hb: uhci0, 4'hc: RMT, else : none"]
pub type HP_BITSCRAMBLER_PERI_RX_SEL_R = crate::FieldReader;
#[doc = "Field `HP_BITSCRAMBLER_PERI_RX_SEL` writer - Set this field to sel peri with DMA RX interface to connec with bitscrambler: 4'h0 : lcd_cam, 4'h1: gpspi2, 4'h2: gpspi3, 4'h3: parl_io, 4'h4: aes, 4'h5: sha, 4'h6: adc, 4'h7: i2s0, 4'h8: i2s1, 4'h9: i2s2, 4'ha: i3c_mst, 4'hb: uhci0, 4'hc: RMT, else : none"]
pub type HP_BITSCRAMBLER_PERI_RX_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `HP_BITSCRAMBLER_PERI_TX_SEL` reader - Set this field to sel peri with DMA TX interface to connec with bitscrambler: 4'h0 : lcd_cam, 4'h1: gpspi2, 4'h2: gpspi3, 4'h3: parl_io, 4'h4: aes, 4'h5: sha, 4'h6: adc, 4'h7: i2s0, 4'h8: i2s1, 4'h9: i2s2, 4'ha: i3c_mst, 4'hb: uhci0, 4'hc: RMT, else : none"]
pub type HP_BITSCRAMBLER_PERI_TX_SEL_R = crate::FieldReader;
#[doc = "Field `HP_BITSCRAMBLER_PERI_TX_SEL` writer - Set this field to sel peri with DMA TX interface to connec with bitscrambler: 4'h0 : lcd_cam, 4'h1: gpspi2, 4'h2: gpspi3, 4'h3: parl_io, 4'h4: aes, 4'h5: sha, 4'h6: adc, 4'h7: i2s0, 4'h8: i2s1, 4'h9: i2s2, 4'ha: i3c_mst, 4'hb: uhci0, 4'hc: RMT, else : none"]
pub type HP_BITSCRAMBLER_PERI_TX_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Set this field to sel peri with DMA RX interface to connec with bitscrambler: 4'h0 : lcd_cam, 4'h1: gpspi2, 4'h2: gpspi3, 4'h3: parl_io, 4'h4: aes, 4'h5: sha, 4'h6: adc, 4'h7: i2s0, 4'h8: i2s1, 4'h9: i2s2, 4'ha: i3c_mst, 4'hb: uhci0, 4'hc: RMT, else : none"]
    #[inline(always)]
    pub fn hp_bitscrambler_peri_rx_sel(&self) -> HP_BITSCRAMBLER_PERI_RX_SEL_R {
        HP_BITSCRAMBLER_PERI_RX_SEL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Set this field to sel peri with DMA TX interface to connec with bitscrambler: 4'h0 : lcd_cam, 4'h1: gpspi2, 4'h2: gpspi3, 4'h3: parl_io, 4'h4: aes, 4'h5: sha, 4'h6: adc, 4'h7: i2s0, 4'h8: i2s1, 4'h9: i2s2, 4'ha: i3c_mst, 4'hb: uhci0, 4'hc: RMT, else : none"]
    #[inline(always)]
    pub fn hp_bitscrambler_peri_tx_sel(&self) -> HP_BITSCRAMBLER_PERI_TX_SEL_R {
        HP_BITSCRAMBLER_PERI_TX_SEL_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HP_BITSCRAMBLER_PERI_SEL")
            .field(
                "hp_bitscrambler_peri_rx_sel",
                &format_args!("{}", self.hp_bitscrambler_peri_rx_sel().bits()),
            )
            .field(
                "hp_bitscrambler_peri_tx_sel",
                &format_args!("{}", self.hp_bitscrambler_peri_tx_sel().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HP_BITSCRAMBLER_PERI_SEL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:3 - Set this field to sel peri with DMA RX interface to connec with bitscrambler: 4'h0 : lcd_cam, 4'h1: gpspi2, 4'h2: gpspi3, 4'h3: parl_io, 4'h4: aes, 4'h5: sha, 4'h6: adc, 4'h7: i2s0, 4'h8: i2s1, 4'h9: i2s2, 4'ha: i3c_mst, 4'hb: uhci0, 4'hc: RMT, else : none"]
    #[inline(always)]
    #[must_use]
    pub fn hp_bitscrambler_peri_rx_sel(
        &mut self,
    ) -> HP_BITSCRAMBLER_PERI_RX_SEL_W<HP_BITSCRAMBLER_PERI_SEL_SPEC> {
        HP_BITSCRAMBLER_PERI_RX_SEL_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - Set this field to sel peri with DMA TX interface to connec with bitscrambler: 4'h0 : lcd_cam, 4'h1: gpspi2, 4'h2: gpspi3, 4'h3: parl_io, 4'h4: aes, 4'h5: sha, 4'h6: adc, 4'h7: i2s0, 4'h8: i2s1, 4'h9: i2s2, 4'ha: i3c_mst, 4'hb: uhci0, 4'hc: RMT, else : none"]
    #[inline(always)]
    #[must_use]
    pub fn hp_bitscrambler_peri_tx_sel(
        &mut self,
    ) -> HP_BITSCRAMBLER_PERI_TX_SEL_W<HP_BITSCRAMBLER_PERI_SEL_SPEC> {
        HP_BITSCRAMBLER_PERI_TX_SEL_W::new(self, 4)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Bitscrambler Peri Sel\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hp_bitscrambler_peri_sel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_bitscrambler_peri_sel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HP_BITSCRAMBLER_PERI_SEL_SPEC;
impl crate::RegisterSpec for HP_BITSCRAMBLER_PERI_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hp_bitscrambler_peri_sel::R`](R) reader structure"]
impl crate::Readable for HP_BITSCRAMBLER_PERI_SEL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hp_bitscrambler_peri_sel::W`](W) writer structure"]
impl crate::Writable for HP_BITSCRAMBLER_PERI_SEL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HP_BITSCRAMBLER_PERI_SEL to value 0xff"]
impl crate::Resettable for HP_BITSCRAMBLER_PERI_SEL_SPEC {
    const RESET_VALUE: Self::Ux = 0xff;
}
