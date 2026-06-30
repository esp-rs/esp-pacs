#[doc = "Register `GOTGCTL` reader"]
pub type R = crate::R<GOTGCTL_SPEC>;
#[doc = "Register `GOTGCTL` writer"]
pub type W = crate::W<GOTGCTL_SPEC>;
#[doc = "Field `VBVALIDOVEN` reader - Mode: Host only VBUS Valid Override Enable (VbvalidOvEn) This bit is used to enable/disable the software to override the Bvalid signal using the GOTGCTL.VbvalidOvVal. - 1'b1 : Internally Bvalid received from the PHY is overridden with GOTGCTL.VbvalidOvVal. - 1'b0 : Override is disabled and bvalid signal from the respective PHY selected is used internally by the controller."]
pub type VBVALIDOVEN_R = crate::BitReader;
#[doc = "Field `VBVALIDOVEN` writer - Mode: Host only VBUS Valid Override Enable (VbvalidOvEn) This bit is used to enable/disable the software to override the Bvalid signal using the GOTGCTL.VbvalidOvVal. - 1'b1 : Internally Bvalid received from the PHY is overridden with GOTGCTL.VbvalidOvVal. - 1'b0 : Override is disabled and bvalid signal from the respective PHY selected is used internally by the controller."]
pub type VBVALIDOVEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VBVALIDOVVAL` reader - Mode: Host only VBUS Valid OverrideValue (VbvalidOvVal) This bit is used to set Override value for vbusvalid signal when GOTGCTL.VbvalidOvEn is set. - 1'b0 : vbusvalid value is 1'b0 when GOTGCTL.VbvalidOvEn =1 - 1'b1 : vbusvalid value is 1'b1 when GOTGCTL.VbvalidOvEn =1"]
pub type VBVALIDOVVAL_R = crate::BitReader;
#[doc = "Field `VBVALIDOVVAL` writer - Mode: Host only VBUS Valid OverrideValue (VbvalidOvVal) This bit is used to set Override value for vbusvalid signal when GOTGCTL.VbvalidOvEn is set. - 1'b0 : vbusvalid value is 1'b0 when GOTGCTL.VbvalidOvEn =1 - 1'b1 : vbusvalid value is 1'b1 when GOTGCTL.VbvalidOvEn =1"]
pub type VBVALIDOVVAL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AVALIDOVEN` reader - Mode: Host only A-Peripheral Session Valid Override Enable (AvalidOvEn) This bit is used to enable/disable the software to override the Avalid signal using the GOTGCTL.AvalidOvVal. - 1'b1: Internally Avalid received from the PHY is overridden with GOTGCTL.AvalidOvVal. - 1'b0: Override is disabled and avalid signal from the respective PHY selected is used internally by the core"]
pub type AVALIDOVEN_R = crate::BitReader;
#[doc = "Field `AVALIDOVEN` writer - Mode: Host only A-Peripheral Session Valid Override Enable (AvalidOvEn) This bit is used to enable/disable the software to override the Avalid signal using the GOTGCTL.AvalidOvVal. - 1'b1: Internally Avalid received from the PHY is overridden with GOTGCTL.AvalidOvVal. - 1'b0: Override is disabled and avalid signal from the respective PHY selected is used internally by the core"]
pub type AVALIDOVEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AVALIDOVVAL` reader - Mode: Host only A-Peripheral Session Valid OverrideValue (AvalidOvVal) This bit is used to set Override value for Avalid signal when GOTGCTL.AvalidOvEn is set. - 1'b0 : Avalid value is 1'b0 when GOTGCTL.AvalidOvEn =1 - 1'b1 : Avalid value is 1'b1 when GOTGCTL.AvalidOvEn =1"]
pub type AVALIDOVVAL_R = crate::BitReader;
#[doc = "Field `AVALIDOVVAL` writer - Mode: Host only A-Peripheral Session Valid OverrideValue (AvalidOvVal) This bit is used to set Override value for Avalid signal when GOTGCTL.AvalidOvEn is set. - 1'b0 : Avalid value is 1'b0 when GOTGCTL.AvalidOvEn =1 - 1'b1 : Avalid value is 1'b1 when GOTGCTL.AvalidOvEn =1"]
pub type AVALIDOVVAL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BVALIDOVEN` reader - Mode: Device only B-Peripheral Session Valid Override Value (BvalidOvEn) This bit is used to enable/disable the software to override the Bvalid signal using the GOTGCTL.BvalidOvVal. - 1'b1 : Internally Bvalid received from the PHY is overridden with GOTGCTL.BvalidOvVal. - 1'b0 : Override is disabled and bvalid signal from the respective PHY selected is used internally by the force"]
pub type BVALIDOVEN_R = crate::BitReader;
#[doc = "Field `BVALIDOVEN` writer - Mode: Device only B-Peripheral Session Valid Override Value (BvalidOvEn) This bit is used to enable/disable the software to override the Bvalid signal using the GOTGCTL.BvalidOvVal. - 1'b1 : Internally Bvalid received from the PHY is overridden with GOTGCTL.BvalidOvVal. - 1'b0 : Override is disabled and bvalid signal from the respective PHY selected is used internally by the force"]
pub type BVALIDOVEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BVALIDOVVAL` reader - Mode: Device only B-Peripheral Session Valid OverrideValue (BvalidOvVal) This bit is used to set Override value for Bvalid signal when GOTGCTL.BvalidOvEn is set. - 1'b0 : Bvalid value is 1'b0 when GOTGCTL.BvalidOvEn =1 - 1'b1 : Bvalid value is 1'b1 when GOTGCTL.BvalidOvEn =1"]
pub type BVALIDOVVAL_R = crate::BitReader;
#[doc = "Field `BVALIDOVVAL` writer - Mode: Device only B-Peripheral Session Valid OverrideValue (BvalidOvVal) This bit is used to set Override value for Bvalid signal when GOTGCTL.BvalidOvEn is set. - 1'b0 : Bvalid value is 1'b0 when GOTGCTL.BvalidOvEn =1 - 1'b1 : Bvalid value is 1'b1 when GOTGCTL.BvalidOvEn =1"]
pub type BVALIDOVVAL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBNCEFLTRBYPASS` reader - Mode: Host and Device Debounce Filter Bypass Bypass Debounce filters for avalid, bvalid, vbusvalid, sessend, iddig signals when enabled. - 1'b0: Disabled - 1'b1: Enabled Note: This register bit is valid only when debounce filters are present in core."]
pub type DBNCEFLTRBYPASS_R = crate::BitReader;
#[doc = "Field `DBNCEFLTRBYPASS` writer - Mode: Host and Device Debounce Filter Bypass Bypass Debounce filters for avalid, bvalid, vbusvalid, sessend, iddig signals when enabled. - 1'b0: Disabled - 1'b1: Enabled Note: This register bit is valid only when debounce filters are present in core."]
pub type DBNCEFLTRBYPASS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CONIDSTS` reader - Mode: Host and Device Connector ID Status (ConIDSts) Indicates the connector ID status on a connect event. - 1'b0: The core is in A-Device mode. - 1'b1: The core is in B-Device mode. Note: The reset value of this register field can be read only after the PHY clock is stable, or if IDDIG_FILTER is enabled, wait for the filter timer to expire to read the correct reset value which ever event is later. Reset: - 1'b0: in host only mode (OTG_MODE = 5 or 6) - 1'b1: in all other configurations"]
pub type CONIDSTS_R = crate::BitReader;
#[doc = "Field `DBNCTIME` reader - Mode: Host only Long/Short Debounce Time (DbncTime) Indicates the debounce time of a detected connection. - 1'b0: Long debounce time, used for physical connections (100 ms + 2.5 micro-sec) - 1'b1: Short debounce time, used for soft connections (2.5 micro-sec)"]
pub type DBNCTIME_R = crate::BitReader;
#[doc = "Field `ASESVLD` reader - Mode: Host only A-Session Valid (ASesVld) Indicates the Host mode transceiver status. - 1'b0: A-session is not valid - 1'b1: A-session is valid Note: If you do not enabled OTG features (such as SRP and HNP), the read reset value will be 1. The vbus assigns the values internally for non-SRP or non-HNP configurations. In case of OTG_MODE=0, the reset value of this bit is 1'b0."]
pub type ASESVLD_R = crate::BitReader;
#[doc = "Field `BSESVLD` reader - Mode: Device only B-Session Valid (BSesVld) Indicates the Device mode transceiver status. - 1'b0: B-session is not valid. - 1'b1: B-session is valid. In OTG mode, you can use this bit to determine if the device is connected or disconnected. Note: - If you do not enable OTG features (such as SRP and HNP), the read reset value will be 1.The vbus assigns the values internally for non- SRP or non-HNP configurations. - In case of OTG_MODE=0, the reset value of this bit is 1'b0. - The reset value of this register field can be read only after the PHY clock is stable, or if IDDIG_FILTER is enabled, wait for the filter timer to expire to read the correct reset value which ever event is later."]
pub type BSESVLD_R = crate::BitReader;
#[doc = "Field `OTGVER` reader - OTG Version (OTGVer) Indicates the OTG revision. - 1'b0: OTG Version 1.3. In this version the core supports Data line pulsing and VBus pulsing for SRP. - 1'b1: OTG Version 2.0. In this version the core supports only Data line pulsing for SRP."]
pub type OTGVER_R = crate::BitReader;
#[doc = "Field `OTGVER` writer - OTG Version (OTGVer) Indicates the OTG revision. - 1'b0: OTG Version 1.3. In this version the core supports Data line pulsing and VBus pulsing for SRP. - 1'b1: OTG Version 2.0. In this version the core supports only Data line pulsing for SRP."]
pub type OTGVER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CURMOD` reader - Current Mode of Operation (CurMod) Mode: Host and Device Indicates the current mode. - 1'b0: Device mode - 1'b1: Host mode Reset: - 1'b1 in Host-only mode (OTG_MODE=5 or 6) - 1'b0 in all other configurations Note: The reset value of this register field can be read only after the PHY clock is stable, or if IDDIG_FILTER is enabled, wait for the filter timer to expire to read the correct reset value which ever event is later."]
pub type CURMOD_R = crate::BitReader;
#[doc = "Field `MULTVALIDBC` reader - Mode: Host and Device Multi Valued ID pin (MultValIdBC) Battery Charger ACA inputs in the following order: - Bit 26: rid_float. - Bit 25: rid_gnd - Bit 24: rid_a - Bit 23: rid_b - Bit 22: rid_c"]
pub type MULTVALIDBC_R = crate::FieldReader;
#[doc = "Field `CHIRPEN` reader - Mode: Device Only This bit when programmed to 1'b1 results in the core asserting chirp_on before sending an actual Chirp K signal on USB. This bit is present only if OTG_BC_SUPPORT = 1.If OTG_BC_SUPPORT!=1, this bit is a reserved bit. Do not set this bit when core is operating in HSIC mode because HSIC always operates at High Speed and High speed chirp is not used"]
pub type CHIRPEN_R = crate::BitReader;
#[doc = "Field `CHIRPEN` writer - Mode: Device Only This bit when programmed to 1'b1 results in the core asserting chirp_on before sending an actual Chirp K signal on USB. This bit is present only if OTG_BC_SUPPORT = 1.If OTG_BC_SUPPORT!=1, this bit is a reserved bit. Do not set this bit when core is operating in HSIC mode because HSIC always operates at High Speed and High speed chirp is not used"]
pub type CHIRPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 2 - Mode: Host only VBUS Valid Override Enable (VbvalidOvEn) This bit is used to enable/disable the software to override the Bvalid signal using the GOTGCTL.VbvalidOvVal. - 1'b1 : Internally Bvalid received from the PHY is overridden with GOTGCTL.VbvalidOvVal. - 1'b0 : Override is disabled and bvalid signal from the respective PHY selected is used internally by the controller."]
    #[inline(always)]
    pub fn vbvalidoven(&self) -> VBVALIDOVEN_R {
        VBVALIDOVEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Mode: Host only VBUS Valid OverrideValue (VbvalidOvVal) This bit is used to set Override value for vbusvalid signal when GOTGCTL.VbvalidOvEn is set. - 1'b0 : vbusvalid value is 1'b0 when GOTGCTL.VbvalidOvEn =1 - 1'b1 : vbusvalid value is 1'b1 when GOTGCTL.VbvalidOvEn =1"]
    #[inline(always)]
    pub fn vbvalidovval(&self) -> VBVALIDOVVAL_R {
        VBVALIDOVVAL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Mode: Host only A-Peripheral Session Valid Override Enable (AvalidOvEn) This bit is used to enable/disable the software to override the Avalid signal using the GOTGCTL.AvalidOvVal. - 1'b1: Internally Avalid received from the PHY is overridden with GOTGCTL.AvalidOvVal. - 1'b0: Override is disabled and avalid signal from the respective PHY selected is used internally by the core"]
    #[inline(always)]
    pub fn avalidoven(&self) -> AVALIDOVEN_R {
        AVALIDOVEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Mode: Host only A-Peripheral Session Valid OverrideValue (AvalidOvVal) This bit is used to set Override value for Avalid signal when GOTGCTL.AvalidOvEn is set. - 1'b0 : Avalid value is 1'b0 when GOTGCTL.AvalidOvEn =1 - 1'b1 : Avalid value is 1'b1 when GOTGCTL.AvalidOvEn =1"]
    #[inline(always)]
    pub fn avalidovval(&self) -> AVALIDOVVAL_R {
        AVALIDOVVAL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Mode: Device only B-Peripheral Session Valid Override Value (BvalidOvEn) This bit is used to enable/disable the software to override the Bvalid signal using the GOTGCTL.BvalidOvVal. - 1'b1 : Internally Bvalid received from the PHY is overridden with GOTGCTL.BvalidOvVal. - 1'b0 : Override is disabled and bvalid signal from the respective PHY selected is used internally by the force"]
    #[inline(always)]
    pub fn bvalidoven(&self) -> BVALIDOVEN_R {
        BVALIDOVEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Mode: Device only B-Peripheral Session Valid OverrideValue (BvalidOvVal) This bit is used to set Override value for Bvalid signal when GOTGCTL.BvalidOvEn is set. - 1'b0 : Bvalid value is 1'b0 when GOTGCTL.BvalidOvEn =1 - 1'b1 : Bvalid value is 1'b1 when GOTGCTL.BvalidOvEn =1"]
    #[inline(always)]
    pub fn bvalidovval(&self) -> BVALIDOVVAL_R {
        BVALIDOVVAL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 15 - Mode: Host and Device Debounce Filter Bypass Bypass Debounce filters for avalid, bvalid, vbusvalid, sessend, iddig signals when enabled. - 1'b0: Disabled - 1'b1: Enabled Note: This register bit is valid only when debounce filters are present in core."]
    #[inline(always)]
    pub fn dbncefltrbypass(&self) -> DBNCEFLTRBYPASS_R {
        DBNCEFLTRBYPASS_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Mode: Host and Device Connector ID Status (ConIDSts) Indicates the connector ID status on a connect event. - 1'b0: The core is in A-Device mode. - 1'b1: The core is in B-Device mode. Note: The reset value of this register field can be read only after the PHY clock is stable, or if IDDIG_FILTER is enabled, wait for the filter timer to expire to read the correct reset value which ever event is later. Reset: - 1'b0: in host only mode (OTG_MODE = 5 or 6) - 1'b1: in all other configurations"]
    #[inline(always)]
    pub fn conidsts(&self) -> CONIDSTS_R {
        CONIDSTS_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Mode: Host only Long/Short Debounce Time (DbncTime) Indicates the debounce time of a detected connection. - 1'b0: Long debounce time, used for physical connections (100 ms + 2.5 micro-sec) - 1'b1: Short debounce time, used for soft connections (2.5 micro-sec)"]
    #[inline(always)]
    pub fn dbnctime(&self) -> DBNCTIME_R {
        DBNCTIME_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Mode: Host only A-Session Valid (ASesVld) Indicates the Host mode transceiver status. - 1'b0: A-session is not valid - 1'b1: A-session is valid Note: If you do not enabled OTG features (such as SRP and HNP), the read reset value will be 1. The vbus assigns the values internally for non-SRP or non-HNP configurations. In case of OTG_MODE=0, the reset value of this bit is 1'b0."]
    #[inline(always)]
    pub fn asesvld(&self) -> ASESVLD_R {
        ASESVLD_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Mode: Device only B-Session Valid (BSesVld) Indicates the Device mode transceiver status. - 1'b0: B-session is not valid. - 1'b1: B-session is valid. In OTG mode, you can use this bit to determine if the device is connected or disconnected. Note: - If you do not enable OTG features (such as SRP and HNP), the read reset value will be 1.The vbus assigns the values internally for non- SRP or non-HNP configurations. - In case of OTG_MODE=0, the reset value of this bit is 1'b0. - The reset value of this register field can be read only after the PHY clock is stable, or if IDDIG_FILTER is enabled, wait for the filter timer to expire to read the correct reset value which ever event is later."]
    #[inline(always)]
    pub fn bsesvld(&self) -> BSESVLD_R {
        BSESVLD_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - OTG Version (OTGVer) Indicates the OTG revision. - 1'b0: OTG Version 1.3. In this version the core supports Data line pulsing and VBus pulsing for SRP. - 1'b1: OTG Version 2.0. In this version the core supports only Data line pulsing for SRP."]
    #[inline(always)]
    pub fn otgver(&self) -> OTGVER_R {
        OTGVER_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Current Mode of Operation (CurMod) Mode: Host and Device Indicates the current mode. - 1'b0: Device mode - 1'b1: Host mode Reset: - 1'b1 in Host-only mode (OTG_MODE=5 or 6) - 1'b0 in all other configurations Note: The reset value of this register field can be read only after the PHY clock is stable, or if IDDIG_FILTER is enabled, wait for the filter timer to expire to read the correct reset value which ever event is later."]
    #[inline(always)]
    pub fn curmod(&self) -> CURMOD_R {
        CURMOD_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 22:26 - Mode: Host and Device Multi Valued ID pin (MultValIdBC) Battery Charger ACA inputs in the following order: - Bit 26: rid_float. - Bit 25: rid_gnd - Bit 24: rid_a - Bit 23: rid_b - Bit 22: rid_c"]
    #[inline(always)]
    pub fn multvalidbc(&self) -> MULTVALIDBC_R {
        MULTVALIDBC_R::new(((self.bits >> 22) & 0x1f) as u8)
    }
    #[doc = "Bit 27 - Mode: Device Only This bit when programmed to 1'b1 results in the core asserting chirp_on before sending an actual Chirp K signal on USB. This bit is present only if OTG_BC_SUPPORT = 1.If OTG_BC_SUPPORT!=1, this bit is a reserved bit. Do not set this bit when core is operating in HSIC mode because HSIC always operates at High Speed and High speed chirp is not used"]
    #[inline(always)]
    pub fn chirpen(&self) -> CHIRPEN_R {
        CHIRPEN_R::new(((self.bits >> 27) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GOTGCTL")
            .field("vbvalidoven", &self.vbvalidoven())
            .field("vbvalidovval", &self.vbvalidovval())
            .field("avalidoven", &self.avalidoven())
            .field("avalidovval", &self.avalidovval())
            .field("bvalidoven", &self.bvalidoven())
            .field("bvalidovval", &self.bvalidovval())
            .field("dbncefltrbypass", &self.dbncefltrbypass())
            .field("conidsts", &self.conidsts())
            .field("dbnctime", &self.dbnctime())
            .field("asesvld", &self.asesvld())
            .field("bsesvld", &self.bsesvld())
            .field("otgver", &self.otgver())
            .field("curmod", &self.curmod())
            .field("multvalidbc", &self.multvalidbc())
            .field("chirpen", &self.chirpen())
            .finish()
    }
}
impl W {
    #[doc = "Bit 2 - Mode: Host only VBUS Valid Override Enable (VbvalidOvEn) This bit is used to enable/disable the software to override the Bvalid signal using the GOTGCTL.VbvalidOvVal. - 1'b1 : Internally Bvalid received from the PHY is overridden with GOTGCTL.VbvalidOvVal. - 1'b0 : Override is disabled and bvalid signal from the respective PHY selected is used internally by the controller."]
    #[inline(always)]
    pub fn vbvalidoven(&mut self) -> VBVALIDOVEN_W<'_, GOTGCTL_SPEC> {
        VBVALIDOVEN_W::new(self, 2)
    }
    #[doc = "Bit 3 - Mode: Host only VBUS Valid OverrideValue (VbvalidOvVal) This bit is used to set Override value for vbusvalid signal when GOTGCTL.VbvalidOvEn is set. - 1'b0 : vbusvalid value is 1'b0 when GOTGCTL.VbvalidOvEn =1 - 1'b1 : vbusvalid value is 1'b1 when GOTGCTL.VbvalidOvEn =1"]
    #[inline(always)]
    pub fn vbvalidovval(&mut self) -> VBVALIDOVVAL_W<'_, GOTGCTL_SPEC> {
        VBVALIDOVVAL_W::new(self, 3)
    }
    #[doc = "Bit 4 - Mode: Host only A-Peripheral Session Valid Override Enable (AvalidOvEn) This bit is used to enable/disable the software to override the Avalid signal using the GOTGCTL.AvalidOvVal. - 1'b1: Internally Avalid received from the PHY is overridden with GOTGCTL.AvalidOvVal. - 1'b0: Override is disabled and avalid signal from the respective PHY selected is used internally by the core"]
    #[inline(always)]
    pub fn avalidoven(&mut self) -> AVALIDOVEN_W<'_, GOTGCTL_SPEC> {
        AVALIDOVEN_W::new(self, 4)
    }
    #[doc = "Bit 5 - Mode: Host only A-Peripheral Session Valid OverrideValue (AvalidOvVal) This bit is used to set Override value for Avalid signal when GOTGCTL.AvalidOvEn is set. - 1'b0 : Avalid value is 1'b0 when GOTGCTL.AvalidOvEn =1 - 1'b1 : Avalid value is 1'b1 when GOTGCTL.AvalidOvEn =1"]
    #[inline(always)]
    pub fn avalidovval(&mut self) -> AVALIDOVVAL_W<'_, GOTGCTL_SPEC> {
        AVALIDOVVAL_W::new(self, 5)
    }
    #[doc = "Bit 6 - Mode: Device only B-Peripheral Session Valid Override Value (BvalidOvEn) This bit is used to enable/disable the software to override the Bvalid signal using the GOTGCTL.BvalidOvVal. - 1'b1 : Internally Bvalid received from the PHY is overridden with GOTGCTL.BvalidOvVal. - 1'b0 : Override is disabled and bvalid signal from the respective PHY selected is used internally by the force"]
    #[inline(always)]
    pub fn bvalidoven(&mut self) -> BVALIDOVEN_W<'_, GOTGCTL_SPEC> {
        BVALIDOVEN_W::new(self, 6)
    }
    #[doc = "Bit 7 - Mode: Device only B-Peripheral Session Valid OverrideValue (BvalidOvVal) This bit is used to set Override value for Bvalid signal when GOTGCTL.BvalidOvEn is set. - 1'b0 : Bvalid value is 1'b0 when GOTGCTL.BvalidOvEn =1 - 1'b1 : Bvalid value is 1'b1 when GOTGCTL.BvalidOvEn =1"]
    #[inline(always)]
    pub fn bvalidovval(&mut self) -> BVALIDOVVAL_W<'_, GOTGCTL_SPEC> {
        BVALIDOVVAL_W::new(self, 7)
    }
    #[doc = "Bit 15 - Mode: Host and Device Debounce Filter Bypass Bypass Debounce filters for avalid, bvalid, vbusvalid, sessend, iddig signals when enabled. - 1'b0: Disabled - 1'b1: Enabled Note: This register bit is valid only when debounce filters are present in core."]
    #[inline(always)]
    pub fn dbncefltrbypass(&mut self) -> DBNCEFLTRBYPASS_W<'_, GOTGCTL_SPEC> {
        DBNCEFLTRBYPASS_W::new(self, 15)
    }
    #[doc = "Bit 20 - OTG Version (OTGVer) Indicates the OTG revision. - 1'b0: OTG Version 1.3. In this version the core supports Data line pulsing and VBus pulsing for SRP. - 1'b1: OTG Version 2.0. In this version the core supports only Data line pulsing for SRP."]
    #[inline(always)]
    pub fn otgver(&mut self) -> OTGVER_W<'_, GOTGCTL_SPEC> {
        OTGVER_W::new(self, 20)
    }
    #[doc = "Bit 27 - Mode: Device Only This bit when programmed to 1'b1 results in the core asserting chirp_on before sending an actual Chirp K signal on USB. This bit is present only if OTG_BC_SUPPORT = 1.If OTG_BC_SUPPORT!=1, this bit is a reserved bit. Do not set this bit when core is operating in HSIC mode because HSIC always operates at High Speed and High speed chirp is not used"]
    #[inline(always)]
    pub fn chirpen(&mut self) -> CHIRPEN_W<'_, GOTGCTL_SPEC> {
        CHIRPEN_W::new(self, 27)
    }
}
#[doc = "The OTG Control and Status register controls the behavior and reflects the status of the OTG function of the controller.\n\nYou can [`read`](crate::Reg::read) this register and get [`gotgctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gotgctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GOTGCTL_SPEC;
impl crate::RegisterSpec for GOTGCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gotgctl::R`](R) reader structure"]
impl crate::Readable for GOTGCTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gotgctl::W`](W) writer structure"]
impl crate::Writable for GOTGCTL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GOTGCTL to value 0x000d_0000"]
impl crate::Resettable for GOTGCTL_SPEC {
    const RESET_VALUE: u32 = 0x000d_0000;
}
