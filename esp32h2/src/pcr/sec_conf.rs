#[doc = "Register `SEC_CONF` reader"]
pub type R = crate::R<SEC_CONF_SPEC>;
#[doc = "Register `SEC_CONF` writer"]
pub type W = crate::W<SEC_CONF_SPEC>;
#[doc = "Field `SEC_CLK_SEL` reader - xxxx"]
pub type SEC_CLK_SEL_R = crate::FieldReader;
#[doc = "Field `SEC_CLK_SEL` writer - xxxx"]
pub type SEC_CLK_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
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
            .field("sec_clk_sel", &self.sec_clk_sel())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - xxxx"]
    #[inline(always)]
    pub fn sec_clk_sel(&mut self) -> SEC_CLK_SEL_W<SEC_CONF_SPEC> {
        SEC_CLK_SEL_W::new(self, 0)
    }
}
#[doc = "xxxx\n\nYou can [`read`](crate::Reg::read) this register and get [`sec_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SEC_CONF_SPEC;
impl crate::RegisterSpec for SEC_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sec_conf::R`](R) reader structure"]
impl crate::Readable for SEC_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sec_conf::W`](W) writer structure"]
impl crate::Writable for SEC_CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SEC_CONF to value 0"]
impl crate::Resettable for SEC_CONF_SPEC {
    const RESET_VALUE: u32 = 0;
}
