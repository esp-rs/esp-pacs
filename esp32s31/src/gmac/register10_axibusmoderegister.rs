#[doc = "Register `REGISTER10_AXIBUSMODEREGISTER` reader"]
pub type R = crate::R<REGISTER10_AXIBUSMODEREGISTER_SPEC>;
#[doc = "Register `REGISTER10_AXIBUSMODEREGISTER` writer"]
pub type W = crate::W<REGISTER10_AXIBUSMODEREGISTER_SPEC>;
#[doc = "Field `UNDEF` reader - AXI Undefined Burst Length This bit is readonly bit and indicates the complement _invert_ value of Bit 16 _FB_ in Register 0 _Bus Mode Register_ When this bit is set to 1, the GMACAXI is allowed to perform any burst length equal to or below the maximum allowed burst length programmed in Bits\\[7:3\\] When this bit is set to 0, the GMACAXI is allowed to perform only fixed burst lengths as indicated by BLEN256, BLEN128, BLEN64, BLEN32, BLEN16, BLEN8, or BLEN4, or a burst length of 1 If UNDEF is set and none of the BLEN bits is set, then GMACAXI is allowed to perform a burst length of 16"]
pub type UNDEF_R = crate::BitReader;
#[doc = "Field `BLEN4` reader - AXI Burst Length 4 When this bit is set to 1, the GMACAXI is allowed to select a burst length of 4 on the AXI master interface Setting this bit has no effect when UNDEF is set to 1"]
pub type BLEN4_R = crate::BitReader;
#[doc = "Field `BLEN4` writer - AXI Burst Length 4 When this bit is set to 1, the GMACAXI is allowed to select a burst length of 4 on the AXI master interface Setting this bit has no effect when UNDEF is set to 1"]
pub type BLEN4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLEN8` reader - AXI Burst Length 8 When this bit is set to 1, the GMACAXI is allowed to select a burst length of 8 on the AXI master interface Setting this bit has no effect when UNDEF is set to 1"]
pub type BLEN8_R = crate::BitReader;
#[doc = "Field `BLEN8` writer - AXI Burst Length 8 When this bit is set to 1, the GMACAXI is allowed to select a burst length of 8 on the AXI master interface Setting this bit has no effect when UNDEF is set to 1"]
pub type BLEN8_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLEN16` reader - AXI Burst Length 16 When this bit is set to 1 or UNDEF is set to 1, the GMACAXI is allowed to select a burst length of 16 on the AXI master interface"]
pub type BLEN16_R = crate::BitReader;
#[doc = "Field `BLEN16` writer - AXI Burst Length 16 When this bit is set to 1 or UNDEF is set to 1, the GMACAXI is allowed to select a burst length of 16 on the AXI master interface"]
pub type BLEN16_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLEN32` reader - AXI Burst Length 32 When this bit is set to 1, the GMACAXI is allowed to select a burst length of 32 on the AXI master interface This bit is present only when the configuration parameter AXI_BL is set to 32 or more Otherwise, this bit is reserved and is readonly _RO_"]
pub type BLEN32_R = crate::BitReader;
#[doc = "Field `BLEN32` writer - AXI Burst Length 32 When this bit is set to 1, the GMACAXI is allowed to select a burst length of 32 on the AXI master interface This bit is present only when the configuration parameter AXI_BL is set to 32 or more Otherwise, this bit is reserved and is readonly _RO_"]
pub type BLEN32_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLEN64` reader - AXI Burst Length 64 When this bit is set to 1, the GMACAXI is allowed to select a burst length of 64 on the AXI master interface This bit is present only when the configuration parameter AXI_BL is set to 64 or more Otherwise, this bit is reserved and is readonly _RO_"]
pub type BLEN64_R = crate::BitReader;
#[doc = "Field `BLEN64` writer - AXI Burst Length 64 When this bit is set to 1, the GMACAXI is allowed to select a burst length of 64 on the AXI master interface This bit is present only when the configuration parameter AXI_BL is set to 64 or more Otherwise, this bit is reserved and is readonly _RO_"]
pub type BLEN64_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLEN128` reader - AXI Burst Length 128 When this bit is set to 1, the GMACAXI is allowed to select a burst length of 128 on the AXI master interface This bit is present only when the configuration parameter AXI_BL is set to 128 or more Otherwise, this bit is reserved and is readonly _RO_"]
pub type BLEN128_R = crate::BitReader;
#[doc = "Field `BLEN128` writer - AXI Burst Length 128 When this bit is set to 1, the GMACAXI is allowed to select a burst length of 128 on the AXI master interface This bit is present only when the configuration parameter AXI_BL is set to 128 or more Otherwise, this bit is reserved and is readonly _RO_"]
pub type BLEN128_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLEN256` reader - AXI Burst Length 256 When this bit is set to 1, the GMACAXI is allowed to select a burst length of 256 on the AXI master interface This bit is present only when the configuration parameter AXI_BL is set to 256 Otherwise, this bit is reserved and is readonly _RO_"]
pub type BLEN256_R = crate::BitReader;
#[doc = "Field `BLEN256` writer - AXI Burst Length 256 When this bit is set to 1, the GMACAXI is allowed to select a burst length of 256 on the AXI master interface This bit is present only when the configuration parameter AXI_BL is set to 256 Otherwise, this bit is reserved and is readonly _RO_"]
pub type BLEN256_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AXI_AAL` reader - AddressAligned Beats This bit is readonly bit and reflects the Bit 25 _AAL_ of Register 0 _Bus Mode Register_ When this bit is set to 1, the GMACAXI performs addressaligned burst transfers on both read and write channels 11:8 Reserved 0H RO"]
pub type AXI_AAL_R = crate::BitReader;
#[doc = "Field `ONEKBBE` reader - 1 KB Boundary Crossing Enable for the GMACAXI Master When set, the GMACAXI master performs burst transfers that do not cross 1 KB boundary When reset, the GMACAXI master performs burst transfers that do not cross 4 KB boundary"]
pub type ONEKBBE_R = crate::BitReader;
#[doc = "Field `ONEKBBE` writer - 1 KB Boundary Crossing Enable for the GMACAXI Master When set, the GMACAXI master performs burst transfers that do not cross 1 KB boundary When reset, the GMACAXI master performs burst transfers that do not cross 4 KB boundary"]
pub type ONEKBBE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RD_OSR_LMT` reader - AXI Maximum Read Outstanding Request Limit This value limits the maximum outstanding request on the AXI read interface Maximum outstanding requests = RD_OSR_LMT+1 Note: Bit 18 is reserved if AXI_GM_MAX_RD_REQUESTS = 4 Bit 19 is reserved if AXI_GM_MAX_RD_REQUESTS != 16 15:14 Reserved 00 RO"]
pub type RD_OSR_LMT_R = crate::FieldReader;
#[doc = "Field `RD_OSR_LMT` writer - AXI Maximum Read Outstanding Request Limit This value limits the maximum outstanding request on the AXI read interface Maximum outstanding requests = RD_OSR_LMT+1 Note: Bit 18 is reserved if AXI_GM_MAX_RD_REQUESTS = 4 Bit 19 is reserved if AXI_GM_MAX_RD_REQUESTS != 16 15:14 Reserved 00 RO"]
pub type RD_OSR_LMT_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WR_OSR_LMT` reader - AXI Maximum Write Outstanding Request Limit This value limits the maximum outstanding request on the AXI write interface Maximum outstanding requests = WR_OSR_LMT+1 Note: Bit 22 is reserved if AXI_GM_MAX_WR_REQUESTS = 4 Bit 23 bit is reserved if AXI_GM_MAX_WR_REQUESTS != 16"]
pub type WR_OSR_LMT_R = crate::FieldReader;
#[doc = "Field `WR_OSR_LMT` writer - AXI Maximum Write Outstanding Request Limit This value limits the maximum outstanding request on the AXI write interface Maximum outstanding requests = WR_OSR_LMT+1 Note: Bit 22 is reserved if AXI_GM_MAX_WR_REQUESTS = 4 Bit 23 bit is reserved if AXI_GM_MAX_WR_REQUESTS != 16"]
pub type WR_OSR_LMT_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `LPI_XIT_FRM` reader - Unlock on Magic Packet or Remote WakeUp Frame When set to 1, this bit enables the GMACAXI to come out of the LPI mode only when the magic packet or remote wakeup frame is received When set to 0, this bit enables the GMACAXI to come out of LPI mode when any frame is received 29:24 Reserved 00H RO"]
pub type LPI_XIT_FRM_R = crate::BitReader;
#[doc = "Field `LPI_XIT_FRM` writer - Unlock on Magic Packet or Remote WakeUp Frame When set to 1, this bit enables the GMACAXI to come out of the LPI mode only when the magic packet or remote wakeup frame is received When set to 0, this bit enables the GMACAXI to come out of LPI mode when any frame is received 29:24 Reserved 00H RO"]
pub type LPI_XIT_FRM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN_LPI` reader - Enable Low Power Interface _LPI_ When set to 1, this bit enables the LPI mode supported by the GMACAXI configuration and accepts the LPI request from the AXI System Clock controller When set to 0, this bit disables the LPI mode and always denies the LPI request from the AXI System Clock controller"]
pub type EN_LPI_R = crate::BitReader;
#[doc = "Field `EN_LPI` writer - Enable Low Power Interface _LPI_ When set to 1, this bit enables the LPI mode supported by the GMACAXI configuration and accepts the LPI request from the AXI System Clock controller When set to 0, this bit disables the LPI mode and always denies the LPI request from the AXI System Clock controller"]
pub type EN_LPI_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - AXI Undefined Burst Length This bit is readonly bit and indicates the complement _invert_ value of Bit 16 _FB_ in Register 0 _Bus Mode Register_ When this bit is set to 1, the GMACAXI is allowed to perform any burst length equal to or below the maximum allowed burst length programmed in Bits\\[7:3\\] When this bit is set to 0, the GMACAXI is allowed to perform only fixed burst lengths as indicated by BLEN256, BLEN128, BLEN64, BLEN32, BLEN16, BLEN8, or BLEN4, or a burst length of 1 If UNDEF is set and none of the BLEN bits is set, then GMACAXI is allowed to perform a burst length of 16"]
    #[inline(always)]
    pub fn undef(&self) -> UNDEF_R {
        UNDEF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - AXI Burst Length 4 When this bit is set to 1, the GMACAXI is allowed to select a burst length of 4 on the AXI master interface Setting this bit has no effect when UNDEF is set to 1"]
    #[inline(always)]
    pub fn blen4(&self) -> BLEN4_R {
        BLEN4_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - AXI Burst Length 8 When this bit is set to 1, the GMACAXI is allowed to select a burst length of 8 on the AXI master interface Setting this bit has no effect when UNDEF is set to 1"]
    #[inline(always)]
    pub fn blen8(&self) -> BLEN8_R {
        BLEN8_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - AXI Burst Length 16 When this bit is set to 1 or UNDEF is set to 1, the GMACAXI is allowed to select a burst length of 16 on the AXI master interface"]
    #[inline(always)]
    pub fn blen16(&self) -> BLEN16_R {
        BLEN16_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - AXI Burst Length 32 When this bit is set to 1, the GMACAXI is allowed to select a burst length of 32 on the AXI master interface This bit is present only when the configuration parameter AXI_BL is set to 32 or more Otherwise, this bit is reserved and is readonly _RO_"]
    #[inline(always)]
    pub fn blen32(&self) -> BLEN32_R {
        BLEN32_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - AXI Burst Length 64 When this bit is set to 1, the GMACAXI is allowed to select a burst length of 64 on the AXI master interface This bit is present only when the configuration parameter AXI_BL is set to 64 or more Otherwise, this bit is reserved and is readonly _RO_"]
    #[inline(always)]
    pub fn blen64(&self) -> BLEN64_R {
        BLEN64_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - AXI Burst Length 128 When this bit is set to 1, the GMACAXI is allowed to select a burst length of 128 on the AXI master interface This bit is present only when the configuration parameter AXI_BL is set to 128 or more Otherwise, this bit is reserved and is readonly _RO_"]
    #[inline(always)]
    pub fn blen128(&self) -> BLEN128_R {
        BLEN128_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - AXI Burst Length 256 When this bit is set to 1, the GMACAXI is allowed to select a burst length of 256 on the AXI master interface This bit is present only when the configuration parameter AXI_BL is set to 256 Otherwise, this bit is reserved and is readonly _RO_"]
    #[inline(always)]
    pub fn blen256(&self) -> BLEN256_R {
        BLEN256_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 12 - AddressAligned Beats This bit is readonly bit and reflects the Bit 25 _AAL_ of Register 0 _Bus Mode Register_ When this bit is set to 1, the GMACAXI performs addressaligned burst transfers on both read and write channels 11:8 Reserved 0H RO"]
    #[inline(always)]
    pub fn axi_aal(&self) -> AXI_AAL_R {
        AXI_AAL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 1 KB Boundary Crossing Enable for the GMACAXI Master When set, the GMACAXI master performs burst transfers that do not cross 1 KB boundary When reset, the GMACAXI master performs burst transfers that do not cross 4 KB boundary"]
    #[inline(always)]
    pub fn onekbbe(&self) -> ONEKBBE_R {
        ONEKBBE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 16:19 - AXI Maximum Read Outstanding Request Limit This value limits the maximum outstanding request on the AXI read interface Maximum outstanding requests = RD_OSR_LMT+1 Note: Bit 18 is reserved if AXI_GM_MAX_RD_REQUESTS = 4 Bit 19 is reserved if AXI_GM_MAX_RD_REQUESTS != 16 15:14 Reserved 00 RO"]
    #[inline(always)]
    pub fn rd_osr_lmt(&self) -> RD_OSR_LMT_R {
        RD_OSR_LMT_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - AXI Maximum Write Outstanding Request Limit This value limits the maximum outstanding request on the AXI write interface Maximum outstanding requests = WR_OSR_LMT+1 Note: Bit 22 is reserved if AXI_GM_MAX_WR_REQUESTS = 4 Bit 23 bit is reserved if AXI_GM_MAX_WR_REQUESTS != 16"]
    #[inline(always)]
    pub fn wr_osr_lmt(&self) -> WR_OSR_LMT_R {
        WR_OSR_LMT_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 30 - Unlock on Magic Packet or Remote WakeUp Frame When set to 1, this bit enables the GMACAXI to come out of the LPI mode only when the magic packet or remote wakeup frame is received When set to 0, this bit enables the GMACAXI to come out of LPI mode when any frame is received 29:24 Reserved 00H RO"]
    #[inline(always)]
    pub fn lpi_xit_frm(&self) -> LPI_XIT_FRM_R {
        LPI_XIT_FRM_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Enable Low Power Interface _LPI_ When set to 1, this bit enables the LPI mode supported by the GMACAXI configuration and accepts the LPI request from the AXI System Clock controller When set to 0, this bit disables the LPI mode and always denies the LPI request from the AXI System Clock controller"]
    #[inline(always)]
    pub fn en_lpi(&self) -> EN_LPI_R {
        EN_LPI_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REGISTER10_AXIBUSMODEREGISTER")
            .field("undef", &self.undef())
            .field("blen4", &self.blen4())
            .field("blen8", &self.blen8())
            .field("blen16", &self.blen16())
            .field("blen32", &self.blen32())
            .field("blen64", &self.blen64())
            .field("blen128", &self.blen128())
            .field("blen256", &self.blen256())
            .field("axi_aal", &self.axi_aal())
            .field("onekbbe", &self.onekbbe())
            .field("rd_osr_lmt", &self.rd_osr_lmt())
            .field("wr_osr_lmt", &self.wr_osr_lmt())
            .field("lpi_xit_frm", &self.lpi_xit_frm())
            .field("en_lpi", &self.en_lpi())
            .finish()
    }
}
impl W {
    #[doc = "Bit 1 - AXI Burst Length 4 When this bit is set to 1, the GMACAXI is allowed to select a burst length of 4 on the AXI master interface Setting this bit has no effect when UNDEF is set to 1"]
    #[inline(always)]
    pub fn blen4(&mut self) -> BLEN4_W<'_, REGISTER10_AXIBUSMODEREGISTER_SPEC> {
        BLEN4_W::new(self, 1)
    }
    #[doc = "Bit 2 - AXI Burst Length 8 When this bit is set to 1, the GMACAXI is allowed to select a burst length of 8 on the AXI master interface Setting this bit has no effect when UNDEF is set to 1"]
    #[inline(always)]
    pub fn blen8(&mut self) -> BLEN8_W<'_, REGISTER10_AXIBUSMODEREGISTER_SPEC> {
        BLEN8_W::new(self, 2)
    }
    #[doc = "Bit 3 - AXI Burst Length 16 When this bit is set to 1 or UNDEF is set to 1, the GMACAXI is allowed to select a burst length of 16 on the AXI master interface"]
    #[inline(always)]
    pub fn blen16(&mut self) -> BLEN16_W<'_, REGISTER10_AXIBUSMODEREGISTER_SPEC> {
        BLEN16_W::new(self, 3)
    }
    #[doc = "Bit 4 - AXI Burst Length 32 When this bit is set to 1, the GMACAXI is allowed to select a burst length of 32 on the AXI master interface This bit is present only when the configuration parameter AXI_BL is set to 32 or more Otherwise, this bit is reserved and is readonly _RO_"]
    #[inline(always)]
    pub fn blen32(&mut self) -> BLEN32_W<'_, REGISTER10_AXIBUSMODEREGISTER_SPEC> {
        BLEN32_W::new(self, 4)
    }
    #[doc = "Bit 5 - AXI Burst Length 64 When this bit is set to 1, the GMACAXI is allowed to select a burst length of 64 on the AXI master interface This bit is present only when the configuration parameter AXI_BL is set to 64 or more Otherwise, this bit is reserved and is readonly _RO_"]
    #[inline(always)]
    pub fn blen64(&mut self) -> BLEN64_W<'_, REGISTER10_AXIBUSMODEREGISTER_SPEC> {
        BLEN64_W::new(self, 5)
    }
    #[doc = "Bit 6 - AXI Burst Length 128 When this bit is set to 1, the GMACAXI is allowed to select a burst length of 128 on the AXI master interface This bit is present only when the configuration parameter AXI_BL is set to 128 or more Otherwise, this bit is reserved and is readonly _RO_"]
    #[inline(always)]
    pub fn blen128(&mut self) -> BLEN128_W<'_, REGISTER10_AXIBUSMODEREGISTER_SPEC> {
        BLEN128_W::new(self, 6)
    }
    #[doc = "Bit 7 - AXI Burst Length 256 When this bit is set to 1, the GMACAXI is allowed to select a burst length of 256 on the AXI master interface This bit is present only when the configuration parameter AXI_BL is set to 256 Otherwise, this bit is reserved and is readonly _RO_"]
    #[inline(always)]
    pub fn blen256(&mut self) -> BLEN256_W<'_, REGISTER10_AXIBUSMODEREGISTER_SPEC> {
        BLEN256_W::new(self, 7)
    }
    #[doc = "Bit 13 - 1 KB Boundary Crossing Enable for the GMACAXI Master When set, the GMACAXI master performs burst transfers that do not cross 1 KB boundary When reset, the GMACAXI master performs burst transfers that do not cross 4 KB boundary"]
    #[inline(always)]
    pub fn onekbbe(&mut self) -> ONEKBBE_W<'_, REGISTER10_AXIBUSMODEREGISTER_SPEC> {
        ONEKBBE_W::new(self, 13)
    }
    #[doc = "Bits 16:19 - AXI Maximum Read Outstanding Request Limit This value limits the maximum outstanding request on the AXI read interface Maximum outstanding requests = RD_OSR_LMT+1 Note: Bit 18 is reserved if AXI_GM_MAX_RD_REQUESTS = 4 Bit 19 is reserved if AXI_GM_MAX_RD_REQUESTS != 16 15:14 Reserved 00 RO"]
    #[inline(always)]
    pub fn rd_osr_lmt(&mut self) -> RD_OSR_LMT_W<'_, REGISTER10_AXIBUSMODEREGISTER_SPEC> {
        RD_OSR_LMT_W::new(self, 16)
    }
    #[doc = "Bits 20:23 - AXI Maximum Write Outstanding Request Limit This value limits the maximum outstanding request on the AXI write interface Maximum outstanding requests = WR_OSR_LMT+1 Note: Bit 22 is reserved if AXI_GM_MAX_WR_REQUESTS = 4 Bit 23 bit is reserved if AXI_GM_MAX_WR_REQUESTS != 16"]
    #[inline(always)]
    pub fn wr_osr_lmt(&mut self) -> WR_OSR_LMT_W<'_, REGISTER10_AXIBUSMODEREGISTER_SPEC> {
        WR_OSR_LMT_W::new(self, 20)
    }
    #[doc = "Bit 30 - Unlock on Magic Packet or Remote WakeUp Frame When set to 1, this bit enables the GMACAXI to come out of the LPI mode only when the magic packet or remote wakeup frame is received When set to 0, this bit enables the GMACAXI to come out of LPI mode when any frame is received 29:24 Reserved 00H RO"]
    #[inline(always)]
    pub fn lpi_xit_frm(&mut self) -> LPI_XIT_FRM_W<'_, REGISTER10_AXIBUSMODEREGISTER_SPEC> {
        LPI_XIT_FRM_W::new(self, 30)
    }
    #[doc = "Bit 31 - Enable Low Power Interface _LPI_ When set to 1, this bit enables the LPI mode supported by the GMACAXI configuration and accepts the LPI request from the AXI System Clock controller When set to 0, this bit disables the LPI mode and always denies the LPI request from the AXI System Clock controller"]
    #[inline(always)]
    pub fn en_lpi(&mut self) -> EN_LPI_W<'_, REGISTER10_AXIBUSMODEREGISTER_SPEC> {
        EN_LPI_W::new(self, 31)
    }
}
#[doc = "Controls AXI master behavior _mainly controls burst splitting and number of outstanding requests_\n\nYou can [`read`](crate::Reg::read) this register and get [`register10_axibusmoderegister::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register10_axibusmoderegister::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REGISTER10_AXIBUSMODEREGISTER_SPEC;
impl crate::RegisterSpec for REGISTER10_AXIBUSMODEREGISTER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`register10_axibusmoderegister::R`](R) reader structure"]
impl crate::Readable for REGISTER10_AXIBUSMODEREGISTER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`register10_axibusmoderegister::W`](W) writer structure"]
impl crate::Writable for REGISTER10_AXIBUSMODEREGISTER_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REGISTER10_AXIBUSMODEREGISTER to value 0x0011_0001"]
impl crate::Resettable for REGISTER10_AXIBUSMODEREGISTER_SPEC {
    const RESET_VALUE: u32 = 0x0011_0001;
}
