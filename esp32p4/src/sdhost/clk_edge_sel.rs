#[doc = "Register `CLK_EDGE_SEL` reader"]
pub type R = crate::R<CLK_EDGE_SEL_SPEC>;
#[doc = "Register `CLK_EDGE_SEL` writer"]
pub type W = crate::W<CLK_EDGE_SEL_SPEC>;
#[doc = "Field `CCLKIN_EDGE_DRV_SEL` reader - It's used to select the clock phase of the output signal from phase 0, phase 90, phase 180, phase 270."]
pub type CCLKIN_EDGE_DRV_SEL_R = crate::FieldReader;
#[doc = "Field `CCLKIN_EDGE_DRV_SEL` writer - It's used to select the clock phase of the output signal from phase 0, phase 90, phase 180, phase 270."]
pub type CCLKIN_EDGE_DRV_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CCLKIN_EDGE_SAM_SEL` reader - It's used to select the clock phase of the input signal from phase 0, phase 90, phase 180, phase 270."]
pub type CCLKIN_EDGE_SAM_SEL_R = crate::FieldReader;
#[doc = "Field `CCLKIN_EDGE_SAM_SEL` writer - It's used to select the clock phase of the input signal from phase 0, phase 90, phase 180, phase 270."]
pub type CCLKIN_EDGE_SAM_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CCLKIN_EDGE_SLF_SEL` reader - It's used to select the clock phase of the internal signal from phase 0, phase 90, phase 180, phase 270."]
pub type CCLKIN_EDGE_SLF_SEL_R = crate::FieldReader;
#[doc = "Field `CCLKIN_EDGE_SLF_SEL` writer - It's used to select the clock phase of the internal signal from phase 0, phase 90, phase 180, phase 270."]
pub type CCLKIN_EDGE_SLF_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CCLLKIN_EDGE_H` reader - The high level of the divider clock. The value should be smaller than CCLKIN_EDGE_L."]
pub type CCLLKIN_EDGE_H_R = crate::FieldReader;
#[doc = "Field `CCLLKIN_EDGE_H` writer - The high level of the divider clock. The value should be smaller than CCLKIN_EDGE_L."]
pub type CCLLKIN_EDGE_H_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CCLLKIN_EDGE_L` reader - The low level of the divider clock. The value should be larger than CCLKIN_EDGE_H."]
pub type CCLLKIN_EDGE_L_R = crate::FieldReader;
#[doc = "Field `CCLLKIN_EDGE_L` writer - The low level of the divider clock. The value should be larger than CCLKIN_EDGE_H."]
pub type CCLLKIN_EDGE_L_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CCLLKIN_EDGE_N` reader - The clock division of cclk_in."]
pub type CCLLKIN_EDGE_N_R = crate::FieldReader;
#[doc = "Field `CCLLKIN_EDGE_N` writer - The clock division of cclk_in."]
pub type CCLLKIN_EDGE_N_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ESDIO_MODE` reader - Enable esdio mode."]
pub type ESDIO_MODE_R = crate::BitReader;
#[doc = "Field `ESDIO_MODE` writer - Enable esdio mode."]
pub type ESDIO_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ESD_MODE` reader - Enable esd mode."]
pub type ESD_MODE_R = crate::BitReader;
#[doc = "Field `ESD_MODE` writer - Enable esd mode."]
pub type ESD_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCLK_EN` reader - Sdio clock enable."]
pub type CCLK_EN_R = crate::BitReader;
#[doc = "Field `CCLK_EN` writer - Sdio clock enable."]
pub type CCLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ULTRA_HIGH_SPEED_MODE` reader - Enable ultra high speed mode, use dll to generate clk."]
pub type ULTRA_HIGH_SPEED_MODE_R = crate::BitReader;
#[doc = "Field `ULTRA_HIGH_SPEED_MODE` writer - Enable ultra high speed mode, use dll to generate clk."]
pub type ULTRA_HIGH_SPEED_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - It's used to select the clock phase of the output signal from phase 0, phase 90, phase 180, phase 270."]
    #[inline(always)]
    pub fn cclkin_edge_drv_sel(&self) -> CCLKIN_EDGE_DRV_SEL_R {
        CCLKIN_EDGE_DRV_SEL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - It's used to select the clock phase of the input signal from phase 0, phase 90, phase 180, phase 270."]
    #[inline(always)]
    pub fn cclkin_edge_sam_sel(&self) -> CCLKIN_EDGE_SAM_SEL_R {
        CCLKIN_EDGE_SAM_SEL_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:8 - It's used to select the clock phase of the internal signal from phase 0, phase 90, phase 180, phase 270."]
    #[inline(always)]
    pub fn cclkin_edge_slf_sel(&self) -> CCLKIN_EDGE_SLF_SEL_R {
        CCLKIN_EDGE_SLF_SEL_R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 9:12 - The high level of the divider clock. The value should be smaller than CCLKIN_EDGE_L."]
    #[inline(always)]
    pub fn ccllkin_edge_h(&self) -> CCLLKIN_EDGE_H_R {
        CCLLKIN_EDGE_H_R::new(((self.bits >> 9) & 0x0f) as u8)
    }
    #[doc = "Bits 13:16 - The low level of the divider clock. The value should be larger than CCLKIN_EDGE_H."]
    #[inline(always)]
    pub fn ccllkin_edge_l(&self) -> CCLLKIN_EDGE_L_R {
        CCLLKIN_EDGE_L_R::new(((self.bits >> 13) & 0x0f) as u8)
    }
    #[doc = "Bits 17:20 - The clock division of cclk_in."]
    #[inline(always)]
    pub fn ccllkin_edge_n(&self) -> CCLLKIN_EDGE_N_R {
        CCLLKIN_EDGE_N_R::new(((self.bits >> 17) & 0x0f) as u8)
    }
    #[doc = "Bit 21 - Enable esdio mode."]
    #[inline(always)]
    pub fn esdio_mode(&self) -> ESDIO_MODE_R {
        ESDIO_MODE_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Enable esd mode."]
    #[inline(always)]
    pub fn esd_mode(&self) -> ESD_MODE_R {
        ESD_MODE_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Sdio clock enable."]
    #[inline(always)]
    pub fn cclk_en(&self) -> CCLK_EN_R {
        CCLK_EN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable ultra high speed mode, use dll to generate clk."]
    #[inline(always)]
    pub fn ultra_high_speed_mode(&self) -> ULTRA_HIGH_SPEED_MODE_R {
        ULTRA_HIGH_SPEED_MODE_R::new(((self.bits >> 24) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLK_EDGE_SEL")
            .field(
                "cclkin_edge_drv_sel",
                &format_args!("{}", self.cclkin_edge_drv_sel().bits()),
            )
            .field(
                "cclkin_edge_sam_sel",
                &format_args!("{}", self.cclkin_edge_sam_sel().bits()),
            )
            .field(
                "cclkin_edge_slf_sel",
                &format_args!("{}", self.cclkin_edge_slf_sel().bits()),
            )
            .field(
                "ccllkin_edge_h",
                &format_args!("{}", self.ccllkin_edge_h().bits()),
            )
            .field(
                "ccllkin_edge_l",
                &format_args!("{}", self.ccllkin_edge_l().bits()),
            )
            .field(
                "ccllkin_edge_n",
                &format_args!("{}", self.ccllkin_edge_n().bits()),
            )
            .field("esdio_mode", &format_args!("{}", self.esdio_mode().bit()))
            .field("esd_mode", &format_args!("{}", self.esd_mode().bit()))
            .field("cclk_en", &format_args!("{}", self.cclk_en().bit()))
            .field(
                "ultra_high_speed_mode",
                &format_args!("{}", self.ultra_high_speed_mode().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CLK_EDGE_SEL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:2 - It's used to select the clock phase of the output signal from phase 0, phase 90, phase 180, phase 270."]
    #[inline(always)]
    #[must_use]
    pub fn cclkin_edge_drv_sel(&mut self) -> CCLKIN_EDGE_DRV_SEL_W<CLK_EDGE_SEL_SPEC> {
        CCLKIN_EDGE_DRV_SEL_W::new(self, 0)
    }
    #[doc = "Bits 3:5 - It's used to select the clock phase of the input signal from phase 0, phase 90, phase 180, phase 270."]
    #[inline(always)]
    #[must_use]
    pub fn cclkin_edge_sam_sel(&mut self) -> CCLKIN_EDGE_SAM_SEL_W<CLK_EDGE_SEL_SPEC> {
        CCLKIN_EDGE_SAM_SEL_W::new(self, 3)
    }
    #[doc = "Bits 6:8 - It's used to select the clock phase of the internal signal from phase 0, phase 90, phase 180, phase 270."]
    #[inline(always)]
    #[must_use]
    pub fn cclkin_edge_slf_sel(&mut self) -> CCLKIN_EDGE_SLF_SEL_W<CLK_EDGE_SEL_SPEC> {
        CCLKIN_EDGE_SLF_SEL_W::new(self, 6)
    }
    #[doc = "Bits 9:12 - The high level of the divider clock. The value should be smaller than CCLKIN_EDGE_L."]
    #[inline(always)]
    #[must_use]
    pub fn ccllkin_edge_h(&mut self) -> CCLLKIN_EDGE_H_W<CLK_EDGE_SEL_SPEC> {
        CCLLKIN_EDGE_H_W::new(self, 9)
    }
    #[doc = "Bits 13:16 - The low level of the divider clock. The value should be larger than CCLKIN_EDGE_H."]
    #[inline(always)]
    #[must_use]
    pub fn ccllkin_edge_l(&mut self) -> CCLLKIN_EDGE_L_W<CLK_EDGE_SEL_SPEC> {
        CCLLKIN_EDGE_L_W::new(self, 13)
    }
    #[doc = "Bits 17:20 - The clock division of cclk_in."]
    #[inline(always)]
    #[must_use]
    pub fn ccllkin_edge_n(&mut self) -> CCLLKIN_EDGE_N_W<CLK_EDGE_SEL_SPEC> {
        CCLLKIN_EDGE_N_W::new(self, 17)
    }
    #[doc = "Bit 21 - Enable esdio mode."]
    #[inline(always)]
    #[must_use]
    pub fn esdio_mode(&mut self) -> ESDIO_MODE_W<CLK_EDGE_SEL_SPEC> {
        ESDIO_MODE_W::new(self, 21)
    }
    #[doc = "Bit 22 - Enable esd mode."]
    #[inline(always)]
    #[must_use]
    pub fn esd_mode(&mut self) -> ESD_MODE_W<CLK_EDGE_SEL_SPEC> {
        ESD_MODE_W::new(self, 22)
    }
    #[doc = "Bit 23 - Sdio clock enable."]
    #[inline(always)]
    #[must_use]
    pub fn cclk_en(&mut self) -> CCLK_EN_W<CLK_EDGE_SEL_SPEC> {
        CCLK_EN_W::new(self, 23)
    }
    #[doc = "Bit 24 - Enable ultra high speed mode, use dll to generate clk."]
    #[inline(always)]
    #[must_use]
    pub fn ultra_high_speed_mode(&mut self) -> ULTRA_HIGH_SPEED_MODE_W<CLK_EDGE_SEL_SPEC> {
        ULTRA_HIGH_SPEED_MODE_W::new(self, 24)
    }
}
#[doc = "SDIO control register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_edge_sel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_edge_sel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLK_EDGE_SEL_SPEC;
impl crate::RegisterSpec for CLK_EDGE_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_edge_sel::R`](R) reader structure"]
impl crate::Readable for CLK_EDGE_SEL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clk_edge_sel::W`](W) writer structure"]
impl crate::Writable for CLK_EDGE_SEL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLK_EDGE_SEL to value 0x0082_0200"]
impl crate::Resettable for CLK_EDGE_SEL_SPEC {
    const RESET_VALUE: u32 = 0x0082_0200;
}
