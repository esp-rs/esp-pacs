#[doc = "Register `MAILBOX` reader"]
pub type R = crate::R<MAILBOX_SPEC>;
#[doc = "Register `MAILBOX` writer"]
pub type W = crate::W<MAILBOX_SPEC>;
#[doc = "Field `LP_MAILBOX_CLK_EN` reader - need_des"]
pub type LP_MAILBOX_CLK_EN_R = crate::BitReader;
#[doc = "Field `LP_MAILBOX_CLK_EN` writer - need_des"]
pub type LP_MAILBOX_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_MAILBOX_RST_EN` reader - need_des"]
pub type LP_MAILBOX_RST_EN_R = crate::BitReader;
#[doc = "Field `LP_MAILBOX_RST_EN` writer - need_des"]
pub type LP_MAILBOX_RST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn lp_mailbox_clk_en(&self) -> LP_MAILBOX_CLK_EN_R {
        LP_MAILBOX_CLK_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn lp_mailbox_rst_en(&self) -> LP_MAILBOX_RST_EN_R {
        LP_MAILBOX_RST_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MAILBOX")
            .field("lp_mailbox_clk_en", &self.lp_mailbox_clk_en())
            .field("lp_mailbox_rst_en", &self.lp_mailbox_rst_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn lp_mailbox_clk_en(&mut self) -> LP_MAILBOX_CLK_EN_W<'_, MAILBOX_SPEC> {
        LP_MAILBOX_CLK_EN_W::new(self, 30)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn lp_mailbox_rst_en(&mut self) -> LP_MAILBOX_RST_EN_W<'_, MAILBOX_SPEC> {
        LP_MAILBOX_RST_EN_W::new(self, 31)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`mailbox::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mailbox::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MAILBOX_SPEC;
impl crate::RegisterSpec for MAILBOX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mailbox::R`](R) reader structure"]
impl crate::Readable for MAILBOX_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mailbox::W`](W) writer structure"]
impl crate::Writable for MAILBOX_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MAILBOX to value 0"]
impl crate::Resettable for MAILBOX_SPEC {}
