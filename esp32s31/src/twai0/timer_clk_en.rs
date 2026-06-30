#[doc = "Register `TIMER_CLK_EN` reader"]
pub type R = crate::R<TIMER_CLK_EN_SPEC>;
#[doc = "Register `TIMER_CLK_EN` writer"]
pub type W = crate::W<TIMER_CLK_EN_SPEC>;
#[doc = "Field `CLK_EN` reader - Set this bit to force enable TWAIFD register configuration clock signal."]
pub type CLK_EN_R = crate::BitReader;
#[doc = "Field `CLK_EN` writer - Set this bit to force enable TWAIFD register configuration clock signal."]
pub type CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_RXBUF_MEM_CLK_ON` reader - Set this bit to force enable TWAIFD RX buffer ram clock signal."]
pub type FORCE_RXBUF_MEM_CLK_ON_R = crate::BitReader;
#[doc = "Field `FORCE_RXBUF_MEM_CLK_ON` writer - Set this bit to force enable TWAIFD RX buffer ram clock signal."]
pub type FORCE_RXBUF_MEM_CLK_ON_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Set this bit to force enable TWAIFD register configuration clock signal."]
    #[inline(always)]
    pub fn clk_en(&self) -> CLK_EN_R {
        CLK_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set this bit to force enable TWAIFD RX buffer ram clock signal."]
    #[inline(always)]
    pub fn force_rxbuf_mem_clk_on(&self) -> FORCE_RXBUF_MEM_CLK_ON_R {
        FORCE_RXBUF_MEM_CLK_ON_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMER_CLK_EN")
            .field("clk_en", &self.clk_en())
            .field("force_rxbuf_mem_clk_on", &self.force_rxbuf_mem_clk_on())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to force enable TWAIFD register configuration clock signal."]
    #[inline(always)]
    pub fn clk_en(&mut self) -> CLK_EN_W<'_, TIMER_CLK_EN_SPEC> {
        CLK_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Set this bit to force enable TWAIFD RX buffer ram clock signal."]
    #[inline(always)]
    pub fn force_rxbuf_mem_clk_on(&mut self) -> FORCE_RXBUF_MEM_CLK_ON_W<'_, TIMER_CLK_EN_SPEC> {
        FORCE_RXBUF_MEM_CLK_ON_W::new(self, 1)
    }
}
#[doc = "TWAIFD timer clock force enable register.\n\nYou can [`read`](crate::Reg::read) this register and get [`timer_clk_en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer_clk_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIMER_CLK_EN_SPEC;
impl crate::RegisterSpec for TIMER_CLK_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timer_clk_en::R`](R) reader structure"]
impl crate::Readable for TIMER_CLK_EN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`timer_clk_en::W`](W) writer structure"]
impl crate::Writable for TIMER_CLK_EN_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TIMER_CLK_EN to value 0"]
impl crate::Resettable for TIMER_CLK_EN_SPEC {}
