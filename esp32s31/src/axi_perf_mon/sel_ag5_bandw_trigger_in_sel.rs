#[doc = "Register `SEL_AG5_BANDW_TRIGGER_IN_SEL` reader"]
pub type R = crate::R<SEL_AG5_BANDW_TRIGGER_IN_SEL_SPEC>;
#[doc = "Register `SEL_AG5_BANDW_TRIGGER_IN_SEL` writer"]
pub type W = crate::W<SEL_AG5_BANDW_TRIGGER_IN_SEL_SPEC>;
#[doc = "Field `SEL_AG5_RD_BANDW_TRIGGER_EN_SEL` reader - Read average bandwidth test, trigger enable by soc, sel source, SW register config is one source"]
pub type SEL_AG5_RD_BANDW_TRIGGER_EN_SEL_R = crate::FieldReader;
#[doc = "Field `SEL_AG5_RD_BANDW_TRIGGER_EN_SEL` writer - Read average bandwidth test, trigger enable by soc, sel source, SW register config is one source"]
pub type SEL_AG5_RD_BANDW_TRIGGER_EN_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SEL_AG5_RD_BANDW_TRIGGER_STOP_SEL` reader - Read average bandwidth test, trigger by soc, sel source, SW register config is one source"]
pub type SEL_AG5_RD_BANDW_TRIGGER_STOP_SEL_R = crate::FieldReader;
#[doc = "Field `SEL_AG5_RD_BANDW_TRIGGER_STOP_SEL` writer - Read average bandwidth test, trigger by soc, sel source, SW register config is one source"]
pub type SEL_AG5_RD_BANDW_TRIGGER_STOP_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SEL_AG5_WR_BANDW_TRIGGER_EN_SEL` reader - Write average bandwidth test, trigger enable by soc, sel source, SW register config is one source"]
pub type SEL_AG5_WR_BANDW_TRIGGER_EN_SEL_R = crate::FieldReader;
#[doc = "Field `SEL_AG5_WR_BANDW_TRIGGER_EN_SEL` writer - Write average bandwidth test, trigger enable by soc, sel source, SW register config is one source"]
pub type SEL_AG5_WR_BANDW_TRIGGER_EN_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SEL_AG5_WR_BANDW_TRIGGER_STOP_SEL` reader - Write average bandwidth test, trigger enable by soc, sel source, SW register config is one source"]
pub type SEL_AG5_WR_BANDW_TRIGGER_STOP_SEL_R = crate::FieldReader;
#[doc = "Field `SEL_AG5_WR_BANDW_TRIGGER_STOP_SEL` writer - Write average bandwidth test, trigger enable by soc, sel source, SW register config is one source"]
pub type SEL_AG5_WR_BANDW_TRIGGER_STOP_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Read average bandwidth test, trigger enable by soc, sel source, SW register config is one source"]
    #[inline(always)]
    pub fn sel_ag5_rd_bandw_trigger_en_sel(&self) -> SEL_AG5_RD_BANDW_TRIGGER_EN_SEL_R {
        SEL_AG5_RD_BANDW_TRIGGER_EN_SEL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Read average bandwidth test, trigger by soc, sel source, SW register config is one source"]
    #[inline(always)]
    pub fn sel_ag5_rd_bandw_trigger_stop_sel(&self) -> SEL_AG5_RD_BANDW_TRIGGER_STOP_SEL_R {
        SEL_AG5_RD_BANDW_TRIGGER_STOP_SEL_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Write average bandwidth test, trigger enable by soc, sel source, SW register config is one source"]
    #[inline(always)]
    pub fn sel_ag5_wr_bandw_trigger_en_sel(&self) -> SEL_AG5_WR_BANDW_TRIGGER_EN_SEL_R {
        SEL_AG5_WR_BANDW_TRIGGER_EN_SEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Write average bandwidth test, trigger enable by soc, sel source, SW register config is one source"]
    #[inline(always)]
    pub fn sel_ag5_wr_bandw_trigger_stop_sel(&self) -> SEL_AG5_WR_BANDW_TRIGGER_STOP_SEL_R {
        SEL_AG5_WR_BANDW_TRIGGER_STOP_SEL_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SEL_AG5_BANDW_TRIGGER_IN_SEL")
            .field(
                "sel_ag5_rd_bandw_trigger_en_sel",
                &self.sel_ag5_rd_bandw_trigger_en_sel(),
            )
            .field(
                "sel_ag5_rd_bandw_trigger_stop_sel",
                &self.sel_ag5_rd_bandw_trigger_stop_sel(),
            )
            .field(
                "sel_ag5_wr_bandw_trigger_en_sel",
                &self.sel_ag5_wr_bandw_trigger_en_sel(),
            )
            .field(
                "sel_ag5_wr_bandw_trigger_stop_sel",
                &self.sel_ag5_wr_bandw_trigger_stop_sel(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - Read average bandwidth test, trigger enable by soc, sel source, SW register config is one source"]
    #[inline(always)]
    pub fn sel_ag5_rd_bandw_trigger_en_sel(
        &mut self,
    ) -> SEL_AG5_RD_BANDW_TRIGGER_EN_SEL_W<'_, SEL_AG5_BANDW_TRIGGER_IN_SEL_SPEC> {
        SEL_AG5_RD_BANDW_TRIGGER_EN_SEL_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - Read average bandwidth test, trigger by soc, sel source, SW register config is one source"]
    #[inline(always)]
    pub fn sel_ag5_rd_bandw_trigger_stop_sel(
        &mut self,
    ) -> SEL_AG5_RD_BANDW_TRIGGER_STOP_SEL_W<'_, SEL_AG5_BANDW_TRIGGER_IN_SEL_SPEC> {
        SEL_AG5_RD_BANDW_TRIGGER_STOP_SEL_W::new(self, 4)
    }
    #[doc = "Bits 8:11 - Write average bandwidth test, trigger enable by soc, sel source, SW register config is one source"]
    #[inline(always)]
    pub fn sel_ag5_wr_bandw_trigger_en_sel(
        &mut self,
    ) -> SEL_AG5_WR_BANDW_TRIGGER_EN_SEL_W<'_, SEL_AG5_BANDW_TRIGGER_IN_SEL_SPEC> {
        SEL_AG5_WR_BANDW_TRIGGER_EN_SEL_W::new(self, 8)
    }
    #[doc = "Bits 12:15 - Write average bandwidth test, trigger enable by soc, sel source, SW register config is one source"]
    #[inline(always)]
    pub fn sel_ag5_wr_bandw_trigger_stop_sel(
        &mut self,
    ) -> SEL_AG5_WR_BANDW_TRIGGER_STOP_SEL_W<'_, SEL_AG5_BANDW_TRIGGER_IN_SEL_SPEC> {
        SEL_AG5_WR_BANDW_TRIGGER_STOP_SEL_W::new(self, 12)
    }
}
#[doc = "reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag5_bandw_trigger_in_sel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag5_bandw_trigger_in_sel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SEL_AG5_BANDW_TRIGGER_IN_SEL_SPEC;
impl crate::RegisterSpec for SEL_AG5_BANDW_TRIGGER_IN_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sel_ag5_bandw_trigger_in_sel::R`](R) reader structure"]
impl crate::Readable for SEL_AG5_BANDW_TRIGGER_IN_SEL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sel_ag5_bandw_trigger_in_sel::W`](W) writer structure"]
impl crate::Writable for SEL_AG5_BANDW_TRIGGER_IN_SEL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SEL_AG5_BANDW_TRIGGER_IN_SEL to value 0"]
impl crate::Resettable for SEL_AG5_BANDW_TRIGGER_IN_SEL_SPEC {}
