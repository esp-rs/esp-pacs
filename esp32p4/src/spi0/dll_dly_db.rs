#[doc = "Register `DLL_DLY_DB` reader"]
pub type R = crate::R<DLL_DLY_DB_SPEC>;
#[doc = "Register `DLL_DLY_DB` writer"]
pub type W = crate::W<DLL_DLY_DB_SPEC>;
#[doc = "Field `DLL_DB_CFG_VLD_CNT` reader - Configures the end time of the debug window."]
pub type DLL_DB_CFG_VLD_CNT_R = crate::FieldReader;
#[doc = "Field `DLL_DB_CFG_VLD_CNT` writer - Configures the end time of the debug window."]
pub type DLL_DB_CFG_VLD_CNT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DLL_DB_CNT_MODE_SEL` reader - 3\\]:1-spi_din\\[15:8\\]. 0-spi_din\\[7:0\\]. \\[2\\]:1-only shift wptr or rptr. 0-both shft wptr and rptr. \\[1\\]:1-wprt\\[3:0\\] and rptr\\[3:0\\]. 0-rptr\\[3:0\\] and wprt\\[3:0\\]. \\[0\\]:1-neg_ptr\\[3:0\\]. 0-pos_prt\\[3:0\\]."]
pub type DLL_DB_CNT_MODE_SEL_R = crate::FieldReader;
#[doc = "Field `DLL_DB_CNT_MODE_SEL` writer - 3\\]:1-spi_din\\[15:8\\]. 0-spi_din\\[7:0\\]. \\[2\\]:1-only shift wptr or rptr. 0-both shft wptr and rptr. \\[1\\]:1-wprt\\[3:0\\] and rptr\\[3:0\\]. 0-rptr\\[3:0\\] and wprt\\[3:0\\]. \\[0\\]:1-neg_ptr\\[3:0\\]. 0-pos_prt\\[3:0\\]."]
pub type DLL_DB_CNT_MODE_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DLL_DB_CNT_CLR` reader - Configures the start time of the debug window. 1: Clear db_vld_cnt to 0 and Get ready for debug. 0: No debug."]
pub type DLL_DB_CNT_CLR_R = crate::BitReader;
#[doc = "Field `DLL_DB_CNT_CLR` writer - Configures the start time of the debug window. 1: Clear db_vld_cnt to 0 and Get ready for debug. 0: No debug."]
pub type DLL_DB_CNT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DLL_DIN_DLY_SEL` reader - Configures the din channel. 1: Use delayed data. 0: Do not use delayed data."]
pub type DLL_DIN_DLY_SEL_R = crate::BitReader;
#[doc = "Field `DLL_DIN_DLY_SEL` writer - Configures the din channel. 1: Use delayed data. 0: Do not use delayed data."]
pub type DLL_DIN_DLY_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Configures the end time of the debug window."]
    #[inline(always)]
    pub fn dll_db_cfg_vld_cnt(&self) -> DLL_DB_CFG_VLD_CNT_R {
        DLL_DB_CFG_VLD_CNT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:11 - 3\\]:1-spi_din\\[15:8\\]. 0-spi_din\\[7:0\\]. \\[2\\]:1-only shift wptr or rptr. 0-both shft wptr and rptr. \\[1\\]:1-wprt\\[3:0\\] and rptr\\[3:0\\]. 0-rptr\\[3:0\\] and wprt\\[3:0\\]. \\[0\\]:1-neg_ptr\\[3:0\\]. 0-pos_prt\\[3:0\\]."]
    #[inline(always)]
    pub fn dll_db_cnt_mode_sel(&self) -> DLL_DB_CNT_MODE_SEL_R {
        DLL_DB_CNT_MODE_SEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - Configures the start time of the debug window. 1: Clear db_vld_cnt to 0 and Get ready for debug. 0: No debug."]
    #[inline(always)]
    pub fn dll_db_cnt_clr(&self) -> DLL_DB_CNT_CLR_R {
        DLL_DB_CNT_CLR_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Configures the din channel. 1: Use delayed data. 0: Do not use delayed data."]
    #[inline(always)]
    pub fn dll_din_dly_sel(&self) -> DLL_DIN_DLY_SEL_R {
        DLL_DIN_DLY_SEL_R::new(((self.bits >> 13) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DLL_DLY_DB")
            .field("dll_db_cfg_vld_cnt", &self.dll_db_cfg_vld_cnt())
            .field("dll_db_cnt_mode_sel", &self.dll_db_cnt_mode_sel())
            .field("dll_db_cnt_clr", &self.dll_db_cnt_clr())
            .field("dll_din_dly_sel", &self.dll_din_dly_sel())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Configures the end time of the debug window."]
    #[inline(always)]
    pub fn dll_db_cfg_vld_cnt(&mut self) -> DLL_DB_CFG_VLD_CNT_W<'_, DLL_DLY_DB_SPEC> {
        DLL_DB_CFG_VLD_CNT_W::new(self, 0)
    }
    #[doc = "Bits 8:11 - 3\\]:1-spi_din\\[15:8\\]. 0-spi_din\\[7:0\\]. \\[2\\]:1-only shift wptr or rptr. 0-both shft wptr and rptr. \\[1\\]:1-wprt\\[3:0\\] and rptr\\[3:0\\]. 0-rptr\\[3:0\\] and wprt\\[3:0\\]. \\[0\\]:1-neg_ptr\\[3:0\\]. 0-pos_prt\\[3:0\\]."]
    #[inline(always)]
    pub fn dll_db_cnt_mode_sel(&mut self) -> DLL_DB_CNT_MODE_SEL_W<'_, DLL_DLY_DB_SPEC> {
        DLL_DB_CNT_MODE_SEL_W::new(self, 8)
    }
    #[doc = "Bit 12 - Configures the start time of the debug window. 1: Clear db_vld_cnt to 0 and Get ready for debug. 0: No debug."]
    #[inline(always)]
    pub fn dll_db_cnt_clr(&mut self) -> DLL_DB_CNT_CLR_W<'_, DLL_DLY_DB_SPEC> {
        DLL_DB_CNT_CLR_W::new(self, 12)
    }
    #[doc = "Bit 13 - Configures the din channel. 1: Use delayed data. 0: Do not use delayed data."]
    #[inline(always)]
    pub fn dll_din_dly_sel(&mut self) -> DLL_DIN_DLY_SEL_W<'_, DLL_DLY_DB_SPEC> {
        DLL_DIN_DLY_SEL_W::new(self, 13)
    }
}
#[doc = "MSPI DLL function and debug configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`dll_dly_db::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dll_dly_db::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DLL_DLY_DB_SPEC;
impl crate::RegisterSpec for DLL_DLY_DB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dll_dly_db::R`](R) reader structure"]
impl crate::Readable for DLL_DLY_DB_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dll_dly_db::W`](W) writer structure"]
impl crate::Writable for DLL_DLY_DB_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DLL_DLY_DB to value 0"]
impl crate::Resettable for DLL_DLY_DB_SPEC {}
