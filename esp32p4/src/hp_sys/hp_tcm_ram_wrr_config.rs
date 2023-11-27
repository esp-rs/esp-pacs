#[doc = "Register `HP_TCM_RAM_WRR_CONFIG` reader"]
pub type R = crate::R<HP_TCM_RAM_WRR_CONFIG_SPEC>;
#[doc = "Register `HP_TCM_RAM_WRR_CONFIG` writer"]
pub type W = crate::W<HP_TCM_RAM_WRR_CONFIG_SPEC>;
#[doc = "Field `HP_REG_TCM_RAM_IBUS0_WT` reader - weight value of ibus0"]
pub type HP_REG_TCM_RAM_IBUS0_WT_R = crate::FieldReader;
#[doc = "Field `HP_REG_TCM_RAM_IBUS0_WT` writer - weight value of ibus0"]
pub type HP_REG_TCM_RAM_IBUS0_WT_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `HP_REG_TCM_RAM_IBUS1_WT` reader - weight value of ibus1"]
pub type HP_REG_TCM_RAM_IBUS1_WT_R = crate::FieldReader;
#[doc = "Field `HP_REG_TCM_RAM_IBUS1_WT` writer - weight value of ibus1"]
pub type HP_REG_TCM_RAM_IBUS1_WT_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `HP_REG_TCM_RAM_IBUS2_WT` reader - weight value of ibus2"]
pub type HP_REG_TCM_RAM_IBUS2_WT_R = crate::FieldReader;
#[doc = "Field `HP_REG_TCM_RAM_IBUS2_WT` writer - weight value of ibus2"]
pub type HP_REG_TCM_RAM_IBUS2_WT_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `HP_REG_TCM_RAM_IBUS3_WT` reader - weight value of ibus3"]
pub type HP_REG_TCM_RAM_IBUS3_WT_R = crate::FieldReader;
#[doc = "Field `HP_REG_TCM_RAM_IBUS3_WT` writer - weight value of ibus3"]
pub type HP_REG_TCM_RAM_IBUS3_WT_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `HP_REG_TCM_RAM_DBUS0_WT` reader - weight value of dbus0"]
pub type HP_REG_TCM_RAM_DBUS0_WT_R = crate::FieldReader;
#[doc = "Field `HP_REG_TCM_RAM_DBUS0_WT` writer - weight value of dbus0"]
pub type HP_REG_TCM_RAM_DBUS0_WT_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `HP_REG_TCM_RAM_DBUS1_WT` reader - weight value of dbus1"]
pub type HP_REG_TCM_RAM_DBUS1_WT_R = crate::FieldReader;
#[doc = "Field `HP_REG_TCM_RAM_DBUS1_WT` writer - weight value of dbus1"]
pub type HP_REG_TCM_RAM_DBUS1_WT_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `HP_REG_TCM_RAM_DBUS2_WT` reader - weight value of dbus2"]
pub type HP_REG_TCM_RAM_DBUS2_WT_R = crate::FieldReader;
#[doc = "Field `HP_REG_TCM_RAM_DBUS2_WT` writer - weight value of dbus2"]
pub type HP_REG_TCM_RAM_DBUS2_WT_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `HP_REG_TCM_RAM_DBUS3_WT` reader - weight value of dbus3"]
pub type HP_REG_TCM_RAM_DBUS3_WT_R = crate::FieldReader;
#[doc = "Field `HP_REG_TCM_RAM_DBUS3_WT` writer - weight value of dbus3"]
pub type HP_REG_TCM_RAM_DBUS3_WT_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `HP_REG_TCM_RAM_DMA_WT` reader - weight value of dma"]
pub type HP_REG_TCM_RAM_DMA_WT_R = crate::FieldReader;
#[doc = "Field `HP_REG_TCM_RAM_DMA_WT` writer - weight value of dma"]
pub type HP_REG_TCM_RAM_DMA_WT_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `HP_REG_TCM_RAM_WRR_HIGH` reader - enable weighted round robin arbitration"]
pub type HP_REG_TCM_RAM_WRR_HIGH_R = crate::BitReader;
#[doc = "Field `HP_REG_TCM_RAM_WRR_HIGH` writer - enable weighted round robin arbitration"]
pub type HP_REG_TCM_RAM_WRR_HIGH_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - weight value of ibus0"]
    #[inline(always)]
    pub fn hp_reg_tcm_ram_ibus0_wt(&self) -> HP_REG_TCM_RAM_IBUS0_WT_R {
        HP_REG_TCM_RAM_IBUS0_WT_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - weight value of ibus1"]
    #[inline(always)]
    pub fn hp_reg_tcm_ram_ibus1_wt(&self) -> HP_REG_TCM_RAM_IBUS1_WT_R {
        HP_REG_TCM_RAM_IBUS1_WT_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:8 - weight value of ibus2"]
    #[inline(always)]
    pub fn hp_reg_tcm_ram_ibus2_wt(&self) -> HP_REG_TCM_RAM_IBUS2_WT_R {
        HP_REG_TCM_RAM_IBUS2_WT_R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 9:11 - weight value of ibus3"]
    #[inline(always)]
    pub fn hp_reg_tcm_ram_ibus3_wt(&self) -> HP_REG_TCM_RAM_IBUS3_WT_R {
        HP_REG_TCM_RAM_IBUS3_WT_R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 12:14 - weight value of dbus0"]
    #[inline(always)]
    pub fn hp_reg_tcm_ram_dbus0_wt(&self) -> HP_REG_TCM_RAM_DBUS0_WT_R {
        HP_REG_TCM_RAM_DBUS0_WT_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 15:17 - weight value of dbus1"]
    #[inline(always)]
    pub fn hp_reg_tcm_ram_dbus1_wt(&self) -> HP_REG_TCM_RAM_DBUS1_WT_R {
        HP_REG_TCM_RAM_DBUS1_WT_R::new(((self.bits >> 15) & 7) as u8)
    }
    #[doc = "Bits 18:20 - weight value of dbus2"]
    #[inline(always)]
    pub fn hp_reg_tcm_ram_dbus2_wt(&self) -> HP_REG_TCM_RAM_DBUS2_WT_R {
        HP_REG_TCM_RAM_DBUS2_WT_R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bits 21:23 - weight value of dbus3"]
    #[inline(always)]
    pub fn hp_reg_tcm_ram_dbus3_wt(&self) -> HP_REG_TCM_RAM_DBUS3_WT_R {
        HP_REG_TCM_RAM_DBUS3_WT_R::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bits 24:26 - weight value of dma"]
    #[inline(always)]
    pub fn hp_reg_tcm_ram_dma_wt(&self) -> HP_REG_TCM_RAM_DMA_WT_R {
        HP_REG_TCM_RAM_DMA_WT_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 31 - enable weighted round robin arbitration"]
    #[inline(always)]
    pub fn hp_reg_tcm_ram_wrr_high(&self) -> HP_REG_TCM_RAM_WRR_HIGH_R {
        HP_REG_TCM_RAM_WRR_HIGH_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HP_TCM_RAM_WRR_CONFIG")
            .field(
                "hp_reg_tcm_ram_ibus0_wt",
                &format_args!("{}", self.hp_reg_tcm_ram_ibus0_wt().bits()),
            )
            .field(
                "hp_reg_tcm_ram_ibus1_wt",
                &format_args!("{}", self.hp_reg_tcm_ram_ibus1_wt().bits()),
            )
            .field(
                "hp_reg_tcm_ram_ibus2_wt",
                &format_args!("{}", self.hp_reg_tcm_ram_ibus2_wt().bits()),
            )
            .field(
                "hp_reg_tcm_ram_ibus3_wt",
                &format_args!("{}", self.hp_reg_tcm_ram_ibus3_wt().bits()),
            )
            .field(
                "hp_reg_tcm_ram_dbus0_wt",
                &format_args!("{}", self.hp_reg_tcm_ram_dbus0_wt().bits()),
            )
            .field(
                "hp_reg_tcm_ram_dbus1_wt",
                &format_args!("{}", self.hp_reg_tcm_ram_dbus1_wt().bits()),
            )
            .field(
                "hp_reg_tcm_ram_dbus2_wt",
                &format_args!("{}", self.hp_reg_tcm_ram_dbus2_wt().bits()),
            )
            .field(
                "hp_reg_tcm_ram_dbus3_wt",
                &format_args!("{}", self.hp_reg_tcm_ram_dbus3_wt().bits()),
            )
            .field(
                "hp_reg_tcm_ram_dma_wt",
                &format_args!("{}", self.hp_reg_tcm_ram_dma_wt().bits()),
            )
            .field(
                "hp_reg_tcm_ram_wrr_high",
                &format_args!("{}", self.hp_reg_tcm_ram_wrr_high().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HP_TCM_RAM_WRR_CONFIG_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:2 - weight value of ibus0"]
    #[inline(always)]
    #[must_use]
    pub fn hp_reg_tcm_ram_ibus0_wt(
        &mut self,
    ) -> HP_REG_TCM_RAM_IBUS0_WT_W<HP_TCM_RAM_WRR_CONFIG_SPEC> {
        HP_REG_TCM_RAM_IBUS0_WT_W::new(self, 0)
    }
    #[doc = "Bits 3:5 - weight value of ibus1"]
    #[inline(always)]
    #[must_use]
    pub fn hp_reg_tcm_ram_ibus1_wt(
        &mut self,
    ) -> HP_REG_TCM_RAM_IBUS1_WT_W<HP_TCM_RAM_WRR_CONFIG_SPEC> {
        HP_REG_TCM_RAM_IBUS1_WT_W::new(self, 3)
    }
    #[doc = "Bits 6:8 - weight value of ibus2"]
    #[inline(always)]
    #[must_use]
    pub fn hp_reg_tcm_ram_ibus2_wt(
        &mut self,
    ) -> HP_REG_TCM_RAM_IBUS2_WT_W<HP_TCM_RAM_WRR_CONFIG_SPEC> {
        HP_REG_TCM_RAM_IBUS2_WT_W::new(self, 6)
    }
    #[doc = "Bits 9:11 - weight value of ibus3"]
    #[inline(always)]
    #[must_use]
    pub fn hp_reg_tcm_ram_ibus3_wt(
        &mut self,
    ) -> HP_REG_TCM_RAM_IBUS3_WT_W<HP_TCM_RAM_WRR_CONFIG_SPEC> {
        HP_REG_TCM_RAM_IBUS3_WT_W::new(self, 9)
    }
    #[doc = "Bits 12:14 - weight value of dbus0"]
    #[inline(always)]
    #[must_use]
    pub fn hp_reg_tcm_ram_dbus0_wt(
        &mut self,
    ) -> HP_REG_TCM_RAM_DBUS0_WT_W<HP_TCM_RAM_WRR_CONFIG_SPEC> {
        HP_REG_TCM_RAM_DBUS0_WT_W::new(self, 12)
    }
    #[doc = "Bits 15:17 - weight value of dbus1"]
    #[inline(always)]
    #[must_use]
    pub fn hp_reg_tcm_ram_dbus1_wt(
        &mut self,
    ) -> HP_REG_TCM_RAM_DBUS1_WT_W<HP_TCM_RAM_WRR_CONFIG_SPEC> {
        HP_REG_TCM_RAM_DBUS1_WT_W::new(self, 15)
    }
    #[doc = "Bits 18:20 - weight value of dbus2"]
    #[inline(always)]
    #[must_use]
    pub fn hp_reg_tcm_ram_dbus2_wt(
        &mut self,
    ) -> HP_REG_TCM_RAM_DBUS2_WT_W<HP_TCM_RAM_WRR_CONFIG_SPEC> {
        HP_REG_TCM_RAM_DBUS2_WT_W::new(self, 18)
    }
    #[doc = "Bits 21:23 - weight value of dbus3"]
    #[inline(always)]
    #[must_use]
    pub fn hp_reg_tcm_ram_dbus3_wt(
        &mut self,
    ) -> HP_REG_TCM_RAM_DBUS3_WT_W<HP_TCM_RAM_WRR_CONFIG_SPEC> {
        HP_REG_TCM_RAM_DBUS3_WT_W::new(self, 21)
    }
    #[doc = "Bits 24:26 - weight value of dma"]
    #[inline(always)]
    #[must_use]
    pub fn hp_reg_tcm_ram_dma_wt(&mut self) -> HP_REG_TCM_RAM_DMA_WT_W<HP_TCM_RAM_WRR_CONFIG_SPEC> {
        HP_REG_TCM_RAM_DMA_WT_W::new(self, 24)
    }
    #[doc = "Bit 31 - enable weighted round robin arbitration"]
    #[inline(always)]
    #[must_use]
    pub fn hp_reg_tcm_ram_wrr_high(
        &mut self,
    ) -> HP_REG_TCM_RAM_WRR_HIGH_W<HP_TCM_RAM_WRR_CONFIG_SPEC> {
        HP_REG_TCM_RAM_WRR_HIGH_W::new(self, 31)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hp_tcm_ram_wrr_config::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_tcm_ram_wrr_config::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HP_TCM_RAM_WRR_CONFIG_SPEC;
impl crate::RegisterSpec for HP_TCM_RAM_WRR_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hp_tcm_ram_wrr_config::R`](R) reader structure"]
impl crate::Readable for HP_TCM_RAM_WRR_CONFIG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hp_tcm_ram_wrr_config::W`](W) writer structure"]
impl crate::Writable for HP_TCM_RAM_WRR_CONFIG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HP_TCM_RAM_WRR_CONFIG to value 0x826e_d93f"]
impl crate::Resettable for HP_TCM_RAM_WRR_CONFIG_SPEC {
    const RESET_VALUE: Self::Ux = 0x826e_d93f;
}
