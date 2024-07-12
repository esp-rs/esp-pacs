#[doc = "Register `PERI_CLK_CTRL10` reader"]
pub type R = crate::R<PERI_CLK_CTRL10_SPEC>;
#[doc = "Register `PERI_CLK_CTRL10` writer"]
pub type W = crate::W<PERI_CLK_CTRL10_SPEC>;
#[doc = "Field `I2C0_CLK_SRC_SEL` reader - Reserved"]
pub type I2C0_CLK_SRC_SEL_R = crate::BitReader;
#[doc = "Field `I2C0_CLK_SRC_SEL` writer - Reserved"]
pub type I2C0_CLK_SRC_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C0_CLK_EN` reader - Reserved"]
pub type I2C0_CLK_EN_R = crate::BitReader;
#[doc = "Field `I2C0_CLK_EN` writer - Reserved"]
pub type I2C0_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C0_CLK_DIV_NUM` reader - Reserved"]
pub type I2C0_CLK_DIV_NUM_R = crate::FieldReader;
#[doc = "Field `I2C0_CLK_DIV_NUM` writer - Reserved"]
pub type I2C0_CLK_DIV_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `I2C0_CLK_DIV_NUMERATOR` reader - Reserved"]
pub type I2C0_CLK_DIV_NUMERATOR_R = crate::FieldReader;
#[doc = "Field `I2C0_CLK_DIV_NUMERATOR` writer - Reserved"]
pub type I2C0_CLK_DIV_NUMERATOR_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `I2C0_CLK_DIV_DENOMINATOR` reader - Reserved"]
pub type I2C0_CLK_DIV_DENOMINATOR_R = crate::FieldReader;
#[doc = "Field `I2C0_CLK_DIV_DENOMINATOR` writer - Reserved"]
pub type I2C0_CLK_DIV_DENOMINATOR_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `I2C1_CLK_SRC_SEL` reader - Reserved"]
pub type I2C1_CLK_SRC_SEL_R = crate::BitReader;
#[doc = "Field `I2C1_CLK_SRC_SEL` writer - Reserved"]
pub type I2C1_CLK_SRC_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C1_CLK_EN` reader - Reserved"]
pub type I2C1_CLK_EN_R = crate::BitReader;
#[doc = "Field `I2C1_CLK_EN` writer - Reserved"]
pub type I2C1_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Reserved"]
    #[inline(always)]
    pub fn i2c0_clk_src_sel(&self) -> I2C0_CLK_SRC_SEL_R {
        I2C0_CLK_SRC_SEL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Reserved"]
    #[inline(always)]
    pub fn i2c0_clk_en(&self) -> I2C0_CLK_EN_R {
        I2C0_CLK_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:9 - Reserved"]
    #[inline(always)]
    pub fn i2c0_clk_div_num(&self) -> I2C0_CLK_DIV_NUM_R {
        I2C0_CLK_DIV_NUM_R::new(((self.bits >> 2) & 0xff) as u8)
    }
    #[doc = "Bits 10:17 - Reserved"]
    #[inline(always)]
    pub fn i2c0_clk_div_numerator(&self) -> I2C0_CLK_DIV_NUMERATOR_R {
        I2C0_CLK_DIV_NUMERATOR_R::new(((self.bits >> 10) & 0xff) as u8)
    }
    #[doc = "Bits 18:25 - Reserved"]
    #[inline(always)]
    pub fn i2c0_clk_div_denominator(&self) -> I2C0_CLK_DIV_DENOMINATOR_R {
        I2C0_CLK_DIV_DENOMINATOR_R::new(((self.bits >> 18) & 0xff) as u8)
    }
    #[doc = "Bit 26 - Reserved"]
    #[inline(always)]
    pub fn i2c1_clk_src_sel(&self) -> I2C1_CLK_SRC_SEL_R {
        I2C1_CLK_SRC_SEL_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Reserved"]
    #[inline(always)]
    pub fn i2c1_clk_en(&self) -> I2C1_CLK_EN_R {
        I2C1_CLK_EN_R::new(((self.bits >> 27) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PERI_CLK_CTRL10")
            .field("i2c0_clk_src_sel", &self.i2c0_clk_src_sel())
            .field("i2c0_clk_en", &self.i2c0_clk_en())
            .field("i2c0_clk_div_num", &self.i2c0_clk_div_num())
            .field("i2c0_clk_div_numerator", &self.i2c0_clk_div_numerator())
            .field("i2c0_clk_div_denominator", &self.i2c0_clk_div_denominator())
            .field("i2c1_clk_src_sel", &self.i2c1_clk_src_sel())
            .field("i2c1_clk_en", &self.i2c1_clk_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn i2c0_clk_src_sel(&mut self) -> I2C0_CLK_SRC_SEL_W<PERI_CLK_CTRL10_SPEC> {
        I2C0_CLK_SRC_SEL_W::new(self, 0)
    }
    #[doc = "Bit 1 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn i2c0_clk_en(&mut self) -> I2C0_CLK_EN_W<PERI_CLK_CTRL10_SPEC> {
        I2C0_CLK_EN_W::new(self, 1)
    }
    #[doc = "Bits 2:9 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn i2c0_clk_div_num(&mut self) -> I2C0_CLK_DIV_NUM_W<PERI_CLK_CTRL10_SPEC> {
        I2C0_CLK_DIV_NUM_W::new(self, 2)
    }
    #[doc = "Bits 10:17 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn i2c0_clk_div_numerator(&mut self) -> I2C0_CLK_DIV_NUMERATOR_W<PERI_CLK_CTRL10_SPEC> {
        I2C0_CLK_DIV_NUMERATOR_W::new(self, 10)
    }
    #[doc = "Bits 18:25 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn i2c0_clk_div_denominator(&mut self) -> I2C0_CLK_DIV_DENOMINATOR_W<PERI_CLK_CTRL10_SPEC> {
        I2C0_CLK_DIV_DENOMINATOR_W::new(self, 18)
    }
    #[doc = "Bit 26 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn i2c1_clk_src_sel(&mut self) -> I2C1_CLK_SRC_SEL_W<PERI_CLK_CTRL10_SPEC> {
        I2C1_CLK_SRC_SEL_W::new(self, 26)
    }
    #[doc = "Bit 27 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn i2c1_clk_en(&mut self) -> I2C1_CLK_EN_W<PERI_CLK_CTRL10_SPEC> {
        I2C1_CLK_EN_W::new(self, 27)
    }
}
#[doc = "Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`peri_clk_ctrl10::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peri_clk_ctrl10::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PERI_CLK_CTRL10_SPEC;
impl crate::RegisterSpec for PERI_CLK_CTRL10_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`peri_clk_ctrl10::R`](R) reader structure"]
impl crate::Readable for PERI_CLK_CTRL10_SPEC {}
#[doc = "`write(|w| ..)` method takes [`peri_clk_ctrl10::W`](W) writer structure"]
impl crate::Writable for PERI_CLK_CTRL10_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PERI_CLK_CTRL10 to value 0"]
impl crate::Resettable for PERI_CLK_CTRL10_SPEC {
    const RESET_VALUE: u32 = 0;
}
