#[doc = "Register `PRO_DRAM0_1` reader"]
pub type R = crate::R<PRO_DRAM0_1_SPEC>;
#[doc = "Register `PRO_DRAM0_1` writer"]
pub type W = crate::W<PRO_DRAM0_1_SPEC>;
#[doc = "Field `PRO_DRAM0_SRAM_0_R` reader - Setting to 1 grants DBUS0 permission to read SRAM Block 0."]
pub type PRO_DRAM0_SRAM_0_R_R = crate::BitReader;
#[doc = "Field `PRO_DRAM0_SRAM_0_R` writer - Setting to 1 grants DBUS0 permission to read SRAM Block 0."]
pub type PRO_DRAM0_SRAM_0_R_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRO_DRAM0_SRAM_0_W` reader - Setting to 1 grants DBUS0 permission to write SRAM Block 0."]
pub type PRO_DRAM0_SRAM_0_W_R = crate::BitReader;
#[doc = "Field `PRO_DRAM0_SRAM_0_W` writer - Setting to 1 grants DBUS0 permission to write SRAM Block 0."]
pub type PRO_DRAM0_SRAM_0_W_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRO_DRAM0_SRAM_1_R` reader - Setting to 1 grants DBUS0 permission to read SRAM Block 1."]
pub type PRO_DRAM0_SRAM_1_R_R = crate::BitReader;
#[doc = "Field `PRO_DRAM0_SRAM_1_R` writer - Setting to 1 grants DBUS0 permission to read SRAM Block 1."]
pub type PRO_DRAM0_SRAM_1_R_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRO_DRAM0_SRAM_1_W` reader - Setting to 1 grants DBUS0 permission to write SRAM Block 1."]
pub type PRO_DRAM0_SRAM_1_W_R = crate::BitReader;
#[doc = "Field `PRO_DRAM0_SRAM_1_W` writer - Setting to 1 grants DBUS0 permission to write SRAM Block 1."]
pub type PRO_DRAM0_SRAM_1_W_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRO_DRAM0_SRAM_2_R` reader - Setting to 1 grants DBUS0 permission to read SRAM Block 2."]
pub type PRO_DRAM0_SRAM_2_R_R = crate::BitReader;
#[doc = "Field `PRO_DRAM0_SRAM_2_R` writer - Setting to 1 grants DBUS0 permission to read SRAM Block 2."]
pub type PRO_DRAM0_SRAM_2_R_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRO_DRAM0_SRAM_2_W` reader - Setting to 1 grants DBUS0 permission to write SRAM Block 2."]
pub type PRO_DRAM0_SRAM_2_W_R = crate::BitReader;
#[doc = "Field `PRO_DRAM0_SRAM_2_W` writer - Setting to 1 grants DBUS0 permission to write SRAM Block 2."]
pub type PRO_DRAM0_SRAM_2_W_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRO_DRAM0_SRAM_3_R` reader - Setting to 1 grants DBUS0 permission to read SRAM Block 3."]
pub type PRO_DRAM0_SRAM_3_R_R = crate::BitReader;
#[doc = "Field `PRO_DRAM0_SRAM_3_R` writer - Setting to 1 grants DBUS0 permission to read SRAM Block 3."]
pub type PRO_DRAM0_SRAM_3_R_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRO_DRAM0_SRAM_3_W` reader - Setting to 1 grants DBUS0 permission to write SRAM Block 3."]
pub type PRO_DRAM0_SRAM_3_W_R = crate::BitReader;
#[doc = "Field `PRO_DRAM0_SRAM_3_W` writer - Setting to 1 grants DBUS0 permission to write SRAM Block 3."]
pub type PRO_DRAM0_SRAM_3_W_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRO_DRAM0_SRAM_4_SPLTADDR` reader - Configure the split address of SRAM Block 4-21 for DBUS0 access."]
pub type PRO_DRAM0_SRAM_4_SPLTADDR_R = crate::FieldReader<u32>;
#[doc = "Field `PRO_DRAM0_SRAM_4_SPLTADDR` writer - Configure the split address of SRAM Block 4-21 for DBUS0 access."]
pub type PRO_DRAM0_SRAM_4_SPLTADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 17, u32>;
#[doc = "Field `PRO_DRAM0_SRAM_4_L_R` reader - Setting to 1 grants DBUS0 permission to read SRAM Block 4-21 low address region."]
pub type PRO_DRAM0_SRAM_4_L_R_R = crate::BitReader;
#[doc = "Field `PRO_DRAM0_SRAM_4_L_R` writer - Setting to 1 grants DBUS0 permission to read SRAM Block 4-21 low address region."]
pub type PRO_DRAM0_SRAM_4_L_R_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRO_DRAM0_SRAM_4_L_W` reader - Setting to 1 grants DBUS0 permission to write SRAM Block 4-21 low address region."]
pub type PRO_DRAM0_SRAM_4_L_W_R = crate::BitReader;
#[doc = "Field `PRO_DRAM0_SRAM_4_L_W` writer - Setting to 1 grants DBUS0 permission to write SRAM Block 4-21 low address region."]
pub type PRO_DRAM0_SRAM_4_L_W_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRO_DRAM0_SRAM_4_H_R` reader - Setting to 1 grants DBUS0 permission to read SRAM Block 4-21 high address region."]
pub type PRO_DRAM0_SRAM_4_H_R_R = crate::BitReader;
#[doc = "Field `PRO_DRAM0_SRAM_4_H_R` writer - Setting to 1 grants DBUS0 permission to read SRAM Block 4-21 high address region."]
pub type PRO_DRAM0_SRAM_4_H_R_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRO_DRAM0_SRAM_4_H_W` reader - Setting to 1 grants DBUS0 permission to write SRAM Block 4-21 high address region."]
pub type PRO_DRAM0_SRAM_4_H_W_R = crate::BitReader;
#[doc = "Field `PRO_DRAM0_SRAM_4_H_W` writer - Setting to 1 grants DBUS0 permission to write SRAM Block 4-21 high address region."]
pub type PRO_DRAM0_SRAM_4_H_W_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Setting to 1 grants DBUS0 permission to read SRAM Block 0."]
    #[inline(always)]
    pub fn pro_dram0_sram_0_r(&self) -> PRO_DRAM0_SRAM_0_R_R {
        PRO_DRAM0_SRAM_0_R_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Setting to 1 grants DBUS0 permission to write SRAM Block 0."]
    #[inline(always)]
    pub fn pro_dram0_sram_0_w(&self) -> PRO_DRAM0_SRAM_0_W_R {
        PRO_DRAM0_SRAM_0_W_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Setting to 1 grants DBUS0 permission to read SRAM Block 1."]
    #[inline(always)]
    pub fn pro_dram0_sram_1_r(&self) -> PRO_DRAM0_SRAM_1_R_R {
        PRO_DRAM0_SRAM_1_R_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Setting to 1 grants DBUS0 permission to write SRAM Block 1."]
    #[inline(always)]
    pub fn pro_dram0_sram_1_w(&self) -> PRO_DRAM0_SRAM_1_W_R {
        PRO_DRAM0_SRAM_1_W_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Setting to 1 grants DBUS0 permission to read SRAM Block 2."]
    #[inline(always)]
    pub fn pro_dram0_sram_2_r(&self) -> PRO_DRAM0_SRAM_2_R_R {
        PRO_DRAM0_SRAM_2_R_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Setting to 1 grants DBUS0 permission to write SRAM Block 2."]
    #[inline(always)]
    pub fn pro_dram0_sram_2_w(&self) -> PRO_DRAM0_SRAM_2_W_R {
        PRO_DRAM0_SRAM_2_W_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Setting to 1 grants DBUS0 permission to read SRAM Block 3."]
    #[inline(always)]
    pub fn pro_dram0_sram_3_r(&self) -> PRO_DRAM0_SRAM_3_R_R {
        PRO_DRAM0_SRAM_3_R_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Setting to 1 grants DBUS0 permission to write SRAM Block 3."]
    #[inline(always)]
    pub fn pro_dram0_sram_3_w(&self) -> PRO_DRAM0_SRAM_3_W_R {
        PRO_DRAM0_SRAM_3_W_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:24 - Configure the split address of SRAM Block 4-21 for DBUS0 access."]
    #[inline(always)]
    pub fn pro_dram0_sram_4_spltaddr(&self) -> PRO_DRAM0_SRAM_4_SPLTADDR_R {
        PRO_DRAM0_SRAM_4_SPLTADDR_R::new((self.bits >> 8) & 0x0001_ffff)
    }
    #[doc = "Bit 25 - Setting to 1 grants DBUS0 permission to read SRAM Block 4-21 low address region."]
    #[inline(always)]
    pub fn pro_dram0_sram_4_l_r(&self) -> PRO_DRAM0_SRAM_4_L_R_R {
        PRO_DRAM0_SRAM_4_L_R_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Setting to 1 grants DBUS0 permission to write SRAM Block 4-21 low address region."]
    #[inline(always)]
    pub fn pro_dram0_sram_4_l_w(&self) -> PRO_DRAM0_SRAM_4_L_W_R {
        PRO_DRAM0_SRAM_4_L_W_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Setting to 1 grants DBUS0 permission to read SRAM Block 4-21 high address region."]
    #[inline(always)]
    pub fn pro_dram0_sram_4_h_r(&self) -> PRO_DRAM0_SRAM_4_H_R_R {
        PRO_DRAM0_SRAM_4_H_R_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Setting to 1 grants DBUS0 permission to write SRAM Block 4-21 high address region."]
    #[inline(always)]
    pub fn pro_dram0_sram_4_h_w(&self) -> PRO_DRAM0_SRAM_4_H_W_R {
        PRO_DRAM0_SRAM_4_H_W_R::new(((self.bits >> 28) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRO_DRAM0_1")
            .field("pro_dram0_sram_0_r", &self.pro_dram0_sram_0_r())
            .field("pro_dram0_sram_0_w", &self.pro_dram0_sram_0_w())
            .field("pro_dram0_sram_1_r", &self.pro_dram0_sram_1_r())
            .field("pro_dram0_sram_1_w", &self.pro_dram0_sram_1_w())
            .field("pro_dram0_sram_2_r", &self.pro_dram0_sram_2_r())
            .field("pro_dram0_sram_2_w", &self.pro_dram0_sram_2_w())
            .field("pro_dram0_sram_3_r", &self.pro_dram0_sram_3_r())
            .field("pro_dram0_sram_3_w", &self.pro_dram0_sram_3_w())
            .field(
                "pro_dram0_sram_4_spltaddr",
                &self.pro_dram0_sram_4_spltaddr(),
            )
            .field("pro_dram0_sram_4_l_r", &self.pro_dram0_sram_4_l_r())
            .field("pro_dram0_sram_4_l_w", &self.pro_dram0_sram_4_l_w())
            .field("pro_dram0_sram_4_h_r", &self.pro_dram0_sram_4_h_r())
            .field("pro_dram0_sram_4_h_w", &self.pro_dram0_sram_4_h_w())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Setting to 1 grants DBUS0 permission to read SRAM Block 0."]
    #[inline(always)]
    #[must_use]
    pub fn pro_dram0_sram_0_r(&mut self) -> PRO_DRAM0_SRAM_0_R_W<PRO_DRAM0_1_SPEC> {
        PRO_DRAM0_SRAM_0_R_W::new(self, 0)
    }
    #[doc = "Bit 1 - Setting to 1 grants DBUS0 permission to write SRAM Block 0."]
    #[inline(always)]
    #[must_use]
    pub fn pro_dram0_sram_0_w(&mut self) -> PRO_DRAM0_SRAM_0_W_W<PRO_DRAM0_1_SPEC> {
        PRO_DRAM0_SRAM_0_W_W::new(self, 1)
    }
    #[doc = "Bit 2 - Setting to 1 grants DBUS0 permission to read SRAM Block 1."]
    #[inline(always)]
    #[must_use]
    pub fn pro_dram0_sram_1_r(&mut self) -> PRO_DRAM0_SRAM_1_R_W<PRO_DRAM0_1_SPEC> {
        PRO_DRAM0_SRAM_1_R_W::new(self, 2)
    }
    #[doc = "Bit 3 - Setting to 1 grants DBUS0 permission to write SRAM Block 1."]
    #[inline(always)]
    #[must_use]
    pub fn pro_dram0_sram_1_w(&mut self) -> PRO_DRAM0_SRAM_1_W_W<PRO_DRAM0_1_SPEC> {
        PRO_DRAM0_SRAM_1_W_W::new(self, 3)
    }
    #[doc = "Bit 4 - Setting to 1 grants DBUS0 permission to read SRAM Block 2."]
    #[inline(always)]
    #[must_use]
    pub fn pro_dram0_sram_2_r(&mut self) -> PRO_DRAM0_SRAM_2_R_W<PRO_DRAM0_1_SPEC> {
        PRO_DRAM0_SRAM_2_R_W::new(self, 4)
    }
    #[doc = "Bit 5 - Setting to 1 grants DBUS0 permission to write SRAM Block 2."]
    #[inline(always)]
    #[must_use]
    pub fn pro_dram0_sram_2_w(&mut self) -> PRO_DRAM0_SRAM_2_W_W<PRO_DRAM0_1_SPEC> {
        PRO_DRAM0_SRAM_2_W_W::new(self, 5)
    }
    #[doc = "Bit 6 - Setting to 1 grants DBUS0 permission to read SRAM Block 3."]
    #[inline(always)]
    #[must_use]
    pub fn pro_dram0_sram_3_r(&mut self) -> PRO_DRAM0_SRAM_3_R_W<PRO_DRAM0_1_SPEC> {
        PRO_DRAM0_SRAM_3_R_W::new(self, 6)
    }
    #[doc = "Bit 7 - Setting to 1 grants DBUS0 permission to write SRAM Block 3."]
    #[inline(always)]
    #[must_use]
    pub fn pro_dram0_sram_3_w(&mut self) -> PRO_DRAM0_SRAM_3_W_W<PRO_DRAM0_1_SPEC> {
        PRO_DRAM0_SRAM_3_W_W::new(self, 7)
    }
    #[doc = "Bits 8:24 - Configure the split address of SRAM Block 4-21 for DBUS0 access."]
    #[inline(always)]
    #[must_use]
    pub fn pro_dram0_sram_4_spltaddr(&mut self) -> PRO_DRAM0_SRAM_4_SPLTADDR_W<PRO_DRAM0_1_SPEC> {
        PRO_DRAM0_SRAM_4_SPLTADDR_W::new(self, 8)
    }
    #[doc = "Bit 25 - Setting to 1 grants DBUS0 permission to read SRAM Block 4-21 low address region."]
    #[inline(always)]
    #[must_use]
    pub fn pro_dram0_sram_4_l_r(&mut self) -> PRO_DRAM0_SRAM_4_L_R_W<PRO_DRAM0_1_SPEC> {
        PRO_DRAM0_SRAM_4_L_R_W::new(self, 25)
    }
    #[doc = "Bit 26 - Setting to 1 grants DBUS0 permission to write SRAM Block 4-21 low address region."]
    #[inline(always)]
    #[must_use]
    pub fn pro_dram0_sram_4_l_w(&mut self) -> PRO_DRAM0_SRAM_4_L_W_W<PRO_DRAM0_1_SPEC> {
        PRO_DRAM0_SRAM_4_L_W_W::new(self, 26)
    }
    #[doc = "Bit 27 - Setting to 1 grants DBUS0 permission to read SRAM Block 4-21 high address region."]
    #[inline(always)]
    #[must_use]
    pub fn pro_dram0_sram_4_h_r(&mut self) -> PRO_DRAM0_SRAM_4_H_R_W<PRO_DRAM0_1_SPEC> {
        PRO_DRAM0_SRAM_4_H_R_W::new(self, 27)
    }
    #[doc = "Bit 28 - Setting to 1 grants DBUS0 permission to write SRAM Block 4-21 high address region."]
    #[inline(always)]
    #[must_use]
    pub fn pro_dram0_sram_4_h_w(&mut self) -> PRO_DRAM0_SRAM_4_H_W_W<PRO_DRAM0_1_SPEC> {
        PRO_DRAM0_SRAM_4_H_W_W::new(self, 28)
    }
}
#[doc = "DBUS permission control register 1.\n\nYou can [`read`](crate::Reg::read) this register and get [`pro_dram0_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pro_dram0_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRO_DRAM0_1_SPEC;
impl crate::RegisterSpec for PRO_DRAM0_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pro_dram0_1::R`](R) reader structure"]
impl crate::Readable for PRO_DRAM0_1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pro_dram0_1::W`](W) writer structure"]
impl crate::Writable for PRO_DRAM0_1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PRO_DRAM0_1 to value 0x1e00_00ff"]
impl crate::Resettable for PRO_DRAM0_1_SPEC {
    const RESET_VALUE: u32 = 0x1e00_00ff;
}
