#[doc = "Register `CONF` reader"]
pub type R = crate::R<CONF_SPEC>;
#[doc = "Register `CONF` writer"]
pub type W = crate::W<CONF_SPEC>;
#[doc = "Field `CLK_EN` reader - Configures whether or not to open register clock gate.\\\\0: Open the clock gate only when application writes registers\\\\1: Force open the clock gate for register"]
pub type CLK_EN_R = crate::BitReader;
#[doc = "Field `CLK_EN` writer - Configures whether or not to open register clock gate.\\\\0: Open the clock gate only when application writes registers\\\\1: Force open the clock gate for register"]
pub type CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REC_RAM_CLK_EN2` reader - Configures whether or not to open the clock gate for rec ram2.\\\\0: Open the clock gate only when application writes or reads rec ram2\\\\1: Force open the clock gate for rec ram2"]
pub type REC_RAM_CLK_EN2_R = crate::BitReader;
#[doc = "Field `REC_RAM_CLK_EN2` writer - Configures whether or not to open the clock gate for rec ram2.\\\\0: Open the clock gate only when application writes or reads rec ram2\\\\1: Force open the clock gate for rec ram2"]
pub type REC_RAM_CLK_EN2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REC_RAM_CLK_EN1` reader - Configures whether or not to open the clock gate for rec ram1.\\\\0: Open the clock gate only when application writes or reads rec ram1\\\\1: Force open the clock gate for rec ram1"]
pub type REC_RAM_CLK_EN1_R = crate::BitReader;
#[doc = "Field `REC_RAM_CLK_EN1` writer - Configures whether or not to open the clock gate for rec ram1.\\\\0: Open the clock gate only when application writes or reads rec ram1\\\\1: Force open the clock gate for rec ram1"]
pub type REC_RAM_CLK_EN1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `QUANT_RAM_CLK_EN2` reader - Configures whether or not to open the clock gate for quant ram2.\\\\0: Open the clock gate only when application writes or reads quant ram2\\\\1: Force open the clock gate for quant ram2"]
pub type QUANT_RAM_CLK_EN2_R = crate::BitReader;
#[doc = "Field `QUANT_RAM_CLK_EN2` writer - Configures whether or not to open the clock gate for quant ram2.\\\\0: Open the clock gate only when application writes or reads quant ram2\\\\1: Force open the clock gate for quant ram2"]
pub type QUANT_RAM_CLK_EN2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `QUANT_RAM_CLK_EN1` reader - Configures whether or not to open the clock gate for quant ram1.\\\\0: Open the clock gate only when application writes or reads quant ram1\\\\1: Force open the clock gate for quant ram1"]
pub type QUANT_RAM_CLK_EN1_R = crate::BitReader;
#[doc = "Field `QUANT_RAM_CLK_EN1` writer - Configures whether or not to open the clock gate for quant ram1.\\\\0: Open the clock gate only when application writes or reads quant ram1\\\\1: Force open the clock gate for quant ram1"]
pub type QUANT_RAM_CLK_EN1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRE_RAM_CLK_EN` reader - Configures whether or not to open the clock gate for pre ram.\\\\0: Open the clock gate only when application writes or reads pre ram\\\\1: Force open the clock gate for pre ram"]
pub type PRE_RAM_CLK_EN_R = crate::BitReader;
#[doc = "Field `PRE_RAM_CLK_EN` writer - Configures whether or not to open the clock gate for pre ram.\\\\0: Open the clock gate only when application writes or reads pre ram\\\\1: Force open the clock gate for pre ram"]
pub type PRE_RAM_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MVD_RAM_CLK_EN` reader - Configures whether or not to open the clock gate for mvd ram.\\\\0: Open the clock gate only when application writes or reads mvd ram\\\\1: Force open the clock gate for mvd ram"]
pub type MVD_RAM_CLK_EN_R = crate::BitReader;
#[doc = "Field `MVD_RAM_CLK_EN` writer - Configures whether or not to open the clock gate for mvd ram.\\\\0: Open the clock gate only when application writes or reads mvd ram\\\\1: Force open the clock gate for mvd ram"]
pub type MVD_RAM_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MC_RAM_CLK_EN` reader - Configures whether or not to open the clock gate for mc ram.\\\\0: Open the clock gate only when application writes or reads mc ram\\\\1: Force open the clock gate for mc ram"]
pub type MC_RAM_CLK_EN_R = crate::BitReader;
#[doc = "Field `MC_RAM_CLK_EN` writer - Configures whether or not to open the clock gate for mc ram.\\\\0: Open the clock gate only when application writes or reads mc ram\\\\1: Force open the clock gate for mc ram"]
pub type MC_RAM_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REF_RAM_CLK_EN` reader - Configures whether or not to open the clock gate for ref ram.\\\\0: Open the clock gate only when application writes or reads ref ram\\\\1: Force open the clock gate for ref ram"]
pub type REF_RAM_CLK_EN_R = crate::BitReader;
#[doc = "Field `REF_RAM_CLK_EN` writer - Configures whether or not to open the clock gate for ref ram.\\\\0: Open the clock gate only when application writes or reads ref ram\\\\1: Force open the clock gate for ref ram"]
pub type REF_RAM_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I4X4_REF_RAM_CLK_EN` reader - Configures whether or not to open the clock gate for i4x4_mode ram.\\\\0: Open the clock gate only when application writes or reads i4x4_mode ram\\\\1: Force open the clock gate for i4x4_mode ram"]
pub type I4X4_REF_RAM_CLK_EN_R = crate::BitReader;
#[doc = "Field `I4X4_REF_RAM_CLK_EN` writer - Configures whether or not to open the clock gate for i4x4_mode ram.\\\\0: Open the clock gate only when application writes or reads i4x4_mode ram\\\\1: Force open the clock gate for i4x4_mode ram"]
pub type I4X4_REF_RAM_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IME_RAM_CLK_EN` reader - Configures whether or not to open the clock gate for ime ram.\\\\0: Open the clock gate only when application writes or reads ime ram\\\\1: Force open the clock gate for ime ram"]
pub type IME_RAM_CLK_EN_R = crate::BitReader;
#[doc = "Field `IME_RAM_CLK_EN` writer - Configures whether or not to open the clock gate for ime ram.\\\\0: Open the clock gate only when application writes or reads ime ram\\\\1: Force open the clock gate for ime ram"]
pub type IME_RAM_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FME_RAM_CLK_EN` reader - Configures whether or not to open the clock gate for fme ram.\\\\0: Open the clock gate only when application writes or readsfme ram\\\\1: Force open the clock gate for fme ram"]
pub type FME_RAM_CLK_EN_R = crate::BitReader;
#[doc = "Field `FME_RAM_CLK_EN` writer - Configures whether or not to open the clock gate for fme ram.\\\\0: Open the clock gate only when application writes or readsfme ram\\\\1: Force open the clock gate for fme ram"]
pub type FME_RAM_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FETCH_RAM_CLK_EN` reader - Configures whether or not to open the clock gate for fetch ram.\\\\0: Open the clock gate only when application writes or reads fetch ram\\\\1: Force open the clock gate for fetch ram"]
pub type FETCH_RAM_CLK_EN_R = crate::BitReader;
#[doc = "Field `FETCH_RAM_CLK_EN` writer - Configures whether or not to open the clock gate for fetch ram.\\\\0: Open the clock gate only when application writes or reads fetch ram\\\\1: Force open the clock gate for fetch ram"]
pub type FETCH_RAM_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DB_RAM_CLK_EN` reader - Configures whether or not to open the clock gate for db ram.\\\\0: Open the clock gate only when application writes or reads db ram\\\\1: Force open the clock gate for db ram"]
pub type DB_RAM_CLK_EN_R = crate::BitReader;
#[doc = "Field `DB_RAM_CLK_EN` writer - Configures whether or not to open the clock gate for db ram.\\\\0: Open the clock gate only when application writes or reads db ram\\\\1: Force open the clock gate for db ram"]
pub type DB_RAM_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CUR_MB_RAM_CLK_EN` reader - Configures whether or not to open the clock gate for cur_mb ram.\\\\0: Open the clock gate only when application writes or reads cur_mb ram\\\\1: Force open the clock gate for cur_mb ram"]
pub type CUR_MB_RAM_CLK_EN_R = crate::BitReader;
#[doc = "Field `CUR_MB_RAM_CLK_EN` writer - Configures whether or not to open the clock gate for cur_mb ram.\\\\0: Open the clock gate only when application writes or reads cur_mb ram\\\\1: Force open the clock gate for cur_mb ram"]
pub type CUR_MB_RAM_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAVLC_RAM_CLK_EN` reader - Configures whether or not to open the clock gate for cavlc ram.\\\\0: Open the clock gate only when application writes or reads cavlc ram\\\\1: Force open the clock gate for cavlc ram"]
pub type CAVLC_RAM_CLK_EN_R = crate::BitReader;
#[doc = "Field `CAVLC_RAM_CLK_EN` writer - Configures whether or not to open the clock gate for cavlc ram.\\\\0: Open the clock gate only when application writes or reads cavlc ram\\\\1: Force open the clock gate for cavlc ram"]
pub type CAVLC_RAM_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IME_CLK_EN` reader - Configures whether or not to open the clock gate for ime.\\\\0: Open the clock gate only when ime work\\\\1: Force open the clock gate for ime"]
pub type IME_CLK_EN_R = crate::BitReader;
#[doc = "Field `IME_CLK_EN` writer - Configures whether or not to open the clock gate for ime.\\\\0: Open the clock gate only when ime work\\\\1: Force open the clock gate for ime"]
pub type IME_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FME_CLK_EN` reader - Configures whether or not to open the clock gate for fme.\\\\0: Open the clock gate only when fme work\\\\1: Force open the clock gate for fme"]
pub type FME_CLK_EN_R = crate::BitReader;
#[doc = "Field `FME_CLK_EN` writer - Configures whether or not to open the clock gate for fme.\\\\0: Open the clock gate only when fme work\\\\1: Force open the clock gate for fme"]
pub type FME_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MC_CLK_EN` reader - Configures whether or not to open the clock gate for mc.\\\\0: Open the clock gate only when mc work\\\\1: Force open the clock gate for mc"]
pub type MC_CLK_EN_R = crate::BitReader;
#[doc = "Field `MC_CLK_EN` writer - Configures whether or not to open the clock gate for mc.\\\\0: Open the clock gate only when mc work\\\\1: Force open the clock gate for mc"]
pub type MC_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTERPOLATOR_CLK_EN` reader - Configures whether or not to open the clock gate for interpolator.\\\\0: Open the clock gate only when interpolator work\\\\1: Force open the clock gate for interpolator"]
pub type INTERPOLATOR_CLK_EN_R = crate::BitReader;
#[doc = "Field `INTERPOLATOR_CLK_EN` writer - Configures whether or not to open the clock gate for interpolator.\\\\0: Open the clock gate only when interpolator work\\\\1: Force open the clock gate for interpolator"]
pub type INTERPOLATOR_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DB_CLK_EN` reader - Configures whether or not to open the clock gate for deblocking filter.\\\\0: Open the clock gate only when deblocking filter work\\\\1: Force open the clock gate for deblocking filter"]
pub type DB_CLK_EN_R = crate::BitReader;
#[doc = "Field `DB_CLK_EN` writer - Configures whether or not to open the clock gate for deblocking filter.\\\\0: Open the clock gate only when deblocking filter work\\\\1: Force open the clock gate for deblocking filter"]
pub type DB_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLAVLC_CLK_EN` reader - Configures whether or not to open the clock gate for cavlc.\\\\0: Open the clock gate only when cavlc work\\\\1: Force open the clock gate for cavlc"]
pub type CLAVLC_CLK_EN_R = crate::BitReader;
#[doc = "Field `CLAVLC_CLK_EN` writer - Configures whether or not to open the clock gate for cavlc.\\\\0: Open the clock gate only when cavlc work\\\\1: Force open the clock gate for cavlc"]
pub type CLAVLC_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTRA_CLK_EN` reader - Configures whether or not to open the clock gate for intra.\\\\0: Open the clock gate only when intra work\\\\1: Force open the clock gate for intra"]
pub type INTRA_CLK_EN_R = crate::BitReader;
#[doc = "Field `INTRA_CLK_EN` writer - Configures whether or not to open the clock gate for intra.\\\\0: Open the clock gate only when intra work\\\\1: Force open the clock gate for intra"]
pub type INTRA_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DECI_CLK_EN` reader - Configures whether or not to open the clock gate for decimate.\\\\0: Open the clock gate only when decimate work\\\\1: Force open the clock gate for decimate"]
pub type DECI_CLK_EN_R = crate::BitReader;
#[doc = "Field `DECI_CLK_EN` writer - Configures whether or not to open the clock gate for decimate.\\\\0: Open the clock gate only when decimate work\\\\1: Force open the clock gate for decimate"]
pub type DECI_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BS_CLK_EN` reader - Configures whether or not to open the clock gate for bs buffer.\\\\0: Open the clock gate only when bs buffer work\\\\1: Force open the clock gate for bs buffer"]
pub type BS_CLK_EN_R = crate::BitReader;
#[doc = "Field `BS_CLK_EN` writer - Configures whether or not to open the clock gate for bs buffer.\\\\0: Open the clock gate only when bs buffer work\\\\1: Force open the clock gate for bs buffer"]
pub type BS_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MV_MERGE_CLK_EN` reader - Configures whether or not to open the clock gate for mv merge.\\\\0: Open the clock gate only when mv merge work\\\\1: Force open the clock gate for mv merge"]
pub type MV_MERGE_CLK_EN_R = crate::BitReader;
#[doc = "Field `MV_MERGE_CLK_EN` writer - Configures whether or not to open the clock gate for mv merge.\\\\0: Open the clock gate only when mv merge work\\\\1: Force open the clock gate for mv merge"]
pub type MV_MERGE_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Configures whether or not to open register clock gate.\\\\0: Open the clock gate only when application writes registers\\\\1: Force open the clock gate for register"]
    #[inline(always)]
    pub fn clk_en(&self) -> CLK_EN_R {
        CLK_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Configures whether or not to open the clock gate for rec ram2.\\\\0: Open the clock gate only when application writes or reads rec ram2\\\\1: Force open the clock gate for rec ram2"]
    #[inline(always)]
    pub fn rec_ram_clk_en2(&self) -> REC_RAM_CLK_EN2_R {
        REC_RAM_CLK_EN2_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Configures whether or not to open the clock gate for rec ram1.\\\\0: Open the clock gate only when application writes or reads rec ram1\\\\1: Force open the clock gate for rec ram1"]
    #[inline(always)]
    pub fn rec_ram_clk_en1(&self) -> REC_RAM_CLK_EN1_R {
        REC_RAM_CLK_EN1_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Configures whether or not to open the clock gate for quant ram2.\\\\0: Open the clock gate only when application writes or reads quant ram2\\\\1: Force open the clock gate for quant ram2"]
    #[inline(always)]
    pub fn quant_ram_clk_en2(&self) -> QUANT_RAM_CLK_EN2_R {
        QUANT_RAM_CLK_EN2_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Configures whether or not to open the clock gate for quant ram1.\\\\0: Open the clock gate only when application writes or reads quant ram1\\\\1: Force open the clock gate for quant ram1"]
    #[inline(always)]
    pub fn quant_ram_clk_en1(&self) -> QUANT_RAM_CLK_EN1_R {
        QUANT_RAM_CLK_EN1_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Configures whether or not to open the clock gate for pre ram.\\\\0: Open the clock gate only when application writes or reads pre ram\\\\1: Force open the clock gate for pre ram"]
    #[inline(always)]
    pub fn pre_ram_clk_en(&self) -> PRE_RAM_CLK_EN_R {
        PRE_RAM_CLK_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Configures whether or not to open the clock gate for mvd ram.\\\\0: Open the clock gate only when application writes or reads mvd ram\\\\1: Force open the clock gate for mvd ram"]
    #[inline(always)]
    pub fn mvd_ram_clk_en(&self) -> MVD_RAM_CLK_EN_R {
        MVD_RAM_CLK_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Configures whether or not to open the clock gate for mc ram.\\\\0: Open the clock gate only when application writes or reads mc ram\\\\1: Force open the clock gate for mc ram"]
    #[inline(always)]
    pub fn mc_ram_clk_en(&self) -> MC_RAM_CLK_EN_R {
        MC_RAM_CLK_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Configures whether or not to open the clock gate for ref ram.\\\\0: Open the clock gate only when application writes or reads ref ram\\\\1: Force open the clock gate for ref ram"]
    #[inline(always)]
    pub fn ref_ram_clk_en(&self) -> REF_RAM_CLK_EN_R {
        REF_RAM_CLK_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Configures whether or not to open the clock gate for i4x4_mode ram.\\\\0: Open the clock gate only when application writes or reads i4x4_mode ram\\\\1: Force open the clock gate for i4x4_mode ram"]
    #[inline(always)]
    pub fn i4x4_ref_ram_clk_en(&self) -> I4X4_REF_RAM_CLK_EN_R {
        I4X4_REF_RAM_CLK_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Configures whether or not to open the clock gate for ime ram.\\\\0: Open the clock gate only when application writes or reads ime ram\\\\1: Force open the clock gate for ime ram"]
    #[inline(always)]
    pub fn ime_ram_clk_en(&self) -> IME_RAM_CLK_EN_R {
        IME_RAM_CLK_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Configures whether or not to open the clock gate for fme ram.\\\\0: Open the clock gate only when application writes or readsfme ram\\\\1: Force open the clock gate for fme ram"]
    #[inline(always)]
    pub fn fme_ram_clk_en(&self) -> FME_RAM_CLK_EN_R {
        FME_RAM_CLK_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Configures whether or not to open the clock gate for fetch ram.\\\\0: Open the clock gate only when application writes or reads fetch ram\\\\1: Force open the clock gate for fetch ram"]
    #[inline(always)]
    pub fn fetch_ram_clk_en(&self) -> FETCH_RAM_CLK_EN_R {
        FETCH_RAM_CLK_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Configures whether or not to open the clock gate for db ram.\\\\0: Open the clock gate only when application writes or reads db ram\\\\1: Force open the clock gate for db ram"]
    #[inline(always)]
    pub fn db_ram_clk_en(&self) -> DB_RAM_CLK_EN_R {
        DB_RAM_CLK_EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Configures whether or not to open the clock gate for cur_mb ram.\\\\0: Open the clock gate only when application writes or reads cur_mb ram\\\\1: Force open the clock gate for cur_mb ram"]
    #[inline(always)]
    pub fn cur_mb_ram_clk_en(&self) -> CUR_MB_RAM_CLK_EN_R {
        CUR_MB_RAM_CLK_EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Configures whether or not to open the clock gate for cavlc ram.\\\\0: Open the clock gate only when application writes or reads cavlc ram\\\\1: Force open the clock gate for cavlc ram"]
    #[inline(always)]
    pub fn cavlc_ram_clk_en(&self) -> CAVLC_RAM_CLK_EN_R {
        CAVLC_RAM_CLK_EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Configures whether or not to open the clock gate for ime.\\\\0: Open the clock gate only when ime work\\\\1: Force open the clock gate for ime"]
    #[inline(always)]
    pub fn ime_clk_en(&self) -> IME_CLK_EN_R {
        IME_CLK_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Configures whether or not to open the clock gate for fme.\\\\0: Open the clock gate only when fme work\\\\1: Force open the clock gate for fme"]
    #[inline(always)]
    pub fn fme_clk_en(&self) -> FME_CLK_EN_R {
        FME_CLK_EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Configures whether or not to open the clock gate for mc.\\\\0: Open the clock gate only when mc work\\\\1: Force open the clock gate for mc"]
    #[inline(always)]
    pub fn mc_clk_en(&self) -> MC_CLK_EN_R {
        MC_CLK_EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Configures whether or not to open the clock gate for interpolator.\\\\0: Open the clock gate only when interpolator work\\\\1: Force open the clock gate for interpolator"]
    #[inline(always)]
    pub fn interpolator_clk_en(&self) -> INTERPOLATOR_CLK_EN_R {
        INTERPOLATOR_CLK_EN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Configures whether or not to open the clock gate for deblocking filter.\\\\0: Open the clock gate only when deblocking filter work\\\\1: Force open the clock gate for deblocking filter"]
    #[inline(always)]
    pub fn db_clk_en(&self) -> DB_CLK_EN_R {
        DB_CLK_EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Configures whether or not to open the clock gate for cavlc.\\\\0: Open the clock gate only when cavlc work\\\\1: Force open the clock gate for cavlc"]
    #[inline(always)]
    pub fn clavlc_clk_en(&self) -> CLAVLC_CLK_EN_R {
        CLAVLC_CLK_EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Configures whether or not to open the clock gate for intra.\\\\0: Open the clock gate only when intra work\\\\1: Force open the clock gate for intra"]
    #[inline(always)]
    pub fn intra_clk_en(&self) -> INTRA_CLK_EN_R {
        INTRA_CLK_EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Configures whether or not to open the clock gate for decimate.\\\\0: Open the clock gate only when decimate work\\\\1: Force open the clock gate for decimate"]
    #[inline(always)]
    pub fn deci_clk_en(&self) -> DECI_CLK_EN_R {
        DECI_CLK_EN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Configures whether or not to open the clock gate for bs buffer.\\\\0: Open the clock gate only when bs buffer work\\\\1: Force open the clock gate for bs buffer"]
    #[inline(always)]
    pub fn bs_clk_en(&self) -> BS_CLK_EN_R {
        BS_CLK_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Configures whether or not to open the clock gate for mv merge.\\\\0: Open the clock gate only when mv merge work\\\\1: Force open the clock gate for mv merge"]
    #[inline(always)]
    pub fn mv_merge_clk_en(&self) -> MV_MERGE_CLK_EN_R {
        MV_MERGE_CLK_EN_R::new(((self.bits >> 25) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONF")
            .field("clk_en", &self.clk_en().bit())
            .field("rec_ram_clk_en2", &self.rec_ram_clk_en2().bit())
            .field("rec_ram_clk_en1", &self.rec_ram_clk_en1().bit())
            .field("quant_ram_clk_en2", &self.quant_ram_clk_en2().bit())
            .field("quant_ram_clk_en1", &self.quant_ram_clk_en1().bit())
            .field("pre_ram_clk_en", &self.pre_ram_clk_en().bit())
            .field("mvd_ram_clk_en", &self.mvd_ram_clk_en().bit())
            .field("mc_ram_clk_en", &self.mc_ram_clk_en().bit())
            .field("ref_ram_clk_en", &self.ref_ram_clk_en().bit())
            .field("i4x4_ref_ram_clk_en", &self.i4x4_ref_ram_clk_en().bit())
            .field("ime_ram_clk_en", &self.ime_ram_clk_en().bit())
            .field("fme_ram_clk_en", &self.fme_ram_clk_en().bit())
            .field("fetch_ram_clk_en", &self.fetch_ram_clk_en().bit())
            .field("db_ram_clk_en", &self.db_ram_clk_en().bit())
            .field("cur_mb_ram_clk_en", &self.cur_mb_ram_clk_en().bit())
            .field("cavlc_ram_clk_en", &self.cavlc_ram_clk_en().bit())
            .field("ime_clk_en", &self.ime_clk_en().bit())
            .field("fme_clk_en", &self.fme_clk_en().bit())
            .field("mc_clk_en", &self.mc_clk_en().bit())
            .field("interpolator_clk_en", &self.interpolator_clk_en().bit())
            .field("db_clk_en", &self.db_clk_en().bit())
            .field("clavlc_clk_en", &self.clavlc_clk_en().bit())
            .field("intra_clk_en", &self.intra_clk_en().bit())
            .field("deci_clk_en", &self.deci_clk_en().bit())
            .field("bs_clk_en", &self.bs_clk_en().bit())
            .field("mv_merge_clk_en", &self.mv_merge_clk_en().bit())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - Configures whether or not to open register clock gate.\\\\0: Open the clock gate only when application writes registers\\\\1: Force open the clock gate for register"]
    #[inline(always)]
    #[must_use]
    pub fn clk_en(&mut self) -> CLK_EN_W<CONF_SPEC> {
        CLK_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Configures whether or not to open the clock gate for rec ram2.\\\\0: Open the clock gate only when application writes or reads rec ram2\\\\1: Force open the clock gate for rec ram2"]
    #[inline(always)]
    #[must_use]
    pub fn rec_ram_clk_en2(&mut self) -> REC_RAM_CLK_EN2_W<CONF_SPEC> {
        REC_RAM_CLK_EN2_W::new(self, 1)
    }
    #[doc = "Bit 2 - Configures whether or not to open the clock gate for rec ram1.\\\\0: Open the clock gate only when application writes or reads rec ram1\\\\1: Force open the clock gate for rec ram1"]
    #[inline(always)]
    #[must_use]
    pub fn rec_ram_clk_en1(&mut self) -> REC_RAM_CLK_EN1_W<CONF_SPEC> {
        REC_RAM_CLK_EN1_W::new(self, 2)
    }
    #[doc = "Bit 3 - Configures whether or not to open the clock gate for quant ram2.\\\\0: Open the clock gate only when application writes or reads quant ram2\\\\1: Force open the clock gate for quant ram2"]
    #[inline(always)]
    #[must_use]
    pub fn quant_ram_clk_en2(&mut self) -> QUANT_RAM_CLK_EN2_W<CONF_SPEC> {
        QUANT_RAM_CLK_EN2_W::new(self, 3)
    }
    #[doc = "Bit 4 - Configures whether or not to open the clock gate for quant ram1.\\\\0: Open the clock gate only when application writes or reads quant ram1\\\\1: Force open the clock gate for quant ram1"]
    #[inline(always)]
    #[must_use]
    pub fn quant_ram_clk_en1(&mut self) -> QUANT_RAM_CLK_EN1_W<CONF_SPEC> {
        QUANT_RAM_CLK_EN1_W::new(self, 4)
    }
    #[doc = "Bit 5 - Configures whether or not to open the clock gate for pre ram.\\\\0: Open the clock gate only when application writes or reads pre ram\\\\1: Force open the clock gate for pre ram"]
    #[inline(always)]
    #[must_use]
    pub fn pre_ram_clk_en(&mut self) -> PRE_RAM_CLK_EN_W<CONF_SPEC> {
        PRE_RAM_CLK_EN_W::new(self, 5)
    }
    #[doc = "Bit 6 - Configures whether or not to open the clock gate for mvd ram.\\\\0: Open the clock gate only when application writes or reads mvd ram\\\\1: Force open the clock gate for mvd ram"]
    #[inline(always)]
    #[must_use]
    pub fn mvd_ram_clk_en(&mut self) -> MVD_RAM_CLK_EN_W<CONF_SPEC> {
        MVD_RAM_CLK_EN_W::new(self, 6)
    }
    #[doc = "Bit 7 - Configures whether or not to open the clock gate for mc ram.\\\\0: Open the clock gate only when application writes or reads mc ram\\\\1: Force open the clock gate for mc ram"]
    #[inline(always)]
    #[must_use]
    pub fn mc_ram_clk_en(&mut self) -> MC_RAM_CLK_EN_W<CONF_SPEC> {
        MC_RAM_CLK_EN_W::new(self, 7)
    }
    #[doc = "Bit 8 - Configures whether or not to open the clock gate for ref ram.\\\\0: Open the clock gate only when application writes or reads ref ram\\\\1: Force open the clock gate for ref ram"]
    #[inline(always)]
    #[must_use]
    pub fn ref_ram_clk_en(&mut self) -> REF_RAM_CLK_EN_W<CONF_SPEC> {
        REF_RAM_CLK_EN_W::new(self, 8)
    }
    #[doc = "Bit 9 - Configures whether or not to open the clock gate for i4x4_mode ram.\\\\0: Open the clock gate only when application writes or reads i4x4_mode ram\\\\1: Force open the clock gate for i4x4_mode ram"]
    #[inline(always)]
    #[must_use]
    pub fn i4x4_ref_ram_clk_en(&mut self) -> I4X4_REF_RAM_CLK_EN_W<CONF_SPEC> {
        I4X4_REF_RAM_CLK_EN_W::new(self, 9)
    }
    #[doc = "Bit 10 - Configures whether or not to open the clock gate for ime ram.\\\\0: Open the clock gate only when application writes or reads ime ram\\\\1: Force open the clock gate for ime ram"]
    #[inline(always)]
    #[must_use]
    pub fn ime_ram_clk_en(&mut self) -> IME_RAM_CLK_EN_W<CONF_SPEC> {
        IME_RAM_CLK_EN_W::new(self, 10)
    }
    #[doc = "Bit 11 - Configures whether or not to open the clock gate for fme ram.\\\\0: Open the clock gate only when application writes or readsfme ram\\\\1: Force open the clock gate for fme ram"]
    #[inline(always)]
    #[must_use]
    pub fn fme_ram_clk_en(&mut self) -> FME_RAM_CLK_EN_W<CONF_SPEC> {
        FME_RAM_CLK_EN_W::new(self, 11)
    }
    #[doc = "Bit 12 - Configures whether or not to open the clock gate for fetch ram.\\\\0: Open the clock gate only when application writes or reads fetch ram\\\\1: Force open the clock gate for fetch ram"]
    #[inline(always)]
    #[must_use]
    pub fn fetch_ram_clk_en(&mut self) -> FETCH_RAM_CLK_EN_W<CONF_SPEC> {
        FETCH_RAM_CLK_EN_W::new(self, 12)
    }
    #[doc = "Bit 13 - Configures whether or not to open the clock gate for db ram.\\\\0: Open the clock gate only when application writes or reads db ram\\\\1: Force open the clock gate for db ram"]
    #[inline(always)]
    #[must_use]
    pub fn db_ram_clk_en(&mut self) -> DB_RAM_CLK_EN_W<CONF_SPEC> {
        DB_RAM_CLK_EN_W::new(self, 13)
    }
    #[doc = "Bit 14 - Configures whether or not to open the clock gate for cur_mb ram.\\\\0: Open the clock gate only when application writes or reads cur_mb ram\\\\1: Force open the clock gate for cur_mb ram"]
    #[inline(always)]
    #[must_use]
    pub fn cur_mb_ram_clk_en(&mut self) -> CUR_MB_RAM_CLK_EN_W<CONF_SPEC> {
        CUR_MB_RAM_CLK_EN_W::new(self, 14)
    }
    #[doc = "Bit 15 - Configures whether or not to open the clock gate for cavlc ram.\\\\0: Open the clock gate only when application writes or reads cavlc ram\\\\1: Force open the clock gate for cavlc ram"]
    #[inline(always)]
    #[must_use]
    pub fn cavlc_ram_clk_en(&mut self) -> CAVLC_RAM_CLK_EN_W<CONF_SPEC> {
        CAVLC_RAM_CLK_EN_W::new(self, 15)
    }
    #[doc = "Bit 16 - Configures whether or not to open the clock gate for ime.\\\\0: Open the clock gate only when ime work\\\\1: Force open the clock gate for ime"]
    #[inline(always)]
    #[must_use]
    pub fn ime_clk_en(&mut self) -> IME_CLK_EN_W<CONF_SPEC> {
        IME_CLK_EN_W::new(self, 16)
    }
    #[doc = "Bit 17 - Configures whether or not to open the clock gate for fme.\\\\0: Open the clock gate only when fme work\\\\1: Force open the clock gate for fme"]
    #[inline(always)]
    #[must_use]
    pub fn fme_clk_en(&mut self) -> FME_CLK_EN_W<CONF_SPEC> {
        FME_CLK_EN_W::new(self, 17)
    }
    #[doc = "Bit 18 - Configures whether or not to open the clock gate for mc.\\\\0: Open the clock gate only when mc work\\\\1: Force open the clock gate for mc"]
    #[inline(always)]
    #[must_use]
    pub fn mc_clk_en(&mut self) -> MC_CLK_EN_W<CONF_SPEC> {
        MC_CLK_EN_W::new(self, 18)
    }
    #[doc = "Bit 19 - Configures whether or not to open the clock gate for interpolator.\\\\0: Open the clock gate only when interpolator work\\\\1: Force open the clock gate for interpolator"]
    #[inline(always)]
    #[must_use]
    pub fn interpolator_clk_en(&mut self) -> INTERPOLATOR_CLK_EN_W<CONF_SPEC> {
        INTERPOLATOR_CLK_EN_W::new(self, 19)
    }
    #[doc = "Bit 20 - Configures whether or not to open the clock gate for deblocking filter.\\\\0: Open the clock gate only when deblocking filter work\\\\1: Force open the clock gate for deblocking filter"]
    #[inline(always)]
    #[must_use]
    pub fn db_clk_en(&mut self) -> DB_CLK_EN_W<CONF_SPEC> {
        DB_CLK_EN_W::new(self, 20)
    }
    #[doc = "Bit 21 - Configures whether or not to open the clock gate for cavlc.\\\\0: Open the clock gate only when cavlc work\\\\1: Force open the clock gate for cavlc"]
    #[inline(always)]
    #[must_use]
    pub fn clavlc_clk_en(&mut self) -> CLAVLC_CLK_EN_W<CONF_SPEC> {
        CLAVLC_CLK_EN_W::new(self, 21)
    }
    #[doc = "Bit 22 - Configures whether or not to open the clock gate for intra.\\\\0: Open the clock gate only when intra work\\\\1: Force open the clock gate for intra"]
    #[inline(always)]
    #[must_use]
    pub fn intra_clk_en(&mut self) -> INTRA_CLK_EN_W<CONF_SPEC> {
        INTRA_CLK_EN_W::new(self, 22)
    }
    #[doc = "Bit 23 - Configures whether or not to open the clock gate for decimate.\\\\0: Open the clock gate only when decimate work\\\\1: Force open the clock gate for decimate"]
    #[inline(always)]
    #[must_use]
    pub fn deci_clk_en(&mut self) -> DECI_CLK_EN_W<CONF_SPEC> {
        DECI_CLK_EN_W::new(self, 23)
    }
    #[doc = "Bit 24 - Configures whether or not to open the clock gate for bs buffer.\\\\0: Open the clock gate only when bs buffer work\\\\1: Force open the clock gate for bs buffer"]
    #[inline(always)]
    #[must_use]
    pub fn bs_clk_en(&mut self) -> BS_CLK_EN_W<CONF_SPEC> {
        BS_CLK_EN_W::new(self, 24)
    }
    #[doc = "Bit 25 - Configures whether or not to open the clock gate for mv merge.\\\\0: Open the clock gate only when mv merge work\\\\1: Force open the clock gate for mv merge"]
    #[inline(always)]
    #[must_use]
    pub fn mv_merge_clk_en(&mut self) -> MV_MERGE_CLK_EN_W<CONF_SPEC> {
        MV_MERGE_CLK_EN_W::new(self, 25)
    }
}
#[doc = "General configuration register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CONF_SPEC;
impl crate::RegisterSpec for CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`conf::R`](R) reader structure"]
impl crate::Readable for CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`conf::W`](W) writer structure"]
impl crate::Writable for CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CONF to value 0"]
impl crate::Resettable for CONF_SPEC {
    const RESET_VALUE: u32 = 0;
}
