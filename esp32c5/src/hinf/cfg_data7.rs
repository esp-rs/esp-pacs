#[doc = "Register `CFG_DATA7` reader"]
pub type R = crate::R<CFG_DATA7_SPEC>;
#[doc = "Register `CFG_DATA7` writer"]
pub type W = crate::W<CFG_DATA7_SPEC>;
#[doc = "Field `PIN_STATE` reader - "]
pub type PIN_STATE_R = crate::FieldReader;
#[doc = "Field `PIN_STATE` writer - "]
pub type PIN_STATE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CHIP_STATE` reader - "]
pub type CHIP_STATE_R = crate::FieldReader;
#[doc = "Field `CHIP_STATE` writer - "]
pub type CHIP_STATE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SDIO_RST` reader - "]
pub type SDIO_RST_R = crate::BitReader;
#[doc = "Field `SDIO_RST` writer - "]
pub type SDIO_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDIO_IOREADY0` reader - "]
pub type SDIO_IOREADY0_R = crate::BitReader;
#[doc = "Field `SDIO_IOREADY0` writer - "]
pub type SDIO_IOREADY0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDIO_MEM_PD` reader - "]
pub type SDIO_MEM_PD_R = crate::BitReader;
#[doc = "Field `SDIO_MEM_PD` writer - "]
pub type SDIO_MEM_PD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ESDIO_DATA1_INT_EN` reader - "]
pub type ESDIO_DATA1_INT_EN_R = crate::BitReader;
#[doc = "Field `ESDIO_DATA1_INT_EN` writer - "]
pub type ESDIO_DATA1_INT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDIO_SWITCH_VOLT_SW` reader - "]
pub type SDIO_SWITCH_VOLT_SW_R = crate::BitReader;
#[doc = "Field `SDIO_SWITCH_VOLT_SW` writer - "]
pub type SDIO_SWITCH_VOLT_SW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DDR50_BLK_LEN_FIX_EN` reader - "]
pub type DDR50_BLK_LEN_FIX_EN_R = crate::BitReader;
#[doc = "Field `DDR50_BLK_LEN_FIX_EN` writer - "]
pub type DDR50_BLK_LEN_FIX_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_EN` reader - "]
pub type CLK_EN_R = crate::BitReader;
#[doc = "Field `CLK_EN` writer - "]
pub type CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDDR50` reader - "]
pub type SDDR50_R = crate::BitReader;
#[doc = "Field `SDDR50` writer - "]
pub type SDDR50_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSDR104` reader - "]
pub type SSDR104_R = crate::BitReader;
#[doc = "Field `SSDR104` writer - "]
pub type SSDR104_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSDR50` reader - "]
pub type SSDR50_R = crate::BitReader;
#[doc = "Field `SSDR50` writer - "]
pub type SSDR50_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDTD` reader - "]
pub type SDTD_R = crate::BitReader;
#[doc = "Field `SDTD` writer - "]
pub type SDTD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDTA` reader - "]
pub type SDTA_R = crate::BitReader;
#[doc = "Field `SDTA` writer - "]
pub type SDTA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDTC` reader - "]
pub type SDTC_R = crate::BitReader;
#[doc = "Field `SDTC` writer - "]
pub type SDTC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAI` reader - "]
pub type SAI_R = crate::BitReader;
#[doc = "Field `SAI` writer - "]
pub type SAI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDIO_WAKEUP_CLR` reader - "]
pub type SDIO_WAKEUP_CLR_R = crate::BitReader;
#[doc = "Field `SDIO_WAKEUP_CLR` writer - "]
pub type SDIO_WAKEUP_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn pin_state(&self) -> PIN_STATE_R {
        PIN_STATE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn chip_state(&self) -> CHIP_STATE_R {
        CHIP_STATE_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn sdio_rst(&self) -> SDIO_RST_R {
        SDIO_RST_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn sdio_ioready0(&self) -> SDIO_IOREADY0_R {
        SDIO_IOREADY0_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn sdio_mem_pd(&self) -> SDIO_MEM_PD_R {
        SDIO_MEM_PD_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn esdio_data1_int_en(&self) -> ESDIO_DATA1_INT_EN_R {
        ESDIO_DATA1_INT_EN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn sdio_switch_volt_sw(&self) -> SDIO_SWITCH_VOLT_SW_R {
        SDIO_SWITCH_VOLT_SW_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn ddr50_blk_len_fix_en(&self) -> DDR50_BLK_LEN_FIX_EN_R {
        DDR50_BLK_LEN_FIX_EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn clk_en(&self) -> CLK_EN_R {
        CLK_EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn sddr50(&self) -> SDDR50_R {
        SDDR50_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn ssdr104(&self) -> SSDR104_R {
        SSDR104_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn ssdr50(&self) -> SSDR50_R {
        SSDR50_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn sdtd(&self) -> SDTD_R {
        SDTD_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn sdta(&self) -> SDTA_R {
        SDTA_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn sdtc(&self) -> SDTC_R {
        SDTC_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn sai(&self) -> SAI_R {
        SAI_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn sdio_wakeup_clr(&self) -> SDIO_WAKEUP_CLR_R {
        SDIO_WAKEUP_CLR_R::new(((self.bits >> 30) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFG_DATA7")
            .field("pin_state", &self.pin_state())
            .field("chip_state", &self.chip_state())
            .field("sdio_rst", &self.sdio_rst())
            .field("sdio_ioready0", &self.sdio_ioready0())
            .field("sdio_mem_pd", &self.sdio_mem_pd())
            .field("esdio_data1_int_en", &self.esdio_data1_int_en())
            .field("sdio_switch_volt_sw", &self.sdio_switch_volt_sw())
            .field("ddr50_blk_len_fix_en", &self.ddr50_blk_len_fix_en())
            .field("clk_en", &self.clk_en())
            .field("sddr50", &self.sddr50())
            .field("ssdr104", &self.ssdr104())
            .field("ssdr50", &self.ssdr50())
            .field("sdtd", &self.sdtd())
            .field("sdta", &self.sdta())
            .field("sdtc", &self.sdtc())
            .field("sai", &self.sai())
            .field("sdio_wakeup_clr", &self.sdio_wakeup_clr())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn pin_state(&mut self) -> PIN_STATE_W<'_, CFG_DATA7_SPEC> {
        PIN_STATE_W::new(self, 0)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn chip_state(&mut self) -> CHIP_STATE_W<'_, CFG_DATA7_SPEC> {
        CHIP_STATE_W::new(self, 8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn sdio_rst(&mut self) -> SDIO_RST_W<'_, CFG_DATA7_SPEC> {
        SDIO_RST_W::new(self, 16)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn sdio_ioready0(&mut self) -> SDIO_IOREADY0_W<'_, CFG_DATA7_SPEC> {
        SDIO_IOREADY0_W::new(self, 17)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn sdio_mem_pd(&mut self) -> SDIO_MEM_PD_W<'_, CFG_DATA7_SPEC> {
        SDIO_MEM_PD_W::new(self, 18)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn esdio_data1_int_en(&mut self) -> ESDIO_DATA1_INT_EN_W<'_, CFG_DATA7_SPEC> {
        ESDIO_DATA1_INT_EN_W::new(self, 19)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn sdio_switch_volt_sw(&mut self) -> SDIO_SWITCH_VOLT_SW_W<'_, CFG_DATA7_SPEC> {
        SDIO_SWITCH_VOLT_SW_W::new(self, 20)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn ddr50_blk_len_fix_en(&mut self) -> DDR50_BLK_LEN_FIX_EN_W<'_, CFG_DATA7_SPEC> {
        DDR50_BLK_LEN_FIX_EN_W::new(self, 21)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn clk_en(&mut self) -> CLK_EN_W<'_, CFG_DATA7_SPEC> {
        CLK_EN_W::new(self, 22)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn sddr50(&mut self) -> SDDR50_W<'_, CFG_DATA7_SPEC> {
        SDDR50_W::new(self, 23)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn ssdr104(&mut self) -> SSDR104_W<'_, CFG_DATA7_SPEC> {
        SSDR104_W::new(self, 24)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn ssdr50(&mut self) -> SSDR50_W<'_, CFG_DATA7_SPEC> {
        SSDR50_W::new(self, 25)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn sdtd(&mut self) -> SDTD_W<'_, CFG_DATA7_SPEC> {
        SDTD_W::new(self, 26)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn sdta(&mut self) -> SDTA_W<'_, CFG_DATA7_SPEC> {
        SDTA_W::new(self, 27)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn sdtc(&mut self) -> SDTC_W<'_, CFG_DATA7_SPEC> {
        SDTC_W::new(self, 28)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn sai(&mut self) -> SAI_W<'_, CFG_DATA7_SPEC> {
        SAI_W::new(self, 29)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn sdio_wakeup_clr(&mut self) -> SDIO_WAKEUP_CLR_W<'_, CFG_DATA7_SPEC> {
        SDIO_WAKEUP_CLR_W::new(self, 30)
    }
}
#[doc = "CFG_DATA7\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_data7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_data7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFG_DATA7_SPEC;
impl crate::RegisterSpec for CFG_DATA7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_data7::R`](R) reader structure"]
impl crate::Readable for CFG_DATA7_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cfg_data7::W`](W) writer structure"]
impl crate::Writable for CFG_DATA7_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CFG_DATA7 to value 0x03a0_0000"]
impl crate::Resettable for CFG_DATA7_SPEC {
    const RESET_VALUE: u32 = 0x03a0_0000;
}
