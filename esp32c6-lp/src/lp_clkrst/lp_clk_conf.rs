#[doc = "Register `LP_CLK_CONF` reader"]
pub struct R(crate::R<LP_CLK_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LP_CLK_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LP_CLK_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LP_CLK_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LP_CLK_CONF` writer"]
pub struct W(crate::W<LP_CLK_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LP_CLK_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<LP_CLK_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LP_CLK_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLOW_CLK_SEL` reader - need_des"]
pub type SLOW_CLK_SEL_R = crate::FieldReader;
#[doc = "Field `SLOW_CLK_SEL` writer - need_des"]
pub type SLOW_CLK_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, LP_CLK_CONF_SPEC, 2, O>;
#[doc = "Field `FAST_CLK_SEL` reader - need_des"]
pub type FAST_CLK_SEL_R = crate::BitReader;
#[doc = "Field `FAST_CLK_SEL` writer - need_des"]
pub type FAST_CLK_SEL_W<'a, const O: u8> = crate::BitWriter<'a, LP_CLK_CONF_SPEC, O>;
#[doc = "Field `LP_PERI_DIV_NUM` reader - need_des"]
pub type LP_PERI_DIV_NUM_R = crate::FieldReader;
#[doc = "Field `LP_PERI_DIV_NUM` writer - need_des"]
pub type LP_PERI_DIV_NUM_W<'a, const O: u8> = crate::FieldWriter<'a, LP_CLK_CONF_SPEC, 8, O>;
impl R {
    #[doc = "Bits 0:1 - need_des"]
    #[inline(always)]
    pub fn slow_clk_sel(&self) -> SLOW_CLK_SEL_R {
        SLOW_CLK_SEL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - need_des"]
    #[inline(always)]
    pub fn fast_clk_sel(&self) -> FAST_CLK_SEL_R {
        FAST_CLK_SEL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:10 - need_des"]
    #[inline(always)]
    pub fn lp_peri_div_num(&self) -> LP_PERI_DIV_NUM_R {
        LP_PERI_DIV_NUM_R::new(((self.bits >> 3) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LP_CLK_CONF")
            .field(
                "slow_clk_sel",
                &format_args!("{}", self.slow_clk_sel().bits()),
            )
            .field(
                "fast_clk_sel",
                &format_args!("{}", self.fast_clk_sel().bit()),
            )
            .field(
                "lp_peri_div_num",
                &format_args!("{}", self.lp_peri_div_num().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<LP_CLK_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:1 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn slow_clk_sel(&mut self) -> SLOW_CLK_SEL_W<0> {
        SLOW_CLK_SEL_W::new(self)
    }
    #[doc = "Bit 2 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn fast_clk_sel(&mut self) -> FAST_CLK_SEL_W<2> {
        FAST_CLK_SEL_W::new(self)
    }
    #[doc = "Bits 3:10 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_peri_div_num(&mut self) -> LP_PERI_DIV_NUM_W<3> {
        LP_PERI_DIV_NUM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "need_des\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lp_clk_conf](index.html) module"]
pub struct LP_CLK_CONF_SPEC;
impl crate::RegisterSpec for LP_CLK_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lp_clk_conf::R](R) reader structure"]
impl crate::Readable for LP_CLK_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lp_clk_conf::W](W) writer structure"]
impl crate::Writable for LP_CLK_CONF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LP_CLK_CONF to value 0x04"]
impl crate::Resettable for LP_CLK_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0x04;
}
