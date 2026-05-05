#[doc = "Register `LP_AONCLKRST_XTAL32K` reader"]
pub type R = crate::R<LP_AONCLKRST_XTAL32K_SPEC>;
#[doc = "Register `LP_AONCLKRST_XTAL32K` writer"]
pub type W = crate::W<LP_AONCLKRST_XTAL32K_SPEC>;
#[doc = "Field `LP_AONCLKRST_DRES_XTAL32K` reader - need_des"]
pub type LP_AONCLKRST_DRES_XTAL32K_R = crate::FieldReader;
#[doc = "Field `LP_AONCLKRST_DRES_XTAL32K` writer - need_des"]
pub type LP_AONCLKRST_DRES_XTAL32K_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `LP_AONCLKRST_DGM_XTAL32K` reader - need_des"]
pub type LP_AONCLKRST_DGM_XTAL32K_R = crate::FieldReader;
#[doc = "Field `LP_AONCLKRST_DGM_XTAL32K` writer - need_des"]
pub type LP_AONCLKRST_DGM_XTAL32K_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `LP_AONCLKRST_DBUF_XTAL32K` reader - need_des"]
pub type LP_AONCLKRST_DBUF_XTAL32K_R = crate::BitReader;
#[doc = "Field `LP_AONCLKRST_DBUF_XTAL32K` writer - need_des"]
pub type LP_AONCLKRST_DBUF_XTAL32K_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_AONCLKRST_DAC_XTAL32K` reader - need_des"]
pub type LP_AONCLKRST_DAC_XTAL32K_R = crate::FieldReader;
#[doc = "Field `LP_AONCLKRST_DAC_XTAL32K` writer - need_des"]
pub type LP_AONCLKRST_DAC_XTAL32K_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 22:24 - need_des"]
    #[inline(always)]
    pub fn lp_aonclkrst_dres_xtal32k(&self) -> LP_AONCLKRST_DRES_XTAL32K_R {
        LP_AONCLKRST_DRES_XTAL32K_R::new(((self.bits >> 22) & 7) as u8)
    }
    #[doc = "Bits 25:27 - need_des"]
    #[inline(always)]
    pub fn lp_aonclkrst_dgm_xtal32k(&self) -> LP_AONCLKRST_DGM_XTAL32K_R {
        LP_AONCLKRST_DGM_XTAL32K_R::new(((self.bits >> 25) & 7) as u8)
    }
    #[doc = "Bit 28 - need_des"]
    #[inline(always)]
    pub fn lp_aonclkrst_dbuf_xtal32k(&self) -> LP_AONCLKRST_DBUF_XTAL32K_R {
        LP_AONCLKRST_DBUF_XTAL32K_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bits 29:31 - need_des"]
    #[inline(always)]
    pub fn lp_aonclkrst_dac_xtal32k(&self) -> LP_AONCLKRST_DAC_XTAL32K_R {
        LP_AONCLKRST_DAC_XTAL32K_R::new(((self.bits >> 29) & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LP_AONCLKRST_XTAL32K")
            .field(
                "lp_aonclkrst_dres_xtal32k",
                &self.lp_aonclkrst_dres_xtal32k(),
            )
            .field("lp_aonclkrst_dgm_xtal32k", &self.lp_aonclkrst_dgm_xtal32k())
            .field(
                "lp_aonclkrst_dbuf_xtal32k",
                &self.lp_aonclkrst_dbuf_xtal32k(),
            )
            .field("lp_aonclkrst_dac_xtal32k", &self.lp_aonclkrst_dac_xtal32k())
            .finish()
    }
}
impl W {
    #[doc = "Bits 22:24 - need_des"]
    #[inline(always)]
    pub fn lp_aonclkrst_dres_xtal32k(
        &mut self,
    ) -> LP_AONCLKRST_DRES_XTAL32K_W<'_, LP_AONCLKRST_XTAL32K_SPEC> {
        LP_AONCLKRST_DRES_XTAL32K_W::new(self, 22)
    }
    #[doc = "Bits 25:27 - need_des"]
    #[inline(always)]
    pub fn lp_aonclkrst_dgm_xtal32k(
        &mut self,
    ) -> LP_AONCLKRST_DGM_XTAL32K_W<'_, LP_AONCLKRST_XTAL32K_SPEC> {
        LP_AONCLKRST_DGM_XTAL32K_W::new(self, 25)
    }
    #[doc = "Bit 28 - need_des"]
    #[inline(always)]
    pub fn lp_aonclkrst_dbuf_xtal32k(
        &mut self,
    ) -> LP_AONCLKRST_DBUF_XTAL32K_W<'_, LP_AONCLKRST_XTAL32K_SPEC> {
        LP_AONCLKRST_DBUF_XTAL32K_W::new(self, 28)
    }
    #[doc = "Bits 29:31 - need_des"]
    #[inline(always)]
    pub fn lp_aonclkrst_dac_xtal32k(
        &mut self,
    ) -> LP_AONCLKRST_DAC_XTAL32K_W<'_, LP_AONCLKRST_XTAL32K_SPEC> {
        LP_AONCLKRST_DAC_XTAL32K_W::new(self, 29)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_aonclkrst_xtal32k::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_aonclkrst_xtal32k::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LP_AONCLKRST_XTAL32K_SPEC;
impl crate::RegisterSpec for LP_AONCLKRST_XTAL32K_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lp_aonclkrst_xtal32k::R`](R) reader structure"]
impl crate::Readable for LP_AONCLKRST_XTAL32K_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lp_aonclkrst_xtal32k::W`](W) writer structure"]
impl crate::Writable for LP_AONCLKRST_XTAL32K_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LP_AONCLKRST_XTAL32K to value 0x66c0_0000"]
impl crate::Resettable for LP_AONCLKRST_XTAL32K_SPEC {
    const RESET_VALUE: u32 = 0x66c0_0000;
}
