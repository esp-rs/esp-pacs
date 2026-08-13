#[doc = "Register `ZERO_DET_CONF` reader"]
pub type R = crate::R<ZERO_DET_CONF_SPEC>;
#[doc = "Register `ZERO_DET_CONF` writer"]
pub type W = crate::W<ZERO_DET_CONF_SPEC>;
#[doc = "Field `ZERO_DET_CLK_EN` reader - Set 1 to enable zero_det apb clock"]
pub type ZERO_DET_CLK_EN_R = crate::BitReader;
#[doc = "Field `ZERO_DET_CLK_EN` writer - Set 1 to enable zero_det apb clock"]
pub type ZERO_DET_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ZERO_DET_RST_EN` reader - Set 1 to reset zero_det module"]
pub type ZERO_DET_RST_EN_R = crate::BitReader;
#[doc = "Field `ZERO_DET_RST_EN` writer - Set 1 to reset zero_det module"]
pub type ZERO_DET_RST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Set 1 to enable zero_det apb clock"]
    #[inline(always)]
    pub fn zero_det_clk_en(&self) -> ZERO_DET_CLK_EN_R {
        ZERO_DET_CLK_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set 1 to reset zero_det module"]
    #[inline(always)]
    pub fn zero_det_rst_en(&self) -> ZERO_DET_RST_EN_R {
        ZERO_DET_RST_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ZERO_DET_CONF")
            .field("zero_det_clk_en", &self.zero_det_clk_en())
            .field("zero_det_rst_en", &self.zero_det_rst_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Set 1 to enable zero_det apb clock"]
    #[inline(always)]
    pub fn zero_det_clk_en(&mut self) -> ZERO_DET_CLK_EN_W<'_, ZERO_DET_CONF_SPEC> {
        ZERO_DET_CLK_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Set 1 to reset zero_det module"]
    #[inline(always)]
    pub fn zero_det_rst_en(&mut self) -> ZERO_DET_RST_EN_W<'_, ZERO_DET_CONF_SPEC> {
        ZERO_DET_RST_EN_W::new(self, 1)
    }
}
#[doc = "ZERO_DET configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`zero_det_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`zero_det_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ZERO_DET_CONF_SPEC;
impl crate::RegisterSpec for ZERO_DET_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`zero_det_conf::R`](R) reader structure"]
impl crate::Readable for ZERO_DET_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`zero_det_conf::W`](W) writer structure"]
impl crate::Writable for ZERO_DET_CONF_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ZERO_DET_CONF to value 0x02"]
impl crate::Resettable for ZERO_DET_CONF_SPEC {
    const RESET_VALUE: u32 = 0x02;
}
