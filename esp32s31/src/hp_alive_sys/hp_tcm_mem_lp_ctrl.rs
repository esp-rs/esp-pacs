#[doc = "Register `HP_TCM_MEM_LP_CTRL` reader"]
pub type R = crate::R<HP_TCM_MEM_LP_CTRL_SPEC>;
#[doc = "Register `HP_TCM_MEM_LP_CTRL` writer"]
pub type W = crate::W<HP_TCM_MEM_LP_CTRL_SPEC>;
#[doc = "Field `HP_TCM_L0_MEM_LP_MODE` reader - Configures layer0 hp_sram memory low power mode in low power stage.\\\\ 0(default): deep sleep\\\\ 1: light sleep\\\\ 2: shut down\\\\ 3: disable low power stage\\\\"]
pub type HP_TCM_L0_MEM_LP_MODE_R = crate::FieldReader;
#[doc = "Field `HP_TCM_L0_MEM_LP_MODE` writer - Configures layer0 hp_sram memory low power mode in low power stage.\\\\ 0(default): deep sleep\\\\ 1: light sleep\\\\ 2: shut down\\\\ 3: disable low power stage\\\\"]
pub type HP_TCM_L0_MEM_LP_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `HP_TCM_L0_MEM_LP_EN` reader - Set this bit to power down layer0 hp_sram memory."]
pub type HP_TCM_L0_MEM_LP_EN_R = crate::BitReader;
#[doc = "Field `HP_TCM_L0_MEM_LP_EN` writer - Set this bit to power down layer0 hp_sram memory."]
pub type HP_TCM_L0_MEM_LP_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_TCM_MEM_L0_LP_FORCE_CTRL` reader - Set this bit to force software control layer0 hp_sram memory, disbale hardware control."]
pub type HP_TCM_MEM_L0_LP_FORCE_CTRL_R = crate::BitReader;
#[doc = "Field `HP_TCM_MEM_L0_LP_FORCE_CTRL` writer - Set this bit to force software control layer0 hp_sram memory, disbale hardware control."]
pub type HP_TCM_MEM_L0_LP_FORCE_CTRL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_TCM_L1_MEM_LP_MODE` reader - Configures layer1 hp_sram memory low power mode in low power stage.\\\\ 0(default): deep sleep\\\\ 1: light sleep\\\\ 2: shut down\\\\ 3: disable low power stage\\\\"]
pub type HP_TCM_L1_MEM_LP_MODE_R = crate::FieldReader;
#[doc = "Field `HP_TCM_L1_MEM_LP_MODE` writer - Configures layer1 hp_sram memory low power mode in low power stage.\\\\ 0(default): deep sleep\\\\ 1: light sleep\\\\ 2: shut down\\\\ 3: disable low power stage\\\\"]
pub type HP_TCM_L1_MEM_LP_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `HP_TCM_L1_MEM_LP_EN` reader - Set this bit to power down layer1 hp_sram memory."]
pub type HP_TCM_L1_MEM_LP_EN_R = crate::BitReader;
#[doc = "Field `HP_TCM_L1_MEM_LP_EN` writer - Set this bit to power down layer1 hp_sram memory."]
pub type HP_TCM_L1_MEM_LP_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_TCM_MEM_L1_LP_FORCE_CTRL` reader - Set this bit to force software control layer1 hp_sram memory, disbale hardware control."]
pub type HP_TCM_MEM_L1_LP_FORCE_CTRL_R = crate::BitReader;
#[doc = "Field `HP_TCM_MEM_L1_LP_FORCE_CTRL` writer - Set this bit to force software control layer1 hp_sram memory, disbale hardware control."]
pub type HP_TCM_MEM_L1_LP_FORCE_CTRL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_TCM_L2_MEM_LP_MODE` reader - Configures layer2 hp_sram memory low power mode in low power stage.\\\\ 0(default): deep sleep\\\\ 1: light sleep\\\\ 2: shut down\\\\ 3: disable low power stage\\\\"]
pub type HP_TCM_L2_MEM_LP_MODE_R = crate::FieldReader;
#[doc = "Field `HP_TCM_L2_MEM_LP_MODE` writer - Configures layer2 hp_sram memory low power mode in low power stage.\\\\ 0(default): deep sleep\\\\ 1: light sleep\\\\ 2: shut down\\\\ 3: disable low power stage\\\\"]
pub type HP_TCM_L2_MEM_LP_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `HP_TCM_L2_MEM_LP_EN` reader - Set this bit to power down layer2 hp_sram memory."]
pub type HP_TCM_L2_MEM_LP_EN_R = crate::BitReader;
#[doc = "Field `HP_TCM_L2_MEM_LP_EN` writer - Set this bit to power down layer2 hp_sram memory."]
pub type HP_TCM_L2_MEM_LP_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_TCM_MEM_L2_LP_FORCE_CTRL` reader - Set this bit to force software control layer2 hp_sram memory, disbale hardware control."]
pub type HP_TCM_MEM_L2_LP_FORCE_CTRL_R = crate::BitReader;
#[doc = "Field `HP_TCM_MEM_L2_LP_FORCE_CTRL` writer - Set this bit to force software control layer2 hp_sram memory, disbale hardware control."]
pub type HP_TCM_MEM_L2_LP_FORCE_CTRL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_TCM_L3_MEM_LP_MODE` reader - Configures layer3 hp_sram memory low power mode in low power stage.\\\\ 0(default): deep sleep\\\\ 1: light sleep\\\\ 2: shut down\\\\ 3: disable low power stage\\\\"]
pub type HP_TCM_L3_MEM_LP_MODE_R = crate::FieldReader;
#[doc = "Field `HP_TCM_L3_MEM_LP_MODE` writer - Configures layer3 hp_sram memory low power mode in low power stage.\\\\ 0(default): deep sleep\\\\ 1: light sleep\\\\ 2: shut down\\\\ 3: disable low power stage\\\\"]
pub type HP_TCM_L3_MEM_LP_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `HP_TCM_L3_MEM_LP_EN` reader - Set this bit to power down layer3 hp_sram memory."]
pub type HP_TCM_L3_MEM_LP_EN_R = crate::BitReader;
#[doc = "Field `HP_TCM_L3_MEM_LP_EN` writer - Set this bit to power down layer3 hp_sram memory."]
pub type HP_TCM_L3_MEM_LP_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_TCM_MEM_L3_LP_FORCE_CTRL` reader - Set this bit to force software control layer3 hp_sram memory, disbale hardware control."]
pub type HP_TCM_MEM_L3_LP_FORCE_CTRL_R = crate::BitReader;
#[doc = "Field `HP_TCM_MEM_L3_LP_FORCE_CTRL` writer - Set this bit to force software control layer3 hp_sram memory, disbale hardware control."]
pub type HP_TCM_MEM_L3_LP_FORCE_CTRL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Configures layer0 hp_sram memory low power mode in low power stage.\\\\ 0(default): deep sleep\\\\ 1: light sleep\\\\ 2: shut down\\\\ 3: disable low power stage\\\\"]
    #[inline(always)]
    pub fn hp_tcm_l0_mem_lp_mode(&self) -> HP_TCM_L0_MEM_LP_MODE_R {
        HP_TCM_L0_MEM_LP_MODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Set this bit to power down layer0 hp_sram memory."]
    #[inline(always)]
    pub fn hp_tcm_l0_mem_lp_en(&self) -> HP_TCM_L0_MEM_LP_EN_R {
        HP_TCM_L0_MEM_LP_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Set this bit to force software control layer0 hp_sram memory, disbale hardware control."]
    #[inline(always)]
    pub fn hp_tcm_mem_l0_lp_force_ctrl(&self) -> HP_TCM_MEM_L0_LP_FORCE_CTRL_R {
        HP_TCM_MEM_L0_LP_FORCE_CTRL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Configures layer1 hp_sram memory low power mode in low power stage.\\\\ 0(default): deep sleep\\\\ 1: light sleep\\\\ 2: shut down\\\\ 3: disable low power stage\\\\"]
    #[inline(always)]
    pub fn hp_tcm_l1_mem_lp_mode(&self) -> HP_TCM_L1_MEM_LP_MODE_R {
        HP_TCM_L1_MEM_LP_MODE_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - Set this bit to power down layer1 hp_sram memory."]
    #[inline(always)]
    pub fn hp_tcm_l1_mem_lp_en(&self) -> HP_TCM_L1_MEM_LP_EN_R {
        HP_TCM_L1_MEM_LP_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Set this bit to force software control layer1 hp_sram memory, disbale hardware control."]
    #[inline(always)]
    pub fn hp_tcm_mem_l1_lp_force_ctrl(&self) -> HP_TCM_MEM_L1_LP_FORCE_CTRL_R {
        HP_TCM_MEM_L1_LP_FORCE_CTRL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Configures layer2 hp_sram memory low power mode in low power stage.\\\\ 0(default): deep sleep\\\\ 1: light sleep\\\\ 2: shut down\\\\ 3: disable low power stage\\\\"]
    #[inline(always)]
    pub fn hp_tcm_l2_mem_lp_mode(&self) -> HP_TCM_L2_MEM_LP_MODE_R {
        HP_TCM_L2_MEM_LP_MODE_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - Set this bit to power down layer2 hp_sram memory."]
    #[inline(always)]
    pub fn hp_tcm_l2_mem_lp_en(&self) -> HP_TCM_L2_MEM_LP_EN_R {
        HP_TCM_L2_MEM_LP_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Set this bit to force software control layer2 hp_sram memory, disbale hardware control."]
    #[inline(always)]
    pub fn hp_tcm_mem_l2_lp_force_ctrl(&self) -> HP_TCM_MEM_L2_LP_FORCE_CTRL_R {
        HP_TCM_MEM_L2_LP_FORCE_CTRL_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - Configures layer3 hp_sram memory low power mode in low power stage.\\\\ 0(default): deep sleep\\\\ 1: light sleep\\\\ 2: shut down\\\\ 3: disable low power stage\\\\"]
    #[inline(always)]
    pub fn hp_tcm_l3_mem_lp_mode(&self) -> HP_TCM_L3_MEM_LP_MODE_R {
        HP_TCM_L3_MEM_LP_MODE_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - Set this bit to power down layer3 hp_sram memory."]
    #[inline(always)]
    pub fn hp_tcm_l3_mem_lp_en(&self) -> HP_TCM_L3_MEM_LP_EN_R {
        HP_TCM_L3_MEM_LP_EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Set this bit to force software control layer3 hp_sram memory, disbale hardware control."]
    #[inline(always)]
    pub fn hp_tcm_mem_l3_lp_force_ctrl(&self) -> HP_TCM_MEM_L3_LP_FORCE_CTRL_R {
        HP_TCM_MEM_L3_LP_FORCE_CTRL_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HP_TCM_MEM_LP_CTRL")
            .field("hp_tcm_l0_mem_lp_mode", &self.hp_tcm_l0_mem_lp_mode())
            .field("hp_tcm_l0_mem_lp_en", &self.hp_tcm_l0_mem_lp_en())
            .field(
                "hp_tcm_mem_l0_lp_force_ctrl",
                &self.hp_tcm_mem_l0_lp_force_ctrl(),
            )
            .field("hp_tcm_l1_mem_lp_mode", &self.hp_tcm_l1_mem_lp_mode())
            .field("hp_tcm_l1_mem_lp_en", &self.hp_tcm_l1_mem_lp_en())
            .field(
                "hp_tcm_mem_l1_lp_force_ctrl",
                &self.hp_tcm_mem_l1_lp_force_ctrl(),
            )
            .field("hp_tcm_l2_mem_lp_mode", &self.hp_tcm_l2_mem_lp_mode())
            .field("hp_tcm_l2_mem_lp_en", &self.hp_tcm_l2_mem_lp_en())
            .field(
                "hp_tcm_mem_l2_lp_force_ctrl",
                &self.hp_tcm_mem_l2_lp_force_ctrl(),
            )
            .field("hp_tcm_l3_mem_lp_mode", &self.hp_tcm_l3_mem_lp_mode())
            .field("hp_tcm_l3_mem_lp_en", &self.hp_tcm_l3_mem_lp_en())
            .field(
                "hp_tcm_mem_l3_lp_force_ctrl",
                &self.hp_tcm_mem_l3_lp_force_ctrl(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - Configures layer0 hp_sram memory low power mode in low power stage.\\\\ 0(default): deep sleep\\\\ 1: light sleep\\\\ 2: shut down\\\\ 3: disable low power stage\\\\"]
    #[inline(always)]
    pub fn hp_tcm_l0_mem_lp_mode(
        &mut self,
    ) -> HP_TCM_L0_MEM_LP_MODE_W<'_, HP_TCM_MEM_LP_CTRL_SPEC> {
        HP_TCM_L0_MEM_LP_MODE_W::new(self, 0)
    }
    #[doc = "Bit 2 - Set this bit to power down layer0 hp_sram memory."]
    #[inline(always)]
    pub fn hp_tcm_l0_mem_lp_en(&mut self) -> HP_TCM_L0_MEM_LP_EN_W<'_, HP_TCM_MEM_LP_CTRL_SPEC> {
        HP_TCM_L0_MEM_LP_EN_W::new(self, 2)
    }
    #[doc = "Bit 3 - Set this bit to force software control layer0 hp_sram memory, disbale hardware control."]
    #[inline(always)]
    pub fn hp_tcm_mem_l0_lp_force_ctrl(
        &mut self,
    ) -> HP_TCM_MEM_L0_LP_FORCE_CTRL_W<'_, HP_TCM_MEM_LP_CTRL_SPEC> {
        HP_TCM_MEM_L0_LP_FORCE_CTRL_W::new(self, 3)
    }
    #[doc = "Bits 4:5 - Configures layer1 hp_sram memory low power mode in low power stage.\\\\ 0(default): deep sleep\\\\ 1: light sleep\\\\ 2: shut down\\\\ 3: disable low power stage\\\\"]
    #[inline(always)]
    pub fn hp_tcm_l1_mem_lp_mode(
        &mut self,
    ) -> HP_TCM_L1_MEM_LP_MODE_W<'_, HP_TCM_MEM_LP_CTRL_SPEC> {
        HP_TCM_L1_MEM_LP_MODE_W::new(self, 4)
    }
    #[doc = "Bit 6 - Set this bit to power down layer1 hp_sram memory."]
    #[inline(always)]
    pub fn hp_tcm_l1_mem_lp_en(&mut self) -> HP_TCM_L1_MEM_LP_EN_W<'_, HP_TCM_MEM_LP_CTRL_SPEC> {
        HP_TCM_L1_MEM_LP_EN_W::new(self, 6)
    }
    #[doc = "Bit 7 - Set this bit to force software control layer1 hp_sram memory, disbale hardware control."]
    #[inline(always)]
    pub fn hp_tcm_mem_l1_lp_force_ctrl(
        &mut self,
    ) -> HP_TCM_MEM_L1_LP_FORCE_CTRL_W<'_, HP_TCM_MEM_LP_CTRL_SPEC> {
        HP_TCM_MEM_L1_LP_FORCE_CTRL_W::new(self, 7)
    }
    #[doc = "Bits 8:9 - Configures layer2 hp_sram memory low power mode in low power stage.\\\\ 0(default): deep sleep\\\\ 1: light sleep\\\\ 2: shut down\\\\ 3: disable low power stage\\\\"]
    #[inline(always)]
    pub fn hp_tcm_l2_mem_lp_mode(
        &mut self,
    ) -> HP_TCM_L2_MEM_LP_MODE_W<'_, HP_TCM_MEM_LP_CTRL_SPEC> {
        HP_TCM_L2_MEM_LP_MODE_W::new(self, 8)
    }
    #[doc = "Bit 10 - Set this bit to power down layer2 hp_sram memory."]
    #[inline(always)]
    pub fn hp_tcm_l2_mem_lp_en(&mut self) -> HP_TCM_L2_MEM_LP_EN_W<'_, HP_TCM_MEM_LP_CTRL_SPEC> {
        HP_TCM_L2_MEM_LP_EN_W::new(self, 10)
    }
    #[doc = "Bit 11 - Set this bit to force software control layer2 hp_sram memory, disbale hardware control."]
    #[inline(always)]
    pub fn hp_tcm_mem_l2_lp_force_ctrl(
        &mut self,
    ) -> HP_TCM_MEM_L2_LP_FORCE_CTRL_W<'_, HP_TCM_MEM_LP_CTRL_SPEC> {
        HP_TCM_MEM_L2_LP_FORCE_CTRL_W::new(self, 11)
    }
    #[doc = "Bits 12:13 - Configures layer3 hp_sram memory low power mode in low power stage.\\\\ 0(default): deep sleep\\\\ 1: light sleep\\\\ 2: shut down\\\\ 3: disable low power stage\\\\"]
    #[inline(always)]
    pub fn hp_tcm_l3_mem_lp_mode(
        &mut self,
    ) -> HP_TCM_L3_MEM_LP_MODE_W<'_, HP_TCM_MEM_LP_CTRL_SPEC> {
        HP_TCM_L3_MEM_LP_MODE_W::new(self, 12)
    }
    #[doc = "Bit 14 - Set this bit to power down layer3 hp_sram memory."]
    #[inline(always)]
    pub fn hp_tcm_l3_mem_lp_en(&mut self) -> HP_TCM_L3_MEM_LP_EN_W<'_, HP_TCM_MEM_LP_CTRL_SPEC> {
        HP_TCM_L3_MEM_LP_EN_W::new(self, 14)
    }
    #[doc = "Bit 15 - Set this bit to force software control layer3 hp_sram memory, disbale hardware control."]
    #[inline(always)]
    pub fn hp_tcm_mem_l3_lp_force_ctrl(
        &mut self,
    ) -> HP_TCM_MEM_L3_LP_FORCE_CTRL_W<'_, HP_TCM_MEM_LP_CTRL_SPEC> {
        HP_TCM_MEM_L3_LP_FORCE_CTRL_W::new(self, 15)
    }
}
#[doc = "configure rmemory power in lp system register\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_tcm_mem_lp_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_tcm_mem_lp_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HP_TCM_MEM_LP_CTRL_SPEC;
impl crate::RegisterSpec for HP_TCM_MEM_LP_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hp_tcm_mem_lp_ctrl::R`](R) reader structure"]
impl crate::Readable for HP_TCM_MEM_LP_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hp_tcm_mem_lp_ctrl::W`](W) writer structure"]
impl crate::Writable for HP_TCM_MEM_LP_CTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HP_TCM_MEM_LP_CTRL to value 0"]
impl crate::Resettable for HP_TCM_MEM_LP_CTRL_SPEC {}
