#[doc = "Register `COMB_PVT_NVT_CONF` reader"]
pub type R = crate::R<COMB_PVT_NVT_CONF_SPEC>;
#[doc = "Register `COMB_PVT_NVT_CONF` writer"]
pub type W = crate::W<COMB_PVT_NVT_CONF_SPEC>;
#[doc = "Field `COMB_PATH_LEN_NVT` reader - reg_comb_path_len_nvt"]
pub type COMB_PATH_LEN_NVT_R = crate::FieldReader;
#[doc = "Field `COMB_PATH_LEN_NVT` writer - reg_comb_path_len_nvt"]
pub type COMB_PATH_LEN_NVT_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `COMB_ERR_CNT_CLR_NVT` writer - reg_comb_err_cnt_clr_nvt"]
pub type COMB_ERR_CNT_CLR_NVT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMB_PVT_MONITOR_EN_NVT` reader - reg_comb_pvt_monitor_en_nvt"]
pub type COMB_PVT_MONITOR_EN_NVT_R = crate::BitReader;
#[doc = "Field `COMB_PVT_MONITOR_EN_NVT` writer - reg_comb_pvt_monitor_en_nvt"]
pub type COMB_PVT_MONITOR_EN_NVT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4 - reg_comb_path_len_nvt"]
    #[inline(always)]
    pub fn comb_path_len_nvt(&self) -> COMB_PATH_LEN_NVT_R {
        COMB_PATH_LEN_NVT_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 6 - reg_comb_pvt_monitor_en_nvt"]
    #[inline(always)]
    pub fn comb_pvt_monitor_en_nvt(&self) -> COMB_PVT_MONITOR_EN_NVT_R {
        COMB_PVT_MONITOR_EN_NVT_R::new(((self.bits >> 6) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("COMB_PVT_NVT_CONF")
            .field(
                "comb_path_len_nvt",
                &format_args!("{}", self.comb_path_len_nvt().bits()),
            )
            .field(
                "comb_pvt_monitor_en_nvt",
                &format_args!("{}", self.comb_pvt_monitor_en_nvt().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<COMB_PVT_NVT_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:4 - reg_comb_path_len_nvt"]
    #[inline(always)]
    #[must_use]
    pub fn comb_path_len_nvt(&mut self) -> COMB_PATH_LEN_NVT_W<COMB_PVT_NVT_CONF_SPEC> {
        COMB_PATH_LEN_NVT_W::new(self, 0)
    }
    #[doc = "Bit 5 - reg_comb_err_cnt_clr_nvt"]
    #[inline(always)]
    #[must_use]
    pub fn comb_err_cnt_clr_nvt(&mut self) -> COMB_ERR_CNT_CLR_NVT_W<COMB_PVT_NVT_CONF_SPEC> {
        COMB_ERR_CNT_CLR_NVT_W::new(self, 5)
    }
    #[doc = "Bit 6 - reg_comb_pvt_monitor_en_nvt"]
    #[inline(always)]
    #[must_use]
    pub fn comb_pvt_monitor_en_nvt(&mut self) -> COMB_PVT_MONITOR_EN_NVT_W<COMB_PVT_NVT_CONF_SPEC> {
        COMB_PVT_MONITOR_EN_NVT_W::new(self, 6)
    }
}
#[doc = "mem pvt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comb_pvt_nvt_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comb_pvt_nvt_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct COMB_PVT_NVT_CONF_SPEC;
impl crate::RegisterSpec for COMB_PVT_NVT_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`comb_pvt_nvt_conf::R`](R) reader structure"]
impl crate::Readable for COMB_PVT_NVT_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`comb_pvt_nvt_conf::W`](W) writer structure"]
impl crate::Writable for COMB_PVT_NVT_CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets COMB_PVT_NVT_CONF to value 0x03"]
impl crate::Resettable for COMB_PVT_NVT_CONF_SPEC {
    const RESET_VALUE: u32 = 0x03;
}
