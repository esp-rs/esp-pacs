#[doc = "Register `REGISTER48_ANCONTROLREGISTER` reader"]
pub type R = crate::R<REGISTER48_ANCONTROLREGISTER_SPEC>;
#[doc = "Register `REGISTER48_ANCONTROLREGISTER` writer"]
pub type W = crate::W<REGISTER48_ANCONTROLREGISTER_SPEC>;
#[doc = "Field `RAN` reader - Restart AutoNegotiation When set, this bit causes autonegotiation to restart if Bit 12 _ANE_ is set This bit is selfclearing after autonegotiation starts This bit should be cleared for normal operation"]
pub type RAN_R = crate::BitReader;
#[doc = "Field `RAN` writer - Restart AutoNegotiation When set, this bit causes autonegotiation to restart if Bit 12 _ANE_ is set This bit is selfclearing after autonegotiation starts This bit should be cleared for normal operation"]
pub type RAN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ANE` reader - AutoNegotiation Enable When set, this bit enables the MAC to perform autonegotiation with the link partner Clearing this bit disables the autonegotiation"]
pub type ANE_R = crate::BitReader;
#[doc = "Field `ANE` writer - AutoNegotiation Enable When set, this bit enables the MAC to perform autonegotiation with the link partner Clearing this bit disables the autonegotiation"]
pub type ANE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ELE` reader - External Loopback Enable When set, this bit causes the PHY to loopback the transmit data into the receive path The pcs_ewrap_o signal is asserted high when this bit is set"]
pub type ELE_R = crate::BitReader;
#[doc = "Field `ELE` writer - External Loopback Enable When set, this bit causes the PHY to loopback the transmit data into the receive path The pcs_ewrap_o signal is asserted high when this bit is set"]
pub type ELE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECD` reader - Enable Comma Detect When set, this bit enables the PHY for comma detection and word resynchronization This bit controls the pcs_en_cdet_o signal on the TBI, RTBI, or SGMII interface"]
pub type ECD_R = crate::BitReader;
#[doc = "Field `ECD` writer - Enable Comma Detect When set, this bit enables the PHY for comma detection and word resynchronization This bit controls the pcs_en_cdet_o signal on the TBI, RTBI, or SGMII interface"]
pub type ECD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LR` reader - Lock to Reference When set, this bit enables the PHY to lock its PLL to the 125 MHz reference clock This bit controls the pcs_lck_ref_o signal on the TBI, RTBI, or SGMII interface"]
pub type LR_R = crate::BitReader;
#[doc = "Field `LR` writer - Lock to Reference When set, this bit enables the PHY to lock its PLL to the 125 MHz reference clock This bit controls the pcs_lck_ref_o signal on the TBI, RTBI, or SGMII interface"]
pub type LR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SGMRAL` reader - SGMII RAL Control When set, this bit forces the SGMII RAL block to operate in the speed configured in the Speed and Port Select bits of the MAC Configuration register This is useful when the SGMII interface is used in a direct MAC to MAC connection _without a PHY_ and any MAC must reconfigure the speed When reset, the SGMII RAL block operates according to the link speed status received on SGMII _from the PHY_ This bit is reserved _and RO_ if the SGMII PHY interface is not selected during core configuration"]
pub type SGMRAL_R = crate::BitReader;
#[doc = "Field `SGMRAL` writer - SGMII RAL Control When set, this bit forces the SGMII RAL block to operate in the speed configured in the Speed and Port Select bits of the MAC Configuration register This is useful when the SGMII interface is used in a direct MAC to MAC connection _without a PHY_ and any MAC must reconfigure the speed When reset, the SGMII RAL block operates according to the link speed status received on SGMII _from the PHY_ This bit is reserved _and RO_ if the SGMII PHY interface is not selected during core configuration"]
pub type SGMRAL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 9 - Restart AutoNegotiation When set, this bit causes autonegotiation to restart if Bit 12 _ANE_ is set This bit is selfclearing after autonegotiation starts This bit should be cleared for normal operation"]
    #[inline(always)]
    pub fn ran(&self) -> RAN_R {
        RAN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - AutoNegotiation Enable When set, this bit enables the MAC to perform autonegotiation with the link partner Clearing this bit disables the autonegotiation"]
    #[inline(always)]
    pub fn ane(&self) -> ANE_R {
        ANE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - External Loopback Enable When set, this bit causes the PHY to loopback the transmit data into the receive path The pcs_ewrap_o signal is asserted high when this bit is set"]
    #[inline(always)]
    pub fn ele(&self) -> ELE_R {
        ELE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable Comma Detect When set, this bit enables the PHY for comma detection and word resynchronization This bit controls the pcs_en_cdet_o signal on the TBI, RTBI, or SGMII interface"]
    #[inline(always)]
    pub fn ecd(&self) -> ECD_R {
        ECD_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Lock to Reference When set, this bit enables the PHY to lock its PLL to the 125 MHz reference clock This bit controls the pcs_lck_ref_o signal on the TBI, RTBI, or SGMII interface"]
    #[inline(always)]
    pub fn lr(&self) -> LR_R {
        LR_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - SGMII RAL Control When set, this bit forces the SGMII RAL block to operate in the speed configured in the Speed and Port Select bits of the MAC Configuration register This is useful when the SGMII interface is used in a direct MAC to MAC connection _without a PHY_ and any MAC must reconfigure the speed When reset, the SGMII RAL block operates according to the link speed status received on SGMII _from the PHY_ This bit is reserved _and RO_ if the SGMII PHY interface is not selected during core configuration"]
    #[inline(always)]
    pub fn sgmral(&self) -> SGMRAL_R {
        SGMRAL_R::new(((self.bits >> 18) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REGISTER48_ANCONTROLREGISTER")
            .field("ran", &self.ran())
            .field("ane", &self.ane())
            .field("ele", &self.ele())
            .field("ecd", &self.ecd())
            .field("lr", &self.lr())
            .field("sgmral", &self.sgmral())
            .finish()
    }
}
impl W {
    #[doc = "Bit 9 - Restart AutoNegotiation When set, this bit causes autonegotiation to restart if Bit 12 _ANE_ is set This bit is selfclearing after autonegotiation starts This bit should be cleared for normal operation"]
    #[inline(always)]
    pub fn ran(&mut self) -> RAN_W<'_, REGISTER48_ANCONTROLREGISTER_SPEC> {
        RAN_W::new(self, 9)
    }
    #[doc = "Bit 12 - AutoNegotiation Enable When set, this bit enables the MAC to perform autonegotiation with the link partner Clearing this bit disables the autonegotiation"]
    #[inline(always)]
    pub fn ane(&mut self) -> ANE_W<'_, REGISTER48_ANCONTROLREGISTER_SPEC> {
        ANE_W::new(self, 12)
    }
    #[doc = "Bit 14 - External Loopback Enable When set, this bit causes the PHY to loopback the transmit data into the receive path The pcs_ewrap_o signal is asserted high when this bit is set"]
    #[inline(always)]
    pub fn ele(&mut self) -> ELE_W<'_, REGISTER48_ANCONTROLREGISTER_SPEC> {
        ELE_W::new(self, 14)
    }
    #[doc = "Bit 16 - Enable Comma Detect When set, this bit enables the PHY for comma detection and word resynchronization This bit controls the pcs_en_cdet_o signal on the TBI, RTBI, or SGMII interface"]
    #[inline(always)]
    pub fn ecd(&mut self) -> ECD_W<'_, REGISTER48_ANCONTROLREGISTER_SPEC> {
        ECD_W::new(self, 16)
    }
    #[doc = "Bit 17 - Lock to Reference When set, this bit enables the PHY to lock its PLL to the 125 MHz reference clock This bit controls the pcs_lck_ref_o signal on the TBI, RTBI, or SGMII interface"]
    #[inline(always)]
    pub fn lr(&mut self) -> LR_W<'_, REGISTER48_ANCONTROLREGISTER_SPEC> {
        LR_W::new(self, 17)
    }
    #[doc = "Bit 18 - SGMII RAL Control When set, this bit forces the SGMII RAL block to operate in the speed configured in the Speed and Port Select bits of the MAC Configuration register This is useful when the SGMII interface is used in a direct MAC to MAC connection _without a PHY_ and any MAC must reconfigure the speed When reset, the SGMII RAL block operates according to the link speed status received on SGMII _from the PHY_ This bit is reserved _and RO_ if the SGMII PHY interface is not selected during core configuration"]
    #[inline(always)]
    pub fn sgmral(&mut self) -> SGMRAL_W<'_, REGISTER48_ANCONTROLREGISTER_SPEC> {
        SGMRAL_W::new(self, 18)
    }
}
#[doc = "Enables and/or restarts autonegotiation This register also enables the Physical Coding Sublayer _PCS_ loopback This register is present only when you select the TBI, RTBI, or SGMII interface in coreConsultant\n\nYou can [`read`](crate::Reg::read) this register and get [`register48_ancontrolregister::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register48_ancontrolregister::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REGISTER48_ANCONTROLREGISTER_SPEC;
impl crate::RegisterSpec for REGISTER48_ANCONTROLREGISTER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`register48_ancontrolregister::R`](R) reader structure"]
impl crate::Readable for REGISTER48_ANCONTROLREGISTER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`register48_ancontrolregister::W`](W) writer structure"]
impl crate::Writable for REGISTER48_ANCONTROLREGISTER_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REGISTER48_ANCONTROLREGISTER to value 0"]
impl crate::Resettable for REGISTER48_ANCONTROLREGISTER_SPEC {}
