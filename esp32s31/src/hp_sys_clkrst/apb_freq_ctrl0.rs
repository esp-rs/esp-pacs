#[doc = "Register `APB_FREQ_CTRL0` reader"]
pub type R = crate::R<APB_FREQ_CTRL0_SPEC>;
#[doc = "Register `APB_FREQ_CTRL0` writer"]
pub type W = crate::W<APB_FREQ_CTRL0_SPEC>;
#[doc = "Field `REG_APB_CLK_DIV_NUM` reader - need_des"]
pub type REG_APB_CLK_DIV_NUM_R = crate::FieldReader;
#[doc = "Field `REG_APB_CLK_DIV_NUM` writer - need_des"]
pub type REG_APB_CLK_DIV_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `REG_APB_CLK_DIV_NUMERATOR` reader - need_des"]
pub type REG_APB_CLK_DIV_NUMERATOR_R = crate::FieldReader;
#[doc = "Field `REG_APB_CLK_DIV_NUMERATOR` writer - need_des"]
pub type REG_APB_CLK_DIV_NUMERATOR_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `REG_APB_CLK_DIV_DENOMINATOR` reader - need_des"]
pub type REG_APB_CLK_DIV_DENOMINATOR_R = crate::FieldReader;
#[doc = "Field `REG_APB_CLK_DIV_DENOMINATOR` writer - need_des"]
pub type REG_APB_CLK_DIV_DENOMINATOR_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:7 - need_des"]
    #[inline(always)]
    pub fn reg_apb_clk_div_num(&self) -> REG_APB_CLK_DIV_NUM_R {
        REG_APB_CLK_DIV_NUM_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:10 - need_des"]
    #[inline(always)]
    pub fn reg_apb_clk_div_numerator(&self) -> REG_APB_CLK_DIV_NUMERATOR_R {
        REG_APB_CLK_DIV_NUMERATOR_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:13 - need_des"]
    #[inline(always)]
    pub fn reg_apb_clk_div_denominator(&self) -> REG_APB_CLK_DIV_DENOMINATOR_R {
        REG_APB_CLK_DIV_DENOMINATOR_R::new(((self.bits >> 11) & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB_FREQ_CTRL0")
            .field("reg_apb_clk_div_num", &self.reg_apb_clk_div_num())
            .field(
                "reg_apb_clk_div_numerator",
                &self.reg_apb_clk_div_numerator(),
            )
            .field(
                "reg_apb_clk_div_denominator",
                &self.reg_apb_clk_div_denominator(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - need_des"]
    #[inline(always)]
    pub fn reg_apb_clk_div_num(&mut self) -> REG_APB_CLK_DIV_NUM_W<'_, APB_FREQ_CTRL0_SPEC> {
        REG_APB_CLK_DIV_NUM_W::new(self, 0)
    }
    #[doc = "Bits 8:10 - need_des"]
    #[inline(always)]
    pub fn reg_apb_clk_div_numerator(
        &mut self,
    ) -> REG_APB_CLK_DIV_NUMERATOR_W<'_, APB_FREQ_CTRL0_SPEC> {
        REG_APB_CLK_DIV_NUMERATOR_W::new(self, 8)
    }
    #[doc = "Bits 11:13 - need_des"]
    #[inline(always)]
    pub fn reg_apb_clk_div_denominator(
        &mut self,
    ) -> REG_APB_CLK_DIV_DENOMINATOR_W<'_, APB_FREQ_CTRL0_SPEC> {
        REG_APB_CLK_DIV_DENOMINATOR_W::new(self, 11)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`apb_freq_ctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb_freq_ctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APB_FREQ_CTRL0_SPEC;
impl crate::RegisterSpec for APB_FREQ_CTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb_freq_ctrl0::R`](R) reader structure"]
impl crate::Readable for APB_FREQ_CTRL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`apb_freq_ctrl0::W`](W) writer structure"]
impl crate::Writable for APB_FREQ_CTRL0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets APB_FREQ_CTRL0 to value 0x01"]
impl crate::Resettable for APB_FREQ_CTRL0_SPEC {
    const RESET_VALUE: u32 = 0x01;
}
