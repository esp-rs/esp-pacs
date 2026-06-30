#[doc = "Register `LP_AONCLKRST_MSPI_DIV` reader"]
pub type R = crate::R<LP_AONCLKRST_MSPI_DIV_SPEC>;
#[doc = "Register `LP_AONCLKRST_MSPI_DIV` writer"]
pub type W = crate::W<LP_AONCLKRST_MSPI_DIV_SPEC>;
#[doc = "Field `LP_AONCLKRST_MSPI_REF_DIV` reader - MSPI ref div value"]
pub type LP_AONCLKRST_MSPI_REF_DIV_R = crate::FieldReader;
#[doc = "Field `LP_AONCLKRST_MSPI_REF_DIV` writer - MSPI ref div value"]
pub type LP_AONCLKRST_MSPI_REF_DIV_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `LP_AONCLKRST_MSPI_FB_DIV` reader - MSPI fb div value"]
pub type LP_AONCLKRST_MSPI_FB_DIV_R = crate::FieldReader;
#[doc = "Field `LP_AONCLKRST_MSPI_FB_DIV` writer - MSPI fb div value"]
pub type LP_AONCLKRST_MSPI_FB_DIV_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `LP_AONCLKRST_MSPI_CHGP_DCUR` reader - MSPI chgp_dcur value"]
pub type LP_AONCLKRST_MSPI_CHGP_DCUR_R = crate::BitReader;
#[doc = "Field `LP_AONCLKRST_MSPI_CHGP_DCUR` writer - MSPI chgp_dcur value"]
pub type LP_AONCLKRST_MSPI_CHGP_DCUR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_AONCLKRST_MSPI_LF_RES` reader - MSPI lf_res value"]
pub type LP_AONCLKRST_MSPI_LF_RES_R = crate::BitReader;
#[doc = "Field `LP_AONCLKRST_MSPI_LF_RES` writer - MSPI lf_res value"]
pub type LP_AONCLKRST_MSPI_LF_RES_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - MSPI ref div value"]
    #[inline(always)]
    pub fn lp_aonclkrst_mspi_ref_div(&self) -> LP_AONCLKRST_MSPI_REF_DIV_R {
        LP_AONCLKRST_MSPI_REF_DIV_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:7 - MSPI fb div value"]
    #[inline(always)]
    pub fn lp_aonclkrst_mspi_fb_div(&self) -> LP_AONCLKRST_MSPI_FB_DIV_R {
        LP_AONCLKRST_MSPI_FB_DIV_R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bit 8 - MSPI chgp_dcur value"]
    #[inline(always)]
    pub fn lp_aonclkrst_mspi_chgp_dcur(&self) -> LP_AONCLKRST_MSPI_CHGP_DCUR_R {
        LP_AONCLKRST_MSPI_CHGP_DCUR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - MSPI lf_res value"]
    #[inline(always)]
    pub fn lp_aonclkrst_mspi_lf_res(&self) -> LP_AONCLKRST_MSPI_LF_RES_R {
        LP_AONCLKRST_MSPI_LF_RES_R::new(((self.bits >> 9) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LP_AONCLKRST_MSPI_DIV")
            .field(
                "lp_aonclkrst_mspi_ref_div",
                &self.lp_aonclkrst_mspi_ref_div(),
            )
            .field("lp_aonclkrst_mspi_fb_div", &self.lp_aonclkrst_mspi_fb_div())
            .field(
                "lp_aonclkrst_mspi_chgp_dcur",
                &self.lp_aonclkrst_mspi_chgp_dcur(),
            )
            .field("lp_aonclkrst_mspi_lf_res", &self.lp_aonclkrst_mspi_lf_res())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - MSPI ref div value"]
    #[inline(always)]
    pub fn lp_aonclkrst_mspi_ref_div(
        &mut self,
    ) -> LP_AONCLKRST_MSPI_REF_DIV_W<'_, LP_AONCLKRST_MSPI_DIV_SPEC> {
        LP_AONCLKRST_MSPI_REF_DIV_W::new(self, 0)
    }
    #[doc = "Bits 3:7 - MSPI fb div value"]
    #[inline(always)]
    pub fn lp_aonclkrst_mspi_fb_div(
        &mut self,
    ) -> LP_AONCLKRST_MSPI_FB_DIV_W<'_, LP_AONCLKRST_MSPI_DIV_SPEC> {
        LP_AONCLKRST_MSPI_FB_DIV_W::new(self, 3)
    }
    #[doc = "Bit 8 - MSPI chgp_dcur value"]
    #[inline(always)]
    pub fn lp_aonclkrst_mspi_chgp_dcur(
        &mut self,
    ) -> LP_AONCLKRST_MSPI_CHGP_DCUR_W<'_, LP_AONCLKRST_MSPI_DIV_SPEC> {
        LP_AONCLKRST_MSPI_CHGP_DCUR_W::new(self, 8)
    }
    #[doc = "Bit 9 - MSPI lf_res value"]
    #[inline(always)]
    pub fn lp_aonclkrst_mspi_lf_res(
        &mut self,
    ) -> LP_AONCLKRST_MSPI_LF_RES_W<'_, LP_AONCLKRST_MSPI_DIV_SPEC> {
        LP_AONCLKRST_MSPI_LF_RES_W::new(self, 9)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_aonclkrst_mspi_div::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_aonclkrst_mspi_div::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LP_AONCLKRST_MSPI_DIV_SPEC;
impl crate::RegisterSpec for LP_AONCLKRST_MSPI_DIV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lp_aonclkrst_mspi_div::R`](R) reader structure"]
impl crate::Readable for LP_AONCLKRST_MSPI_DIV_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lp_aonclkrst_mspi_div::W`](W) writer structure"]
impl crate::Writable for LP_AONCLKRST_MSPI_DIV_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LP_AONCLKRST_MSPI_DIV to value 0xc1"]
impl crate::Resettable for LP_AONCLKRST_MSPI_DIV_SPEC {
    const RESET_VALUE: u32 = 0xc1;
}
