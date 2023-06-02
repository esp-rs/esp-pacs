#[doc = "Register `CFG_DATA7` reader"]
pub struct R(crate::R<CFG_DATA7_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFG_DATA7_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFG_DATA7_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFG_DATA7_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFG_DATA7` writer"]
pub struct W(crate::W<CFG_DATA7_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFG_DATA7_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CFG_DATA7_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFG_DATA7_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PIN_STATE` reader - configure cis addr 318 and 574"]
pub type PIN_STATE_R = crate::FieldReader;
#[doc = "Field `PIN_STATE` writer - configure cis addr 318 and 574"]
pub type PIN_STATE_W<'a, const O: u8> = crate::FieldWriter<'a, CFG_DATA7_SPEC, 8, O>;
#[doc = "Field `CHIP_STATE` reader - configure cis addr 312, 315, 568 and 571"]
pub type CHIP_STATE_R = crate::FieldReader;
#[doc = "Field `CHIP_STATE` writer - configure cis addr 312, 315, 568 and 571"]
pub type CHIP_STATE_W<'a, const O: u8> = crate::FieldWriter<'a, CFG_DATA7_SPEC, 8, O>;
#[doc = "Field `SDIO_RST` reader - soft reset control for sdio module"]
pub type SDIO_RST_R = crate::BitReader;
#[doc = "Field `SDIO_RST` writer - soft reset control for sdio module"]
pub type SDIO_RST_W<'a, const O: u8> = crate::BitWriter<'a, CFG_DATA7_SPEC, O>;
#[doc = "Field `SDIO_IOREADY0` reader - sdio io ready, high enable"]
pub type SDIO_IOREADY0_R = crate::BitReader;
#[doc = "Field `SDIO_IOREADY0` writer - sdio io ready, high enable"]
pub type SDIO_IOREADY0_W<'a, const O: u8> = crate::BitWriter<'a, CFG_DATA7_SPEC, O>;
#[doc = "Field `SDIO_MEM_PD` reader - sdio memory power down, high active"]
pub type SDIO_MEM_PD_R = crate::BitReader;
#[doc = "Field `SDIO_MEM_PD` writer - sdio memory power down, high active"]
pub type SDIO_MEM_PD_W<'a, const O: u8> = crate::BitWriter<'a, CFG_DATA7_SPEC, O>;
#[doc = "Field `ESDIO_DATA1_INT_EN` reader - enable sdio interrupt on data1 line"]
pub type ESDIO_DATA1_INT_EN_R = crate::BitReader;
#[doc = "Field `ESDIO_DATA1_INT_EN` writer - enable sdio interrupt on data1 line"]
pub type ESDIO_DATA1_INT_EN_W<'a, const O: u8> = crate::BitWriter<'a, CFG_DATA7_SPEC, O>;
#[doc = "Field `SDIO_SWITCH_VOLT_SW` reader - control switch voltage change to 1.8V by software. 0:3.3V,1:1.8V"]
pub type SDIO_SWITCH_VOLT_SW_R = crate::BitReader;
#[doc = "Field `SDIO_SWITCH_VOLT_SW` writer - control switch voltage change to 1.8V by software. 0:3.3V,1:1.8V"]
pub type SDIO_SWITCH_VOLT_SW_W<'a, const O: u8> = crate::BitWriter<'a, CFG_DATA7_SPEC, O>;
#[doc = "Field `DDR50_BLK_LEN_FIX_EN` reader - enable block length to be fixed to 512 bytes in ddr50 mode"]
pub type DDR50_BLK_LEN_FIX_EN_R = crate::BitReader;
#[doc = "Field `DDR50_BLK_LEN_FIX_EN` writer - enable block length to be fixed to 512 bytes in ddr50 mode"]
pub type DDR50_BLK_LEN_FIX_EN_W<'a, const O: u8> = crate::BitWriter<'a, CFG_DATA7_SPEC, O>;
#[doc = "Field `CLK_EN` reader - sdio apb clock for configuration force on control:0-gating,1-force on."]
pub type CLK_EN_R = crate::BitReader;
#[doc = "Field `CLK_EN` writer - sdio apb clock for configuration force on control:0-gating,1-force on."]
pub type CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, CFG_DATA7_SPEC, O>;
#[doc = "Field `SDDR50` reader - configure if support sdr50 mode in cccr"]
pub type SDDR50_R = crate::BitReader;
#[doc = "Field `SDDR50` writer - configure if support sdr50 mode in cccr"]
pub type SDDR50_W<'a, const O: u8> = crate::BitWriter<'a, CFG_DATA7_SPEC, O>;
#[doc = "Field `SSDR104` reader - configure if support sdr104 mode in cccr"]
pub type SSDR104_R = crate::BitReader;
#[doc = "Field `SSDR104` writer - configure if support sdr104 mode in cccr"]
pub type SSDR104_W<'a, const O: u8> = crate::BitWriter<'a, CFG_DATA7_SPEC, O>;
#[doc = "Field `SSDR50` reader - configure if support ddr50 mode in cccr"]
pub type SSDR50_R = crate::BitReader;
#[doc = "Field `SSDR50` writer - configure if support ddr50 mode in cccr"]
pub type SSDR50_W<'a, const O: u8> = crate::BitWriter<'a, CFG_DATA7_SPEC, O>;
#[doc = "Field `SDTD` reader - configure if support driver type D in cccr"]
pub type SDTD_R = crate::BitReader;
#[doc = "Field `SDTD` writer - configure if support driver type D in cccr"]
pub type SDTD_W<'a, const O: u8> = crate::BitWriter<'a, CFG_DATA7_SPEC, O>;
#[doc = "Field `SDTA` reader - configure if support driver type A in cccr"]
pub type SDTA_R = crate::BitReader;
#[doc = "Field `SDTA` writer - configure if support driver type A in cccr"]
pub type SDTA_W<'a, const O: u8> = crate::BitWriter<'a, CFG_DATA7_SPEC, O>;
#[doc = "Field `SDTC` reader - configure if support driver type C in cccr"]
pub type SDTC_R = crate::BitReader;
#[doc = "Field `SDTC` writer - configure if support driver type C in cccr"]
pub type SDTC_W<'a, const O: u8> = crate::BitWriter<'a, CFG_DATA7_SPEC, O>;
#[doc = "Field `SAI` reader - configure if support asynchronous interrupt in cccr"]
pub type SAI_R = crate::BitReader;
#[doc = "Field `SAI` writer - configure if support asynchronous interrupt in cccr"]
pub type SAI_W<'a, const O: u8> = crate::BitWriter<'a, CFG_DATA7_SPEC, O>;
#[doc = "Field `SDIO_WAKEUP_CLR` writer - clear sdio_wake_up signal after the chip wakes up"]
pub type SDIO_WAKEUP_CLR_W<'a, const O: u8> = crate::BitWriter<'a, CFG_DATA7_SPEC, O>;
impl R {
    #[doc = "Bits 0:7 - configure cis addr 318 and 574"]
    #[inline(always)]
    pub fn pin_state(&self) -> PIN_STATE_R {
        PIN_STATE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - configure cis addr 312, 315, 568 and 571"]
    #[inline(always)]
    pub fn chip_state(&self) -> CHIP_STATE_R {
        CHIP_STATE_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16 - soft reset control for sdio module"]
    #[inline(always)]
    pub fn sdio_rst(&self) -> SDIO_RST_R {
        SDIO_RST_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - sdio io ready, high enable"]
    #[inline(always)]
    pub fn sdio_ioready0(&self) -> SDIO_IOREADY0_R {
        SDIO_IOREADY0_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - sdio memory power down, high active"]
    #[inline(always)]
    pub fn sdio_mem_pd(&self) -> SDIO_MEM_PD_R {
        SDIO_MEM_PD_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - enable sdio interrupt on data1 line"]
    #[inline(always)]
    pub fn esdio_data1_int_en(&self) -> ESDIO_DATA1_INT_EN_R {
        ESDIO_DATA1_INT_EN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - control switch voltage change to 1.8V by software. 0:3.3V,1:1.8V"]
    #[inline(always)]
    pub fn sdio_switch_volt_sw(&self) -> SDIO_SWITCH_VOLT_SW_R {
        SDIO_SWITCH_VOLT_SW_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - enable block length to be fixed to 512 bytes in ddr50 mode"]
    #[inline(always)]
    pub fn ddr50_blk_len_fix_en(&self) -> DDR50_BLK_LEN_FIX_EN_R {
        DDR50_BLK_LEN_FIX_EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - sdio apb clock for configuration force on control:0-gating,1-force on."]
    #[inline(always)]
    pub fn clk_en(&self) -> CLK_EN_R {
        CLK_EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - configure if support sdr50 mode in cccr"]
    #[inline(always)]
    pub fn sddr50(&self) -> SDDR50_R {
        SDDR50_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - configure if support sdr104 mode in cccr"]
    #[inline(always)]
    pub fn ssdr104(&self) -> SSDR104_R {
        SSDR104_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - configure if support ddr50 mode in cccr"]
    #[inline(always)]
    pub fn ssdr50(&self) -> SSDR50_R {
        SSDR50_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - configure if support driver type D in cccr"]
    #[inline(always)]
    pub fn sdtd(&self) -> SDTD_R {
        SDTD_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - configure if support driver type A in cccr"]
    #[inline(always)]
    pub fn sdta(&self) -> SDTA_R {
        SDTA_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - configure if support driver type C in cccr"]
    #[inline(always)]
    pub fn sdtc(&self) -> SDTC_R {
        SDTC_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - configure if support asynchronous interrupt in cccr"]
    #[inline(always)]
    pub fn sai(&self) -> SAI_R {
        SAI_R::new(((self.bits >> 29) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFG_DATA7")
            .field("pin_state", &format_args!("{}", self.pin_state().bits()))
            .field("chip_state", &format_args!("{}", self.chip_state().bits()))
            .field("sdio_rst", &format_args!("{}", self.sdio_rst().bit()))
            .field(
                "sdio_ioready0",
                &format_args!("{}", self.sdio_ioready0().bit()),
            )
            .field("sdio_mem_pd", &format_args!("{}", self.sdio_mem_pd().bit()))
            .field(
                "esdio_data1_int_en",
                &format_args!("{}", self.esdio_data1_int_en().bit()),
            )
            .field(
                "sdio_switch_volt_sw",
                &format_args!("{}", self.sdio_switch_volt_sw().bit()),
            )
            .field(
                "ddr50_blk_len_fix_en",
                &format_args!("{}", self.ddr50_blk_len_fix_en().bit()),
            )
            .field("clk_en", &format_args!("{}", self.clk_en().bit()))
            .field("sddr50", &format_args!("{}", self.sddr50().bit()))
            .field("ssdr104", &format_args!("{}", self.ssdr104().bit()))
            .field("ssdr50", &format_args!("{}", self.ssdr50().bit()))
            .field("sdtd", &format_args!("{}", self.sdtd().bit()))
            .field("sdta", &format_args!("{}", self.sdta().bit()))
            .field("sdtc", &format_args!("{}", self.sdtc().bit()))
            .field("sai", &format_args!("{}", self.sai().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CFG_DATA7_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7 - configure cis addr 318 and 574"]
    #[inline(always)]
    #[must_use]
    pub fn pin_state(&mut self) -> PIN_STATE_W<0> {
        PIN_STATE_W::new(self)
    }
    #[doc = "Bits 8:15 - configure cis addr 312, 315, 568 and 571"]
    #[inline(always)]
    #[must_use]
    pub fn chip_state(&mut self) -> CHIP_STATE_W<8> {
        CHIP_STATE_W::new(self)
    }
    #[doc = "Bit 16 - soft reset control for sdio module"]
    #[inline(always)]
    #[must_use]
    pub fn sdio_rst(&mut self) -> SDIO_RST_W<16> {
        SDIO_RST_W::new(self)
    }
    #[doc = "Bit 17 - sdio io ready, high enable"]
    #[inline(always)]
    #[must_use]
    pub fn sdio_ioready0(&mut self) -> SDIO_IOREADY0_W<17> {
        SDIO_IOREADY0_W::new(self)
    }
    #[doc = "Bit 18 - sdio memory power down, high active"]
    #[inline(always)]
    #[must_use]
    pub fn sdio_mem_pd(&mut self) -> SDIO_MEM_PD_W<18> {
        SDIO_MEM_PD_W::new(self)
    }
    #[doc = "Bit 19 - enable sdio interrupt on data1 line"]
    #[inline(always)]
    #[must_use]
    pub fn esdio_data1_int_en(&mut self) -> ESDIO_DATA1_INT_EN_W<19> {
        ESDIO_DATA1_INT_EN_W::new(self)
    }
    #[doc = "Bit 20 - control switch voltage change to 1.8V by software. 0:3.3V,1:1.8V"]
    #[inline(always)]
    #[must_use]
    pub fn sdio_switch_volt_sw(&mut self) -> SDIO_SWITCH_VOLT_SW_W<20> {
        SDIO_SWITCH_VOLT_SW_W::new(self)
    }
    #[doc = "Bit 21 - enable block length to be fixed to 512 bytes in ddr50 mode"]
    #[inline(always)]
    #[must_use]
    pub fn ddr50_blk_len_fix_en(&mut self) -> DDR50_BLK_LEN_FIX_EN_W<21> {
        DDR50_BLK_LEN_FIX_EN_W::new(self)
    }
    #[doc = "Bit 22 - sdio apb clock for configuration force on control:0-gating,1-force on."]
    #[inline(always)]
    #[must_use]
    pub fn clk_en(&mut self) -> CLK_EN_W<22> {
        CLK_EN_W::new(self)
    }
    #[doc = "Bit 23 - configure if support sdr50 mode in cccr"]
    #[inline(always)]
    #[must_use]
    pub fn sddr50(&mut self) -> SDDR50_W<23> {
        SDDR50_W::new(self)
    }
    #[doc = "Bit 24 - configure if support sdr104 mode in cccr"]
    #[inline(always)]
    #[must_use]
    pub fn ssdr104(&mut self) -> SSDR104_W<24> {
        SSDR104_W::new(self)
    }
    #[doc = "Bit 25 - configure if support ddr50 mode in cccr"]
    #[inline(always)]
    #[must_use]
    pub fn ssdr50(&mut self) -> SSDR50_W<25> {
        SSDR50_W::new(self)
    }
    #[doc = "Bit 26 - configure if support driver type D in cccr"]
    #[inline(always)]
    #[must_use]
    pub fn sdtd(&mut self) -> SDTD_W<26> {
        SDTD_W::new(self)
    }
    #[doc = "Bit 27 - configure if support driver type A in cccr"]
    #[inline(always)]
    #[must_use]
    pub fn sdta(&mut self) -> SDTA_W<27> {
        SDTA_W::new(self)
    }
    #[doc = "Bit 28 - configure if support driver type C in cccr"]
    #[inline(always)]
    #[must_use]
    pub fn sdtc(&mut self) -> SDTC_W<28> {
        SDTC_W::new(self)
    }
    #[doc = "Bit 29 - configure if support asynchronous interrupt in cccr"]
    #[inline(always)]
    #[must_use]
    pub fn sai(&mut self) -> SAI_W<29> {
        SAI_W::new(self)
    }
    #[doc = "Bit 30 - clear sdio_wake_up signal after the chip wakes up"]
    #[inline(always)]
    #[must_use]
    pub fn sdio_wakeup_clr(&mut self) -> SDIO_WAKEUP_CLR_W<30> {
        SDIO_WAKEUP_CLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SDIO configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfg_data7](index.html) module"]
pub struct CFG_DATA7_SPEC;
impl crate::RegisterSpec for CFG_DATA7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfg_data7::R](R) reader structure"]
impl crate::Readable for CFG_DATA7_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfg_data7::W](W) writer structure"]
impl crate::Writable for CFG_DATA7_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFG_DATA7 to value 0x2382_0000"]
impl crate::Resettable for CFG_DATA7_SPEC {
    const RESET_VALUE: Self::Ux = 0x2382_0000;
}
