#[doc = "Register `MEM_TEST_CONF` reader"]
pub type R = crate::R<MEM_TEST_CONF_SPEC>;
#[doc = "Register `MEM_TEST_CONF` writer"]
pub type W = crate::W<MEM_TEST_CONF_SPEC>;
#[doc = "Field `HP_MEM_WPULSE` reader - This field controls hp system memory WPULSE parameter. 0b000 for 1.1V/1.0V/0.9V operating Voltage."]
pub type HP_MEM_WPULSE_R = crate::FieldReader;
#[doc = "Field `HP_MEM_WPULSE` writer - This field controls hp system memory WPULSE parameter. 0b000 for 1.1V/1.0V/0.9V operating Voltage."]
pub type HP_MEM_WPULSE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `HP_MEM_WA` reader - This field controls hp system memory WA parameter. 0b100 for 1.1V operating Voltage, 0b101 for 1.0V, 0b110 for 0.9V."]
pub type HP_MEM_WA_R = crate::FieldReader;
#[doc = "Field `HP_MEM_WA` writer - This field controls hp system memory WA parameter. 0b100 for 1.1V operating Voltage, 0b101 for 1.0V, 0b110 for 0.9V."]
pub type HP_MEM_WA_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `HP_MEM_RA` reader - This field controls hp system memory RA parameter. 0b00 for 1.1V/1.0V operating Voltage, 0b01 for 0.9V."]
pub type HP_MEM_RA_R = crate::FieldReader;
#[doc = "Field `HP_MEM_RA` writer - This field controls hp system memory RA parameter. 0b00 for 1.1V/1.0V operating Voltage, 0b01 for 0.9V."]
pub type HP_MEM_RA_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `HP_MEM_RM` reader - This field controls hp system memory RM parameter. 0b0011 for 1.1V operating Voltage, 0b0010 for 1.0V, 0b0000 for 0.9V."]
pub type HP_MEM_RM_R = crate::FieldReader;
#[doc = "Field `HP_MEM_RM` writer - This field controls hp system memory RM parameter. 0b0011 for 1.1V operating Voltage, 0b0010 for 1.0V, 0b0000 for 0.9V."]
pub type HP_MEM_RM_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ROM_RM` reader - This field controls rom RM parameter. 0b0011 for 1.1V operating Voltage, 0b0010 for 1.0V, 0b0010(default) or 0b0001(slow) for 0.9V."]
pub type ROM_RM_R = crate::FieldReader;
#[doc = "Field `ROM_RM` writer - This field controls rom RM parameter. 0b0011 for 1.1V operating Voltage, 0b0010 for 1.0V, 0b0010(default) or 0b0001(slow) for 0.9V."]
pub type ROM_RM_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:2 - This field controls hp system memory WPULSE parameter. 0b000 for 1.1V/1.0V/0.9V operating Voltage."]
    #[inline(always)]
    pub fn hp_mem_wpulse(&self) -> HP_MEM_WPULSE_R {
        HP_MEM_WPULSE_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - This field controls hp system memory WA parameter. 0b100 for 1.1V operating Voltage, 0b101 for 1.0V, 0b110 for 0.9V."]
    #[inline(always)]
    pub fn hp_mem_wa(&self) -> HP_MEM_WA_R {
        HP_MEM_WA_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:7 - This field controls hp system memory RA parameter. 0b00 for 1.1V/1.0V operating Voltage, 0b01 for 0.9V."]
    #[inline(always)]
    pub fn hp_mem_ra(&self) -> HP_MEM_RA_R {
        HP_MEM_RA_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:11 - This field controls hp system memory RM parameter. 0b0011 for 1.1V operating Voltage, 0b0010 for 1.0V, 0b0000 for 0.9V."]
    #[inline(always)]
    pub fn hp_mem_rm(&self) -> HP_MEM_RM_R {
        HP_MEM_RM_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - This field controls rom RM parameter. 0b0011 for 1.1V operating Voltage, 0b0010 for 1.0V, 0b0010(default) or 0b0001(slow) for 0.9V."]
    #[inline(always)]
    pub fn rom_rm(&self) -> ROM_RM_R {
        ROM_RM_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MEM_TEST_CONF")
            .field("hp_mem_wpulse", &self.hp_mem_wpulse())
            .field("hp_mem_wa", &self.hp_mem_wa())
            .field("hp_mem_ra", &self.hp_mem_ra())
            .field("hp_mem_rm", &self.hp_mem_rm())
            .field("rom_rm", &self.rom_rm())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - This field controls hp system memory WPULSE parameter. 0b000 for 1.1V/1.0V/0.9V operating Voltage."]
    #[inline(always)]
    pub fn hp_mem_wpulse(&mut self) -> HP_MEM_WPULSE_W<MEM_TEST_CONF_SPEC> {
        HP_MEM_WPULSE_W::new(self, 0)
    }
    #[doc = "Bits 3:5 - This field controls hp system memory WA parameter. 0b100 for 1.1V operating Voltage, 0b101 for 1.0V, 0b110 for 0.9V."]
    #[inline(always)]
    pub fn hp_mem_wa(&mut self) -> HP_MEM_WA_W<MEM_TEST_CONF_SPEC> {
        HP_MEM_WA_W::new(self, 3)
    }
    #[doc = "Bits 6:7 - This field controls hp system memory RA parameter. 0b00 for 1.1V/1.0V operating Voltage, 0b01 for 0.9V."]
    #[inline(always)]
    pub fn hp_mem_ra(&mut self) -> HP_MEM_RA_W<MEM_TEST_CONF_SPEC> {
        HP_MEM_RA_W::new(self, 6)
    }
    #[doc = "Bits 8:11 - This field controls hp system memory RM parameter. 0b0011 for 1.1V operating Voltage, 0b0010 for 1.0V, 0b0000 for 0.9V."]
    #[inline(always)]
    pub fn hp_mem_rm(&mut self) -> HP_MEM_RM_W<MEM_TEST_CONF_SPEC> {
        HP_MEM_RM_W::new(self, 8)
    }
    #[doc = "Bits 12:15 - This field controls rom RM parameter. 0b0011 for 1.1V operating Voltage, 0b0010 for 1.0V, 0b0010(default) or 0b0001(slow) for 0.9V."]
    #[inline(always)]
    pub fn rom_rm(&mut self) -> ROM_RM_W<MEM_TEST_CONF_SPEC> {
        ROM_RM_W::new(self, 12)
    }
}
#[doc = "MEM_TEST configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`mem_test_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mem_test_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MEM_TEST_CONF_SPEC;
impl crate::RegisterSpec for MEM_TEST_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_test_conf::R`](R) reader structure"]
impl crate::Readable for MEM_TEST_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mem_test_conf::W`](W) writer structure"]
impl crate::Writable for MEM_TEST_CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEM_TEST_CONF to value 0x2228"]
impl crate::Resettable for MEM_TEST_CONF_SPEC {
    const RESET_VALUE: u32 = 0x2228;
}
