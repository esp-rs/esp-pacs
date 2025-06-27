#[doc = "Register `PRO_IRAM0_2` reader"]
pub type R = crate::R<PRO_IRAM0_2_SPEC>;
#[doc = "Register `PRO_IRAM0_2` writer"]
pub type W = crate::W<PRO_IRAM0_2_SPEC>;
#[doc = "Field `PRO_IRAM0_SRAM_4_SPLTADDR` reader - Configure the split address of SRAM Block 4-21 for IBUS access."]
pub type PRO_IRAM0_SRAM_4_SPLTADDR_R = crate::FieldReader<u32>;
#[doc = "Field `PRO_IRAM0_SRAM_4_SPLTADDR` writer - Configure the split address of SRAM Block 4-21 for IBUS access."]
pub type PRO_IRAM0_SRAM_4_SPLTADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 17, u32>;
#[doc = "Field `PRO_IRAM0_SRAM_4_L_F` reader - Setting to 1 grants IBUS permission to fetch SRAM Block 4-21 low address region."]
pub type PRO_IRAM0_SRAM_4_L_F_R = crate::BitReader;
#[doc = "Field `PRO_IRAM0_SRAM_4_L_F` writer - Setting to 1 grants IBUS permission to fetch SRAM Block 4-21 low address region."]
pub type PRO_IRAM0_SRAM_4_L_F_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRO_IRAM0_SRAM_4_L_R` reader - Setting to 1 grants IBUS permission to read SRAM Block 4-21 low address region."]
pub type PRO_IRAM0_SRAM_4_L_R_R = crate::BitReader;
#[doc = "Field `PRO_IRAM0_SRAM_4_L_R` writer - Setting to 1 grants IBUS permission to read SRAM Block 4-21 low address region."]
pub type PRO_IRAM0_SRAM_4_L_R_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRO_IRAM0_SRAM_4_L_W` reader - Setting to 1 grants IBUS permission to write SRAM Block 4-21 low address region."]
pub type PRO_IRAM0_SRAM_4_L_W_R = crate::BitReader;
#[doc = "Field `PRO_IRAM0_SRAM_4_L_W` writer - Setting to 1 grants IBUS permission to write SRAM Block 4-21 low address region."]
pub type PRO_IRAM0_SRAM_4_L_W_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRO_IRAM0_SRAM_4_H_F` reader - Setting to 1 grants IBUS permission to fetch SRAM Block 4-21 high address region."]
pub type PRO_IRAM0_SRAM_4_H_F_R = crate::BitReader;
#[doc = "Field `PRO_IRAM0_SRAM_4_H_F` writer - Setting to 1 grants IBUS permission to fetch SRAM Block 4-21 high address region."]
pub type PRO_IRAM0_SRAM_4_H_F_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRO_IRAM0_SRAM_4_H_R` reader - Setting to 1 grants IBUS permission to read SRAM Block 4-21 high address region."]
pub type PRO_IRAM0_SRAM_4_H_R_R = crate::BitReader;
#[doc = "Field `PRO_IRAM0_SRAM_4_H_R` writer - Setting to 1 grants IBUS permission to read SRAM Block 4-21 high address region."]
pub type PRO_IRAM0_SRAM_4_H_R_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRO_IRAM0_SRAM_4_H_W` reader - Setting to 1 grants IBUS permission to write SRAM Block 4-21 high address region."]
pub type PRO_IRAM0_SRAM_4_H_W_R = crate::BitReader;
#[doc = "Field `PRO_IRAM0_SRAM_4_H_W` writer - Setting to 1 grants IBUS permission to write SRAM Block 4-21 high address region."]
pub type PRO_IRAM0_SRAM_4_H_W_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:16 - Configure the split address of SRAM Block 4-21 for IBUS access."]
    #[inline(always)]
    pub fn pro_iram0_sram_4_spltaddr(&self) -> PRO_IRAM0_SRAM_4_SPLTADDR_R {
        PRO_IRAM0_SRAM_4_SPLTADDR_R::new(self.bits & 0x0001_ffff)
    }
    #[doc = "Bit 17 - Setting to 1 grants IBUS permission to fetch SRAM Block 4-21 low address region."]
    #[inline(always)]
    pub fn pro_iram0_sram_4_l_f(&self) -> PRO_IRAM0_SRAM_4_L_F_R {
        PRO_IRAM0_SRAM_4_L_F_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Setting to 1 grants IBUS permission to read SRAM Block 4-21 low address region."]
    #[inline(always)]
    pub fn pro_iram0_sram_4_l_r(&self) -> PRO_IRAM0_SRAM_4_L_R_R {
        PRO_IRAM0_SRAM_4_L_R_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Setting to 1 grants IBUS permission to write SRAM Block 4-21 low address region."]
    #[inline(always)]
    pub fn pro_iram0_sram_4_l_w(&self) -> PRO_IRAM0_SRAM_4_L_W_R {
        PRO_IRAM0_SRAM_4_L_W_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Setting to 1 grants IBUS permission to fetch SRAM Block 4-21 high address region."]
    #[inline(always)]
    pub fn pro_iram0_sram_4_h_f(&self) -> PRO_IRAM0_SRAM_4_H_F_R {
        PRO_IRAM0_SRAM_4_H_F_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Setting to 1 grants IBUS permission to read SRAM Block 4-21 high address region."]
    #[inline(always)]
    pub fn pro_iram0_sram_4_h_r(&self) -> PRO_IRAM0_SRAM_4_H_R_R {
        PRO_IRAM0_SRAM_4_H_R_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Setting to 1 grants IBUS permission to write SRAM Block 4-21 high address region."]
    #[inline(always)]
    pub fn pro_iram0_sram_4_h_w(&self) -> PRO_IRAM0_SRAM_4_H_W_R {
        PRO_IRAM0_SRAM_4_H_W_R::new(((self.bits >> 22) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRO_IRAM0_2")
            .field(
                "pro_iram0_sram_4_spltaddr",
                &self.pro_iram0_sram_4_spltaddr(),
            )
            .field("pro_iram0_sram_4_l_f", &self.pro_iram0_sram_4_l_f())
            .field("pro_iram0_sram_4_l_r", &self.pro_iram0_sram_4_l_r())
            .field("pro_iram0_sram_4_l_w", &self.pro_iram0_sram_4_l_w())
            .field("pro_iram0_sram_4_h_f", &self.pro_iram0_sram_4_h_f())
            .field("pro_iram0_sram_4_h_r", &self.pro_iram0_sram_4_h_r())
            .field("pro_iram0_sram_4_h_w", &self.pro_iram0_sram_4_h_w())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:16 - Configure the split address of SRAM Block 4-21 for IBUS access."]
    #[inline(always)]
    pub fn pro_iram0_sram_4_spltaddr(&mut self) -> PRO_IRAM0_SRAM_4_SPLTADDR_W<PRO_IRAM0_2_SPEC> {
        PRO_IRAM0_SRAM_4_SPLTADDR_W::new(self, 0)
    }
    #[doc = "Bit 17 - Setting to 1 grants IBUS permission to fetch SRAM Block 4-21 low address region."]
    #[inline(always)]
    pub fn pro_iram0_sram_4_l_f(&mut self) -> PRO_IRAM0_SRAM_4_L_F_W<PRO_IRAM0_2_SPEC> {
        PRO_IRAM0_SRAM_4_L_F_W::new(self, 17)
    }
    #[doc = "Bit 18 - Setting to 1 grants IBUS permission to read SRAM Block 4-21 low address region."]
    #[inline(always)]
    pub fn pro_iram0_sram_4_l_r(&mut self) -> PRO_IRAM0_SRAM_4_L_R_W<PRO_IRAM0_2_SPEC> {
        PRO_IRAM0_SRAM_4_L_R_W::new(self, 18)
    }
    #[doc = "Bit 19 - Setting to 1 grants IBUS permission to write SRAM Block 4-21 low address region."]
    #[inline(always)]
    pub fn pro_iram0_sram_4_l_w(&mut self) -> PRO_IRAM0_SRAM_4_L_W_W<PRO_IRAM0_2_SPEC> {
        PRO_IRAM0_SRAM_4_L_W_W::new(self, 19)
    }
    #[doc = "Bit 20 - Setting to 1 grants IBUS permission to fetch SRAM Block 4-21 high address region."]
    #[inline(always)]
    pub fn pro_iram0_sram_4_h_f(&mut self) -> PRO_IRAM0_SRAM_4_H_F_W<PRO_IRAM0_2_SPEC> {
        PRO_IRAM0_SRAM_4_H_F_W::new(self, 20)
    }
    #[doc = "Bit 21 - Setting to 1 grants IBUS permission to read SRAM Block 4-21 high address region."]
    #[inline(always)]
    pub fn pro_iram0_sram_4_h_r(&mut self) -> PRO_IRAM0_SRAM_4_H_R_W<PRO_IRAM0_2_SPEC> {
        PRO_IRAM0_SRAM_4_H_R_W::new(self, 21)
    }
    #[doc = "Bit 22 - Setting to 1 grants IBUS permission to write SRAM Block 4-21 high address region."]
    #[inline(always)]
    pub fn pro_iram0_sram_4_h_w(&mut self) -> PRO_IRAM0_SRAM_4_H_W_W<PRO_IRAM0_2_SPEC> {
        PRO_IRAM0_SRAM_4_H_W_W::new(self, 22)
    }
}
#[doc = "IBUS permission control register 2.\n\nYou can [`read`](crate::Reg::read) this register and get [`pro_iram0_2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pro_iram0_2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRO_IRAM0_2_SPEC;
impl crate::RegisterSpec for PRO_IRAM0_2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pro_iram0_2::R`](R) reader structure"]
impl crate::Readable for PRO_IRAM0_2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pro_iram0_2::W`](W) writer structure"]
impl crate::Writable for PRO_IRAM0_2_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRO_IRAM0_2 to value 0x007e_0000"]
impl crate::Resettable for PRO_IRAM0_2_SPEC {
    const RESET_VALUE: u32 = 0x007e_0000;
}
