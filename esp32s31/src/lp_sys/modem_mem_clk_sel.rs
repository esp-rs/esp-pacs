#[doc = "Register `MODEM_MEM_CLK_SEL` reader"]
pub type R = crate::R<MODEM_MEM_CLK_SEL_SPEC>;
#[doc = "Register `MODEM_MEM_CLK_SEL` writer"]
pub type W = crate::W<MODEM_MEM_CLK_SEL_SPEC>;
#[doc = "Field `MODEM_MEM_CLK_SEL` reader - "]
pub type MODEM_MEM_CLK_SEL_R = crate::BitReader;
#[doc = "Field `MODEM_MEM_CLK_SEL` writer - "]
pub type MODEM_MEM_CLK_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn modem_mem_clk_sel(&self) -> MODEM_MEM_CLK_SEL_R {
        MODEM_MEM_CLK_SEL_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MODEM_MEM_CLK_SEL")
            .field("modem_mem_clk_sel", &self.modem_mem_clk_sel())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn modem_mem_clk_sel(&mut self) -> MODEM_MEM_CLK_SEL_W<'_, MODEM_MEM_CLK_SEL_SPEC> {
        MODEM_MEM_CLK_SEL_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`modem_mem_clk_sel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`modem_mem_clk_sel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MODEM_MEM_CLK_SEL_SPEC;
impl crate::RegisterSpec for MODEM_MEM_CLK_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`modem_mem_clk_sel::R`](R) reader structure"]
impl crate::Readable for MODEM_MEM_CLK_SEL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`modem_mem_clk_sel::W`](W) writer structure"]
impl crate::Writable for MODEM_MEM_CLK_SEL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MODEM_MEM_CLK_SEL to value 0"]
impl crate::Resettable for MODEM_MEM_CLK_SEL_SPEC {}
