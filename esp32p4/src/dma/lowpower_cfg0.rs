#[doc = "Register `LOWPOWER_CFG0` reader"]
pub type R = crate::R<LOWPOWER_CFG0_SPEC>;
#[doc = "Register `LOWPOWER_CFG0` writer"]
pub type W = crate::W<LOWPOWER_CFG0_SPEC>;
#[doc = "Field `GBL_CSLP_EN` reader - NA"]
pub type GBL_CSLP_EN_R = crate::BitReader;
#[doc = "Field `GBL_CSLP_EN` writer - NA"]
pub type GBL_CSLP_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHNL_CSLP_EN` reader - NA"]
pub type CHNL_CSLP_EN_R = crate::BitReader;
#[doc = "Field `CHNL_CSLP_EN` writer - NA"]
pub type CHNL_CSLP_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SBIU_CSLP_EN` reader - NA"]
pub type SBIU_CSLP_EN_R = crate::BitReader;
#[doc = "Field `SBIU_CSLP_EN` writer - NA"]
pub type SBIU_CSLP_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MXIF_CSLP_EN` reader - NA"]
pub type MXIF_CSLP_EN_R = crate::BitReader;
#[doc = "Field `MXIF_CSLP_EN` writer - NA"]
pub type MXIF_CSLP_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn gbl_cslp_en(&self) -> GBL_CSLP_EN_R {
        GBL_CSLP_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - NA"]
    #[inline(always)]
    pub fn chnl_cslp_en(&self) -> CHNL_CSLP_EN_R {
        CHNL_CSLP_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - NA"]
    #[inline(always)]
    pub fn sbiu_cslp_en(&self) -> SBIU_CSLP_EN_R {
        SBIU_CSLP_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - NA"]
    #[inline(always)]
    pub fn mxif_cslp_en(&self) -> MXIF_CSLP_EN_R {
        MXIF_CSLP_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LOWPOWER_CFG0")
            .field("gbl_cslp_en", &self.gbl_cslp_en())
            .field("chnl_cslp_en", &self.chnl_cslp_en())
            .field("sbiu_cslp_en", &self.sbiu_cslp_en())
            .field("mxif_cslp_en", &self.mxif_cslp_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn gbl_cslp_en(&mut self) -> GBL_CSLP_EN_W<LOWPOWER_CFG0_SPEC> {
        GBL_CSLP_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - NA"]
    #[inline(always)]
    pub fn chnl_cslp_en(&mut self) -> CHNL_CSLP_EN_W<LOWPOWER_CFG0_SPEC> {
        CHNL_CSLP_EN_W::new(self, 1)
    }
    #[doc = "Bit 2 - NA"]
    #[inline(always)]
    pub fn sbiu_cslp_en(&mut self) -> SBIU_CSLP_EN_W<LOWPOWER_CFG0_SPEC> {
        SBIU_CSLP_EN_W::new(self, 2)
    }
    #[doc = "Bit 3 - NA"]
    #[inline(always)]
    pub fn mxif_cslp_en(&mut self) -> MXIF_CSLP_EN_W<LOWPOWER_CFG0_SPEC> {
        MXIF_CSLP_EN_W::new(self, 3)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`lowpower_cfg0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lowpower_cfg0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LOWPOWER_CFG0_SPEC;
impl crate::RegisterSpec for LOWPOWER_CFG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lowpower_cfg0::R`](R) reader structure"]
impl crate::Readable for LOWPOWER_CFG0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lowpower_cfg0::W`](W) writer structure"]
impl crate::Writable for LOWPOWER_CFG0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LOWPOWER_CFG0 to value 0x0f"]
impl crate::Resettable for LOWPOWER_CFG0_SPEC {
    const RESET_VALUE: u32 = 0x0f;
}
