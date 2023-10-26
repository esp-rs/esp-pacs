#[doc = "Register `SYSCLK_CONF` reader"]
pub type R = crate::R<SYSCLK_CONF_SPEC>;
#[doc = "Register `SYSCLK_CONF` writer"]
pub type W = crate::W<SYSCLK_CONF_SPEC>;
#[doc = "Field `CLK_320M_EN` reader - "]
pub type CLK_320M_EN_R = crate::BitReader;
#[doc = "Field `CLK_320M_EN` writer - "]
pub type CLK_320M_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CLK_EN` reader - "]
pub type CLK_EN_R = crate::BitReader;
#[doc = "Field `CLK_EN` writer - "]
pub type CLK_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RST_TICK_CNT` reader - "]
pub type RST_TICK_CNT_R = crate::BitReader;
#[doc = "Field `RST_TICK_CNT` writer - "]
pub type RST_TICK_CNT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn clk_320m_en(&self) -> CLK_320M_EN_R {
        CLK_320M_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn clk_en(&self) -> CLK_EN_R {
        CLK_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn rst_tick_cnt(&self) -> RST_TICK_CNT_R {
        RST_TICK_CNT_R::new(((self.bits >> 12) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SYSCLK_CONF")
            .field("clk_320m_en", &format_args!("{}", self.clk_320m_en().bit()))
            .field("clk_en", &format_args!("{}", self.clk_en().bit()))
            .field(
                "rst_tick_cnt",
                &format_args!("{}", self.rst_tick_cnt().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SYSCLK_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn clk_320m_en(&mut self) -> CLK_320M_EN_W<SYSCLK_CONF_SPEC, 10> {
        CLK_320M_EN_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn clk_en(&mut self) -> CLK_EN_W<SYSCLK_CONF_SPEC, 11> {
        CLK_EN_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn rst_tick_cnt(&mut self) -> RST_TICK_CNT_W<SYSCLK_CONF_SPEC, 12> {
        RST_TICK_CNT_W::new(self)
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
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sysclk_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sysclk_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYSCLK_CONF_SPEC;
impl crate::RegisterSpec for SYSCLK_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sysclk_conf::R`](R) reader structure"]
impl crate::Readable for SYSCLK_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sysclk_conf::W`](W) writer structure"]
impl crate::Writable for SYSCLK_CONF_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SYSCLK_CONF to value 0"]
impl crate::Resettable for SYSCLK_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
