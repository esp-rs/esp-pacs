#[doc = "Register `RDN_ECO_CS` reader"]
pub type R = crate::R<RDN_ECO_CS_SPEC>;
#[doc = "Register `RDN_ECO_CS` writer"]
pub type W = crate::W<RDN_ECO_CS_SPEC>;
#[doc = "Field `REG_HP_SYS_RDN_ECO_EN` reader - NA"]
pub type REG_HP_SYS_RDN_ECO_EN_R = crate::BitReader;
#[doc = "Field `REG_HP_SYS_RDN_ECO_EN` writer - NA"]
pub type REG_HP_SYS_RDN_ECO_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_HP_SYS_RDN_ECO_RESULT` reader - NA"]
pub type REG_HP_SYS_RDN_ECO_RESULT_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn reg_hp_sys_rdn_eco_en(&self) -> REG_HP_SYS_RDN_ECO_EN_R {
        REG_HP_SYS_RDN_ECO_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - NA"]
    #[inline(always)]
    pub fn reg_hp_sys_rdn_eco_result(&self) -> REG_HP_SYS_RDN_ECO_RESULT_R {
        REG_HP_SYS_RDN_ECO_RESULT_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RDN_ECO_CS")
            .field("reg_hp_sys_rdn_eco_en", &self.reg_hp_sys_rdn_eco_en())
            .field(
                "reg_hp_sys_rdn_eco_result",
                &self.reg_hp_sys_rdn_eco_result(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn reg_hp_sys_rdn_eco_en(&mut self) -> REG_HP_SYS_RDN_ECO_EN_W<RDN_ECO_CS_SPEC> {
        REG_HP_SYS_RDN_ECO_EN_W::new(self, 0)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`rdn_eco_cs::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rdn_eco_cs::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RDN_ECO_CS_SPEC;
impl crate::RegisterSpec for RDN_ECO_CS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rdn_eco_cs::R`](R) reader structure"]
impl crate::Readable for RDN_ECO_CS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rdn_eco_cs::W`](W) writer structure"]
impl crate::Writable for RDN_ECO_CS_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RDN_ECO_CS to value 0"]
impl crate::Resettable for RDN_ECO_CS_SPEC {
    const RESET_VALUE: u32 = 0;
}
