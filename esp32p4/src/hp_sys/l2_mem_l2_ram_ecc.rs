#[doc = "Register `L2_MEM_L2_RAM_ECC` reader"]
pub type R = crate::R<L2_MEM_L2_RAM_ECC_SPEC>;
#[doc = "Register `L2_MEM_L2_RAM_ECC` writer"]
pub type W = crate::W<L2_MEM_L2_RAM_ECC_SPEC>;
#[doc = "Field `REG_L2_RAM_UNIT0_ECC_EN` reader - NA"]
pub type REG_L2_RAM_UNIT0_ECC_EN_R = crate::BitReader;
#[doc = "Field `REG_L2_RAM_UNIT0_ECC_EN` writer - NA"]
pub type REG_L2_RAM_UNIT0_ECC_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_L2_RAM_UNIT1_ECC_EN` reader - NA"]
pub type REG_L2_RAM_UNIT1_ECC_EN_R = crate::BitReader;
#[doc = "Field `REG_L2_RAM_UNIT1_ECC_EN` writer - NA"]
pub type REG_L2_RAM_UNIT1_ECC_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_L2_RAM_UNIT2_ECC_EN` reader - NA"]
pub type REG_L2_RAM_UNIT2_ECC_EN_R = crate::BitReader;
#[doc = "Field `REG_L2_RAM_UNIT2_ECC_EN` writer - NA"]
pub type REG_L2_RAM_UNIT2_ECC_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_L2_RAM_UNIT3_ECC_EN` reader - NA"]
pub type REG_L2_RAM_UNIT3_ECC_EN_R = crate::BitReader;
#[doc = "Field `REG_L2_RAM_UNIT3_ECC_EN` writer - NA"]
pub type REG_L2_RAM_UNIT3_ECC_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_L2_RAM_UNIT4_ECC_EN` reader - NA"]
pub type REG_L2_RAM_UNIT4_ECC_EN_R = crate::BitReader;
#[doc = "Field `REG_L2_RAM_UNIT4_ECC_EN` writer - NA"]
pub type REG_L2_RAM_UNIT4_ECC_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_L2_RAM_UNIT5_ECC_EN` reader - NA"]
pub type REG_L2_RAM_UNIT5_ECC_EN_R = crate::BitReader;
#[doc = "Field `REG_L2_RAM_UNIT5_ECC_EN` writer - NA"]
pub type REG_L2_RAM_UNIT5_ECC_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn reg_l2_ram_unit0_ecc_en(&self) -> REG_L2_RAM_UNIT0_ECC_EN_R {
        REG_L2_RAM_UNIT0_ECC_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - NA"]
    #[inline(always)]
    pub fn reg_l2_ram_unit1_ecc_en(&self) -> REG_L2_RAM_UNIT1_ECC_EN_R {
        REG_L2_RAM_UNIT1_ECC_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - NA"]
    #[inline(always)]
    pub fn reg_l2_ram_unit2_ecc_en(&self) -> REG_L2_RAM_UNIT2_ECC_EN_R {
        REG_L2_RAM_UNIT2_ECC_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - NA"]
    #[inline(always)]
    pub fn reg_l2_ram_unit3_ecc_en(&self) -> REG_L2_RAM_UNIT3_ECC_EN_R {
        REG_L2_RAM_UNIT3_ECC_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - NA"]
    #[inline(always)]
    pub fn reg_l2_ram_unit4_ecc_en(&self) -> REG_L2_RAM_UNIT4_ECC_EN_R {
        REG_L2_RAM_UNIT4_ECC_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - NA"]
    #[inline(always)]
    pub fn reg_l2_ram_unit5_ecc_en(&self) -> REG_L2_RAM_UNIT5_ECC_EN_R {
        REG_L2_RAM_UNIT5_ECC_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L2_MEM_L2_RAM_ECC")
            .field("reg_l2_ram_unit0_ecc_en", &self.reg_l2_ram_unit0_ecc_en())
            .field("reg_l2_ram_unit1_ecc_en", &self.reg_l2_ram_unit1_ecc_en())
            .field("reg_l2_ram_unit2_ecc_en", &self.reg_l2_ram_unit2_ecc_en())
            .field("reg_l2_ram_unit3_ecc_en", &self.reg_l2_ram_unit3_ecc_en())
            .field("reg_l2_ram_unit4_ecc_en", &self.reg_l2_ram_unit4_ecc_en())
            .field("reg_l2_ram_unit5_ecc_en", &self.reg_l2_ram_unit5_ecc_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn reg_l2_ram_unit0_ecc_en(&mut self) -> REG_L2_RAM_UNIT0_ECC_EN_W<L2_MEM_L2_RAM_ECC_SPEC> {
        REG_L2_RAM_UNIT0_ECC_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - NA"]
    #[inline(always)]
    pub fn reg_l2_ram_unit1_ecc_en(&mut self) -> REG_L2_RAM_UNIT1_ECC_EN_W<L2_MEM_L2_RAM_ECC_SPEC> {
        REG_L2_RAM_UNIT1_ECC_EN_W::new(self, 1)
    }
    #[doc = "Bit 2 - NA"]
    #[inline(always)]
    pub fn reg_l2_ram_unit2_ecc_en(&mut self) -> REG_L2_RAM_UNIT2_ECC_EN_W<L2_MEM_L2_RAM_ECC_SPEC> {
        REG_L2_RAM_UNIT2_ECC_EN_W::new(self, 2)
    }
    #[doc = "Bit 3 - NA"]
    #[inline(always)]
    pub fn reg_l2_ram_unit3_ecc_en(&mut self) -> REG_L2_RAM_UNIT3_ECC_EN_W<L2_MEM_L2_RAM_ECC_SPEC> {
        REG_L2_RAM_UNIT3_ECC_EN_W::new(self, 3)
    }
    #[doc = "Bit 4 - NA"]
    #[inline(always)]
    pub fn reg_l2_ram_unit4_ecc_en(&mut self) -> REG_L2_RAM_UNIT4_ECC_EN_W<L2_MEM_L2_RAM_ECC_SPEC> {
        REG_L2_RAM_UNIT4_ECC_EN_W::new(self, 4)
    }
    #[doc = "Bit 5 - NA"]
    #[inline(always)]
    pub fn reg_l2_ram_unit5_ecc_en(&mut self) -> REG_L2_RAM_UNIT5_ECC_EN_W<L2_MEM_L2_RAM_ECC_SPEC> {
        REG_L2_RAM_UNIT5_ECC_EN_W::new(self, 5)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`l2_mem_l2_ram_ecc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l2_mem_l2_ram_ecc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L2_MEM_L2_RAM_ECC_SPEC;
impl crate::RegisterSpec for L2_MEM_L2_RAM_ECC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l2_mem_l2_ram_ecc::R`](R) reader structure"]
impl crate::Readable for L2_MEM_L2_RAM_ECC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`l2_mem_l2_ram_ecc::W`](W) writer structure"]
impl crate::Writable for L2_MEM_L2_RAM_ECC_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets L2_MEM_L2_RAM_ECC to value 0"]
impl crate::Resettable for L2_MEM_L2_RAM_ECC_SPEC {}
