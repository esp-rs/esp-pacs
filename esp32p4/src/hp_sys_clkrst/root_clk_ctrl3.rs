#[doc = "Register `ROOT_CLK_CTRL3` reader"]
pub type R = crate::R<ROOT_CLK_CTRL3_SPEC>;
#[doc = "Register `ROOT_CLK_CTRL3` writer"]
pub type W = crate::W<ROOT_CLK_CTRL3_SPEC>;
#[doc = "Field `REG_APB_CLK_DIV_DENOMINATOR` reader - Reserved"]
pub type REG_APB_CLK_DIV_DENOMINATOR_R = crate::FieldReader;
#[doc = "Field `REG_APB_CLK_DIV_DENOMINATOR` writer - Reserved"]
pub type REG_APB_CLK_DIV_DENOMINATOR_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Reserved"]
    #[inline(always)]
    pub fn reg_apb_clk_div_denominator(&self) -> REG_APB_CLK_DIV_DENOMINATOR_R {
        REG_APB_CLK_DIV_DENOMINATOR_R::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ROOT_CLK_CTRL3")
            .field(
                "reg_apb_clk_div_denominator",
                &format_args!("{}", self.reg_apb_clk_div_denominator().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<ROOT_CLK_CTRL3_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:7 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reg_apb_clk_div_denominator(
        &mut self,
    ) -> REG_APB_CLK_DIV_DENOMINATOR_W<ROOT_CLK_CTRL3_SPEC> {
        REG_APB_CLK_DIV_DENOMINATOR_W::new(self, 0)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`root_clk_ctrl3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`root_clk_ctrl3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ROOT_CLK_CTRL3_SPEC;
impl crate::RegisterSpec for ROOT_CLK_CTRL3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`root_clk_ctrl3::R`](R) reader structure"]
impl crate::Readable for ROOT_CLK_CTRL3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`root_clk_ctrl3::W`](W) writer structure"]
impl crate::Writable for ROOT_CLK_CTRL3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ROOT_CLK_CTRL3 to value 0"]
impl crate::Resettable for ROOT_CLK_CTRL3_SPEC {
    const RESET_VALUE: u32 = 0;
}
