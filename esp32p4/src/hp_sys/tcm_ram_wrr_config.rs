#[doc = "Register `TCM_RAM_WRR_CONFIG` reader"]
pub type R = crate::R<TCM_RAM_WRR_CONFIG_SPEC>;
#[doc = "Register `TCM_RAM_WRR_CONFIG` writer"]
pub type W = crate::W<TCM_RAM_WRR_CONFIG_SPEC>;
#[doc = "Field `REG_TCM_RAM_IBUS0_WT` reader - weight value of ibus0"]
pub type REG_TCM_RAM_IBUS0_WT_R = crate::FieldReader;
#[doc = "Field `REG_TCM_RAM_IBUS0_WT` writer - weight value of ibus0"]
pub type REG_TCM_RAM_IBUS0_WT_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `REG_TCM_RAM_IBUS1_WT` reader - weight value of ibus1"]
pub type REG_TCM_RAM_IBUS1_WT_R = crate::FieldReader;
#[doc = "Field `REG_TCM_RAM_IBUS1_WT` writer - weight value of ibus1"]
pub type REG_TCM_RAM_IBUS1_WT_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `REG_TCM_RAM_IBUS2_WT` reader - weight value of ibus2"]
pub type REG_TCM_RAM_IBUS2_WT_R = crate::FieldReader;
#[doc = "Field `REG_TCM_RAM_IBUS2_WT` writer - weight value of ibus2"]
pub type REG_TCM_RAM_IBUS2_WT_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `REG_TCM_RAM_IBUS3_WT` reader - weight value of ibus3"]
pub type REG_TCM_RAM_IBUS3_WT_R = crate::FieldReader;
#[doc = "Field `REG_TCM_RAM_IBUS3_WT` writer - weight value of ibus3"]
pub type REG_TCM_RAM_IBUS3_WT_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `REG_TCM_RAM_DBUS0_WT` reader - weight value of dbus0"]
pub type REG_TCM_RAM_DBUS0_WT_R = crate::FieldReader;
#[doc = "Field `REG_TCM_RAM_DBUS0_WT` writer - weight value of dbus0"]
pub type REG_TCM_RAM_DBUS0_WT_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `REG_TCM_RAM_DBUS1_WT` reader - weight value of dbus1"]
pub type REG_TCM_RAM_DBUS1_WT_R = crate::FieldReader;
#[doc = "Field `REG_TCM_RAM_DBUS1_WT` writer - weight value of dbus1"]
pub type REG_TCM_RAM_DBUS1_WT_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `REG_TCM_RAM_DBUS2_WT` reader - weight value of dbus2"]
pub type REG_TCM_RAM_DBUS2_WT_R = crate::FieldReader;
#[doc = "Field `REG_TCM_RAM_DBUS2_WT` writer - weight value of dbus2"]
pub type REG_TCM_RAM_DBUS2_WT_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `REG_TCM_RAM_DBUS3_WT` reader - weight value of dbus3"]
pub type REG_TCM_RAM_DBUS3_WT_R = crate::FieldReader;
#[doc = "Field `REG_TCM_RAM_DBUS3_WT` writer - weight value of dbus3"]
pub type REG_TCM_RAM_DBUS3_WT_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `REG_TCM_RAM_DMA_WT` reader - weight value of dma"]
pub type REG_TCM_RAM_DMA_WT_R = crate::FieldReader;
#[doc = "Field `REG_TCM_RAM_DMA_WT` writer - weight value of dma"]
pub type REG_TCM_RAM_DMA_WT_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `REG_TCM_RAM_WRR_HIGH` reader - enable weighted round robin arbitration"]
pub type REG_TCM_RAM_WRR_HIGH_R = crate::BitReader;
#[doc = "Field `REG_TCM_RAM_WRR_HIGH` writer - enable weighted round robin arbitration"]
pub type REG_TCM_RAM_WRR_HIGH_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - weight value of ibus0"]
    #[inline(always)]
    pub fn reg_tcm_ram_ibus0_wt(&self) -> REG_TCM_RAM_IBUS0_WT_R {
        REG_TCM_RAM_IBUS0_WT_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - weight value of ibus1"]
    #[inline(always)]
    pub fn reg_tcm_ram_ibus1_wt(&self) -> REG_TCM_RAM_IBUS1_WT_R {
        REG_TCM_RAM_IBUS1_WT_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:8 - weight value of ibus2"]
    #[inline(always)]
    pub fn reg_tcm_ram_ibus2_wt(&self) -> REG_TCM_RAM_IBUS2_WT_R {
        REG_TCM_RAM_IBUS2_WT_R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 9:11 - weight value of ibus3"]
    #[inline(always)]
    pub fn reg_tcm_ram_ibus3_wt(&self) -> REG_TCM_RAM_IBUS3_WT_R {
        REG_TCM_RAM_IBUS3_WT_R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 12:14 - weight value of dbus0"]
    #[inline(always)]
    pub fn reg_tcm_ram_dbus0_wt(&self) -> REG_TCM_RAM_DBUS0_WT_R {
        REG_TCM_RAM_DBUS0_WT_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 15:17 - weight value of dbus1"]
    #[inline(always)]
    pub fn reg_tcm_ram_dbus1_wt(&self) -> REG_TCM_RAM_DBUS1_WT_R {
        REG_TCM_RAM_DBUS1_WT_R::new(((self.bits >> 15) & 7) as u8)
    }
    #[doc = "Bits 18:20 - weight value of dbus2"]
    #[inline(always)]
    pub fn reg_tcm_ram_dbus2_wt(&self) -> REG_TCM_RAM_DBUS2_WT_R {
        REG_TCM_RAM_DBUS2_WT_R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bits 21:23 - weight value of dbus3"]
    #[inline(always)]
    pub fn reg_tcm_ram_dbus3_wt(&self) -> REG_TCM_RAM_DBUS3_WT_R {
        REG_TCM_RAM_DBUS3_WT_R::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bits 24:26 - weight value of dma"]
    #[inline(always)]
    pub fn reg_tcm_ram_dma_wt(&self) -> REG_TCM_RAM_DMA_WT_R {
        REG_TCM_RAM_DMA_WT_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 31 - enable weighted round robin arbitration"]
    #[inline(always)]
    pub fn reg_tcm_ram_wrr_high(&self) -> REG_TCM_RAM_WRR_HIGH_R {
        REG_TCM_RAM_WRR_HIGH_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TCM_RAM_WRR_CONFIG")
            .field("reg_tcm_ram_ibus0_wt", &self.reg_tcm_ram_ibus0_wt())
            .field("reg_tcm_ram_ibus1_wt", &self.reg_tcm_ram_ibus1_wt())
            .field("reg_tcm_ram_ibus2_wt", &self.reg_tcm_ram_ibus2_wt())
            .field("reg_tcm_ram_ibus3_wt", &self.reg_tcm_ram_ibus3_wt())
            .field("reg_tcm_ram_dbus0_wt", &self.reg_tcm_ram_dbus0_wt())
            .field("reg_tcm_ram_dbus1_wt", &self.reg_tcm_ram_dbus1_wt())
            .field("reg_tcm_ram_dbus2_wt", &self.reg_tcm_ram_dbus2_wt())
            .field("reg_tcm_ram_dbus3_wt", &self.reg_tcm_ram_dbus3_wt())
            .field("reg_tcm_ram_dma_wt", &self.reg_tcm_ram_dma_wt())
            .field("reg_tcm_ram_wrr_high", &self.reg_tcm_ram_wrr_high())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - weight value of ibus0"]
    #[inline(always)]
    #[must_use]
    pub fn reg_tcm_ram_ibus0_wt(&mut self) -> REG_TCM_RAM_IBUS0_WT_W<TCM_RAM_WRR_CONFIG_SPEC> {
        REG_TCM_RAM_IBUS0_WT_W::new(self, 0)
    }
    #[doc = "Bits 3:5 - weight value of ibus1"]
    #[inline(always)]
    #[must_use]
    pub fn reg_tcm_ram_ibus1_wt(&mut self) -> REG_TCM_RAM_IBUS1_WT_W<TCM_RAM_WRR_CONFIG_SPEC> {
        REG_TCM_RAM_IBUS1_WT_W::new(self, 3)
    }
    #[doc = "Bits 6:8 - weight value of ibus2"]
    #[inline(always)]
    #[must_use]
    pub fn reg_tcm_ram_ibus2_wt(&mut self) -> REG_TCM_RAM_IBUS2_WT_W<TCM_RAM_WRR_CONFIG_SPEC> {
        REG_TCM_RAM_IBUS2_WT_W::new(self, 6)
    }
    #[doc = "Bits 9:11 - weight value of ibus3"]
    #[inline(always)]
    #[must_use]
    pub fn reg_tcm_ram_ibus3_wt(&mut self) -> REG_TCM_RAM_IBUS3_WT_W<TCM_RAM_WRR_CONFIG_SPEC> {
        REG_TCM_RAM_IBUS3_WT_W::new(self, 9)
    }
    #[doc = "Bits 12:14 - weight value of dbus0"]
    #[inline(always)]
    #[must_use]
    pub fn reg_tcm_ram_dbus0_wt(&mut self) -> REG_TCM_RAM_DBUS0_WT_W<TCM_RAM_WRR_CONFIG_SPEC> {
        REG_TCM_RAM_DBUS0_WT_W::new(self, 12)
    }
    #[doc = "Bits 15:17 - weight value of dbus1"]
    #[inline(always)]
    #[must_use]
    pub fn reg_tcm_ram_dbus1_wt(&mut self) -> REG_TCM_RAM_DBUS1_WT_W<TCM_RAM_WRR_CONFIG_SPEC> {
        REG_TCM_RAM_DBUS1_WT_W::new(self, 15)
    }
    #[doc = "Bits 18:20 - weight value of dbus2"]
    #[inline(always)]
    #[must_use]
    pub fn reg_tcm_ram_dbus2_wt(&mut self) -> REG_TCM_RAM_DBUS2_WT_W<TCM_RAM_WRR_CONFIG_SPEC> {
        REG_TCM_RAM_DBUS2_WT_W::new(self, 18)
    }
    #[doc = "Bits 21:23 - weight value of dbus3"]
    #[inline(always)]
    #[must_use]
    pub fn reg_tcm_ram_dbus3_wt(&mut self) -> REG_TCM_RAM_DBUS3_WT_W<TCM_RAM_WRR_CONFIG_SPEC> {
        REG_TCM_RAM_DBUS3_WT_W::new(self, 21)
    }
    #[doc = "Bits 24:26 - weight value of dma"]
    #[inline(always)]
    #[must_use]
    pub fn reg_tcm_ram_dma_wt(&mut self) -> REG_TCM_RAM_DMA_WT_W<TCM_RAM_WRR_CONFIG_SPEC> {
        REG_TCM_RAM_DMA_WT_W::new(self, 24)
    }
    #[doc = "Bit 31 - enable weighted round robin arbitration"]
    #[inline(always)]
    #[must_use]
    pub fn reg_tcm_ram_wrr_high(&mut self) -> REG_TCM_RAM_WRR_HIGH_W<TCM_RAM_WRR_CONFIG_SPEC> {
        REG_TCM_RAM_WRR_HIGH_W::new(self, 31)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`tcm_ram_wrr_config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcm_ram_wrr_config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TCM_RAM_WRR_CONFIG_SPEC;
impl crate::RegisterSpec for TCM_RAM_WRR_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tcm_ram_wrr_config::R`](R) reader structure"]
impl crate::Readable for TCM_RAM_WRR_CONFIG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tcm_ram_wrr_config::W`](W) writer structure"]
impl crate::Writable for TCM_RAM_WRR_CONFIG_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TCM_RAM_WRR_CONFIG to value 0x826e_d93f"]
impl crate::Resettable for TCM_RAM_WRR_CONFIG_SPEC {
    const RESET_VALUE: u32 = 0x826e_d93f;
}
