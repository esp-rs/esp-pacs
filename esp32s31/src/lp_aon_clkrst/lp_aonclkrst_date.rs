#[doc = "Register `LP_AONCLKRST_DATE` reader"]
pub type R = crate::R<LP_AONCLKRST_DATE_SPEC>;
#[doc = "Register `LP_AONCLKRST_DATE` writer"]
pub type W = crate::W<LP_AONCLKRST_DATE_SPEC>;
#[doc = "Field `LP_AONCLKRST_DATE` reader - need_des"]
pub type LP_AONCLKRST_DATE_R = crate::FieldReader<u32>;
#[doc = "Field `LP_AONCLKRST_DATE` writer - need_des"]
pub type LP_AONCLKRST_DATE_W<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
#[doc = "Field `LP_AONCLKRST_CLK_EN` reader - need_des"]
pub type LP_AONCLKRST_CLK_EN_R = crate::BitReader;
#[doc = "Field `LP_AONCLKRST_CLK_EN` writer - need_des"]
pub type LP_AONCLKRST_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:30 - need_des"]
    #[inline(always)]
    pub fn lp_aonclkrst_date(&self) -> LP_AONCLKRST_DATE_R {
        LP_AONCLKRST_DATE_R::new(self.bits & 0x7fff_ffff)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn lp_aonclkrst_clk_en(&self) -> LP_AONCLKRST_CLK_EN_R {
        LP_AONCLKRST_CLK_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LP_AONCLKRST_DATE")
            .field("lp_aonclkrst_date", &self.lp_aonclkrst_date())
            .field("lp_aonclkrst_clk_en", &self.lp_aonclkrst_clk_en())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:30 - need_des"]
    #[inline(always)]
    pub fn lp_aonclkrst_date(&mut self) -> LP_AONCLKRST_DATE_W<'_, LP_AONCLKRST_DATE_SPEC> {
        LP_AONCLKRST_DATE_W::new(self, 0)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn lp_aonclkrst_clk_en(&mut self) -> LP_AONCLKRST_CLK_EN_W<'_, LP_AONCLKRST_DATE_SPEC> {
        LP_AONCLKRST_CLK_EN_W::new(self, 31)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_aonclkrst_date::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_aonclkrst_date::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LP_AONCLKRST_DATE_SPEC;
impl crate::RegisterSpec for LP_AONCLKRST_DATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lp_aonclkrst_date::R`](R) reader structure"]
impl crate::Readable for LP_AONCLKRST_DATE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lp_aonclkrst_date::W`](W) writer structure"]
impl crate::Writable for LP_AONCLKRST_DATE_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LP_AONCLKRST_DATE to value 0x0250_7300"]
impl crate::Resettable for LP_AONCLKRST_DATE_SPEC {
    const RESET_VALUE: u32 = 0x0250_7300;
}
