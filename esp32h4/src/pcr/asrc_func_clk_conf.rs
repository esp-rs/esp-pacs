#[doc = "Register `ASRC_FUNC_CLK_CONF` reader"]
pub type R = crate::R<ASRC_FUNC_CLK_CONF_SPEC>;
#[doc = "Register `ASRC_FUNC_CLK_CONF` writer"]
pub type W = crate::W<ASRC_FUNC_CLK_CONF_SPEC>;
#[doc = "Field `ASRC_APB_CLK_EN` reader - Set 1 to enable audio_sample rate converter apb clock"]
pub type ASRC_APB_CLK_EN_R = crate::BitReader;
#[doc = "Field `ASRC_APB_CLK_EN` writer - Set 1 to enable audio_sample rate converter apb clock"]
pub type ASRC_APB_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ASRC_FUNC_CLK_EN` reader - Set 1 to enable audio_sample rate converter function clock"]
pub type ASRC_FUNC_CLK_EN_R = crate::BitReader;
#[doc = "Field `ASRC_FUNC_CLK_EN` writer - Set 1 to enable audio_sample rate converter function clock"]
pub type ASRC_FUNC_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ASRC_RST_EN` reader - Set 1 to reset audio_sample_rate_converter module"]
pub type ASRC_RST_EN_R = crate::BitReader;
#[doc = "Field `ASRC_RST_EN` writer - Set 1 to reset audio_sample_rate_converter module"]
pub type ASRC_RST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 22 - Set 1 to enable audio_sample rate converter apb clock"]
    #[inline(always)]
    pub fn asrc_apb_clk_en(&self) -> ASRC_APB_CLK_EN_R {
        ASRC_APB_CLK_EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Set 1 to enable audio_sample rate converter function clock"]
    #[inline(always)]
    pub fn asrc_func_clk_en(&self) -> ASRC_FUNC_CLK_EN_R {
        ASRC_FUNC_CLK_EN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Set 1 to reset audio_sample_rate_converter module"]
    #[inline(always)]
    pub fn asrc_rst_en(&self) -> ASRC_RST_EN_R {
        ASRC_RST_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ASRC_FUNC_CLK_CONF")
            .field("asrc_apb_clk_en", &self.asrc_apb_clk_en())
            .field("asrc_func_clk_en", &self.asrc_func_clk_en())
            .field("asrc_rst_en", &self.asrc_rst_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 22 - Set 1 to enable audio_sample rate converter apb clock"]
    #[inline(always)]
    pub fn asrc_apb_clk_en(&mut self) -> ASRC_APB_CLK_EN_W<'_, ASRC_FUNC_CLK_CONF_SPEC> {
        ASRC_APB_CLK_EN_W::new(self, 22)
    }
    #[doc = "Bit 23 - Set 1 to enable audio_sample rate converter function clock"]
    #[inline(always)]
    pub fn asrc_func_clk_en(&mut self) -> ASRC_FUNC_CLK_EN_W<'_, ASRC_FUNC_CLK_CONF_SPEC> {
        ASRC_FUNC_CLK_EN_W::new(self, 23)
    }
    #[doc = "Bit 24 - Set 1 to reset audio_sample_rate_converter module"]
    #[inline(always)]
    pub fn asrc_rst_en(&mut self) -> ASRC_RST_EN_W<'_, ASRC_FUNC_CLK_CONF_SPEC> {
        ASRC_RST_EN_W::new(self, 24)
    }
}
#[doc = "AUDIO_SAMPLE_RATE_FUNC_CLK configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`asrc_func_clk_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`asrc_func_clk_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ASRC_FUNC_CLK_CONF_SPEC;
impl crate::RegisterSpec for ASRC_FUNC_CLK_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`asrc_func_clk_conf::R`](R) reader structure"]
impl crate::Readable for ASRC_FUNC_CLK_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`asrc_func_clk_conf::W`](W) writer structure"]
impl crate::Writable for ASRC_FUNC_CLK_CONF_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ASRC_FUNC_CLK_CONF to value 0x00c0_0000"]
impl crate::Resettable for ASRC_FUNC_CLK_CONF_SPEC {
    const RESET_VALUE: u32 = 0x00c0_0000;
}
