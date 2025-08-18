#[doc = "Register `ROOT_CLK_CTRL3` reader"]
pub type R = crate::R<ROOT_CLK_CTRL3_SPEC>;
#[doc = "Register `ROOT_CLK_CTRL3` writer"]
pub type W = crate::W<ROOT_CLK_CTRL3_SPEC>;
#[doc = "Field `APB_CLK_DIV_DENOMINATOR` reader - Reserved"]
pub type APB_CLK_DIV_DENOMINATOR_R = crate::FieldReader;
#[doc = "Field `APB_CLK_DIV_DENOMINATOR` writer - Reserved"]
pub type APB_CLK_DIV_DENOMINATOR_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Reserved"]
    #[inline(always)]
    pub fn apb_clk_div_denominator(&self) -> APB_CLK_DIV_DENOMINATOR_R {
        APB_CLK_DIV_DENOMINATOR_R::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ROOT_CLK_CTRL3")
            .field("apb_clk_div_denominator", &self.apb_clk_div_denominator())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Reserved"]
    #[inline(always)]
    pub fn apb_clk_div_denominator(
        &mut self,
    ) -> APB_CLK_DIV_DENOMINATOR_W<'_, ROOT_CLK_CTRL3_SPEC> {
        APB_CLK_DIV_DENOMINATOR_W::new(self, 0)
    }
}
#[doc = "Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`root_clk_ctrl3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`root_clk_ctrl3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ROOT_CLK_CTRL3_SPEC;
impl crate::RegisterSpec for ROOT_CLK_CTRL3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`root_clk_ctrl3::R`](R) reader structure"]
impl crate::Readable for ROOT_CLK_CTRL3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`root_clk_ctrl3::W`](W) writer structure"]
impl crate::Writable for ROOT_CLK_CTRL3_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ROOT_CLK_CTRL3 to value 0"]
impl crate::Resettable for ROOT_CLK_CTRL3_SPEC {}
