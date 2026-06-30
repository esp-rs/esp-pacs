#[doc = "Register `HP_PAD_I2S1_CTRL` reader"]
pub type R = crate::R<HP_PAD_I2S1_CTRL_SPEC>;
#[doc = "Register `HP_PAD_I2S1_CTRL` writer"]
pub type W = crate::W<HP_PAD_I2S1_CTRL_SPEC>;
#[doc = "Field `HP_PAD_I2S1_MCLK_EN` reader - I2S1 MCLK Clock From Pad Enable."]
pub type HP_PAD_I2S1_MCLK_EN_R = crate::BitReader;
#[doc = "Field `HP_PAD_I2S1_MCLK_EN` writer - I2S1 MCLK Clock From Pad Enable."]
pub type HP_PAD_I2S1_MCLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 31 - I2S1 MCLK Clock From Pad Enable."]
    #[inline(always)]
    pub fn hp_pad_i2s1_mclk_en(&self) -> HP_PAD_I2S1_MCLK_EN_R {
        HP_PAD_I2S1_MCLK_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HP_PAD_I2S1_CTRL")
            .field("hp_pad_i2s1_mclk_en", &self.hp_pad_i2s1_mclk_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 31 - I2S1 MCLK Clock From Pad Enable."]
    #[inline(always)]
    pub fn hp_pad_i2s1_mclk_en(&mut self) -> HP_PAD_I2S1_MCLK_EN_W<'_, HP_PAD_I2S1_CTRL_SPEC> {
        HP_PAD_I2S1_MCLK_EN_W::new(self, 31)
    }
}
#[doc = "HP Clock Control Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_pad_i2s1_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_pad_i2s1_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HP_PAD_I2S1_CTRL_SPEC;
impl crate::RegisterSpec for HP_PAD_I2S1_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hp_pad_i2s1_ctrl::R`](R) reader structure"]
impl crate::Readable for HP_PAD_I2S1_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hp_pad_i2s1_ctrl::W`](W) writer structure"]
impl crate::Writable for HP_PAD_I2S1_CTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HP_PAD_I2S1_CTRL to value 0x8000_0000"]
impl crate::Resettable for HP_PAD_I2S1_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x8000_0000;
}
