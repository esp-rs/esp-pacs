#[doc = "Register `WIFI_CLK_EN` reader"]
pub type R = crate::R<WIFI_CLK_EN_SPEC>;
#[doc = "Register `WIFI_CLK_EN` writer"]
pub type W = crate::W<WIFI_CLK_EN_SPEC>;
#[doc = "Field `I2C_CLK_EN` reader - Set this bit to enable the I2C clock. Clear the bit to disable it."]
pub type I2C_CLK_EN_R = crate::BitReader;
#[doc = "Field `I2C_CLK_EN` writer - Set this bit to enable the I2C clock. Clear the bit to disable it."]
pub type I2C_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UNUSED_BIT12` reader - Unused clock enable bit. ESP-IDF clears it during peripheral clock initialization."]
pub type UNUSED_BIT12_R = crate::BitReader;
#[doc = "Field `UNUSED_BIT12` writer - Unused clock enable bit. ESP-IDF clears it during peripheral clock initialization."]
pub type UNUSED_BIT12_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RNG_CLK_EN` reader - Set this bit to enable the RNG clock. Clear the bit to disable it."]
pub type RNG_CLK_EN_R = crate::BitReader;
#[doc = "Field `RNG_CLK_EN` writer - Set this bit to enable the RNG clock. Clear the bit to disable it."]
pub type RNG_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 5 - Set this bit to enable the I2C clock. Clear the bit to disable it."]
    #[inline(always)]
    pub fn i2c_clk_en(&self) -> I2C_CLK_EN_R {
        I2C_CLK_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 12 - Unused clock enable bit. ESP-IDF clears it during peripheral clock initialization."]
    #[inline(always)]
    pub fn unused_bit12(&self) -> UNUSED_BIT12_R {
        UNUSED_BIT12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 15 - Set this bit to enable the RNG clock. Clear the bit to disable it."]
    #[inline(always)]
    pub fn rng_clk_en(&self) -> RNG_CLK_EN_R {
        RNG_CLK_EN_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WIFI_CLK_EN")
            .field("i2c_clk_en", &self.i2c_clk_en())
            .field("unused_bit12", &self.unused_bit12())
            .field("rng_clk_en", &self.rng_clk_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 5 - Set this bit to enable the I2C clock. Clear the bit to disable it."]
    #[inline(always)]
    pub fn i2c_clk_en(&mut self) -> I2C_CLK_EN_W<'_, WIFI_CLK_EN_SPEC> {
        I2C_CLK_EN_W::new(self, 5)
    }
    #[doc = "Bit 12 - Unused clock enable bit. ESP-IDF clears it during peripheral clock initialization."]
    #[inline(always)]
    pub fn unused_bit12(&mut self) -> UNUSED_BIT12_W<'_, WIFI_CLK_EN_SPEC> {
        UNUSED_BIT12_W::new(self, 12)
    }
    #[doc = "Bit 15 - Set this bit to enable the RNG clock. Clear the bit to disable it."]
    #[inline(always)]
    pub fn rng_clk_en(&mut self) -> RNG_CLK_EN_W<'_, WIFI_CLK_EN_SPEC> {
        RNG_CLK_EN_W::new(self, 15)
    }
}
#[doc = "APB_CTRL_WIFI_CLK_EN_REG\n\nYou can [`read`](crate::Reg::read) this register and get [`wifi_clk_en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wifi_clk_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WIFI_CLK_EN_SPEC;
impl crate::RegisterSpec for WIFI_CLK_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wifi_clk_en::R`](R) reader structure"]
impl crate::Readable for WIFI_CLK_EN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`wifi_clk_en::W`](W) writer structure"]
impl crate::Writable for WIFI_CLK_EN_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WIFI_CLK_EN to value 0xfffc_e030"]
impl crate::Resettable for WIFI_CLK_EN_SPEC {
    const RESET_VALUE: u32 = 0xfffc_e030;
}
