#[doc = "Register `SEL_AG_WR_ADDR_REGION_SEL` reader"]
pub type R = crate::R<SEL_AG_WR_ADDR_REGION_SEL_SPEC>;
#[doc = "Register `SEL_AG_WR_ADDR_REGION_SEL` writer"]
pub type W = crate::W<SEL_AG_WR_ADDR_REGION_SEL_SPEC>;
#[doc = "Field `SEL_AG0_WR_ADDR_REGION_SEL` reader - SW config Write region select, use with mask and filter, only when addr in one region and this region has been sel, will measure the transaction data num"]
pub type SEL_AG0_WR_ADDR_REGION_SEL_R = crate::FieldReader;
#[doc = "Field `SEL_AG0_WR_ADDR_REGION_SEL` writer - SW config Write region select, use with mask and filter, only when addr in one region and this region has been sel, will measure the transaction data num"]
pub type SEL_AG0_WR_ADDR_REGION_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SEL_AG1_WR_ADDR_REGION_SEL` reader - SW config Write region select, use with mask and filter, only when addr in one region and this region has been sel, will measure the transaction data num"]
pub type SEL_AG1_WR_ADDR_REGION_SEL_R = crate::FieldReader;
#[doc = "Field `SEL_AG1_WR_ADDR_REGION_SEL` writer - SW config Write region select, use with mask and filter, only when addr in one region and this region has been sel, will measure the transaction data num"]
pub type SEL_AG1_WR_ADDR_REGION_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SEL_AG2_WR_ADDR_REGION_SEL` reader - SW config Write region select, use with mask and filter, only when addr in one region and this region has been sel, will measure the transaction data num"]
pub type SEL_AG2_WR_ADDR_REGION_SEL_R = crate::FieldReader;
#[doc = "Field `SEL_AG2_WR_ADDR_REGION_SEL` writer - SW config Write region select, use with mask and filter, only when addr in one region and this region has been sel, will measure the transaction data num"]
pub type SEL_AG2_WR_ADDR_REGION_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SEL_AG3_WR_ADDR_REGION_SEL` reader - SW config Write region select, use with mask and filter, only when addr in one region and this region has been sel, will measure the transaction data num"]
pub type SEL_AG3_WR_ADDR_REGION_SEL_R = crate::FieldReader;
#[doc = "Field `SEL_AG3_WR_ADDR_REGION_SEL` writer - SW config Write region select, use with mask and filter, only when addr in one region and this region has been sel, will measure the transaction data num"]
pub type SEL_AG3_WR_ADDR_REGION_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SEL_AG4_WR_ADDR_REGION_SEL` reader - SW config Write region select, use with mask and filter, only when addr in one region and this region has been sel, will measure the transaction data num"]
pub type SEL_AG4_WR_ADDR_REGION_SEL_R = crate::FieldReader;
#[doc = "Field `SEL_AG4_WR_ADDR_REGION_SEL` writer - SW config Write region select, use with mask and filter, only when addr in one region and this region has been sel, will measure the transaction data num"]
pub type SEL_AG4_WR_ADDR_REGION_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SEL_AG5_WR_ADDR_REGION_SEL` reader - SW config Write region select, use with mask and filter, only when addr in one region and this region has been sel, will measure the transaction data num"]
pub type SEL_AG5_WR_ADDR_REGION_SEL_R = crate::FieldReader;
#[doc = "Field `SEL_AG5_WR_ADDR_REGION_SEL` writer - SW config Write region select, use with mask and filter, only when addr in one region and this region has been sel, will measure the transaction data num"]
pub type SEL_AG5_WR_ADDR_REGION_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SEL_AG6_WR_ADDR_REGION_SEL` reader - SW config Write region select, use with mask and filter, only when addr in one region and this region has been sel, will measure the transaction data num"]
pub type SEL_AG6_WR_ADDR_REGION_SEL_R = crate::FieldReader;
#[doc = "Field `SEL_AG6_WR_ADDR_REGION_SEL` writer - SW config Write region select, use with mask and filter, only when addr in one region and this region has been sel, will measure the transaction data num"]
pub type SEL_AG6_WR_ADDR_REGION_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - SW config Write region select, use with mask and filter, only when addr in one region and this region has been sel, will measure the transaction data num"]
    #[inline(always)]
    pub fn sel_ag0_wr_addr_region_sel(&self) -> SEL_AG0_WR_ADDR_REGION_SEL_R {
        SEL_AG0_WR_ADDR_REGION_SEL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - SW config Write region select, use with mask and filter, only when addr in one region and this region has been sel, will measure the transaction data num"]
    #[inline(always)]
    pub fn sel_ag1_wr_addr_region_sel(&self) -> SEL_AG1_WR_ADDR_REGION_SEL_R {
        SEL_AG1_WR_ADDR_REGION_SEL_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:8 - SW config Write region select, use with mask and filter, only when addr in one region and this region has been sel, will measure the transaction data num"]
    #[inline(always)]
    pub fn sel_ag2_wr_addr_region_sel(&self) -> SEL_AG2_WR_ADDR_REGION_SEL_R {
        SEL_AG2_WR_ADDR_REGION_SEL_R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 9:11 - SW config Write region select, use with mask and filter, only when addr in one region and this region has been sel, will measure the transaction data num"]
    #[inline(always)]
    pub fn sel_ag3_wr_addr_region_sel(&self) -> SEL_AG3_WR_ADDR_REGION_SEL_R {
        SEL_AG3_WR_ADDR_REGION_SEL_R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 12:14 - SW config Write region select, use with mask and filter, only when addr in one region and this region has been sel, will measure the transaction data num"]
    #[inline(always)]
    pub fn sel_ag4_wr_addr_region_sel(&self) -> SEL_AG4_WR_ADDR_REGION_SEL_R {
        SEL_AG4_WR_ADDR_REGION_SEL_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 15:17 - SW config Write region select, use with mask and filter, only when addr in one region and this region has been sel, will measure the transaction data num"]
    #[inline(always)]
    pub fn sel_ag5_wr_addr_region_sel(&self) -> SEL_AG5_WR_ADDR_REGION_SEL_R {
        SEL_AG5_WR_ADDR_REGION_SEL_R::new(((self.bits >> 15) & 7) as u8)
    }
    #[doc = "Bits 18:20 - SW config Write region select, use with mask and filter, only when addr in one region and this region has been sel, will measure the transaction data num"]
    #[inline(always)]
    pub fn sel_ag6_wr_addr_region_sel(&self) -> SEL_AG6_WR_ADDR_REGION_SEL_R {
        SEL_AG6_WR_ADDR_REGION_SEL_R::new(((self.bits >> 18) & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SEL_AG_WR_ADDR_REGION_SEL")
            .field(
                "sel_ag0_wr_addr_region_sel",
                &self.sel_ag0_wr_addr_region_sel(),
            )
            .field(
                "sel_ag1_wr_addr_region_sel",
                &self.sel_ag1_wr_addr_region_sel(),
            )
            .field(
                "sel_ag2_wr_addr_region_sel",
                &self.sel_ag2_wr_addr_region_sel(),
            )
            .field(
                "sel_ag3_wr_addr_region_sel",
                &self.sel_ag3_wr_addr_region_sel(),
            )
            .field(
                "sel_ag4_wr_addr_region_sel",
                &self.sel_ag4_wr_addr_region_sel(),
            )
            .field(
                "sel_ag5_wr_addr_region_sel",
                &self.sel_ag5_wr_addr_region_sel(),
            )
            .field(
                "sel_ag6_wr_addr_region_sel",
                &self.sel_ag6_wr_addr_region_sel(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - SW config Write region select, use with mask and filter, only when addr in one region and this region has been sel, will measure the transaction data num"]
    #[inline(always)]
    pub fn sel_ag0_wr_addr_region_sel(
        &mut self,
    ) -> SEL_AG0_WR_ADDR_REGION_SEL_W<'_, SEL_AG_WR_ADDR_REGION_SEL_SPEC> {
        SEL_AG0_WR_ADDR_REGION_SEL_W::new(self, 0)
    }
    #[doc = "Bits 3:5 - SW config Write region select, use with mask and filter, only when addr in one region and this region has been sel, will measure the transaction data num"]
    #[inline(always)]
    pub fn sel_ag1_wr_addr_region_sel(
        &mut self,
    ) -> SEL_AG1_WR_ADDR_REGION_SEL_W<'_, SEL_AG_WR_ADDR_REGION_SEL_SPEC> {
        SEL_AG1_WR_ADDR_REGION_SEL_W::new(self, 3)
    }
    #[doc = "Bits 6:8 - SW config Write region select, use with mask and filter, only when addr in one region and this region has been sel, will measure the transaction data num"]
    #[inline(always)]
    pub fn sel_ag2_wr_addr_region_sel(
        &mut self,
    ) -> SEL_AG2_WR_ADDR_REGION_SEL_W<'_, SEL_AG_WR_ADDR_REGION_SEL_SPEC> {
        SEL_AG2_WR_ADDR_REGION_SEL_W::new(self, 6)
    }
    #[doc = "Bits 9:11 - SW config Write region select, use with mask and filter, only when addr in one region and this region has been sel, will measure the transaction data num"]
    #[inline(always)]
    pub fn sel_ag3_wr_addr_region_sel(
        &mut self,
    ) -> SEL_AG3_WR_ADDR_REGION_SEL_W<'_, SEL_AG_WR_ADDR_REGION_SEL_SPEC> {
        SEL_AG3_WR_ADDR_REGION_SEL_W::new(self, 9)
    }
    #[doc = "Bits 12:14 - SW config Write region select, use with mask and filter, only when addr in one region and this region has been sel, will measure the transaction data num"]
    #[inline(always)]
    pub fn sel_ag4_wr_addr_region_sel(
        &mut self,
    ) -> SEL_AG4_WR_ADDR_REGION_SEL_W<'_, SEL_AG_WR_ADDR_REGION_SEL_SPEC> {
        SEL_AG4_WR_ADDR_REGION_SEL_W::new(self, 12)
    }
    #[doc = "Bits 15:17 - SW config Write region select, use with mask and filter, only when addr in one region and this region has been sel, will measure the transaction data num"]
    #[inline(always)]
    pub fn sel_ag5_wr_addr_region_sel(
        &mut self,
    ) -> SEL_AG5_WR_ADDR_REGION_SEL_W<'_, SEL_AG_WR_ADDR_REGION_SEL_SPEC> {
        SEL_AG5_WR_ADDR_REGION_SEL_W::new(self, 15)
    }
    #[doc = "Bits 18:20 - SW config Write region select, use with mask and filter, only when addr in one region and this region has been sel, will measure the transaction data num"]
    #[inline(always)]
    pub fn sel_ag6_wr_addr_region_sel(
        &mut self,
    ) -> SEL_AG6_WR_ADDR_REGION_SEL_W<'_, SEL_AG_WR_ADDR_REGION_SEL_SPEC> {
        SEL_AG6_WR_ADDR_REGION_SEL_W::new(self, 18)
    }
}
#[doc = "reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sel_ag_wr_addr_region_sel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sel_ag_wr_addr_region_sel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SEL_AG_WR_ADDR_REGION_SEL_SPEC;
impl crate::RegisterSpec for SEL_AG_WR_ADDR_REGION_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sel_ag_wr_addr_region_sel::R`](R) reader structure"]
impl crate::Readable for SEL_AG_WR_ADDR_REGION_SEL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sel_ag_wr_addr_region_sel::W`](W) writer structure"]
impl crate::Writable for SEL_AG_WR_ADDR_REGION_SEL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SEL_AG_WR_ADDR_REGION_SEL to value 0"]
impl crate::Resettable for SEL_AG_WR_ADDR_REGION_SEL_SPEC {}
