#[doc = "Register `REG_CLKEN` reader"]
pub type R = crate::R<REG_CLKEN_SPEC>;
#[doc = "Register `REG_CLKEN` writer"]
pub type W = crate::W<REG_CLKEN_SPEC>;
#[doc = "Field `CLK_EN` reader - N/A"]
pub type CLK_EN_R = crate::BitReader;
#[doc = "Field `CLK_EN` writer - N/A"]
pub type CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - N/A"]
    #[inline(always)]
    pub fn clk_en(&self) -> CLK_EN_R {
        CLK_EN_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REG_CLKEN")
            .field("clk_en", &self.clk_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - N/A"]
    #[inline(always)]
    pub fn clk_en(&mut self) -> CLK_EN_W<REG_CLKEN_SPEC> {
        CLK_EN_W::new(self, 0)
    }
}
#[doc = "N/A\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_clken::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_clken::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REG_CLKEN_SPEC;
impl crate::RegisterSpec for REG_CLKEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_clken::R`](R) reader structure"]
impl crate::Readable for REG_CLKEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`reg_clken::W`](W) writer structure"]
impl crate::Writable for REG_CLKEN_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_CLKEN to value 0"]
impl crate::Resettable for REG_CLKEN_SPEC {}
