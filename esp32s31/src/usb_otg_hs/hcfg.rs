#[doc = "Register `HCFG` reader"]
pub type R = crate::R<HCFG_SPEC>;
#[doc = "Register `HCFG` writer"]
pub type W = crate::W<HCFG_SPEC>;
#[doc = "Field `FSLSPCLKSEL` reader - FS/LS PHY Clock Select (FSLSPclkSel) When the core is in FS Host mode - 2'b00: PHY clock is running at 30/60 MHz - 2'b01: PHY clock is running at 48 MHz - Others: Reserved When the core is in LS Host mode - 2'b00: PHY clock is running at 30/60 MHz. When the UTMI+/ULPI PHY Low Power mode is not selected, use 30/60 MHz. - 2'b01: PHY clock is running at 48 MHz. When the UTMI+ PHY Low Power mode is selected, use 48MHz If the PHY supplies a 48 MHz clock during LS mode. - 2'b10: PHY clock is running at 6 MHz. In USB 1.1 FS mode, use 6 MHz when the UTMI+ PHY Low Power mode is selected and the PHY supplies a 6 MHz clock during LS mode. If you select a 6 MHz clock during LS mode, you must do a soft reset. - 2'b11: Reserved Notes: - When Core in FS mode, the internal and external clocks have the same frequency. - When Core in LS mode, -- If FSLSPclkSel = 2'b00: Internal and external clocks have the same frequency -- If FSLSPclkSel = 2'b10: Internal clock is the divided by eight version of external 48 MHz clock"]
pub type FSLSPCLKSEL_R = crate::FieldReader;
#[doc = "Field `FSLSPCLKSEL` writer - FS/LS PHY Clock Select (FSLSPclkSel) When the core is in FS Host mode - 2'b00: PHY clock is running at 30/60 MHz - 2'b01: PHY clock is running at 48 MHz - Others: Reserved When the core is in LS Host mode - 2'b00: PHY clock is running at 30/60 MHz. When the UTMI+/ULPI PHY Low Power mode is not selected, use 30/60 MHz. - 2'b01: PHY clock is running at 48 MHz. When the UTMI+ PHY Low Power mode is selected, use 48MHz If the PHY supplies a 48 MHz clock during LS mode. - 2'b10: PHY clock is running at 6 MHz. In USB 1.1 FS mode, use 6 MHz when the UTMI+ PHY Low Power mode is selected and the PHY supplies a 6 MHz clock during LS mode. If you select a 6 MHz clock during LS mode, you must do a soft reset. - 2'b11: Reserved Notes: - When Core in FS mode, the internal and external clocks have the same frequency. - When Core in LS mode, -- If FSLSPclkSel = 2'b00: Internal and external clocks have the same frequency -- If FSLSPclkSel = 2'b10: Internal clock is the divided by eight version of external 48 MHz clock"]
pub type FSLSPCLKSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FSLSSUPP` reader - FS- and LS-Only Support (FSLSSupp) The application uses this bit to control the core's enumeration speed. Using this bit, the application can make the core enumerate as a FS host, even If the connected device supports HS traffic. Do not make changes to this field after initial programming. - 1'b0: HS/FS/LS, based on the maximum speed supported by the connected device - 1'b1: FS/LS-only, even If the connected device can support HS"]
pub type FSLSSUPP_R = crate::BitReader;
#[doc = "Field `FSLSSUPP` writer - FS- and LS-Only Support (FSLSSupp) The application uses this bit to control the core's enumeration speed. Using this bit, the application can make the core enumerate as a FS host, even If the connected device supports HS traffic. Do not make changes to this field after initial programming. - 1'b0: HS/FS/LS, based on the maximum speed supported by the connected device - 1'b1: FS/LS-only, even If the connected device can support HS"]
pub type FSLSSUPP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENA32KHZS` reader - Enable 32 KHz Suspend mode (Ena32KHzS) This bit can be set only in FS PHY interface is selected. Else, this bit needs to be set to zero. When FS PHY interface is chosen and this bit is set, the core expects that the PHY clock during Suspend is switched from 48 MHz to 32 KHz."]
pub type ENA32KHZS_R = crate::BitReader;
#[doc = "Field `ENA32KHZS` writer - Enable 32 KHz Suspend mode (Ena32KHzS) This bit can be set only in FS PHY interface is selected. Else, this bit needs to be set to zero. When FS PHY interface is chosen and this bit is set, the core expects that the PHY clock during Suspend is switched from 48 MHz to 32 KHz."]
pub type ENA32KHZS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESVALID` reader - Resume Validation Period (ResValid) This field is effective only when HCFG.Ena32KHzS is set. It controls the resume period when the core resumes from suspend. The core counts for 'ResValid' number of clock cycles to detect a valid resume when this is set."]
pub type RESVALID_R = crate::FieldReader;
#[doc = "Field `RESVALID` writer - Resume Validation Period (ResValid) This field is effective only when HCFG.Ena32KHzS is set. It controls the resume period when the core resumes from suspend. The core counts for 'ResValid' number of clock cycles to detect a valid resume when this is set."]
pub type RESVALID_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DESCDMA` reader - Enable Scatter/gather DMA in Host mode (DescDMA) When the Scatter/Gather DMA option selected during configuration of the RTL, the application can set this bit during initialization to enable the Scatter/Gather DMA operation. Note: This bit must be modified only once after a reset. The following combinations are available for programming: - GAHBCFG.DMAEn=0,HCFG.DescDMA=0 => Slave mode - GAHBCFG.DMAEn=0,HCFG.DescDMA=1 => Invalid - GAHBCFG.DMAEn=1,HCFG.DescDMA=0 => Buffered DMA mode - GAHBCFG.DMAEn=1,HCFG.DescDMA=1 => Scatter/Gather DMA mode"]
pub type DESCDMA_R = crate::BitReader;
#[doc = "Field `DESCDMA` writer - Enable Scatter/gather DMA in Host mode (DescDMA) When the Scatter/Gather DMA option selected during configuration of the RTL, the application can set this bit during initialization to enable the Scatter/Gather DMA operation. Note: This bit must be modified only once after a reset. The following combinations are available for programming: - GAHBCFG.DMAEn=0,HCFG.DescDMA=0 => Slave mode - GAHBCFG.DMAEn=0,HCFG.DescDMA=1 => Invalid - GAHBCFG.DMAEn=1,HCFG.DescDMA=0 => Buffered DMA mode - GAHBCFG.DMAEn=1,HCFG.DescDMA=1 => Scatter/Gather DMA mode"]
pub type DESCDMA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRLISTEN` reader - Frame List Entries(FrListEn) The value in the register specifies the number of entries in the Frame list. This field is valid only in Scatter/Gather DMA mode. - 2'b00: 8 Entries - 2'b01: 16 Entries - 2'b10: 32 Entries - 2'b11: 64 Entries"]
pub type FRLISTEN_R = crate::FieldReader;
#[doc = "Field `FRLISTEN` writer - Frame List Entries(FrListEn) The value in the register specifies the number of entries in the Frame list. This field is valid only in Scatter/Gather DMA mode. - 2'b00: 8 Entries - 2'b01: 16 Entries - 2'b10: 32 Entries - 2'b11: 64 Entries"]
pub type FRLISTEN_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PERSCHEDENA` reader - Enable Periodic Scheduling (PerSchedEna): Applicable in host DDMA mode only. Enables periodic scheduling within the core. Initially, the bit is reset. The core does not process any periodic channels. As soon as this bit is set, the core gets ready to start scheduling periodic channels and sets HCFG.PerSchedStat. The setting of HCFG.PerSchedStat indicates the core has enabled periodic scheduling. Once HCFG.PerSchedEna is set, the application is not supposed to again reset the bit unless HCFG.PerSchedStat is set. As soon as this bit is reset, the core gets ready to stop scheduling periodic channels and resets HCFG.PerSchedStat."]
pub type PERSCHEDENA_R = crate::BitReader;
#[doc = "Field `PERSCHEDENA` writer - Enable Periodic Scheduling (PerSchedEna): Applicable in host DDMA mode only. Enables periodic scheduling within the core. Initially, the bit is reset. The core does not process any periodic channels. As soon as this bit is set, the core gets ready to start scheduling periodic channels and sets HCFG.PerSchedStat. The setting of HCFG.PerSchedStat indicates the core has enabled periodic scheduling. Once HCFG.PerSchedEna is set, the application is not supposed to again reset the bit unless HCFG.PerSchedStat is set. As soon as this bit is reset, the core gets ready to stop scheduling periodic channels and resets HCFG.PerSchedStat."]
pub type PERSCHEDENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODECHTIMEN` reader - Mode Change Ready Timer Enable (ModeChTimEn) This bit is used to enable/disable the Host core to wait 200 PHY clock cycles at the end of Resume to change the opmode signal to the PHY to 00 after Suspend or LPM. - 1'b0 : The Host core waits for either 200 PHY clock cycles or a linestate of SE0 at the end of resume to the change the opmode from 2'b10 to 2'b00 - 1'b1 : The Host core waits only for a linestate of SE0 at the end of resume to change the opmode from 2'b10 to 2'b00."]
pub type MODECHTIMEN_R = crate::BitReader;
#[doc = "Field `MODECHTIMEN` writer - Mode Change Ready Timer Enable (ModeChTimEn) This bit is used to enable/disable the Host core to wait 200 PHY clock cycles at the end of Resume to change the opmode signal to the PHY to 00 after Suspend or LPM. - 1'b0 : The Host core waits for either 200 PHY clock cycles or a linestate of SE0 at the end of resume to the change the opmode from 2'b10 to 2'b00 - 1'b1 : The Host core waits only for a linestate of SE0 at the end of resume to change the opmode from 2'b10 to 2'b00."]
pub type MODECHTIMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - FS/LS PHY Clock Select (FSLSPclkSel) When the core is in FS Host mode - 2'b00: PHY clock is running at 30/60 MHz - 2'b01: PHY clock is running at 48 MHz - Others: Reserved When the core is in LS Host mode - 2'b00: PHY clock is running at 30/60 MHz. When the UTMI+/ULPI PHY Low Power mode is not selected, use 30/60 MHz. - 2'b01: PHY clock is running at 48 MHz. When the UTMI+ PHY Low Power mode is selected, use 48MHz If the PHY supplies a 48 MHz clock during LS mode. - 2'b10: PHY clock is running at 6 MHz. In USB 1.1 FS mode, use 6 MHz when the UTMI+ PHY Low Power mode is selected and the PHY supplies a 6 MHz clock during LS mode. If you select a 6 MHz clock during LS mode, you must do a soft reset. - 2'b11: Reserved Notes: - When Core in FS mode, the internal and external clocks have the same frequency. - When Core in LS mode, -- If FSLSPclkSel = 2'b00: Internal and external clocks have the same frequency -- If FSLSPclkSel = 2'b10: Internal clock is the divided by eight version of external 48 MHz clock"]
    #[inline(always)]
    pub fn fslspclksel(&self) -> FSLSPCLKSEL_R {
        FSLSPCLKSEL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - FS- and LS-Only Support (FSLSSupp) The application uses this bit to control the core's enumeration speed. Using this bit, the application can make the core enumerate as a FS host, even If the connected device supports HS traffic. Do not make changes to this field after initial programming. - 1'b0: HS/FS/LS, based on the maximum speed supported by the connected device - 1'b1: FS/LS-only, even If the connected device can support HS"]
    #[inline(always)]
    pub fn fslssupp(&self) -> FSLSSUPP_R {
        FSLSSUPP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable 32 KHz Suspend mode (Ena32KHzS) This bit can be set only in FS PHY interface is selected. Else, this bit needs to be set to zero. When FS PHY interface is chosen and this bit is set, the core expects that the PHY clock during Suspend is switched from 48 MHz to 32 KHz."]
    #[inline(always)]
    pub fn ena32khzs(&self) -> ENA32KHZS_R {
        ENA32KHZS_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:15 - Resume Validation Period (ResValid) This field is effective only when HCFG.Ena32KHzS is set. It controls the resume period when the core resumes from suspend. The core counts for 'ResValid' number of clock cycles to detect a valid resume when this is set."]
    #[inline(always)]
    pub fn resvalid(&self) -> RESVALID_R {
        RESVALID_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 23 - Enable Scatter/gather DMA in Host mode (DescDMA) When the Scatter/Gather DMA option selected during configuration of the RTL, the application can set this bit during initialization to enable the Scatter/Gather DMA operation. Note: This bit must be modified only once after a reset. The following combinations are available for programming: - GAHBCFG.DMAEn=0,HCFG.DescDMA=0 => Slave mode - GAHBCFG.DMAEn=0,HCFG.DescDMA=1 => Invalid - GAHBCFG.DMAEn=1,HCFG.DescDMA=0 => Buffered DMA mode - GAHBCFG.DMAEn=1,HCFG.DescDMA=1 => Scatter/Gather DMA mode"]
    #[inline(always)]
    pub fn descdma(&self) -> DESCDMA_R {
        DESCDMA_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:25 - Frame List Entries(FrListEn) The value in the register specifies the number of entries in the Frame list. This field is valid only in Scatter/Gather DMA mode. - 2'b00: 8 Entries - 2'b01: 16 Entries - 2'b10: 32 Entries - 2'b11: 64 Entries"]
    #[inline(always)]
    pub fn frlisten(&self) -> FRLISTEN_R {
        FRLISTEN_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 26 - Enable Periodic Scheduling (PerSchedEna): Applicable in host DDMA mode only. Enables periodic scheduling within the core. Initially, the bit is reset. The core does not process any periodic channels. As soon as this bit is set, the core gets ready to start scheduling periodic channels and sets HCFG.PerSchedStat. The setting of HCFG.PerSchedStat indicates the core has enabled periodic scheduling. Once HCFG.PerSchedEna is set, the application is not supposed to again reset the bit unless HCFG.PerSchedStat is set. As soon as this bit is reset, the core gets ready to stop scheduling periodic channels and resets HCFG.PerSchedStat."]
    #[inline(always)]
    pub fn perschedena(&self) -> PERSCHEDENA_R {
        PERSCHEDENA_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 31 - Mode Change Ready Timer Enable (ModeChTimEn) This bit is used to enable/disable the Host core to wait 200 PHY clock cycles at the end of Resume to change the opmode signal to the PHY to 00 after Suspend or LPM. - 1'b0 : The Host core waits for either 200 PHY clock cycles or a linestate of SE0 at the end of resume to the change the opmode from 2'b10 to 2'b00 - 1'b1 : The Host core waits only for a linestate of SE0 at the end of resume to change the opmode from 2'b10 to 2'b00."]
    #[inline(always)]
    pub fn modechtimen(&self) -> MODECHTIMEN_R {
        MODECHTIMEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HCFG")
            .field("fslspclksel", &self.fslspclksel())
            .field("fslssupp", &self.fslssupp())
            .field("ena32khzs", &self.ena32khzs())
            .field("resvalid", &self.resvalid())
            .field("descdma", &self.descdma())
            .field("frlisten", &self.frlisten())
            .field("perschedena", &self.perschedena())
            .field("modechtimen", &self.modechtimen())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - FS/LS PHY Clock Select (FSLSPclkSel) When the core is in FS Host mode - 2'b00: PHY clock is running at 30/60 MHz - 2'b01: PHY clock is running at 48 MHz - Others: Reserved When the core is in LS Host mode - 2'b00: PHY clock is running at 30/60 MHz. When the UTMI+/ULPI PHY Low Power mode is not selected, use 30/60 MHz. - 2'b01: PHY clock is running at 48 MHz. When the UTMI+ PHY Low Power mode is selected, use 48MHz If the PHY supplies a 48 MHz clock during LS mode. - 2'b10: PHY clock is running at 6 MHz. In USB 1.1 FS mode, use 6 MHz when the UTMI+ PHY Low Power mode is selected and the PHY supplies a 6 MHz clock during LS mode. If you select a 6 MHz clock during LS mode, you must do a soft reset. - 2'b11: Reserved Notes: - When Core in FS mode, the internal and external clocks have the same frequency. - When Core in LS mode, -- If FSLSPclkSel = 2'b00: Internal and external clocks have the same frequency -- If FSLSPclkSel = 2'b10: Internal clock is the divided by eight version of external 48 MHz clock"]
    #[inline(always)]
    pub fn fslspclksel(&mut self) -> FSLSPCLKSEL_W<'_, HCFG_SPEC> {
        FSLSPCLKSEL_W::new(self, 0)
    }
    #[doc = "Bit 2 - FS- and LS-Only Support (FSLSSupp) The application uses this bit to control the core's enumeration speed. Using this bit, the application can make the core enumerate as a FS host, even If the connected device supports HS traffic. Do not make changes to this field after initial programming. - 1'b0: HS/FS/LS, based on the maximum speed supported by the connected device - 1'b1: FS/LS-only, even If the connected device can support HS"]
    #[inline(always)]
    pub fn fslssupp(&mut self) -> FSLSSUPP_W<'_, HCFG_SPEC> {
        FSLSSUPP_W::new(self, 2)
    }
    #[doc = "Bit 7 - Enable 32 KHz Suspend mode (Ena32KHzS) This bit can be set only in FS PHY interface is selected. Else, this bit needs to be set to zero. When FS PHY interface is chosen and this bit is set, the core expects that the PHY clock during Suspend is switched from 48 MHz to 32 KHz."]
    #[inline(always)]
    pub fn ena32khzs(&mut self) -> ENA32KHZS_W<'_, HCFG_SPEC> {
        ENA32KHZS_W::new(self, 7)
    }
    #[doc = "Bits 8:15 - Resume Validation Period (ResValid) This field is effective only when HCFG.Ena32KHzS is set. It controls the resume period when the core resumes from suspend. The core counts for 'ResValid' number of clock cycles to detect a valid resume when this is set."]
    #[inline(always)]
    pub fn resvalid(&mut self) -> RESVALID_W<'_, HCFG_SPEC> {
        RESVALID_W::new(self, 8)
    }
    #[doc = "Bit 23 - Enable Scatter/gather DMA in Host mode (DescDMA) When the Scatter/Gather DMA option selected during configuration of the RTL, the application can set this bit during initialization to enable the Scatter/Gather DMA operation. Note: This bit must be modified only once after a reset. The following combinations are available for programming: - GAHBCFG.DMAEn=0,HCFG.DescDMA=0 => Slave mode - GAHBCFG.DMAEn=0,HCFG.DescDMA=1 => Invalid - GAHBCFG.DMAEn=1,HCFG.DescDMA=0 => Buffered DMA mode - GAHBCFG.DMAEn=1,HCFG.DescDMA=1 => Scatter/Gather DMA mode"]
    #[inline(always)]
    pub fn descdma(&mut self) -> DESCDMA_W<'_, HCFG_SPEC> {
        DESCDMA_W::new(self, 23)
    }
    #[doc = "Bits 24:25 - Frame List Entries(FrListEn) The value in the register specifies the number of entries in the Frame list. This field is valid only in Scatter/Gather DMA mode. - 2'b00: 8 Entries - 2'b01: 16 Entries - 2'b10: 32 Entries - 2'b11: 64 Entries"]
    #[inline(always)]
    pub fn frlisten(&mut self) -> FRLISTEN_W<'_, HCFG_SPEC> {
        FRLISTEN_W::new(self, 24)
    }
    #[doc = "Bit 26 - Enable Periodic Scheduling (PerSchedEna): Applicable in host DDMA mode only. Enables periodic scheduling within the core. Initially, the bit is reset. The core does not process any periodic channels. As soon as this bit is set, the core gets ready to start scheduling periodic channels and sets HCFG.PerSchedStat. The setting of HCFG.PerSchedStat indicates the core has enabled periodic scheduling. Once HCFG.PerSchedEna is set, the application is not supposed to again reset the bit unless HCFG.PerSchedStat is set. As soon as this bit is reset, the core gets ready to stop scheduling periodic channels and resets HCFG.PerSchedStat."]
    #[inline(always)]
    pub fn perschedena(&mut self) -> PERSCHEDENA_W<'_, HCFG_SPEC> {
        PERSCHEDENA_W::new(self, 26)
    }
    #[doc = "Bit 31 - Mode Change Ready Timer Enable (ModeChTimEn) This bit is used to enable/disable the Host core to wait 200 PHY clock cycles at the end of Resume to change the opmode signal to the PHY to 00 after Suspend or LPM. - 1'b0 : The Host core waits for either 200 PHY clock cycles or a linestate of SE0 at the end of resume to the change the opmode from 2'b10 to 2'b00 - 1'b1 : The Host core waits only for a linestate of SE0 at the end of resume to change the opmode from 2'b10 to 2'b00."]
    #[inline(always)]
    pub fn modechtimen(&mut self) -> MODECHTIMEN_W<'_, HCFG_SPEC> {
        MODECHTIMEN_W::new(self, 31)
    }
}
#[doc = "This register is used to configure the controller in Host mode.\n\nYou can [`read`](crate::Reg::read) this register and get [`hcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HCFG_SPEC;
impl crate::RegisterSpec for HCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcfg::R`](R) reader structure"]
impl crate::Readable for HCFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hcfg::W`](W) writer structure"]
impl crate::Writable for HCFG_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HCFG to value 0x0200"]
impl crate::Resettable for HCFG_SPEC {
    const RESET_VALUE: u32 = 0x0200;
}
