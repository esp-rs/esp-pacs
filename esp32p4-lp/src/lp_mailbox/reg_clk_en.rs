#[doc = "Register `REG_CLK_EN` reader"]
pub type R = crate::R<REG_CLK_EN_SPEC>;
#[doc = "Register `REG_CLK_EN` writer"]
pub type W = crate::W<REG_CLK_EN_SPEC>;
#[doc = "Field `REG_CLK_EN` reader - MB_REG_CLK_EN"]
pub type REG_CLK_EN_R = crate::BitReader;
#[doc = "Field `REG_CLK_EN` writer - MB_REG_CLK_EN"]
pub type REG_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - MB_REG_CLK_EN"]
    #[inline(always)]
    pub fn reg_clk_en(&self) -> REG_CLK_EN_R {
        REG_CLK_EN_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REG_CLK_EN")
            .field("reg_clk_en", &self.reg_clk_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - MB_REG_CLK_EN"]
    #[inline(always)]
    pub fn reg_clk_en(&mut self) -> REG_CLK_EN_W<'_, REG_CLK_EN_SPEC> {
        REG_CLK_EN_W::new(self, 0)
    }
}
#[doc = "MB_REG_CLK_EN\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_clk_en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_clk_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REG_CLK_EN_SPEC;
impl crate::RegisterSpec for REG_CLK_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_clk_en::R`](R) reader structure"]
impl crate::Readable for REG_CLK_EN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`reg_clk_en::W`](W) writer structure"]
impl crate::Writable for REG_CLK_EN_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_CLK_EN to value 0"]
impl crate::Resettable for REG_CLK_EN_SPEC {}
