#[doc = "Register `HP_CLK_EN` reader"]
pub type R = crate::R<HP_CLK_EN_SPEC>;
#[doc = "Register `HP_CLK_EN` writer"]
pub type W = crate::W<HP_CLK_EN_SPEC>;
#[doc = "Field `HP_REG_CLK_EN` reader - NA"]
pub type HP_REG_CLK_EN_R = crate::BitReader;
#[doc = "Field `HP_REG_CLK_EN` writer - NA"]
pub type HP_REG_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn hp_reg_clk_en(&self) -> HP_REG_CLK_EN_R {
        HP_REG_CLK_EN_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HP_CLK_EN")
            .field(
                "hp_reg_clk_en",
                &format_args!("{}", self.hp_reg_clk_en().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HP_CLK_EN_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn hp_reg_clk_en(&mut self) -> HP_REG_CLK_EN_W<HP_CLK_EN_SPEC> {
        HP_REG_CLK_EN_W::new(self, 0)
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
#[doc = "NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hp_clk_en::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_clk_en::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HP_CLK_EN_SPEC;
impl crate::RegisterSpec for HP_CLK_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hp_clk_en::R`](R) reader structure"]
impl crate::Readable for HP_CLK_EN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hp_clk_en::W`](W) writer structure"]
impl crate::Writable for HP_CLK_EN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HP_CLK_EN to value 0"]
impl crate::Resettable for HP_CLK_EN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
