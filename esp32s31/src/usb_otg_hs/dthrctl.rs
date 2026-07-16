#[doc = "Register `DTHRCTL` reader"]
pub type R = crate::R<DTHRCTL_SPEC>;
#[doc = "Register `DTHRCTL` writer"]
pub type W = crate::W<DTHRCTL_SPEC>;
#[doc = "Field `NONISOTHREN` reader - Non-ISO IN Endpoints Threshold Enable. (NonISOThrEn) When this bit is Set, the core enables thresholding for Non Isochronous IN endpoints."]
pub type NONISOTHREN_R = crate::BitReader;
#[doc = "Field `NONISOTHREN` writer - Non-ISO IN Endpoints Threshold Enable. (NonISOThrEn) When this bit is Set, the core enables thresholding for Non Isochronous IN endpoints."]
pub type NONISOTHREN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ISOTHREN` reader - ISO IN Endpoints Threshold Enable. (ISOThrEn) When this bit is Set, the core enables thresholding for isochronous IN endpoints."]
pub type ISOTHREN_R = crate::BitReader;
#[doc = "Field `ISOTHREN` writer - ISO IN Endpoints Threshold Enable. (ISOThrEn) When this bit is Set, the core enables thresholding for isochronous IN endpoints."]
pub type ISOTHREN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXTHRLEN` reader - Transmit Threshold Length (TxThrLen) This field specifies Transmit thresholding size in DWORDS. This also forms the MAC threshold and specifies the amount of data in bytes to be in the corresponding endpoint transmit FIFO, before the core can start transmit on the USB. The threshold length has to be at least eight DWORDS when the value of AHBThrRatio is 2'h00. In case the AHBThrRatio is non zero the application needs to ensure that the AHB Threshold value does not go below the recommended eight DWORD. This field controls both isochronous and non-isochronous IN endpoint thresholds. The recommended value for ThrLen is to be the same as the programmed AHB Burst Length (GAHBCFG.HBstLen). Note: - When OTG_ARCHITECTURE=0, the reset value of this register field is 0. - When OTG_ARCHITECTURE=2, the reset value of this register field is 8."]
pub type TXTHRLEN_R = crate::FieldReader<u16>;
#[doc = "Field `TXTHRLEN` writer - Transmit Threshold Length (TxThrLen) This field specifies Transmit thresholding size in DWORDS. This also forms the MAC threshold and specifies the amount of data in bytes to be in the corresponding endpoint transmit FIFO, before the core can start transmit on the USB. The threshold length has to be at least eight DWORDS when the value of AHBThrRatio is 2'h00. In case the AHBThrRatio is non zero the application needs to ensure that the AHB Threshold value does not go below the recommended eight DWORD. This field controls both isochronous and non-isochronous IN endpoint thresholds. The recommended value for ThrLen is to be the same as the programmed AHB Burst Length (GAHBCFG.HBstLen). Note: - When OTG_ARCHITECTURE=0, the reset value of this register field is 0. - When OTG_ARCHITECTURE=2, the reset value of this register field is 8."]
pub type TXTHRLEN_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `AHBTHRRATIO` reader - AHB Threshold Ratio (AHBThrRatio) These bits define the ratio between the AHB threshold and the MAC threshold for the transmit path only. The AHB threshold always remains less than or equal to the USB threshold, because this does not increase overhead. Both the AHB and the MAC threshold must be DWORD-aligned. The application needs to program TxThrLen and the AHBThrRatio to make the AHB Threshold value DWORD aligned. If the AHB threshold value is not DWORD aligned, the core might not behave correctly. When programming the TxThrLen and AHBThrRatio, the application must ensure that the minimum AHB threshold value does not go below 8 DWORDS to meet the USB turnaround time requirements. - 2'b00: AHB threshold = MAC threshold - 2'b01: AHB threshold = MAC threshold / 2 - 2'b10: AHB threshold = MAC threshold / 4 - 2'b11: AHB threshold = MAC threshold / 8"]
pub type AHBTHRRATIO_R = crate::FieldReader;
#[doc = "Field `AHBTHRRATIO` writer - AHB Threshold Ratio (AHBThrRatio) These bits define the ratio between the AHB threshold and the MAC threshold for the transmit path only. The AHB threshold always remains less than or equal to the USB threshold, because this does not increase overhead. Both the AHB and the MAC threshold must be DWORD-aligned. The application needs to program TxThrLen and the AHBThrRatio to make the AHB Threshold value DWORD aligned. If the AHB threshold value is not DWORD aligned, the core might not behave correctly. When programming the TxThrLen and AHBThrRatio, the application must ensure that the minimum AHB threshold value does not go below 8 DWORDS to meet the USB turnaround time requirements. - 2'b00: AHB threshold = MAC threshold - 2'b01: AHB threshold = MAC threshold / 2 - 2'b10: AHB threshold = MAC threshold / 4 - 2'b11: AHB threshold = MAC threshold / 8"]
pub type AHBTHRRATIO_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RXTHREN` reader - Receive Threshold Enable (RxThrEn) When this bit is set, the core enables thresholding in the receive direction. Note: We recommends that you do not enable RxThrEn, because it may cause issues in the RxFIFO especially during error conditions such as RxError and Babble."]
pub type RXTHREN_R = crate::BitReader;
#[doc = "Field `RXTHREN` writer - Receive Threshold Enable (RxThrEn) When this bit is set, the core enables thresholding in the receive direction. Note: We recommends that you do not enable RxThrEn, because it may cause issues in the RxFIFO especially during error conditions such as RxError and Babble."]
pub type RXTHREN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXTHRLEN` reader - Receive Threshold Length (RxThrLen) This field specifies Receive thresholding size in DWORDS. This field also specifies the amount of data received on the USB before the core can start transmitting on the AHB. The threshold length has to be at least eight DWORDS. The recommended value for ThrLen is to be the same as the programmed AHB Burst Length (GAHBCFG.HBstLen)."]
pub type RXTHRLEN_R = crate::FieldReader<u16>;
#[doc = "Field `RXTHRLEN` writer - Receive Threshold Length (RxThrLen) This field specifies Receive thresholding size in DWORDS. This field also specifies the amount of data received on the USB before the core can start transmitting on the AHB. The threshold length has to be at least eight DWORDS. The recommended value for ThrLen is to be the same as the programmed AHB Burst Length (GAHBCFG.HBstLen)."]
pub type RXTHRLEN_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `ARBPRKEN` reader - Arbiter Parking Enable (ArbPrkEn) This bit controls internal DMA arbiter parking for IN endpoints. If thresholding is enabled and this bit is set to one, then the arbiter parks on the IN endpoint for which there is a token received on the USB. This is done to avoid getting into underrun conditions. By default, arbiter parking is enabled."]
pub type ARBPRKEN_R = crate::BitReader;
#[doc = "Field `ARBPRKEN` writer - Arbiter Parking Enable (ArbPrkEn) This bit controls internal DMA arbiter parking for IN endpoints. If thresholding is enabled and this bit is set to one, then the arbiter parks on the IN endpoint for which there is a token received on the USB. This is done to avoid getting into underrun conditions. By default, arbiter parking is enabled."]
pub type ARBPRKEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Non-ISO IN Endpoints Threshold Enable. (NonISOThrEn) When this bit is Set, the core enables thresholding for Non Isochronous IN endpoints."]
    #[inline(always)]
    pub fn nonisothren(&self) -> NONISOTHREN_R {
        NONISOTHREN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ISO IN Endpoints Threshold Enable. (ISOThrEn) When this bit is Set, the core enables thresholding for isochronous IN endpoints."]
    #[inline(always)]
    pub fn isothren(&self) -> ISOTHREN_R {
        ISOTHREN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:10 - Transmit Threshold Length (TxThrLen) This field specifies Transmit thresholding size in DWORDS. This also forms the MAC threshold and specifies the amount of data in bytes to be in the corresponding endpoint transmit FIFO, before the core can start transmit on the USB. The threshold length has to be at least eight DWORDS when the value of AHBThrRatio is 2'h00. In case the AHBThrRatio is non zero the application needs to ensure that the AHB Threshold value does not go below the recommended eight DWORD. This field controls both isochronous and non-isochronous IN endpoint thresholds. The recommended value for ThrLen is to be the same as the programmed AHB Burst Length (GAHBCFG.HBstLen). Note: - When OTG_ARCHITECTURE=0, the reset value of this register field is 0. - When OTG_ARCHITECTURE=2, the reset value of this register field is 8."]
    #[inline(always)]
    pub fn txthrlen(&self) -> TXTHRLEN_R {
        TXTHRLEN_R::new(((self.bits >> 2) & 0x01ff) as u16)
    }
    #[doc = "Bits 11:12 - AHB Threshold Ratio (AHBThrRatio) These bits define the ratio between the AHB threshold and the MAC threshold for the transmit path only. The AHB threshold always remains less than or equal to the USB threshold, because this does not increase overhead. Both the AHB and the MAC threshold must be DWORD-aligned. The application needs to program TxThrLen and the AHBThrRatio to make the AHB Threshold value DWORD aligned. If the AHB threshold value is not DWORD aligned, the core might not behave correctly. When programming the TxThrLen and AHBThrRatio, the application must ensure that the minimum AHB threshold value does not go below 8 DWORDS to meet the USB turnaround time requirements. - 2'b00: AHB threshold = MAC threshold - 2'b01: AHB threshold = MAC threshold / 2 - 2'b10: AHB threshold = MAC threshold / 4 - 2'b11: AHB threshold = MAC threshold / 8"]
    #[inline(always)]
    pub fn ahbthrratio(&self) -> AHBTHRRATIO_R {
        AHBTHRRATIO_R::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bit 16 - Receive Threshold Enable (RxThrEn) When this bit is set, the core enables thresholding in the receive direction. Note: We recommends that you do not enable RxThrEn, because it may cause issues in the RxFIFO especially during error conditions such as RxError and Babble."]
    #[inline(always)]
    pub fn rxthren(&self) -> RXTHREN_R {
        RXTHREN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:25 - Receive Threshold Length (RxThrLen) This field specifies Receive thresholding size in DWORDS. This field also specifies the amount of data received on the USB before the core can start transmitting on the AHB. The threshold length has to be at least eight DWORDS. The recommended value for ThrLen is to be the same as the programmed AHB Burst Length (GAHBCFG.HBstLen)."]
    #[inline(always)]
    pub fn rxthrlen(&self) -> RXTHRLEN_R {
        RXTHRLEN_R::new(((self.bits >> 17) & 0x01ff) as u16)
    }
    #[doc = "Bit 27 - Arbiter Parking Enable (ArbPrkEn) This bit controls internal DMA arbiter parking for IN endpoints. If thresholding is enabled and this bit is set to one, then the arbiter parks on the IN endpoint for which there is a token received on the USB. This is done to avoid getting into underrun conditions. By default, arbiter parking is enabled."]
    #[inline(always)]
    pub fn arbprken(&self) -> ARBPRKEN_R {
        ARBPRKEN_R::new(((self.bits >> 27) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DTHRCTL")
            .field("nonisothren", &self.nonisothren())
            .field("isothren", &self.isothren())
            .field("txthrlen", &self.txthrlen())
            .field("ahbthrratio", &self.ahbthrratio())
            .field("rxthren", &self.rxthren())
            .field("rxthrlen", &self.rxthrlen())
            .field("arbprken", &self.arbprken())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Non-ISO IN Endpoints Threshold Enable. (NonISOThrEn) When this bit is Set, the core enables thresholding for Non Isochronous IN endpoints."]
    #[inline(always)]
    pub fn nonisothren(&mut self) -> NONISOTHREN_W<'_, DTHRCTL_SPEC> {
        NONISOTHREN_W::new(self, 0)
    }
    #[doc = "Bit 1 - ISO IN Endpoints Threshold Enable. (ISOThrEn) When this bit is Set, the core enables thresholding for isochronous IN endpoints."]
    #[inline(always)]
    pub fn isothren(&mut self) -> ISOTHREN_W<'_, DTHRCTL_SPEC> {
        ISOTHREN_W::new(self, 1)
    }
    #[doc = "Bits 2:10 - Transmit Threshold Length (TxThrLen) This field specifies Transmit thresholding size in DWORDS. This also forms the MAC threshold and specifies the amount of data in bytes to be in the corresponding endpoint transmit FIFO, before the core can start transmit on the USB. The threshold length has to be at least eight DWORDS when the value of AHBThrRatio is 2'h00. In case the AHBThrRatio is non zero the application needs to ensure that the AHB Threshold value does not go below the recommended eight DWORD. This field controls both isochronous and non-isochronous IN endpoint thresholds. The recommended value for ThrLen is to be the same as the programmed AHB Burst Length (GAHBCFG.HBstLen). Note: - When OTG_ARCHITECTURE=0, the reset value of this register field is 0. - When OTG_ARCHITECTURE=2, the reset value of this register field is 8."]
    #[inline(always)]
    pub fn txthrlen(&mut self) -> TXTHRLEN_W<'_, DTHRCTL_SPEC> {
        TXTHRLEN_W::new(self, 2)
    }
    #[doc = "Bits 11:12 - AHB Threshold Ratio (AHBThrRatio) These bits define the ratio between the AHB threshold and the MAC threshold for the transmit path only. The AHB threshold always remains less than or equal to the USB threshold, because this does not increase overhead. Both the AHB and the MAC threshold must be DWORD-aligned. The application needs to program TxThrLen and the AHBThrRatio to make the AHB Threshold value DWORD aligned. If the AHB threshold value is not DWORD aligned, the core might not behave correctly. When programming the TxThrLen and AHBThrRatio, the application must ensure that the minimum AHB threshold value does not go below 8 DWORDS to meet the USB turnaround time requirements. - 2'b00: AHB threshold = MAC threshold - 2'b01: AHB threshold = MAC threshold / 2 - 2'b10: AHB threshold = MAC threshold / 4 - 2'b11: AHB threshold = MAC threshold / 8"]
    #[inline(always)]
    pub fn ahbthrratio(&mut self) -> AHBTHRRATIO_W<'_, DTHRCTL_SPEC> {
        AHBTHRRATIO_W::new(self, 11)
    }
    #[doc = "Bit 16 - Receive Threshold Enable (RxThrEn) When this bit is set, the core enables thresholding in the receive direction. Note: We recommends that you do not enable RxThrEn, because it may cause issues in the RxFIFO especially during error conditions such as RxError and Babble."]
    #[inline(always)]
    pub fn rxthren(&mut self) -> RXTHREN_W<'_, DTHRCTL_SPEC> {
        RXTHREN_W::new(self, 16)
    }
    #[doc = "Bits 17:25 - Receive Threshold Length (RxThrLen) This field specifies Receive thresholding size in DWORDS. This field also specifies the amount of data received on the USB before the core can start transmitting on the AHB. The threshold length has to be at least eight DWORDS. The recommended value for ThrLen is to be the same as the programmed AHB Burst Length (GAHBCFG.HBstLen)."]
    #[inline(always)]
    pub fn rxthrlen(&mut self) -> RXTHRLEN_W<'_, DTHRCTL_SPEC> {
        RXTHRLEN_W::new(self, 17)
    }
    #[doc = "Bit 27 - Arbiter Parking Enable (ArbPrkEn) This bit controls internal DMA arbiter parking for IN endpoints. If thresholding is enabled and this bit is set to one, then the arbiter parks on the IN endpoint for which there is a token received on the USB. This is done to avoid getting into underrun conditions. By default, arbiter parking is enabled."]
    #[inline(always)]
    pub fn arbprken(&mut self) -> ARBPRKEN_W<'_, DTHRCTL_SPEC> {
        ARBPRKEN_W::new(self, 27)
    }
}
#[doc = "This register contains the Receive and Transmit Threshold characteristics of the Device controller.\n\nYou can [`read`](crate::Reg::read) this register and get [`dthrctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dthrctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DTHRCTL_SPEC;
impl crate::RegisterSpec for DTHRCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dthrctl::R`](R) reader structure"]
impl crate::Readable for DTHRCTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dthrctl::W`](W) writer structure"]
impl crate::Writable for DTHRCTL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DTHRCTL to value 0x0810_0020"]
impl crate::Resettable for DTHRCTL_SPEC {
    const RESET_VALUE: u32 = 0x0810_0020;
}
