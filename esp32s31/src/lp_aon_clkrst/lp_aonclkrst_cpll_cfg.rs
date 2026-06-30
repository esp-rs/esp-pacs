#[doc = "Register `LP_AONCLKRST_CPLL_CFG` reader"]
pub type R = crate::R<LP_AONCLKRST_CPLL_CFG_SPEC>;
#[doc = "Register `LP_AONCLKRST_CPLL_CFG` writer"]
pub type W = crate::W<LP_AONCLKRST_CPLL_CFG_SPEC>;
#[doc = "Field `LP_AONCLKRST_CPLL_DBIAS` reader - cpll dbias value"]
pub type LP_AONCLKRST_CPLL_DBIAS_R = crate::FieldReader;
#[doc = "Field `LP_AONCLKRST_CPLL_DBIAS` writer - cpll dbias value"]
pub type LP_AONCLKRST_CPLL_DBIAS_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `LP_AONCLKRST_CPLL_DCHGP` reader - cpll dchgp value"]
pub type LP_AONCLKRST_CPLL_DCHGP_R = crate::FieldReader;
#[doc = "Field `LP_AONCLKRST_CPLL_DCHGP` writer - cpll dchgp value"]
pub type LP_AONCLKRST_CPLL_DCHGP_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `LP_AONCLKRST_CPLL_DR1` reader - cpll dr1 value"]
pub type LP_AONCLKRST_CPLL_DR1_R = crate::FieldReader;
#[doc = "Field `LP_AONCLKRST_CPLL_DR1` writer - cpll dr1 value"]
pub type LP_AONCLKRST_CPLL_DR1_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `LP_AONCLKRST_CPLL_DR3` reader - cpll dr3 value"]
pub type LP_AONCLKRST_CPLL_DR3_R = crate::FieldReader;
#[doc = "Field `LP_AONCLKRST_CPLL_DR3` writer - cpll dr3 value"]
pub type LP_AONCLKRST_CPLL_DR3_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - cpll dbias value"]
    #[inline(always)]
    pub fn lp_aonclkrst_cpll_dbias(&self) -> LP_AONCLKRST_CPLL_DBIAS_R {
        LP_AONCLKRST_CPLL_DBIAS_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - cpll dchgp value"]
    #[inline(always)]
    pub fn lp_aonclkrst_cpll_dchgp(&self) -> LP_AONCLKRST_CPLL_DCHGP_R {
        LP_AONCLKRST_CPLL_DCHGP_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:8 - cpll dr1 value"]
    #[inline(always)]
    pub fn lp_aonclkrst_cpll_dr1(&self) -> LP_AONCLKRST_CPLL_DR1_R {
        LP_AONCLKRST_CPLL_DR1_R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 9:11 - cpll dr3 value"]
    #[inline(always)]
    pub fn lp_aonclkrst_cpll_dr3(&self) -> LP_AONCLKRST_CPLL_DR3_R {
        LP_AONCLKRST_CPLL_DR3_R::new(((self.bits >> 9) & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LP_AONCLKRST_CPLL_CFG")
            .field("lp_aonclkrst_cpll_dbias", &self.lp_aonclkrst_cpll_dbias())
            .field("lp_aonclkrst_cpll_dchgp", &self.lp_aonclkrst_cpll_dchgp())
            .field("lp_aonclkrst_cpll_dr1", &self.lp_aonclkrst_cpll_dr1())
            .field("lp_aonclkrst_cpll_dr3", &self.lp_aonclkrst_cpll_dr3())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - cpll dbias value"]
    #[inline(always)]
    pub fn lp_aonclkrst_cpll_dbias(
        &mut self,
    ) -> LP_AONCLKRST_CPLL_DBIAS_W<'_, LP_AONCLKRST_CPLL_CFG_SPEC> {
        LP_AONCLKRST_CPLL_DBIAS_W::new(self, 0)
    }
    #[doc = "Bits 3:5 - cpll dchgp value"]
    #[inline(always)]
    pub fn lp_aonclkrst_cpll_dchgp(
        &mut self,
    ) -> LP_AONCLKRST_CPLL_DCHGP_W<'_, LP_AONCLKRST_CPLL_CFG_SPEC> {
        LP_AONCLKRST_CPLL_DCHGP_W::new(self, 3)
    }
    #[doc = "Bits 6:8 - cpll dr1 value"]
    #[inline(always)]
    pub fn lp_aonclkrst_cpll_dr1(
        &mut self,
    ) -> LP_AONCLKRST_CPLL_DR1_W<'_, LP_AONCLKRST_CPLL_CFG_SPEC> {
        LP_AONCLKRST_CPLL_DR1_W::new(self, 6)
    }
    #[doc = "Bits 9:11 - cpll dr3 value"]
    #[inline(always)]
    pub fn lp_aonclkrst_cpll_dr3(
        &mut self,
    ) -> LP_AONCLKRST_CPLL_DR3_W<'_, LP_AONCLKRST_CPLL_CFG_SPEC> {
        LP_AONCLKRST_CPLL_DR3_W::new(self, 9)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_aonclkrst_cpll_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_aonclkrst_cpll_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LP_AONCLKRST_CPLL_CFG_SPEC;
impl crate::RegisterSpec for LP_AONCLKRST_CPLL_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lp_aonclkrst_cpll_cfg::R`](R) reader structure"]
impl crate::Readable for LP_AONCLKRST_CPLL_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lp_aonclkrst_cpll_cfg::W`](W) writer structure"]
impl crate::Writable for LP_AONCLKRST_CPLL_CFG_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LP_AONCLKRST_CPLL_CFG to value 0"]
impl crate::Resettable for LP_AONCLKRST_CPLL_CFG_SPEC {}
