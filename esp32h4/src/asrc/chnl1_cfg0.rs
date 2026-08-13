#[doc = "Register `CHNL1_CFG0` reader"]
pub type R = crate::R<CHNL1_CFG0_SPEC>;
#[doc = "Register `CHNL1_CFG0` writer"]
pub type W = crate::W<CHNL1_CFG0_SPEC>;
#[doc = "Field `CHNL1_RS2_STG1_BYPASS` reader - Set this bit to bypass stage 1 re-sampler in channel1."]
pub type CHNL1_RS2_STG1_BYPASS_R = crate::BitReader;
#[doc = "Field `CHNL1_RS2_STG1_BYPASS` writer - Set this bit to bypass stage 1 re-sampler in channel1."]
pub type CHNL1_RS2_STG1_BYPASS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHNL1_RS2_STG0_BYPASS` reader - Set this bit to bypass stage 0 re-sampler in channel1."]
pub type CHNL1_RS2_STG0_BYPASS_R = crate::BitReader;
#[doc = "Field `CHNL1_RS2_STG0_BYPASS` writer - Set this bit to bypass stage 0 re-sampler in channel1."]
pub type CHNL1_RS2_STG0_BYPASS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHNL1_FRAC_BYPASS` reader - Set this bit to bypass fractional re-sampler in channel1."]
pub type CHNL1_FRAC_BYPASS_R = crate::BitReader;
#[doc = "Field `CHNL1_FRAC_BYPASS` writer - Set this bit to bypass fractional re-sampler in channel1."]
pub type CHNL1_FRAC_BYPASS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHNL1_RS2_STG1_MODE` reader - Write this bit to configure stage 1 re-sampler mode in channel1, 0: interpolation by factor of 2, 1: decimation by factor of 2."]
pub type CHNL1_RS2_STG1_MODE_R = crate::BitReader;
#[doc = "Field `CHNL1_RS2_STG1_MODE` writer - Write this bit to configure stage 1 re-sampler mode in channel1, 0: interpolation by factor of 2, 1: decimation by factor of 2."]
pub type CHNL1_RS2_STG1_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHNL1_RS2_STG0_MODE` reader - Write this bit to configure stage 0 re-sampler mode in channel1, 0: interpolation by factor of 2, 1: decimation by factor of 2."]
pub type CHNL1_RS2_STG0_MODE_R = crate::BitReader;
#[doc = "Field `CHNL1_RS2_STG0_MODE` writer - Write this bit to configure stage 0 re-sampler mode in channel1, 0: interpolation by factor of 2, 1: decimation by factor of 2."]
pub type CHNL1_RS2_STG0_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHNL1_FRAC_AHEAD` reader - Set this bit to move fraction re-sampler to the first stage in channel1, it should be 1 when input frequency is higher the output."]
pub type CHNL1_FRAC_AHEAD_R = crate::BitReader;
#[doc = "Field `CHNL1_FRAC_AHEAD` writer - Set this bit to move fraction re-sampler to the first stage in channel1, it should be 1 when input frequency is higher the output."]
pub type CHNL1_FRAC_AHEAD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHNL1_RS2_SELPHASE0` reader - Set this bit to disable shift phase when 2 interpolation by factor of 2 in channel1."]
pub type CHNL1_RS2_SELPHASE0_R = crate::BitReader;
#[doc = "Field `CHNL1_RS2_SELPHASE0` writer - Set this bit to disable shift phase when 2 interpolation by factor of 2 in channel1."]
pub type CHNL1_RS2_SELPHASE0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHNL1_MODE` reader - Write the bit to configure the channel mode,0: in and out are both mono, 1: in and out is both dual, 2: in is mono, out is dual, 3, in is dual, out is mono."]
pub type CHNL1_MODE_R = crate::FieldReader;
#[doc = "Field `CHNL1_MODE` writer - Write the bit to configure the channel mode,0: in and out are both mono, 1: in and out is both dual, 2: in is mono, out is dual, 3, in is dual, out is mono."]
pub type CHNL1_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CHNL1_SEL` reader - Write the bit to configure which 16bits data will be processing."]
pub type CHNL1_SEL_R = crate::BitReader;
#[doc = "Field `CHNL1_SEL` writer - Write the bit to configure which 16bits data will be processing."]
pub type CHNL1_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Set this bit to bypass stage 1 re-sampler in channel1."]
    #[inline(always)]
    pub fn chnl1_rs2_stg1_bypass(&self) -> CHNL1_RS2_STG1_BYPASS_R {
        CHNL1_RS2_STG1_BYPASS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set this bit to bypass stage 0 re-sampler in channel1."]
    #[inline(always)]
    pub fn chnl1_rs2_stg0_bypass(&self) -> CHNL1_RS2_STG0_BYPASS_R {
        CHNL1_RS2_STG0_BYPASS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set this bit to bypass fractional re-sampler in channel1."]
    #[inline(always)]
    pub fn chnl1_frac_bypass(&self) -> CHNL1_FRAC_BYPASS_R {
        CHNL1_FRAC_BYPASS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Write this bit to configure stage 1 re-sampler mode in channel1, 0: interpolation by factor of 2, 1: decimation by factor of 2."]
    #[inline(always)]
    pub fn chnl1_rs2_stg1_mode(&self) -> CHNL1_RS2_STG1_MODE_R {
        CHNL1_RS2_STG1_MODE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Write this bit to configure stage 0 re-sampler mode in channel1, 0: interpolation by factor of 2, 1: decimation by factor of 2."]
    #[inline(always)]
    pub fn chnl1_rs2_stg0_mode(&self) -> CHNL1_RS2_STG0_MODE_R {
        CHNL1_RS2_STG0_MODE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Set this bit to move fraction re-sampler to the first stage in channel1, it should be 1 when input frequency is higher the output."]
    #[inline(always)]
    pub fn chnl1_frac_ahead(&self) -> CHNL1_FRAC_AHEAD_R {
        CHNL1_FRAC_AHEAD_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Set this bit to disable shift phase when 2 interpolation by factor of 2 in channel1."]
    #[inline(always)]
    pub fn chnl1_rs2_selphase0(&self) -> CHNL1_RS2_SELPHASE0_R {
        CHNL1_RS2_SELPHASE0_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:8 - Write the bit to configure the channel mode,0: in and out are both mono, 1: in and out is both dual, 2: in is mono, out is dual, 3, in is dual, out is mono."]
    #[inline(always)]
    pub fn chnl1_mode(&self) -> CHNL1_MODE_R {
        CHNL1_MODE_R::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bit 9 - Write the bit to configure which 16bits data will be processing."]
    #[inline(always)]
    pub fn chnl1_sel(&self) -> CHNL1_SEL_R {
        CHNL1_SEL_R::new(((self.bits >> 9) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CHNL1_CFG0")
            .field("chnl1_rs2_stg1_bypass", &self.chnl1_rs2_stg1_bypass())
            .field("chnl1_rs2_stg0_bypass", &self.chnl1_rs2_stg0_bypass())
            .field("chnl1_frac_bypass", &self.chnl1_frac_bypass())
            .field("chnl1_rs2_stg1_mode", &self.chnl1_rs2_stg1_mode())
            .field("chnl1_rs2_stg0_mode", &self.chnl1_rs2_stg0_mode())
            .field("chnl1_frac_ahead", &self.chnl1_frac_ahead())
            .field("chnl1_rs2_selphase0", &self.chnl1_rs2_selphase0())
            .field("chnl1_mode", &self.chnl1_mode())
            .field("chnl1_sel", &self.chnl1_sel())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to bypass stage 1 re-sampler in channel1."]
    #[inline(always)]
    pub fn chnl1_rs2_stg1_bypass(&mut self) -> CHNL1_RS2_STG1_BYPASS_W<'_, CHNL1_CFG0_SPEC> {
        CHNL1_RS2_STG1_BYPASS_W::new(self, 0)
    }
    #[doc = "Bit 1 - Set this bit to bypass stage 0 re-sampler in channel1."]
    #[inline(always)]
    pub fn chnl1_rs2_stg0_bypass(&mut self) -> CHNL1_RS2_STG0_BYPASS_W<'_, CHNL1_CFG0_SPEC> {
        CHNL1_RS2_STG0_BYPASS_W::new(self, 1)
    }
    #[doc = "Bit 2 - Set this bit to bypass fractional re-sampler in channel1."]
    #[inline(always)]
    pub fn chnl1_frac_bypass(&mut self) -> CHNL1_FRAC_BYPASS_W<'_, CHNL1_CFG0_SPEC> {
        CHNL1_FRAC_BYPASS_W::new(self, 2)
    }
    #[doc = "Bit 3 - Write this bit to configure stage 1 re-sampler mode in channel1, 0: interpolation by factor of 2, 1: decimation by factor of 2."]
    #[inline(always)]
    pub fn chnl1_rs2_stg1_mode(&mut self) -> CHNL1_RS2_STG1_MODE_W<'_, CHNL1_CFG0_SPEC> {
        CHNL1_RS2_STG1_MODE_W::new(self, 3)
    }
    #[doc = "Bit 4 - Write this bit to configure stage 0 re-sampler mode in channel1, 0: interpolation by factor of 2, 1: decimation by factor of 2."]
    #[inline(always)]
    pub fn chnl1_rs2_stg0_mode(&mut self) -> CHNL1_RS2_STG0_MODE_W<'_, CHNL1_CFG0_SPEC> {
        CHNL1_RS2_STG0_MODE_W::new(self, 4)
    }
    #[doc = "Bit 5 - Set this bit to move fraction re-sampler to the first stage in channel1, it should be 1 when input frequency is higher the output."]
    #[inline(always)]
    pub fn chnl1_frac_ahead(&mut self) -> CHNL1_FRAC_AHEAD_W<'_, CHNL1_CFG0_SPEC> {
        CHNL1_FRAC_AHEAD_W::new(self, 5)
    }
    #[doc = "Bit 6 - Set this bit to disable shift phase when 2 interpolation by factor of 2 in channel1."]
    #[inline(always)]
    pub fn chnl1_rs2_selphase0(&mut self) -> CHNL1_RS2_SELPHASE0_W<'_, CHNL1_CFG0_SPEC> {
        CHNL1_RS2_SELPHASE0_W::new(self, 6)
    }
    #[doc = "Bits 7:8 - Write the bit to configure the channel mode,0: in and out are both mono, 1: in and out is both dual, 2: in is mono, out is dual, 3, in is dual, out is mono."]
    #[inline(always)]
    pub fn chnl1_mode(&mut self) -> CHNL1_MODE_W<'_, CHNL1_CFG0_SPEC> {
        CHNL1_MODE_W::new(self, 7)
    }
    #[doc = "Bit 9 - Write the bit to configure which 16bits data will be processing."]
    #[inline(always)]
    pub fn chnl1_sel(&mut self) -> CHNL1_SEL_W<'_, CHNL1_CFG0_SPEC> {
        CHNL1_SEL_W::new(self, 9)
    }
}
#[doc = "Control and configuration registers\n\nYou can [`read`](crate::Reg::read) this register and get [`chnl1_cfg0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chnl1_cfg0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CHNL1_CFG0_SPEC;
impl crate::RegisterSpec for CHNL1_CFG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chnl1_cfg0::R`](R) reader structure"]
impl crate::Readable for CHNL1_CFG0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`chnl1_cfg0::W`](W) writer structure"]
impl crate::Writable for CHNL1_CFG0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CHNL1_CFG0 to value 0x07"]
impl crate::Resettable for CHNL1_CFG0_SPEC {
    const RESET_VALUE: u32 = 0x07;
}
