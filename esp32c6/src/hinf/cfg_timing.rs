#[doc = "Register `CFG_TIMING` reader"]
pub type R = crate::R<CFG_TIMING_SPEC>;
#[doc = "Register `CFG_TIMING` writer"]
pub type W = crate::W<CFG_TIMING_SPEC>;
#[doc = "Field `NCRC` reader - configure Ncrc parameter in sdr50/104 mode, no more than 6."]
pub type NCRC_R = crate::FieldReader;
#[doc = "Field `NCRC` writer - configure Ncrc parameter in sdr50/104 mode, no more than 6."]
pub type NCRC_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PST_END_CMD_LOW_VALUE` reader - configure cycles to lower cmd after voltage is changed to 1.8V."]
pub type PST_END_CMD_LOW_VALUE_R = crate::FieldReader;
#[doc = "Field `PST_END_CMD_LOW_VALUE` writer - configure cycles to lower cmd after voltage is changed to 1.8V."]
pub type PST_END_CMD_LOW_VALUE_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `PST_END_DATA_LOW_VALUE` reader - configure cycles to lower data after voltage is changed to 1.8V."]
pub type PST_END_DATA_LOW_VALUE_R = crate::FieldReader;
#[doc = "Field `PST_END_DATA_LOW_VALUE` writer - configure cycles to lower data after voltage is changed to 1.8V."]
pub type PST_END_DATA_LOW_VALUE_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `SDCLK_STOP_THRES` reader - Configure the number of cycles of module clk to judge sdclk has stopped"]
pub type SDCLK_STOP_THRES_R = crate::FieldReader<u16>;
#[doc = "Field `SDCLK_STOP_THRES` writer - Configure the number of cycles of module clk to judge sdclk has stopped"]
pub type SDCLK_STOP_THRES_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `SAMPLE_CLK_DIVIDER` reader - module clk divider to sample sdclk"]
pub type SAMPLE_CLK_DIVIDER_R = crate::FieldReader;
#[doc = "Field `SAMPLE_CLK_DIVIDER` writer - module clk divider to sample sdclk"]
pub type SAMPLE_CLK_DIVIDER_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:2 - configure Ncrc parameter in sdr50/104 mode, no more than 6."]
    #[inline(always)]
    pub fn ncrc(&self) -> NCRC_R {
        NCRC_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:9 - configure cycles to lower cmd after voltage is changed to 1.8V."]
    #[inline(always)]
    pub fn pst_end_cmd_low_value(&self) -> PST_END_CMD_LOW_VALUE_R {
        PST_END_CMD_LOW_VALUE_R::new(((self.bits >> 3) & 0x7f) as u8)
    }
    #[doc = "Bits 10:15 - configure cycles to lower data after voltage is changed to 1.8V."]
    #[inline(always)]
    pub fn pst_end_data_low_value(&self) -> PST_END_DATA_LOW_VALUE_R {
        PST_END_DATA_LOW_VALUE_R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bits 16:26 - Configure the number of cycles of module clk to judge sdclk has stopped"]
    #[inline(always)]
    pub fn sdclk_stop_thres(&self) -> SDCLK_STOP_THRES_R {
        SDCLK_STOP_THRES_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
    #[doc = "Bits 28:31 - module clk divider to sample sdclk"]
    #[inline(always)]
    pub fn sample_clk_divider(&self) -> SAMPLE_CLK_DIVIDER_R {
        SAMPLE_CLK_DIVIDER_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFG_TIMING")
            .field("ncrc", &format_args!("{}", self.ncrc().bits()))
            .field(
                "pst_end_cmd_low_value",
                &format_args!("{}", self.pst_end_cmd_low_value().bits()),
            )
            .field(
                "pst_end_data_low_value",
                &format_args!("{}", self.pst_end_data_low_value().bits()),
            )
            .field(
                "sdclk_stop_thres",
                &format_args!("{}", self.sdclk_stop_thres().bits()),
            )
            .field(
                "sample_clk_divider",
                &format_args!("{}", self.sample_clk_divider().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CFG_TIMING_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:2 - configure Ncrc parameter in sdr50/104 mode, no more than 6."]
    #[inline(always)]
    #[must_use]
    pub fn ncrc(&mut self) -> NCRC_W<CFG_TIMING_SPEC> {
        NCRC_W::new(self, 0)
    }
    #[doc = "Bits 3:9 - configure cycles to lower cmd after voltage is changed to 1.8V."]
    #[inline(always)]
    #[must_use]
    pub fn pst_end_cmd_low_value(&mut self) -> PST_END_CMD_LOW_VALUE_W<CFG_TIMING_SPEC> {
        PST_END_CMD_LOW_VALUE_W::new(self, 3)
    }
    #[doc = "Bits 10:15 - configure cycles to lower data after voltage is changed to 1.8V."]
    #[inline(always)]
    #[must_use]
    pub fn pst_end_data_low_value(&mut self) -> PST_END_DATA_LOW_VALUE_W<CFG_TIMING_SPEC> {
        PST_END_DATA_LOW_VALUE_W::new(self, 10)
    }
    #[doc = "Bits 16:26 - Configure the number of cycles of module clk to judge sdclk has stopped"]
    #[inline(always)]
    #[must_use]
    pub fn sdclk_stop_thres(&mut self) -> SDCLK_STOP_THRES_W<CFG_TIMING_SPEC> {
        SDCLK_STOP_THRES_W::new(self, 16)
    }
    #[doc = "Bits 28:31 - module clk divider to sample sdclk"]
    #[inline(always)]
    #[must_use]
    pub fn sample_clk_divider(&mut self) -> SAMPLE_CLK_DIVIDER_W<CFG_TIMING_SPEC> {
        SAMPLE_CLK_DIVIDER_W::new(self, 28)
    }
}
#[doc = "Timing configuration registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_timing::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_timing::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFG_TIMING_SPEC;
impl crate::RegisterSpec for CFG_TIMING_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_timing::R`](R) reader structure"]
impl crate::Readable for CFG_TIMING_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cfg_timing::W`](W) writer structure"]
impl crate::Writable for CFG_TIMING_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_TIMING to value 0x1578_0812"]
impl crate::Resettable for CFG_TIMING_SPEC {
    const RESET_VALUE: u32 = 0x1578_0812;
}
