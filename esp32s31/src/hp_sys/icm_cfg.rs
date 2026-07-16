#[doc = "Register `ICM_CFG` reader"]
pub type R = crate::R<ICM_CFG_SPEC>;
#[doc = "Register `ICM_CFG` writer"]
pub type W = crate::W<ICM_CFG_SPEC>;
#[doc = "Field `ICM_DIS_RD` reader - 0:mean master can acess slave , 1:mean master can not rd slave ,will returen decerro on axi bus"]
pub type ICM_DIS_RD_R = crate::FieldReader;
#[doc = "Field `ICM_DIS_RD` writer - 0:mean master can acess slave , 1:mean master can not rd slave ,will returen decerro on axi bus"]
pub type ICM_DIS_RD_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ICM_DIS_WR` reader - 0:mean master can acess slave , 1:mean master can not wr slave ,will returen decerro on axi bus"]
pub type ICM_DIS_WR_R = crate::FieldReader;
#[doc = "Field `ICM_DIS_WR` writer - 0:mean master can acess slave , 1:mean master can not wr slave ,will returen decerro on axi bus"]
pub type ICM_DIS_WR_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SECURE` reader - axi_matrix slave ,0: non-secure, 1: secure , total 3 slave."]
pub type SECURE_R = crate::FieldReader;
#[doc = "Field `SECURE` writer - axi_matrix slave ,0: non-secure, 1: secure , total 3 slave."]
pub type SECURE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - 0:mean master can acess slave , 1:mean master can not rd slave ,will returen decerro on axi bus"]
    #[inline(always)]
    pub fn icm_dis_rd(&self) -> ICM_DIS_RD_R {
        ICM_DIS_RD_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - 0:mean master can acess slave , 1:mean master can not wr slave ,will returen decerro on axi bus"]
    #[inline(always)]
    pub fn icm_dis_wr(&self) -> ICM_DIS_WR_R {
        ICM_DIS_WR_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:8 - axi_matrix slave ,0: non-secure, 1: secure , total 3 slave."]
    #[inline(always)]
    pub fn secure(&self) -> SECURE_R {
        SECURE_R::new(((self.bits >> 6) & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ICM_CFG")
            .field("icm_dis_rd", &self.icm_dis_rd())
            .field("icm_dis_wr", &self.icm_dis_wr())
            .field("secure", &self.secure())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - 0:mean master can acess slave , 1:mean master can not rd slave ,will returen decerro on axi bus"]
    #[inline(always)]
    pub fn icm_dis_rd(&mut self) -> ICM_DIS_RD_W<'_, ICM_CFG_SPEC> {
        ICM_DIS_RD_W::new(self, 0)
    }
    #[doc = "Bits 3:5 - 0:mean master can acess slave , 1:mean master can not wr slave ,will returen decerro on axi bus"]
    #[inline(always)]
    pub fn icm_dis_wr(&mut self) -> ICM_DIS_WR_W<'_, ICM_CFG_SPEC> {
        ICM_DIS_WR_W::new(self, 3)
    }
    #[doc = "Bits 6:8 - axi_matrix slave ,0: non-secure, 1: secure , total 3 slave."]
    #[inline(always)]
    pub fn secure(&mut self) -> SECURE_W<'_, ICM_CFG_SPEC> {
        SECURE_W::new(self, 6)
    }
}
#[doc = "hp system axi icm ctrl register\n\nYou can [`read`](crate::Reg::read) this register and get [`icm_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icm_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ICM_CFG_SPEC;
impl crate::RegisterSpec for ICM_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icm_cfg::R`](R) reader structure"]
impl crate::Readable for ICM_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`icm_cfg::W`](W) writer structure"]
impl crate::Writable for ICM_CFG_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ICM_CFG to value 0"]
impl crate::Resettable for ICM_CFG_SPEC {}
