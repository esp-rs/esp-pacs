#[doc = "Register `CONF` reader"]
pub type R = crate::R<CONF_SPEC>;
#[doc = "Register `CONF` writer"]
pub type W = crate::W<CONF_SPEC>;
#[doc = "Field `NOISE_SOURCE_SEL` reader - sel noise source"]
pub type NOISE_SOURCE_SEL_R = crate::FieldReader;
#[doc = "Field `NOISE_SOURCE_SEL` writer - sel noise source"]
pub type NOISE_SOURCE_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `NOISE_POS_SEL` reader - sel noise pos"]
pub type NOISE_POS_SEL_R = crate::FieldReader;
#[doc = "Field `NOISE_POS_SEL` writer - sel noise pos"]
pub type NOISE_POS_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `REPETITION_VALUE_C` reader - parameter"]
pub type REPETITION_VALUE_C_R = crate::FieldReader<u16>;
#[doc = "Field `REPETITION_VALUE_C` writer - parameter"]
pub type REPETITION_VALUE_C_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `ADPATIVE_VALUE_C` reader - parameter"]
pub type ADPATIVE_VALUE_C_R = crate::FieldReader<u16>;
#[doc = "Field `ADPATIVE_VALUE_C` writer - parameter"]
pub type ADPATIVE_VALUE_C_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `FIFO_RESET` writer - fifo_reset"]
pub type FIFO_RESET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RANDOM_OUTPUT_MODE` reader - standart and nonstandart output"]
pub type RANDOM_OUTPUT_MODE_R = crate::BitReader;
#[doc = "Field `RANDOM_OUTPUT_MODE` writer - standart and nonstandart output"]
pub type RANDOM_OUTPUT_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NOISE_CRC_EN` reader - standart and nonstandart output"]
pub type NOISE_CRC_EN_R = crate::BitReader;
#[doc = "Field `NOISE_CRC_EN` writer - standart and nonstandart output"]
pub type NOISE_CRC_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAMPLE_ENABLE` reader - 1: enable rng sample loop"]
pub type SAMPLE_ENABLE_R = crate::BitReader;
#[doc = "Field `SAMPLE_ENABLE` writer - 1: enable rng sample loop"]
pub type SAMPLE_ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4 - sel noise source"]
    #[inline(always)]
    pub fn noise_source_sel(&self) -> NOISE_SOURCE_SEL_R {
        NOISE_SOURCE_SEL_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - sel noise pos"]
    #[inline(always)]
    pub fn noise_pos_sel(&self) -> NOISE_POS_SEL_R {
        NOISE_POS_SEL_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:18 - parameter"]
    #[inline(always)]
    pub fn repetition_value_c(&self) -> REPETITION_VALUE_C_R {
        REPETITION_VALUE_C_R::new(((self.bits >> 10) & 0x01ff) as u16)
    }
    #[doc = "Bits 19:27 - parameter"]
    #[inline(always)]
    pub fn adpative_value_c(&self) -> ADPATIVE_VALUE_C_R {
        ADPATIVE_VALUE_C_R::new(((self.bits >> 19) & 0x01ff) as u16)
    }
    #[doc = "Bit 29 - standart and nonstandart output"]
    #[inline(always)]
    pub fn random_output_mode(&self) -> RANDOM_OUTPUT_MODE_R {
        RANDOM_OUTPUT_MODE_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - standart and nonstandart output"]
    #[inline(always)]
    pub fn noise_crc_en(&self) -> NOISE_CRC_EN_R {
        NOISE_CRC_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 1: enable rng sample loop"]
    #[inline(always)]
    pub fn sample_enable(&self) -> SAMPLE_ENABLE_R {
        SAMPLE_ENABLE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONF")
            .field("noise_source_sel", &self.noise_source_sel())
            .field("noise_pos_sel", &self.noise_pos_sel())
            .field("repetition_value_c", &self.repetition_value_c())
            .field("adpative_value_c", &self.adpative_value_c())
            .field("random_output_mode", &self.random_output_mode())
            .field("noise_crc_en", &self.noise_crc_en())
            .field("sample_enable", &self.sample_enable())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:4 - sel noise source"]
    #[inline(always)]
    pub fn noise_source_sel(&mut self) -> NOISE_SOURCE_SEL_W<'_, CONF_SPEC> {
        NOISE_SOURCE_SEL_W::new(self, 0)
    }
    #[doc = "Bits 5:9 - sel noise pos"]
    #[inline(always)]
    pub fn noise_pos_sel(&mut self) -> NOISE_POS_SEL_W<'_, CONF_SPEC> {
        NOISE_POS_SEL_W::new(self, 5)
    }
    #[doc = "Bits 10:18 - parameter"]
    #[inline(always)]
    pub fn repetition_value_c(&mut self) -> REPETITION_VALUE_C_W<'_, CONF_SPEC> {
        REPETITION_VALUE_C_W::new(self, 10)
    }
    #[doc = "Bits 19:27 - parameter"]
    #[inline(always)]
    pub fn adpative_value_c(&mut self) -> ADPATIVE_VALUE_C_W<'_, CONF_SPEC> {
        ADPATIVE_VALUE_C_W::new(self, 19)
    }
    #[doc = "Bit 28 - fifo_reset"]
    #[inline(always)]
    pub fn fifo_reset(&mut self) -> FIFO_RESET_W<'_, CONF_SPEC> {
        FIFO_RESET_W::new(self, 28)
    }
    #[doc = "Bit 29 - standart and nonstandart output"]
    #[inline(always)]
    pub fn random_output_mode(&mut self) -> RANDOM_OUTPUT_MODE_W<'_, CONF_SPEC> {
        RANDOM_OUTPUT_MODE_W::new(self, 29)
    }
    #[doc = "Bit 30 - standart and nonstandart output"]
    #[inline(always)]
    pub fn noise_crc_en(&mut self) -> NOISE_CRC_EN_W<'_, CONF_SPEC> {
        NOISE_CRC_EN_W::new(self, 30)
    }
    #[doc = "Bit 31 - 1: enable rng sample loop"]
    #[inline(always)]
    pub fn sample_enable(&mut self) -> SAMPLE_ENABLE_W<'_, CONF_SPEC> {
        SAMPLE_ENABLE_W::new(self, 31)
    }
}
#[doc = "CFG reg\n\nYou can [`read`](crate::Reg::read) this register and get [`conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CONF_SPEC;
impl crate::RegisterSpec for CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`conf::R`](R) reader structure"]
impl crate::Readable for CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`conf::W`](W) writer structure"]
impl crate::Writable for CONF_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CONF to value 0x4000_0000"]
impl crate::Resettable for CONF_SPEC {
    const RESET_VALUE: u32 = 0x4000_0000;
}
