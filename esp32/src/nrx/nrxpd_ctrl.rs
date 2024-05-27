#[doc = "Register `NRXPD_CTRL` reader"]
pub type R = crate::R<NRXPD_CTRL_SPEC>;
#[doc = "Register `NRXPD_CTRL` writer"]
pub type W = crate::W<NRXPD_CTRL_SPEC>;
#[doc = "Field `DEMAP_FORCE_PD` reader - "]
pub type DEMAP_FORCE_PD_R = crate::BitReader;
#[doc = "Field `DEMAP_FORCE_PD` writer - "]
pub type DEMAP_FORCE_PD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEMAP_FORCE_PU` reader - "]
pub type DEMAP_FORCE_PU_R = crate::BitReader;
#[doc = "Field `DEMAP_FORCE_PU` writer - "]
pub type DEMAP_FORCE_PU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VIT_FORCE_PD` reader - "]
pub type VIT_FORCE_PD_R = crate::BitReader;
#[doc = "Field `VIT_FORCE_PD` writer - "]
pub type VIT_FORCE_PD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VIT_FORCE_PU` reader - "]
pub type VIT_FORCE_PU_R = crate::BitReader;
#[doc = "Field `VIT_FORCE_PU` writer - "]
pub type VIT_FORCE_PU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_ROT_FORCE_PD` reader - "]
pub type RX_ROT_FORCE_PD_R = crate::BitReader;
#[doc = "Field `RX_ROT_FORCE_PD` writer - "]
pub type RX_ROT_FORCE_PD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_ROT_FORCE_PU` reader - "]
pub type RX_ROT_FORCE_PU_R = crate::BitReader;
#[doc = "Field `RX_ROT_FORCE_PU` writer - "]
pub type RX_ROT_FORCE_PU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHAN_EST_FORCE_PD` reader - "]
pub type CHAN_EST_FORCE_PD_R = crate::BitReader;
#[doc = "Field `CHAN_EST_FORCE_PD` writer - "]
pub type CHAN_EST_FORCE_PD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHAN_EST_FORCE_PU` reader - "]
pub type CHAN_EST_FORCE_PU_R = crate::BitReader;
#[doc = "Field `CHAN_EST_FORCE_PU` writer - "]
pub type CHAN_EST_FORCE_PU_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn demap_force_pd(&self) -> DEMAP_FORCE_PD_R {
        DEMAP_FORCE_PD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn demap_force_pu(&self) -> DEMAP_FORCE_PU_R {
        DEMAP_FORCE_PU_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn vit_force_pd(&self) -> VIT_FORCE_PD_R {
        VIT_FORCE_PD_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn vit_force_pu(&self) -> VIT_FORCE_PU_R {
        VIT_FORCE_PU_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn rx_rot_force_pd(&self) -> RX_ROT_FORCE_PD_R {
        RX_ROT_FORCE_PD_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn rx_rot_force_pu(&self) -> RX_ROT_FORCE_PU_R {
        RX_ROT_FORCE_PU_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn chan_est_force_pd(&self) -> CHAN_EST_FORCE_PD_R {
        CHAN_EST_FORCE_PD_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn chan_est_force_pu(&self) -> CHAN_EST_FORCE_PU_R {
        CHAN_EST_FORCE_PU_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NRXPD_CTRL")
            .field("demap_force_pd", &self.demap_force_pd())
            .field("demap_force_pu", &self.demap_force_pu())
            .field("vit_force_pd", &self.vit_force_pd())
            .field("vit_force_pu", &self.vit_force_pu())
            .field("rx_rot_force_pd", &self.rx_rot_force_pd())
            .field("rx_rot_force_pu", &self.rx_rot_force_pu())
            .field("chan_est_force_pd", &self.chan_est_force_pd())
            .field("chan_est_force_pu", &self.chan_est_force_pu())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn demap_force_pd(&mut self) -> DEMAP_FORCE_PD_W<NRXPD_CTRL_SPEC> {
        DEMAP_FORCE_PD_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn demap_force_pu(&mut self) -> DEMAP_FORCE_PU_W<NRXPD_CTRL_SPEC> {
        DEMAP_FORCE_PU_W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn vit_force_pd(&mut self) -> VIT_FORCE_PD_W<NRXPD_CTRL_SPEC> {
        VIT_FORCE_PD_W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn vit_force_pu(&mut self) -> VIT_FORCE_PU_W<NRXPD_CTRL_SPEC> {
        VIT_FORCE_PU_W::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn rx_rot_force_pd(&mut self) -> RX_ROT_FORCE_PD_W<NRXPD_CTRL_SPEC> {
        RX_ROT_FORCE_PD_W::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn rx_rot_force_pu(&mut self) -> RX_ROT_FORCE_PU_W<NRXPD_CTRL_SPEC> {
        RX_ROT_FORCE_PU_W::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn chan_est_force_pd(&mut self) -> CHAN_EST_FORCE_PD_W<NRXPD_CTRL_SPEC> {
        CHAN_EST_FORCE_PD_W::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn chan_est_force_pu(&mut self) -> CHAN_EST_FORCE_PU_W<NRXPD_CTRL_SPEC> {
        CHAN_EST_FORCE_PU_W::new(self, 7)
    }
}
#[doc = "WiFi RX control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nrxpd_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nrxpd_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NRXPD_CTRL_SPEC;
impl crate::RegisterSpec for NRXPD_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nrxpd_ctrl::R`](R) reader structure"]
impl crate::Readable for NRXPD_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`nrxpd_ctrl::W`](W) writer structure"]
impl crate::Writable for NRXPD_CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets NRXPD_CTRL to value 0"]
impl crate::Resettable for NRXPD_CTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
