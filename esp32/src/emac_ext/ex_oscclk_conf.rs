#[doc = "Register `EX_OSCCLK_CONF` reader"]
pub type R = crate::R<EX_OSCCLK_CONF_SPEC>;
#[doc = "Register `EX_OSCCLK_CONF` writer"]
pub type W = crate::W<EX_OSCCLK_CONF_SPEC>;
#[doc = "Field `DIV_NUM_10M` reader - "]
pub type DIV_NUM_10M_R = crate::FieldReader;
#[doc = "Field `DIV_NUM_10M` writer - "]
pub type DIV_NUM_10M_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O>;
#[doc = "Field `H_DIV_NUM_10M` reader - "]
pub type H_DIV_NUM_10M_R = crate::FieldReader;
#[doc = "Field `H_DIV_NUM_10M` writer - "]
pub type H_DIV_NUM_10M_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O>;
#[doc = "Field `DIV_NUM_100M` reader - "]
pub type DIV_NUM_100M_R = crate::FieldReader;
#[doc = "Field `DIV_NUM_100M` writer - "]
pub type DIV_NUM_100M_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O>;
#[doc = "Field `H_DIV_NUM_100M` reader - "]
pub type H_DIV_NUM_100M_R = crate::FieldReader;
#[doc = "Field `H_DIV_NUM_100M` writer - "]
pub type H_DIV_NUM_100M_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O>;
#[doc = "Field `CLK_SEL` reader - "]
pub type CLK_SEL_R = crate::BitReader;
#[doc = "Field `CLK_SEL` writer - "]
pub type CLK_SEL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn div_num_10m(&self) -> DIV_NUM_10M_R {
        DIV_NUM_10M_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:11"]
    #[inline(always)]
    pub fn h_div_num_10m(&self) -> H_DIV_NUM_10M_R {
        H_DIV_NUM_10M_R::new(((self.bits >> 6) & 0x3f) as u8)
    }
    #[doc = "Bits 12:17"]
    #[inline(always)]
    pub fn div_num_100m(&self) -> DIV_NUM_100M_R {
        DIV_NUM_100M_R::new(((self.bits >> 12) & 0x3f) as u8)
    }
    #[doc = "Bits 18:23"]
    #[inline(always)]
    pub fn h_div_num_100m(&self) -> H_DIV_NUM_100M_R {
        H_DIV_NUM_100M_R::new(((self.bits >> 18) & 0x3f) as u8)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn clk_sel(&self) -> CLK_SEL_R {
        CLK_SEL_R::new(((self.bits >> 24) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EX_OSCCLK_CONF")
            .field(
                "div_num_10m",
                &format_args!("{}", self.div_num_10m().bits()),
            )
            .field(
                "h_div_num_10m",
                &format_args!("{}", self.h_div_num_10m().bits()),
            )
            .field(
                "div_num_100m",
                &format_args!("{}", self.div_num_100m().bits()),
            )
            .field(
                "h_div_num_100m",
                &format_args!("{}", self.h_div_num_100m().bits()),
            )
            .field("clk_sel", &format_args!("{}", self.clk_sel().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<EX_OSCCLK_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    #[must_use]
    pub fn div_num_10m(&mut self) -> DIV_NUM_10M_W<EX_OSCCLK_CONF_SPEC, 0> {
        DIV_NUM_10M_W::new(self)
    }
    #[doc = "Bits 6:11"]
    #[inline(always)]
    #[must_use]
    pub fn h_div_num_10m(&mut self) -> H_DIV_NUM_10M_W<EX_OSCCLK_CONF_SPEC, 6> {
        H_DIV_NUM_10M_W::new(self)
    }
    #[doc = "Bits 12:17"]
    #[inline(always)]
    #[must_use]
    pub fn div_num_100m(&mut self) -> DIV_NUM_100M_W<EX_OSCCLK_CONF_SPEC, 12> {
        DIV_NUM_100M_W::new(self)
    }
    #[doc = "Bits 18:23"]
    #[inline(always)]
    #[must_use]
    pub fn h_div_num_100m(&mut self) -> H_DIV_NUM_100M_W<EX_OSCCLK_CONF_SPEC, 18> {
        H_DIV_NUM_100M_W::new(self)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    #[must_use]
    pub fn clk_sel(&mut self) -> CLK_SEL_W<EX_OSCCLK_CONF_SPEC, 24> {
        CLK_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "RMII clock half and whole divider settings\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ex_oscclk_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ex_oscclk_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EX_OSCCLK_CONF_SPEC;
impl crate::RegisterSpec for EX_OSCCLK_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ex_oscclk_conf::R`](R) reader structure"]
impl crate::Readable for EX_OSCCLK_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ex_oscclk_conf::W`](W) writer structure"]
impl crate::Writable for EX_OSCCLK_CONF_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EX_OSCCLK_CONF to value 0"]
impl crate::Resettable for EX_OSCCLK_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
