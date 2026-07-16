#[doc = "Register `CHNL0_CFG0` reader"]
pub type R = crate::R<CHNL0_CFG0_SPEC>;
#[doc = "Register `CHNL0_CFG0` writer"]
pub type W = crate::W<CHNL0_CFG0_SPEC>;
#[doc = "Field `CHNL0_RS2_STG1_BYPASS` reader - Set this bit to bypass stage 1 re-sampler in channel0."]
pub type CHNL0_RS2_STG1_BYPASS_R = crate::BitReader;
#[doc = "Field `CHNL0_RS2_STG1_BYPASS` writer - Set this bit to bypass stage 1 re-sampler in channel0."]
pub type CHNL0_RS2_STG1_BYPASS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHNL0_RS2_STG0_BYPASS` reader - Set this bit to bypass stage 0 re-sampler in channel0."]
pub type CHNL0_RS2_STG0_BYPASS_R = crate::BitReader;
#[doc = "Field `CHNL0_RS2_STG0_BYPASS` writer - Set this bit to bypass stage 0 re-sampler in channel0."]
pub type CHNL0_RS2_STG0_BYPASS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHNL0_FRAC_BYPASS` reader - Set this bit to bypass fractional re-sampler in channel0."]
pub type CHNL0_FRAC_BYPASS_R = crate::BitReader;
#[doc = "Field `CHNL0_FRAC_BYPASS` writer - Set this bit to bypass fractional re-sampler in channel0."]
pub type CHNL0_FRAC_BYPASS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHNL0_RS2_STG1_MODE` reader - Write this bit to configure stage 1 re-sampler mode in channel0, 0: interpolation by factor of 2, 1: decimation by factor of 2."]
pub type CHNL0_RS2_STG1_MODE_R = crate::BitReader;
#[doc = "Field `CHNL0_RS2_STG1_MODE` writer - Write this bit to configure stage 1 re-sampler mode in channel0, 0: interpolation by factor of 2, 1: decimation by factor of 2."]
pub type CHNL0_RS2_STG1_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHNL0_RS2_STG0_MODE` reader - Write this bit to configure stage 0 re-sampler mode in channel0, 0: interpolation by factor of 2, 1: decimation by factor of 2."]
pub type CHNL0_RS2_STG0_MODE_R = crate::BitReader;
#[doc = "Field `CHNL0_RS2_STG0_MODE` writer - Write this bit to configure stage 0 re-sampler mode in channel0, 0: interpolation by factor of 2, 1: decimation by factor of 2."]
pub type CHNL0_RS2_STG0_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHNL0_FRAC_AHEAD` reader - Set this bit to move fraction re-sampler to the first stage in channel0, it should be 1 when input frequency is higher the output."]
pub type CHNL0_FRAC_AHEAD_R = crate::BitReader;
#[doc = "Field `CHNL0_FRAC_AHEAD` writer - Set this bit to move fraction re-sampler to the first stage in channel0, it should be 1 when input frequency is higher the output."]
pub type CHNL0_FRAC_AHEAD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHNL0_RS2_SELPHASE0` reader - Set this bit to disable shift phase when 2 interpolation by factor of 2 in channel0."]
pub type CHNL0_RS2_SELPHASE0_R = crate::BitReader;
#[doc = "Field `CHNL0_RS2_SELPHASE0` writer - Set this bit to disable shift phase when 2 interpolation by factor of 2 in channel0."]
pub type CHNL0_RS2_SELPHASE0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHNL0_MODE` reader - Write the bit to configure the channel mode,0: in and out are both mono, 1: in and out is both dual, 2: in is mono, out is dual, 3, in is dual, out is mono."]
pub type CHNL0_MODE_R = crate::FieldReader;
#[doc = "Field `CHNL0_MODE` writer - Write the bit to configure the channel mode,0: in and out are both mono, 1: in and out is both dual, 2: in is mono, out is dual, 3, in is dual, out is mono."]
pub type CHNL0_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CHNL0_SEL` reader - Write the bit to configure which 16bits data will be processing."]
pub type CHNL0_SEL_R = crate::BitReader;
#[doc = "Field `CHNL0_SEL` writer - Write the bit to configure which 16bits data will be processing."]
pub type CHNL0_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Set this bit to bypass stage 1 re-sampler in channel0."]
    #[inline(always)]
    pub fn chnl0_rs2_stg1_bypass(&self) -> CHNL0_RS2_STG1_BYPASS_R {
        CHNL0_RS2_STG1_BYPASS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set this bit to bypass stage 0 re-sampler in channel0."]
    #[inline(always)]
    pub fn chnl0_rs2_stg0_bypass(&self) -> CHNL0_RS2_STG0_BYPASS_R {
        CHNL0_RS2_STG0_BYPASS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set this bit to bypass fractional re-sampler in channel0."]
    #[inline(always)]
    pub fn chnl0_frac_bypass(&self) -> CHNL0_FRAC_BYPASS_R {
        CHNL0_FRAC_BYPASS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Write this bit to configure stage 1 re-sampler mode in channel0, 0: interpolation by factor of 2, 1: decimation by factor of 2."]
    #[inline(always)]
    pub fn chnl0_rs2_stg1_mode(&self) -> CHNL0_RS2_STG1_MODE_R {
        CHNL0_RS2_STG1_MODE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Write this bit to configure stage 0 re-sampler mode in channel0, 0: interpolation by factor of 2, 1: decimation by factor of 2."]
    #[inline(always)]
    pub fn chnl0_rs2_stg0_mode(&self) -> CHNL0_RS2_STG0_MODE_R {
        CHNL0_RS2_STG0_MODE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Set this bit to move fraction re-sampler to the first stage in channel0, it should be 1 when input frequency is higher the output."]
    #[inline(always)]
    pub fn chnl0_frac_ahead(&self) -> CHNL0_FRAC_AHEAD_R {
        CHNL0_FRAC_AHEAD_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Set this bit to disable shift phase when 2 interpolation by factor of 2 in channel0."]
    #[inline(always)]
    pub fn chnl0_rs2_selphase0(&self) -> CHNL0_RS2_SELPHASE0_R {
        CHNL0_RS2_SELPHASE0_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:8 - Write the bit to configure the channel mode,0: in and out are both mono, 1: in and out is both dual, 2: in is mono, out is dual, 3, in is dual, out is mono."]
    #[inline(always)]
    pub fn chnl0_mode(&self) -> CHNL0_MODE_R {
        CHNL0_MODE_R::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bit 9 - Write the bit to configure which 16bits data will be processing."]
    #[inline(always)]
    pub fn chnl0_sel(&self) -> CHNL0_SEL_R {
        CHNL0_SEL_R::new(((self.bits >> 9) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CHNL0_CFG0")
            .field("chnl0_rs2_stg1_bypass", &self.chnl0_rs2_stg1_bypass())
            .field("chnl0_rs2_stg0_bypass", &self.chnl0_rs2_stg0_bypass())
            .field("chnl0_frac_bypass", &self.chnl0_frac_bypass())
            .field("chnl0_rs2_stg1_mode", &self.chnl0_rs2_stg1_mode())
            .field("chnl0_rs2_stg0_mode", &self.chnl0_rs2_stg0_mode())
            .field("chnl0_frac_ahead", &self.chnl0_frac_ahead())
            .field("chnl0_rs2_selphase0", &self.chnl0_rs2_selphase0())
            .field("chnl0_mode", &self.chnl0_mode())
            .field("chnl0_sel", &self.chnl0_sel())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to bypass stage 1 re-sampler in channel0."]
    #[inline(always)]
    pub fn chnl0_rs2_stg1_bypass(&mut self) -> CHNL0_RS2_STG1_BYPASS_W<'_, CHNL0_CFG0_SPEC> {
        CHNL0_RS2_STG1_BYPASS_W::new(self, 0)
    }
    #[doc = "Bit 1 - Set this bit to bypass stage 0 re-sampler in channel0."]
    #[inline(always)]
    pub fn chnl0_rs2_stg0_bypass(&mut self) -> CHNL0_RS2_STG0_BYPASS_W<'_, CHNL0_CFG0_SPEC> {
        CHNL0_RS2_STG0_BYPASS_W::new(self, 1)
    }
    #[doc = "Bit 2 - Set this bit to bypass fractional re-sampler in channel0."]
    #[inline(always)]
    pub fn chnl0_frac_bypass(&mut self) -> CHNL0_FRAC_BYPASS_W<'_, CHNL0_CFG0_SPEC> {
        CHNL0_FRAC_BYPASS_W::new(self, 2)
    }
    #[doc = "Bit 3 - Write this bit to configure stage 1 re-sampler mode in channel0, 0: interpolation by factor of 2, 1: decimation by factor of 2."]
    #[inline(always)]
    pub fn chnl0_rs2_stg1_mode(&mut self) -> CHNL0_RS2_STG1_MODE_W<'_, CHNL0_CFG0_SPEC> {
        CHNL0_RS2_STG1_MODE_W::new(self, 3)
    }
    #[doc = "Bit 4 - Write this bit to configure stage 0 re-sampler mode in channel0, 0: interpolation by factor of 2, 1: decimation by factor of 2."]
    #[inline(always)]
    pub fn chnl0_rs2_stg0_mode(&mut self) -> CHNL0_RS2_STG0_MODE_W<'_, CHNL0_CFG0_SPEC> {
        CHNL0_RS2_STG0_MODE_W::new(self, 4)
    }
    #[doc = "Bit 5 - Set this bit to move fraction re-sampler to the first stage in channel0, it should be 1 when input frequency is higher the output."]
    #[inline(always)]
    pub fn chnl0_frac_ahead(&mut self) -> CHNL0_FRAC_AHEAD_W<'_, CHNL0_CFG0_SPEC> {
        CHNL0_FRAC_AHEAD_W::new(self, 5)
    }
    #[doc = "Bit 6 - Set this bit to disable shift phase when 2 interpolation by factor of 2 in channel0."]
    #[inline(always)]
    pub fn chnl0_rs2_selphase0(&mut self) -> CHNL0_RS2_SELPHASE0_W<'_, CHNL0_CFG0_SPEC> {
        CHNL0_RS2_SELPHASE0_W::new(self, 6)
    }
    #[doc = "Bits 7:8 - Write the bit to configure the channel mode,0: in and out are both mono, 1: in and out is both dual, 2: in is mono, out is dual, 3, in is dual, out is mono."]
    #[inline(always)]
    pub fn chnl0_mode(&mut self) -> CHNL0_MODE_W<'_, CHNL0_CFG0_SPEC> {
        CHNL0_MODE_W::new(self, 7)
    }
    #[doc = "Bit 9 - Write the bit to configure which 16bits data will be processing."]
    #[inline(always)]
    pub fn chnl0_sel(&mut self) -> CHNL0_SEL_W<'_, CHNL0_CFG0_SPEC> {
        CHNL0_SEL_W::new(self, 9)
    }
}
#[doc = "Control and configuration registers\n\nYou can [`read`](crate::Reg::read) this register and get [`chnl0_cfg0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chnl0_cfg0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CHNL0_CFG0_SPEC;
impl crate::RegisterSpec for CHNL0_CFG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chnl0_cfg0::R`](R) reader structure"]
impl crate::Readable for CHNL0_CFG0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`chnl0_cfg0::W`](W) writer structure"]
impl crate::Writable for CHNL0_CFG0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CHNL0_CFG0 to value 0x07"]
impl crate::Resettable for CHNL0_CFG0_SPEC {
    const RESET_VALUE: u32 = 0x07;
}
