#[doc = "Register `PAD_CFG` reader"]
pub type R = crate::R<PAD_CFG_SPEC>;
#[doc = "Register `PAD_CFG` writer"]
pub type W = crate::W<PAD_CFG_SPEC>;
#[doc = "Field `HS_MODE_PAD_0` reader - 1 to let DAC PAD 0 enter High speed mode"]
pub type HS_MODE_PAD_0_R = crate::BitReader;
#[doc = "Field `HS_MODE_PAD_0` writer - 1 to let DAC PAD 0 enter High speed mode"]
pub type HS_MODE_PAD_0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XPD_PAD_0` reader - 1 to power on DAC PAD 0"]
pub type XPD_PAD_0_R = crate::BitReader;
#[doc = "Field `XPD_PAD_0` writer - 1 to power on DAC PAD 0"]
pub type XPD_PAD_0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN_ADC_MUX_PAD_0` reader - 1 to enable adc mux for DAC PAD 0"]
pub type EN_ADC_MUX_PAD_0_R = crate::BitReader;
#[doc = "Field `EN_ADC_MUX_PAD_0` writer - 1 to enable adc mux for DAC PAD 0"]
pub type EN_ADC_MUX_PAD_0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSAMP_LS_PAD_0` reader - 1 to start sample and hold mode for DAC PAD 0"]
pub type DSAMP_LS_PAD_0_R = crate::BitReader;
#[doc = "Field `DSAMP_LS_PAD_0` writer - 1 to start sample and hold mode for DAC PAD 0"]
pub type DSAMP_LS_PAD_0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCAL_LS_PAD_0` reader - 1 to Calibrate the buffer of DAC PAD 0 (DBUF of pad 1 must be 1 during this operation)"]
pub type DCAL_LS_PAD_0_R = crate::BitReader;
#[doc = "Field `DCAL_LS_PAD_0` writer - 1 to Calibrate the buffer of DAC PAD 0 (DBUF of pad 1 must be 1 during this operation)"]
pub type DCAL_LS_PAD_0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBUF_LS_PAD_0` reader - 1 to turn on the buffer of DAC PAD 0"]
pub type DBUF_LS_PAD_0_R = crate::BitReader;
#[doc = "Field `DBUF_LS_PAD_0` writer - 1 to turn on the buffer of DAC PAD 0"]
pub type DBUF_LS_PAD_0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ON_LS_PAD_0` reader - 1 to turn on the DAC PAD 0 during sample and hold mode"]
pub type ON_LS_PAD_0_R = crate::BitReader;
#[doc = "Field `ON_LS_PAD_0` writer - 1 to turn on the DAC PAD 0 during sample and hold mode"]
pub type ON_LS_PAD_0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HS_MODE_PAD_1` reader - 1 to let DAC PAD 1 enter High speed mode"]
pub type HS_MODE_PAD_1_R = crate::BitReader;
#[doc = "Field `HS_MODE_PAD_1` writer - 1 to let DAC PAD 1 enter High speed mode"]
pub type HS_MODE_PAD_1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XPD_PAD_1` reader - 1 to power on DAC PAD 1"]
pub type XPD_PAD_1_R = crate::BitReader;
#[doc = "Field `XPD_PAD_1` writer - 1 to power on DAC PAD 1"]
pub type XPD_PAD_1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN_ADC_MUX_PAD_1` reader - 1 to enable adc mux for DAC PAD 1"]
pub type EN_ADC_MUX_PAD_1_R = crate::BitReader;
#[doc = "Field `EN_ADC_MUX_PAD_1` writer - 1 to enable adc mux for DAC PAD 1"]
pub type EN_ADC_MUX_PAD_1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSAMP_LS_PAD_1` reader - 1 to start sample and hold mode for DAC PAD 1"]
pub type DSAMP_LS_PAD_1_R = crate::BitReader;
#[doc = "Field `DSAMP_LS_PAD_1` writer - 1 to start sample and hold mode for DAC PAD 1"]
pub type DSAMP_LS_PAD_1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCAL_LS_PAD_1` reader - 1 to Calibrate the buffer of DAC PAD 1 (DBUF of pad 1 must be 1 during this operation)"]
pub type DCAL_LS_PAD_1_R = crate::BitReader;
#[doc = "Field `DCAL_LS_PAD_1` writer - 1 to Calibrate the buffer of DAC PAD 1 (DBUF of pad 1 must be 1 during this operation)"]
pub type DCAL_LS_PAD_1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBUF_LS_PAD_1` reader - 1 to turn on the buffer of DAC PAD 1"]
pub type DBUF_LS_PAD_1_R = crate::BitReader;
#[doc = "Field `DBUF_LS_PAD_1` writer - 1 to turn on the buffer of DAC PAD 1"]
pub type DBUF_LS_PAD_1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ON_LS_PAD_1` reader - 1 to turn on the DAC PAD 1 during sample and hold mode"]
pub type ON_LS_PAD_1_R = crate::BitReader;
#[doc = "Field `ON_LS_PAD_1` writer - 1 to turn on the DAC PAD 1 during sample and hold mode"]
pub type ON_LS_PAD_1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIV_NUM` reader - divide number between clk ls and clk hs"]
pub type DIV_NUM_R = crate::FieldReader<u16>;
#[doc = "Field `DIV_NUM` writer - divide number between clk ls and clk hs"]
pub type DIV_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `DBUF_RANGE_LS_PAD_0` reader - 1:when buffer is activated, allow using full code of maximum bitwidth 0:when buffer is not activated, should always be 0"]
pub type DBUF_RANGE_LS_PAD_0_R = crate::BitReader;
#[doc = "Field `DBUF_RANGE_LS_PAD_0` writer - 1:when buffer is activated, allow using full code of maximum bitwidth 0:when buffer is not activated, should always be 0"]
pub type DBUF_RANGE_LS_PAD_0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBUF_RANGE_LS_PAD_1` reader - 1:when buffer is activated, allow using full code of maximum bitwidth 0:when buffer is not activated, should always be 0"]
pub type DBUF_RANGE_LS_PAD_1_R = crate::BitReader;
#[doc = "Field `DBUF_RANGE_LS_PAD_1` writer - 1:when buffer is activated, allow using full code of maximum bitwidth 0:when buffer is not activated, should always be 0"]
pub type DBUF_RANGE_LS_PAD_1_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 1 to let DAC PAD 0 enter High speed mode"]
    #[inline(always)]
    pub fn hs_mode_pad_0(&self) -> HS_MODE_PAD_0_R {
        HS_MODE_PAD_0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1 to power on DAC PAD 0"]
    #[inline(always)]
    pub fn xpd_pad_0(&self) -> XPD_PAD_0_R {
        XPD_PAD_0_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 1 to enable adc mux for DAC PAD 0"]
    #[inline(always)]
    pub fn en_adc_mux_pad_0(&self) -> EN_ADC_MUX_PAD_0_R {
        EN_ADC_MUX_PAD_0_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 1 to start sample and hold mode for DAC PAD 0"]
    #[inline(always)]
    pub fn dsamp_ls_pad_0(&self) -> DSAMP_LS_PAD_0_R {
        DSAMP_LS_PAD_0_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 1 to Calibrate the buffer of DAC PAD 0 (DBUF of pad 1 must be 1 during this operation)"]
    #[inline(always)]
    pub fn dcal_ls_pad_0(&self) -> DCAL_LS_PAD_0_R {
        DCAL_LS_PAD_0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 1 to turn on the buffer of DAC PAD 0"]
    #[inline(always)]
    pub fn dbuf_ls_pad_0(&self) -> DBUF_LS_PAD_0_R {
        DBUF_LS_PAD_0_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 1 to turn on the DAC PAD 0 during sample and hold mode"]
    #[inline(always)]
    pub fn on_ls_pad_0(&self) -> ON_LS_PAD_0_R {
        ON_LS_PAD_0_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 1 to let DAC PAD 1 enter High speed mode"]
    #[inline(always)]
    pub fn hs_mode_pad_1(&self) -> HS_MODE_PAD_1_R {
        HS_MODE_PAD_1_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 1 to power on DAC PAD 1"]
    #[inline(always)]
    pub fn xpd_pad_1(&self) -> XPD_PAD_1_R {
        XPD_PAD_1_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 1 to enable adc mux for DAC PAD 1"]
    #[inline(always)]
    pub fn en_adc_mux_pad_1(&self) -> EN_ADC_MUX_PAD_1_R {
        EN_ADC_MUX_PAD_1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 1 to start sample and hold mode for DAC PAD 1"]
    #[inline(always)]
    pub fn dsamp_ls_pad_1(&self) -> DSAMP_LS_PAD_1_R {
        DSAMP_LS_PAD_1_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 1 to Calibrate the buffer of DAC PAD 1 (DBUF of pad 1 must be 1 during this operation)"]
    #[inline(always)]
    pub fn dcal_ls_pad_1(&self) -> DCAL_LS_PAD_1_R {
        DCAL_LS_PAD_1_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 1 to turn on the buffer of DAC PAD 1"]
    #[inline(always)]
    pub fn dbuf_ls_pad_1(&self) -> DBUF_LS_PAD_1_R {
        DBUF_LS_PAD_1_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 1 to turn on the DAC PAD 1 during sample and hold mode"]
    #[inline(always)]
    pub fn on_ls_pad_1(&self) -> ON_LS_PAD_1_R {
        ON_LS_PAD_1_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:29 - divide number between clk ls and clk hs"]
    #[inline(always)]
    pub fn div_num(&self) -> DIV_NUM_R {
        DIV_NUM_R::new(((self.bits >> 14) & 0xffff) as u16)
    }
    #[doc = "Bit 30 - 1:when buffer is activated, allow using full code of maximum bitwidth 0:when buffer is not activated, should always be 0"]
    #[inline(always)]
    pub fn dbuf_range_ls_pad_0(&self) -> DBUF_RANGE_LS_PAD_0_R {
        DBUF_RANGE_LS_PAD_0_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 1:when buffer is activated, allow using full code of maximum bitwidth 0:when buffer is not activated, should always be 0"]
    #[inline(always)]
    pub fn dbuf_range_ls_pad_1(&self) -> DBUF_RANGE_LS_PAD_1_R {
        DBUF_RANGE_LS_PAD_1_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PAD_CFG")
            .field("hs_mode_pad_0", &self.hs_mode_pad_0())
            .field("xpd_pad_0", &self.xpd_pad_0())
            .field("en_adc_mux_pad_0", &self.en_adc_mux_pad_0())
            .field("dsamp_ls_pad_0", &self.dsamp_ls_pad_0())
            .field("dcal_ls_pad_0", &self.dcal_ls_pad_0())
            .field("dbuf_ls_pad_0", &self.dbuf_ls_pad_0())
            .field("on_ls_pad_0", &self.on_ls_pad_0())
            .field("hs_mode_pad_1", &self.hs_mode_pad_1())
            .field("xpd_pad_1", &self.xpd_pad_1())
            .field("en_adc_mux_pad_1", &self.en_adc_mux_pad_1())
            .field("dsamp_ls_pad_1", &self.dsamp_ls_pad_1())
            .field("dcal_ls_pad_1", &self.dcal_ls_pad_1())
            .field("dbuf_ls_pad_1", &self.dbuf_ls_pad_1())
            .field("on_ls_pad_1", &self.on_ls_pad_1())
            .field("div_num", &self.div_num())
            .field("dbuf_range_ls_pad_0", &self.dbuf_range_ls_pad_0())
            .field("dbuf_range_ls_pad_1", &self.dbuf_range_ls_pad_1())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - 1 to let DAC PAD 0 enter High speed mode"]
    #[inline(always)]
    pub fn hs_mode_pad_0(&mut self) -> HS_MODE_PAD_0_W<'_, PAD_CFG_SPEC> {
        HS_MODE_PAD_0_W::new(self, 0)
    }
    #[doc = "Bit 1 - 1 to power on DAC PAD 0"]
    #[inline(always)]
    pub fn xpd_pad_0(&mut self) -> XPD_PAD_0_W<'_, PAD_CFG_SPEC> {
        XPD_PAD_0_W::new(self, 1)
    }
    #[doc = "Bit 2 - 1 to enable adc mux for DAC PAD 0"]
    #[inline(always)]
    pub fn en_adc_mux_pad_0(&mut self) -> EN_ADC_MUX_PAD_0_W<'_, PAD_CFG_SPEC> {
        EN_ADC_MUX_PAD_0_W::new(self, 2)
    }
    #[doc = "Bit 3 - 1 to start sample and hold mode for DAC PAD 0"]
    #[inline(always)]
    pub fn dsamp_ls_pad_0(&mut self) -> DSAMP_LS_PAD_0_W<'_, PAD_CFG_SPEC> {
        DSAMP_LS_PAD_0_W::new(self, 3)
    }
    #[doc = "Bit 4 - 1 to Calibrate the buffer of DAC PAD 0 (DBUF of pad 1 must be 1 during this operation)"]
    #[inline(always)]
    pub fn dcal_ls_pad_0(&mut self) -> DCAL_LS_PAD_0_W<'_, PAD_CFG_SPEC> {
        DCAL_LS_PAD_0_W::new(self, 4)
    }
    #[doc = "Bit 5 - 1 to turn on the buffer of DAC PAD 0"]
    #[inline(always)]
    pub fn dbuf_ls_pad_0(&mut self) -> DBUF_LS_PAD_0_W<'_, PAD_CFG_SPEC> {
        DBUF_LS_PAD_0_W::new(self, 5)
    }
    #[doc = "Bit 6 - 1 to turn on the DAC PAD 0 during sample and hold mode"]
    #[inline(always)]
    pub fn on_ls_pad_0(&mut self) -> ON_LS_PAD_0_W<'_, PAD_CFG_SPEC> {
        ON_LS_PAD_0_W::new(self, 6)
    }
    #[doc = "Bit 7 - 1 to let DAC PAD 1 enter High speed mode"]
    #[inline(always)]
    pub fn hs_mode_pad_1(&mut self) -> HS_MODE_PAD_1_W<'_, PAD_CFG_SPEC> {
        HS_MODE_PAD_1_W::new(self, 7)
    }
    #[doc = "Bit 8 - 1 to power on DAC PAD 1"]
    #[inline(always)]
    pub fn xpd_pad_1(&mut self) -> XPD_PAD_1_W<'_, PAD_CFG_SPEC> {
        XPD_PAD_1_W::new(self, 8)
    }
    #[doc = "Bit 9 - 1 to enable adc mux for DAC PAD 1"]
    #[inline(always)]
    pub fn en_adc_mux_pad_1(&mut self) -> EN_ADC_MUX_PAD_1_W<'_, PAD_CFG_SPEC> {
        EN_ADC_MUX_PAD_1_W::new(self, 9)
    }
    #[doc = "Bit 10 - 1 to start sample and hold mode for DAC PAD 1"]
    #[inline(always)]
    pub fn dsamp_ls_pad_1(&mut self) -> DSAMP_LS_PAD_1_W<'_, PAD_CFG_SPEC> {
        DSAMP_LS_PAD_1_W::new(self, 10)
    }
    #[doc = "Bit 11 - 1 to Calibrate the buffer of DAC PAD 1 (DBUF of pad 1 must be 1 during this operation)"]
    #[inline(always)]
    pub fn dcal_ls_pad_1(&mut self) -> DCAL_LS_PAD_1_W<'_, PAD_CFG_SPEC> {
        DCAL_LS_PAD_1_W::new(self, 11)
    }
    #[doc = "Bit 12 - 1 to turn on the buffer of DAC PAD 1"]
    #[inline(always)]
    pub fn dbuf_ls_pad_1(&mut self) -> DBUF_LS_PAD_1_W<'_, PAD_CFG_SPEC> {
        DBUF_LS_PAD_1_W::new(self, 12)
    }
    #[doc = "Bit 13 - 1 to turn on the DAC PAD 1 during sample and hold mode"]
    #[inline(always)]
    pub fn on_ls_pad_1(&mut self) -> ON_LS_PAD_1_W<'_, PAD_CFG_SPEC> {
        ON_LS_PAD_1_W::new(self, 13)
    }
    #[doc = "Bits 14:29 - divide number between clk ls and clk hs"]
    #[inline(always)]
    pub fn div_num(&mut self) -> DIV_NUM_W<'_, PAD_CFG_SPEC> {
        DIV_NUM_W::new(self, 14)
    }
    #[doc = "Bit 30 - 1:when buffer is activated, allow using full code of maximum bitwidth 0:when buffer is not activated, should always be 0"]
    #[inline(always)]
    pub fn dbuf_range_ls_pad_0(&mut self) -> DBUF_RANGE_LS_PAD_0_W<'_, PAD_CFG_SPEC> {
        DBUF_RANGE_LS_PAD_0_W::new(self, 30)
    }
    #[doc = "Bit 31 - 1:when buffer is activated, allow using full code of maximum bitwidth 0:when buffer is not activated, should always be 0"]
    #[inline(always)]
    pub fn dbuf_range_ls_pad_1(&mut self) -> DBUF_RANGE_LS_PAD_1_W<'_, PAD_CFG_SPEC> {
        DBUF_RANGE_LS_PAD_1_W::new(self, 31)
    }
}
#[doc = "configure dac pad register\n\nYou can [`read`](crate::Reg::read) this register and get [`pad_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PAD_CFG_SPEC;
impl crate::RegisterSpec for PAD_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pad_cfg::R`](R) reader structure"]
impl crate::Readable for PAD_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pad_cfg::W`](W) writer structure"]
impl crate::Writable for PAD_CFG_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PAD_CFG to value 0x4000"]
impl crate::Resettable for PAD_CFG_SPEC {
    const RESET_VALUE: u32 = 0x4000;
}
