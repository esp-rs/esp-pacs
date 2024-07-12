#[doc = "Register `PERI_CLK_CTRL120` reader"]
pub type R = crate::R<PERI_CLK_CTRL120_SPEC>;
#[doc = "Register `PERI_CLK_CTRL120` writer"]
pub type W = crate::W<PERI_CLK_CTRL120_SPEC>;
#[doc = "Field `CAM_CLK_DIV_NUM` reader - Reserved"]
pub type CAM_CLK_DIV_NUM_R = crate::FieldReader;
#[doc = "Field `CAM_CLK_DIV_NUM` writer - Reserved"]
pub type CAM_CLK_DIV_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CAM_CLK_DIV_NUMERATOR` reader - Reserved"]
pub type CAM_CLK_DIV_NUMERATOR_R = crate::FieldReader;
#[doc = "Field `CAM_CLK_DIV_NUMERATOR` writer - Reserved"]
pub type CAM_CLK_DIV_NUMERATOR_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CAM_CLK_DIV_DENOMINATOR` reader - Reserved"]
pub type CAM_CLK_DIV_DENOMINATOR_R = crate::FieldReader;
#[doc = "Field `CAM_CLK_DIV_DENOMINATOR` writer - Reserved"]
pub type CAM_CLK_DIV_DENOMINATOR_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Reserved"]
    #[inline(always)]
    pub fn cam_clk_div_num(&self) -> CAM_CLK_DIV_NUM_R {
        CAM_CLK_DIV_NUM_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Reserved"]
    #[inline(always)]
    pub fn cam_clk_div_numerator(&self) -> CAM_CLK_DIV_NUMERATOR_R {
        CAM_CLK_DIV_NUMERATOR_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Reserved"]
    #[inline(always)]
    pub fn cam_clk_div_denominator(&self) -> CAM_CLK_DIV_DENOMINATOR_R {
        CAM_CLK_DIV_DENOMINATOR_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PERI_CLK_CTRL120")
            .field("cam_clk_div_num", &self.cam_clk_div_num())
            .field("cam_clk_div_numerator", &self.cam_clk_div_numerator())
            .field("cam_clk_div_denominator", &self.cam_clk_div_denominator())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn cam_clk_div_num(&mut self) -> CAM_CLK_DIV_NUM_W<PERI_CLK_CTRL120_SPEC> {
        CAM_CLK_DIV_NUM_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn cam_clk_div_numerator(&mut self) -> CAM_CLK_DIV_NUMERATOR_W<PERI_CLK_CTRL120_SPEC> {
        CAM_CLK_DIV_NUMERATOR_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn cam_clk_div_denominator(&mut self) -> CAM_CLK_DIV_DENOMINATOR_W<PERI_CLK_CTRL120_SPEC> {
        CAM_CLK_DIV_DENOMINATOR_W::new(self, 16)
    }
}
#[doc = "Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`peri_clk_ctrl120::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peri_clk_ctrl120::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PERI_CLK_CTRL120_SPEC;
impl crate::RegisterSpec for PERI_CLK_CTRL120_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`peri_clk_ctrl120::R`](R) reader structure"]
impl crate::Readable for PERI_CLK_CTRL120_SPEC {}
#[doc = "`write(|w| ..)` method takes [`peri_clk_ctrl120::W`](W) writer structure"]
impl crate::Writable for PERI_CLK_CTRL120_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PERI_CLK_CTRL120 to value 0"]
impl crate::Resettable for PERI_CLK_CTRL120_SPEC {
    const RESET_VALUE: u32 = 0;
}
