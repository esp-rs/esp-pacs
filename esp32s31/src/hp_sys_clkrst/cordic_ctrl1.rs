#[doc = "Register `CORDIC_CTRL1` reader"]
pub type R = crate::R<CORDIC_CTRL1_SPEC>;
#[doc = "Register `CORDIC_CTRL1` writer"]
pub type W = crate::W<CORDIC_CTRL1_SPEC>;
#[doc = "Field `CORDIC_CLK_DIV_NUM` reader - need_des"]
pub type CORDIC_CLK_DIV_NUM_R = crate::FieldReader;
#[doc = "Field `CORDIC_CLK_DIV_NUM` writer - need_des"]
pub type CORDIC_CLK_DIV_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CORDIC_CLK_DIV_NUMERATOR` reader - need_des"]
pub type CORDIC_CLK_DIV_NUMERATOR_R = crate::FieldReader;
#[doc = "Field `CORDIC_CLK_DIV_NUMERATOR` writer - need_des"]
pub type CORDIC_CLK_DIV_NUMERATOR_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CORDIC_CLK_DIV_DENONIMATOR` reader - need_des"]
pub type CORDIC_CLK_DIV_DENONIMATOR_R = crate::FieldReader;
#[doc = "Field `CORDIC_CLK_DIV_DENONIMATOR` writer - need_des"]
pub type CORDIC_CLK_DIV_DENONIMATOR_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - need_des"]
    #[inline(always)]
    pub fn cordic_clk_div_num(&self) -> CORDIC_CLK_DIV_NUM_R {
        CORDIC_CLK_DIV_NUM_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - need_des"]
    #[inline(always)]
    pub fn cordic_clk_div_numerator(&self) -> CORDIC_CLK_DIV_NUMERATOR_R {
        CORDIC_CLK_DIV_NUMERATOR_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - need_des"]
    #[inline(always)]
    pub fn cordic_clk_div_denonimator(&self) -> CORDIC_CLK_DIV_DENONIMATOR_R {
        CORDIC_CLK_DIV_DENONIMATOR_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORDIC_CTRL1")
            .field("cordic_clk_div_num", &self.cordic_clk_div_num())
            .field("cordic_clk_div_numerator", &self.cordic_clk_div_numerator())
            .field(
                "cordic_clk_div_denonimator",
                &self.cordic_clk_div_denonimator(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - need_des"]
    #[inline(always)]
    pub fn cordic_clk_div_num(&mut self) -> CORDIC_CLK_DIV_NUM_W<'_, CORDIC_CTRL1_SPEC> {
        CORDIC_CLK_DIV_NUM_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - need_des"]
    #[inline(always)]
    pub fn cordic_clk_div_numerator(
        &mut self,
    ) -> CORDIC_CLK_DIV_NUMERATOR_W<'_, CORDIC_CTRL1_SPEC> {
        CORDIC_CLK_DIV_NUMERATOR_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - need_des"]
    #[inline(always)]
    pub fn cordic_clk_div_denonimator(
        &mut self,
    ) -> CORDIC_CLK_DIV_DENONIMATOR_W<'_, CORDIC_CTRL1_SPEC> {
        CORDIC_CLK_DIV_DENONIMATOR_W::new(self, 16)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`cordic_ctrl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cordic_ctrl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CORDIC_CTRL1_SPEC;
impl crate::RegisterSpec for CORDIC_CTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cordic_ctrl1::R`](R) reader structure"]
impl crate::Readable for CORDIC_CTRL1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cordic_ctrl1::W`](W) writer structure"]
impl crate::Writable for CORDIC_CTRL1_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CORDIC_CTRL1 to value 0"]
impl crate::Resettable for CORDIC_CTRL1_SPEC {}
