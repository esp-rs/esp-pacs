#[doc = "Register `SYS` reader"]
pub type R = crate::R<SYS_SPEC>;
#[doc = "Register `SYS` writer"]
pub type W = crate::W<SYS_SPEC>;
#[doc = "Field `CLK_EN` reader - Reserved"]
pub type CLK_EN_R = crate::BitReader;
#[doc = "Field `CLK_EN` writer - Reserved"]
pub type CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 31 - Reserved"]
    #[inline(always)]
    pub fn clk_en(&self) -> CLK_EN_R {
        CLK_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SYS")
            .field("clk_en", &self.clk_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 31 - Reserved"]
    #[inline(always)]
    pub fn clk_en(&mut self) -> CLK_EN_W<'_, SYS_SPEC> {
        CLK_EN_W::new(self, 31)
    }
}
#[doc = "Trace and Debug registers\n\nYou can [`read`](crate::Reg::read) this register and get [`sys::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sys::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYS_SPEC;
impl crate::RegisterSpec for SYS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sys::R`](R) reader structure"]
impl crate::Readable for SYS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sys::W`](W) writer structure"]
impl crate::Writable for SYS_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SYS to value 0"]
impl crate::Resettable for SYS_SPEC {}
