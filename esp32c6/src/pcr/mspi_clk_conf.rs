#[doc = "Register `MSPI_CLK_CONF` reader"]
pub type R = crate::R<MSPI_CLK_CONF_SPEC>;
#[doc = "Register `MSPI_CLK_CONF` writer"]
pub type W = crate::W<MSPI_CLK_CONF_SPEC>;
#[doc = "Field `MSPI_FAST_LS_DIV_NUM` reader - Set as one within (0,1,2) to generate div1(default)/div2/div4 of low-speed clock-source to drive clk_mspi_fast. Only avaiable whe the clck-source is a low-speed clock-source such as XTAL/FOSC."]
pub type MSPI_FAST_LS_DIV_NUM_R = crate::FieldReader;
#[doc = "Field `MSPI_FAST_LS_DIV_NUM` writer - Set as one within (0,1,2) to generate div1(default)/div2/div4 of low-speed clock-source to drive clk_mspi_fast. Only avaiable whe the clck-source is a low-speed clock-source such as XTAL/FOSC."]
pub type MSPI_FAST_LS_DIV_NUM_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `MSPI_FAST_HS_DIV_NUM` reader - Set as one within (3,4,5) to generate div4(default)/div5/div6 of high-speed clock-source to drive clk_mspi_fast. Only avaiable whe the clck-source is a high-speed clock-source such as SPLL."]
pub type MSPI_FAST_HS_DIV_NUM_R = crate::FieldReader;
#[doc = "Field `MSPI_FAST_HS_DIV_NUM` writer - Set as one within (3,4,5) to generate div4(default)/div5/div6 of high-speed clock-source to drive clk_mspi_fast. Only avaiable whe the clck-source is a high-speed clock-source such as SPLL."]
pub type MSPI_FAST_HS_DIV_NUM_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Set as one within (0,1,2) to generate div1(default)/div2/div4 of low-speed clock-source to drive clk_mspi_fast. Only avaiable whe the clck-source is a low-speed clock-source such as XTAL/FOSC."]
    #[inline(always)]
    pub fn mspi_fast_ls_div_num(&self) -> MSPI_FAST_LS_DIV_NUM_R {
        MSPI_FAST_LS_DIV_NUM_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Set as one within (3,4,5) to generate div4(default)/div5/div6 of high-speed clock-source to drive clk_mspi_fast. Only avaiable whe the clck-source is a high-speed clock-source such as SPLL."]
    #[inline(always)]
    pub fn mspi_fast_hs_div_num(&self) -> MSPI_FAST_HS_DIV_NUM_R {
        MSPI_FAST_HS_DIV_NUM_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MSPI_CLK_CONF")
            .field(
                "mspi_fast_ls_div_num",
                &format_args!("{}", self.mspi_fast_ls_div_num().bits()),
            )
            .field(
                "mspi_fast_hs_div_num",
                &format_args!("{}", self.mspi_fast_hs_div_num().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<MSPI_CLK_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7 - Set as one within (0,1,2) to generate div1(default)/div2/div4 of low-speed clock-source to drive clk_mspi_fast. Only avaiable whe the clck-source is a low-speed clock-source such as XTAL/FOSC."]
    #[inline(always)]
    #[must_use]
    pub fn mspi_fast_ls_div_num(&mut self) -> MSPI_FAST_LS_DIV_NUM_W<MSPI_CLK_CONF_SPEC, 0> {
        MSPI_FAST_LS_DIV_NUM_W::new(self)
    }
    #[doc = "Bits 8:15 - Set as one within (3,4,5) to generate div4(default)/div5/div6 of high-speed clock-source to drive clk_mspi_fast. Only avaiable whe the clck-source is a high-speed clock-source such as SPLL."]
    #[inline(always)]
    #[must_use]
    pub fn mspi_fast_hs_div_num(&mut self) -> MSPI_FAST_HS_DIV_NUM_W<MSPI_CLK_CONF_SPEC, 8> {
        MSPI_FAST_HS_DIV_NUM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "MSPI_CLK configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mspi_clk_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mspi_clk_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MSPI_CLK_CONF_SPEC;
impl crate::RegisterSpec for MSPI_CLK_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mspi_clk_conf::R`](R) reader structure"]
impl crate::Readable for MSPI_CLK_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mspi_clk_conf::W`](W) writer structure"]
impl crate::Writable for MSPI_CLK_CONF_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MSPI_CLK_CONF to value 0x0300"]
impl crate::Resettable for MSPI_CLK_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0x0300;
}
