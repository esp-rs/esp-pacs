#[doc = "Register `SEC_CONF` reader"]
pub type R = crate::R<SEC_CONF_SPEC>;
#[doc = "Register `SEC_CONF` writer"]
pub type W = crate::W<SEC_CONF_SPEC>;
#[doc = "Field `SEC_CLK_SEL` reader - xxxx"]
pub type SEC_CLK_SEL_R = crate::FieldReader;
#[doc = "Field `SEC_CLK_SEL` writer - xxxx"]
pub type SEC_CLK_SEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
impl R {
    #[doc = "Bits 0:1 - xxxx"]
    #[inline(always)]
    pub fn sec_clk_sel(&self) -> SEC_CLK_SEL_R {
        SEC_CLK_SEL_R::new((self.bits & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SEC_CONF")
            .field(
                "sec_clk_sel",
                &format_args!("{}", self.sec_clk_sel().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SEC_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:1 - xxxx"]
    #[inline(always)]
    #[must_use]
    pub fn sec_clk_sel(&mut self) -> SEC_CLK_SEL_W<SEC_CONF_SPEC, 0> {
        SEC_CLK_SEL_W::new(self)
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
#[doc = "xxxx\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sec_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sec_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SEC_CONF_SPEC;
impl crate::RegisterSpec for SEC_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sec_conf::R`](R) reader structure"]
impl crate::Readable for SEC_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sec_conf::W`](W) writer structure"]
impl crate::Writable for SEC_CONF_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SEC_CONF to value 0"]
impl crate::Resettable for SEC_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
