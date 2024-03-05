#[doc = "Register `CONF` reader"]
pub type R = crate::R<CONF_SPEC>;
#[doc = "Register `CONF` writer"]
pub type W = crate::W<CONF_SPEC>;
#[doc = "Field `APB_CLK_SEL` reader - reg_apb_clk_sel."]
pub type APB_CLK_SEL_R = crate::FieldReader;
#[doc = "Field `APB_CLK_SEL` writer - reg_apb_clk_sel."]
pub type APB_CLK_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CLK_EN` reader - reg_clk_en."]
pub type CLK_EN_R = crate::BitReader;
#[doc = "Field `CLK_EN` writer - reg_clk_en."]
pub type CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - reg_apb_clk_sel."]
    #[inline(always)]
    pub fn apb_clk_sel(&self) -> APB_CLK_SEL_R {
        APB_CLK_SEL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 31 - reg_clk_en."]
    #[inline(always)]
    pub fn clk_en(&self) -> CLK_EN_R {
        CLK_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONF")
            .field(
                "apb_clk_sel",
                &format_args!("{}", self.apb_clk_sel().bits()),
            )
            .field("clk_en", &format_args!("{}", self.clk_en().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:1 - reg_apb_clk_sel."]
    #[inline(always)]
    #[must_use]
    pub fn apb_clk_sel(&mut self) -> APB_CLK_SEL_W<CONF_SPEC> {
        APB_CLK_SEL_W::new(self, 0)
    }
    #[doc = "Bit 31 - reg_clk_en."]
    #[inline(always)]
    #[must_use]
    pub fn clk_en(&mut self) -> CLK_EN_W<CONF_SPEC> {
        CLK_EN_W::new(self, 31)
    }
}
#[doc = "LEDC_CONF.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CONF_SPEC;
impl crate::RegisterSpec for CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`conf::R`](R) reader structure"]
impl crate::Readable for CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`conf::W`](W) writer structure"]
impl crate::Writable for CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CONF to value 0"]
impl crate::Resettable for CONF_SPEC {
    const RESET_VALUE: u32 = 0;
}
